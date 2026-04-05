//! Ergonomic macros for building and extracting NGAP protocol IEs.
//!
//! # Builder macros
//!
//! ## `build_ngap!` — build a complete NGAP PDU
//!
//! IE IDs are auto-derived from the variant suffix via `__ngap_ie_id!` (generated
//! by the build script from `#[asn(key = N)]` attributes in the auto-generated code).
//! Values are auto-converted via `.into()`: raw values (e.g. `u64`) are converted
//! to newtypes, already-correct types use the identity `From<T> for T`.
//!
//! ```ignore
//! use oxirush_ngap::{build_ngap, ngap::*};
//!
//! let pdu = build_ngap!(SuccessfulOutcome, InitialContextSetup,
//!     REJECT, InitialContextSetupResponse,
//!     IGNORE AMF_UE_NGAP_ID(1u64),
//!     IGNORE RAN_UE_NGAP_ID(0u32),
//! );
//! ```
//!
//! Arguments:
//! - `$direction` — `InitiatingMessage`, `SuccessfulOutcome`, or `UnsuccessfulOutcome`
//! - `$proc` — procedure suffix (e.g. `InitialContextSetup`) — auto-derives `Id_` variant and procedure code
//! - `$outer_crit` — outer criticality: `REJECT`, `IGNORE`, or `NOTIFY`
//! - `$msg` — the message struct name (e.g. `InitialContextSetupResponse`)
//! - IEs: `$ie_crit $ie_name($ie_value)` — IE name derives both `Id_` variant and IE ID
//!
//! ## `build_ngap_ie!` — build a single Protocol IE entry
//!
//! Useful when you need to conditionally include IEs or build them separately.
//!
//! ```ignore
//! use oxirush_ngap::{build_ngap_ie, ngap::*};
//!
//! let ie = build_ngap_ie!(NGSetupResponse, REJECT AMFName("test".to_string()));
//! ```
//!
//! # Extraction macro
//!
//! ## `extract_ngap_ies!` — extract IEs from a decoded NGAP message
//!
//! Iterates over a `&[ProtocolIEs_Entry]` slice, pattern-matching each IE's
//! `EntryValue` variant. Required fields are unwrapped; if any required field
//! is missing, returns `Err(MissingIeError)` from the enclosing function.
//! Optional fields stay as `Option<T>`.
//!
//! The enclosing function must return `Result<_, MissingIeError>` (or a type
//! that implements `From<MissingIeError>`).
//!
//! When `=> expr` is omitted, defaults to `binding.0` (newtype unwrap).
//!
//! ```ignore
//! use oxirush_ngap::{extract_ngap_ies, ngap::*, macros::MissingIeError};
//!
//! fn handle(msg: &UplinkNASTransport) -> Result<Vec<String>, MissingIeError> {
//!     extract_ngap_ies!(msg, UplinkNASTransport,
//!         req amf_id:  u64      = AMF_UE_NGAP_ID(id),              // default .0
//!         req nas_pdu: Vec<u8>  = NAS_PDU(pdu) => pdu.0.clone(),   // custom expr
//!         opt cause:   String   = Cause(c) => format!("{c:?}"),     // optional
//!     );
//!     // amf_id: u64, nas_pdu: Vec<u8>, cause: Option<String>
//!     Ok(vec![])
//! }
//! ```
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
    ($msg_var:expr, $msg:ident,
        $($kind:ident $name:ident : $ty:ty = $variant:ident ($bind:ident) $(=> $expr:expr)?),+
        $(,)?
    ) => {
        paste::paste! {
            $( let mut $name : Option<$ty> = None; )+
            for _ie in &$msg_var.protocol_i_es.0 {
                match &_ie.value {
                    $( [< $msg ProtocolIEs_EntryValue >] :: [< Id_ $variant >] ( $bind ) => {
                        $name = Some($crate::extract_ngap_ies!(@val $bind $(, $expr)?));
                    } )+
                    _ => {}
                }
            }
            $( $crate::extract_ngap_ies!(@check $kind $name); )+
        }
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
    ($direction:ident, $proc:ident,
     $outer_crit:ident, $msg:ident,
     $($ie_crit:ident $ie_name:ident ($($ie_value:tt)+)),*
     $(,)?
    ) => {
        paste::paste! {
            {
                let ies = vec![
                    $( [< $msg ProtocolIEs_Entry >] {
                        id: $crate::ngap::ProtocolIE_ID($crate::__ngap_ie_id!($ie_name)),
                        criticality: $crate::ngap::Criticality($crate::ngap::Criticality::$ie_crit),
                        value: [< $msg ProtocolIEs_EntryValue >]::[< Id_ $ie_name >](($($ie_value)+).into()),
                    }, )*
                ];
                $crate::ngap::NGAP_PDU::$direction($crate::ngap::$direction {
                    procedure_code: $crate::ngap::ProcedureCode($crate::__ngap_proc_code!($proc)),
                    criticality: $crate::ngap::Criticality($crate::ngap::Criticality::$outer_crit),
                    value: [< $direction Value >]::[< Id_ $proc >]($msg {
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
    ($msg:ident, $crit:ident $ie_name:ident ($($value:tt)+)) => {
        paste::paste! {
            [< $msg ProtocolIEs_Entry >] {
                id: $crate::ngap::ProtocolIE_ID($crate::__ngap_ie_id!($ie_name)),
                criticality: $crate::ngap::Criticality($crate::ngap::Criticality::$crit),
                value: [< $msg ProtocolIEs_EntryValue >]::[< Id_ $ie_name >](($($value)+).into()),
            }
        }
    };
}
