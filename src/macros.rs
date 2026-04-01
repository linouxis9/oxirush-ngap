/// Ergonomic macros for building and extracting NGAP protocol IEs.
///
/// # Builder macros
///
/// ## `build_ngap!` — build a complete NGAP PDU
///
/// ```ignore
/// use oxirush_ngap::{build_ngap, ngap::*, macros::*};
///
/// let pdu = build_ngap!(SuccessfulOutcome, Id_InitialContextSetup,
///     PROC_INITIAL_CONTEXT_SETUP, REJECT, InitialContextSetupResponse,
///     IGNORE IE_AMF_UE_NGAP_ID => Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID(1)),
///     IGNORE IE_RAN_UE_NGAP_ID => Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID(0)),
/// );
/// ```
///
/// Arguments:
/// - `$direction` — `InitiatingMessage`, `SuccessfulOutcome`, or `UnsuccessfulOutcome`
/// - `$outer_variant` — the variant inside `{Direction}Value` (e.g. `Id_NGSetup`)
/// - `$proc_code` — NGAP procedure code (u8)
/// - `$outer_crit` — outer criticality: `REJECT`, `IGNORE`, or `NOTIFY`
/// - `$msg` — the message struct name (e.g. `NGSetupResponse`)
/// - IEs: `$ie_crit $ie_id => $ie_variant($ie_value)`
///
/// ## `build_ngap_ie!` — build a single Protocol IE entry
///
/// Useful when you need to conditionally include IEs or build them separately.
///
/// ```ignore
/// use oxirush_ngap::{build_ngap_ie, ngap::*, macros::*};
///
/// let ie = build_ngap_ie!(NGSetupResponse, REJECT IE_AMF_UE_NGAP_ID =>
///     Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID(1))
/// );
/// ```
///
/// # Extraction macro
///
/// ## `extract_ngap_ies!` — extract IEs from a decoded NGAP message
///
/// Iterates over a `&[ProtocolIEs_Entry]` slice, pattern-matching each IE's
/// `EntryValue` variant. Required fields are unwrapped; if any required field
/// is missing, returns `Err(MissingIeError)` from the enclosing function.
/// Optional fields stay as `Option<T>`.
///
/// The enclosing function must return `Result<_, MissingIeError>` (or a type
/// that implements `From<MissingIeError>`).
///
/// When `=> expr` is omitted, defaults to `binding.0` (newtype unwrap).
///
/// ```ignore
/// use oxirush_ngap::{extract_ngap_ies, ngap::*, macros::MissingIeError};
///
/// fn handle(msg: &UplinkNASTransport) -> Result<Vec<String>, MissingIeError> {
///     extract_ngap_ies!(&msg.protocol_i_es.0, UplinkNASTransportProtocolIEs_EntryValue,
///         req amf_id:  u64      = Id_AMF_UE_NGAP_ID(id),              // default .0
///         req nas_pdu: Vec<u8>  = Id_NAS_PDU(pdu) => pdu.0.clone(),   // custom expr
///         opt cause:   String   = Id_Cause(c) => format!("{c:?}"),     // optional
///     );
///     // amf_id: u64, nas_pdu: Vec<u8>, cause: Option<String>
///     Ok(vec![])
/// }
/// ```
use core::fmt;

/// Error returned by [`extract_ngap_ies!`] when a required IE is missing.
#[derive(Debug, Clone)]
pub struct MissingIeError {
    /// Name of the missing required IE field (as written in the macro invocation).
    pub ie_name: &'static str,
}

impl fmt::Display for MissingIeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "missing required NGAP IE: {}", self.ie_name)
    }
}

impl std::error::Error for MissingIeError {}

/// Extract NGAP protocol IEs with `req` (required) and `opt` (optional) fields.
///
/// See [module-level documentation](self) for usage.
#[macro_export]
macro_rules! extract_ngap_ies {
    ($ies:expr, $prefix:ident,
        $($kind:ident $name:ident : $ty:ty = $variant:ident ($bind:ident) $(=> $expr:expr)?),+
        $(,)?
    ) => {
        $( let mut $name : Option<$ty> = None; )+
        for _ie in $ies {
            match &_ie.value {
                $( $prefix :: $variant ( $bind ) => {
                    $name = Some($crate::extract_ngap_ies!(@val $bind $(, $expr)?));
                } )+
                _ => {}
            }
        }
        $( $crate::extract_ngap_ies!(@check $kind $name); )+
    };
    // Default: newtype .0
    (@val $bind:ident) => { $bind.0 };
    // Custom expression
    (@val $_bind:ident, $expr:expr) => { $expr };
    // Required: unwrap or return Err
    (@check req $name:ident) => {
        let $name = match $name {
            Some(_v) => _v,
            None => {
                return Err($crate::macros::MissingIeError {
                    ie_name: stringify!($name),
                });
            }
        };
    };
    // Optional: no-op (stays as Option<T>)
    (@check opt $name:ident) => {};
}

/// Build a complete `NGAP_PDU` from a direction, procedure code, message type, and IEs.
///
/// See [module-level documentation](self) for usage.
#[macro_export]
macro_rules! build_ngap {
    ($direction:ident, $outer_variant:ident,
     $proc_code:expr, $outer_crit:ident, $msg:ident,
     $($ie_crit:ident $ie_id:expr => $ie_variant:ident ($($ie_value:tt)+)),*
     $(,)?
    ) => {
        paste::paste! {
            {
                let ies = vec![
                    $( [< $msg ProtocolIEs_Entry >] {
                        id: $crate::ngap::ProtocolIE_ID($ie_id),
                        criticality: $crate::ngap::Criticality($crate::ngap::Criticality::$ie_crit),
                        value: [< $msg ProtocolIEs_EntryValue >]::$ie_variant($($ie_value)+),
                    }, )*
                ];
                $crate::ngap::NGAP_PDU::$direction($crate::ngap::$direction {
                    procedure_code: $crate::ngap::ProcedureCode($proc_code),
                    criticality: $crate::ngap::Criticality($crate::ngap::Criticality::$outer_crit),
                    value: [< $direction Value >]::$outer_variant($msg {
                        protocol_i_es: [< $msg ProtocolIEs >](ies),
                    }),
                })
            }
        }
    };
}

/// Build a single NGAP Protocol IE entry for a given message type.
///
/// See [module-level documentation](self) for usage.
#[macro_export]
macro_rules! build_ngap_ie {
    ($msg:ident, $crit:ident $ie_id:expr => $variant:ident ($($value:tt)+)) => {
        paste::paste! {
            [< $msg ProtocolIEs_Entry >] {
                id: $crate::ngap::ProtocolIE_ID($ie_id),
                criticality: $crate::ngap::Criticality($crate::ngap::Criticality::$crit),
                value: [< $msg ProtocolIEs_EntryValue >]::$variant($($value)+),
            }
        }
    };
}

