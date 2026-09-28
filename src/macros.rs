//! Ergonomic macros for building, extracting, and mutating NGAP protocol IEs.
//!
//! `rasn-compiler` represents NGAP information-object open types as
//! [`rasn::types::Any`]. The macros keep that representation internal: callers
//! construct and receive concrete NGAP types, while values are APER-encoded into
//! and decoded from the open type at the protocol boundary.
//!
//! # Builder macros
//!
//! ## `build_ngap!` — build a complete NGAP PDU
//!
//! IE IDs and procedure codes are generated from the ASN.1 object sets. Values
//! are converted with `.into()`, so primitive values such as `u32` can be passed
//! directly for generated newtypes.
//!
//! ```ignore
//! use oxirush_ngap::{build_ngap, ngap::*};
//!
//! let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
//!     IGNORE, UEContextReleaseRequest,
//!     REJECT AMF_UE_NGAP_ID(1u64),
//!     REJECT RAN_UE_NGAP_ID(7u32),
//!     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
//! );
//! ```
//!
//! Arguments:
//! - `$direction` — `InitiatingMessage`, `SuccessfulOutcome`, or `UnsuccessfulOutcome`
//! - `$proc` — procedure name, used to derive its ASN.1 procedure code
//! - `$outer_crit` — outer criticality: `REJECT`, `IGNORE`, or `NOTIFY`
//! - `$msg` — message struct name, such as `UEContextReleaseRequest`
//! - IEs — `$ie_crit $ie_name($ie_value)`, with the IE ID derived from its name
//!
//! ## `build_ngap_ie!` — build one Protocol IE entry
//!
//! This is useful for conditionally included IEs or IEs built separately.
//!
//! ```ignore
//! use oxirush_ngap::{build_ngap_ie, ngap::*};
//!
//! let ie = build_ngap_ie!(UEContextReleaseRequest,
//!     REJECT AMF_UE_NGAP_ID(1u64)
//! );
//! ```
//!
//! An IE name is its ASN.1 identifier without `id-` and with `_` for `-`. The
//! name of a type is an alias only for the one IE of that type: the type of
//! both id-SONConfigurationTransferDL and id-SONConfigurationTransferUL is
//! addressed by the IE names.
//!
//! ```
//! use oxirush_ngap::{build_ngap_ie, ngap::*};
//!
//! fn downlink(transfer: SONConfigurationTransfer) -> AnonymousDownlinkRANConfigurationTransferProtocolIEs {
//!     build_ngap_ie!(DownlinkRANConfigurationTransfer, IGNORE SONConfigurationTransferDL(transfer))
//! }
//! ```
//!
//! ```compile_fail
//! use oxirush_ngap::{build_ngap_ie, ngap::*};
//!
//! fn downlink(transfer: SONConfigurationTransfer) -> AnonymousDownlinkRANConfigurationTransferProtocolIEs {
//!     build_ngap_ie!(DownlinkRANConfigurationTransfer, IGNORE SONConfigurationTransfer(transfer))
//! }
//! ```
//!
//! # Extraction macro
//!
//! ## `extract_ngap_ies!` — extract IEs from a decoded NGAP message
//!
//! The macro iterates through `protocol_ies`, matches generated ASN.1 IE IDs,
//! and APER-decodes each matching open type into its concrete generated type.
//! Required fields are unwrapped; a missing or invalid required field returns
//! [`MissingIeError`] from the enclosing function. Optional fields remain
//! `Option<T>`.
//!
//! The enclosing function must return `Result<_, MissingIeError>` or a type that
//! implements `From<MissingIeError>`. Without `=> expression`, extraction uses
//! `binding.0`, which unwraps the usual single-field generated newtype.
//!
//! ```ignore
//! use oxirush_ngap::{extract_ngap_ies, macros::MissingIeError, ngap::*};
//!
//! fn handle(msg: &UplinkNASTransport) -> Result<Vec<u8>, MissingIeError> {
//!     extract_ngap_ies!(msg, UplinkNASTransport,
//!         req amf_id: u64 = AMF_UE_NGAP_ID(id),
//!         req nas_pdu: Vec<u8> = NAS_PDU(pdu) => pdu.0.to_vec(),
//!         opt ran_id: u32 = RAN_UE_NGAP_ID(id),
//!     );
//!     let _ = (amf_id, ran_id);
//!     Ok(nas_pdu)
//! }
//! ```
//!
//! ## `with_ngap_ie_mut!` — locate and mutate one decoded NGAP IE
//!
//! This decodes the matching open type, passes the concrete value into the
//! expression, then re-encodes it. It returns `true` only when the IE was found,
//! decoded and re-encoded successfully, and the expression returned `true`.
//! The setter form `IeName(binding) = value` expands to `binding.0 = value`.
//!
//! ```ignore
//! use oxirush_ngap::{ngap::*, with_ngap_ie_mut};
//!
//! let updated = with_ngap_ie_mut!(message, UplinkNASTransport,
//!     AMF_UE_NGAP_ID(id) = 42u64
//! );
//! assert!(updated);
//! ```

use core::fmt;

/// Error returned by `extract_ngap_ies!` when a required IE is absent or invalid.
#[derive(Debug, Clone)]
pub struct MissingIeError {
    /// Name of the required field as written in the macro invocation.
    pub ie_name: &'static str,
}

impl fmt::Display for MissingIeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "missing or invalid required NGAP IE: {}",
            self.ie_name
        )
    }
}

impl std::error::Error for MissingIeError {}

/// Extract typed NGAP protocol IEs with `req` and `opt` semantics.
#[macro_export]
macro_rules! extract_ngap_ies {
    ($msg_var:expr, $msg:ident,
        $($kind:ident $name:ident : $ty:ty = $ie_name:ident ($bind:ident) $(=> $expr:expr)?),+
        $(,)?
    ) => {
        $( let mut $name: Option<$ty> = None; )+
        for _ie in &$msg_var.protocol_ies.0 {
            $(
                if _ie.id.0 == $crate::__ngap_ie_id!($ie_name) {
                    // `$bind` is out of scope where `$name` is assigned, as
                    // both may have the same name.
                    let _value: Option<$ty> = match $crate::__ngap_decode_ie!($ie_name, &_ie.value) {
                        Ok($bind) => Some($crate::extract_ngap_ies!(@val $bind $(, $expr)?)),
                        Err(_) => None,
                    };
                    if _value.is_some() {
                        $name = _value;
                    }
                }
            )+
        }
        $( $crate::extract_ngap_ies!(@check $kind $name); )+
    };
    (@val $bind:ident) => { $bind.0 };
    (@val $_bind:ident, $expr:expr) => { $expr };
    (@check req $name:ident) => {
        let $name = match $name {
            Some(value) => value,
            None => {
                return Err($crate::macros::MissingIeError {
                    ie_name: stringify!($name),
                });
            }
        };
    };
    (@check opt $name:ident) => {};
}

/// Locate, decode, mutate, and re-encode one NGAP protocol IE.
#[macro_export]
macro_rules! with_ngap_ie_mut {
    ($msg_var:expr, $msg:ident, $ie_name:ident($bind:ident) = $value:expr $(,)?) => {{
        $crate::with_ngap_ie_mut!($msg_var, $msg, $ie_name($bind) => {
            $bind.0 = $value;
            true
        })
    }};
    ($msg_var:expr, $msg:ident, $ie_name:ident($bind:ident) => $expr:expr $(,)?) => {{
        let mut matched = false;
        for ie in &mut $msg_var.protocol_ies.0 {
            if ie.id.0 == $crate::__ngap_ie_id!($ie_name) {
                if let Ok(mut $bind) = $crate::__ngap_decode_ie!($ie_name, &ie.value) {
                    let result = $expr;
                    if let Ok(value) = $crate::ngap::encode_open_type(&$bind) {
                        ie.value = value;
                        matched = result;
                    }
                }
                break;
            }
        }
        matched
    }};
}

/// Build a complete `NGAP_PDU` from a direction, procedure, message, and IEs.
///
/// # Panics
///
/// Panics if a value cannot be APER-encoded into its open type, such as an
/// integer outside its constraint.
#[macro_export]
macro_rules! build_ngap {
    ($direction:ident, $proc:ident,
     $outer_crit:ident, $msg:ident,
     $($ie_crit:ident $ie_name:ident ($($ie_value:tt)+)),*
     $(,)?
    ) => {
        $crate::__paste::paste! {{
            let ies = vec![
                $( $crate::ngap::[< Anonymous $msg ProtocolIEs >] {
                    id: $crate::ngap::ProtocolIEID($crate::__ngap_ie_id!($ie_name)),
                    criticality: $crate::build_ngap!(@criticality $ie_crit),
                    value: $crate::__ngap_encode_ie!($ie_name, ($($ie_value)+))
                        .expect("failed to APER-encode NGAP IE open type"),
                }, )*
            ];
            let message = $crate::ngap::$msg::new(
                $crate::ngap::[< $msg ProtocolIEs >](ies),
            );
            let value = $crate::ngap::encode_open_type(&message)
                .expect("failed to APER-encode NGAP message open type");
            $crate::build_ngap!(@pdu $direction, $proc, $outer_crit, value)
        }}
    };
    (@criticality REJECT) => { $crate::ngap::Criticality::reject };
    (@criticality IGNORE) => { $crate::ngap::Criticality::ignore };
    (@criticality NOTIFY) => { $crate::ngap::Criticality::notify };
    (@pdu InitiatingMessage, $proc:ident, $criticality:ident, $value:expr) => {
        $crate::ngap::NGAP_PDU::initiatingMessage($crate::ngap::InitiatingMessage {
            procedure_code: $crate::ngap::ProcedureCode($crate::__ngap_proc_code!($proc)),
            criticality: $crate::build_ngap!(@criticality $criticality),
            value: $value,
        })
    };
    (@pdu SuccessfulOutcome, $proc:ident, $criticality:ident, $value:expr) => {
        $crate::ngap::NGAP_PDU::successfulOutcome($crate::ngap::SuccessfulOutcome {
            procedure_code: $crate::ngap::ProcedureCode($crate::__ngap_proc_code!($proc)),
            criticality: $crate::build_ngap!(@criticality $criticality),
            value: $value,
        })
    };
    (@pdu UnsuccessfulOutcome, $proc:ident, $criticality:ident, $value:expr) => {
        $crate::ngap::NGAP_PDU::unsuccessfulOutcome($crate::ngap::UnsuccessfulOutcome {
            procedure_code: $crate::ngap::ProcedureCode($crate::__ngap_proc_code!($proc)),
            criticality: $crate::build_ngap!(@criticality $criticality),
            value: $value,
        })
    };
}

/// Build one NGAP Protocol IE entry for a message type.
///
/// # Panics
///
/// Panics if the value cannot be APER-encoded into its open type.
#[macro_export]
macro_rules! build_ngap_ie {
    ($msg:ident, $criticality:ident $ie_name:ident ($($value:tt)+)) => {
        $crate::__paste::paste! {
            $crate::ngap::[< Anonymous $msg ProtocolIEs >] {
                id: $crate::ngap::ProtocolIEID($crate::__ngap_ie_id!($ie_name)),
                criticality: $crate::build_ngap!(@criticality $criticality),
                value: $crate::__ngap_encode_ie!($ie_name, ($($value)+))
                    .expect("failed to APER-encode NGAP IE open type"),
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::ngap::*;

    #[test]
    fn mutating_macro_updates_newtype_payload() {
        let mut pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
            IGNORE, UEContextReleaseRequest,
            REJECT AMF_UE_NGAP_ID(1u64),
            REJECT RAN_UE_NGAP_ID(7u32),
            IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
        );

        let NGAP_PDU::initiatingMessage(message) = &mut pdu else {
            panic!("expected initiating message");
        };
        let request: UEContextReleaseRequest =
            rasn::aper::decode(message.value.as_bytes()).expect("decode request");
        let mut request = request;

        assert!(with_ngap_ie_mut!(
            request,
            UEContextReleaseRequest,
            AMF_UE_NGAP_ID(id) = 42u64
        ));

        let ie = request
            .protocol_ies
            .0
            .iter()
            .find(|ie| ie.id.0 == 10)
            .expect("AMF UE ID");
        let id: AMFUENGAPID = rasn::aper::decode(ie.value.as_bytes()).expect("decode ID");
        assert_eq!(id.0, 42);
    }

    #[test]
    fn builder_macros_cover_ie_and_outcome_forms() {
        let ie = build_ngap_ie!(
            UEContextReleaseRequest,
            REJECT AMF_UE_NGAP_ID(1u64)
        );
        assert_eq!(ie.id.0, 10);

        let successful = build_ngap!(SuccessfulOutcome, NGSetup, REJECT, NGSetupResponse,);
        assert!(successful.is_successful());
        assert_eq!(successful.procedure_code(), 21);

        let unsuccessful = build_ngap!(UnsuccessfulOutcome, NGSetup, REJECT, NGSetupFailure,);
        assert!(unsuccessful.is_unsuccessful());
        assert_eq!(unsuccessful.procedure_code(), 21);
    }

    /// The IEs whose type is an OCTET STRING (CONTAINING ...) carry the
    /// encoded transfer as octets.
    #[test]
    fn octet_string_containing_ies_have_macros() -> Result<(), crate::macros::MissingIeError> {
        let transfer = MBSSessionSetupOrModRequestTransfer::new(
            MBSSessionSetupOrModRequestTransferProtocolIEs(vec![]),
        );
        let octets = rasn::aper::encode(&transfer).expect("encode transfer");
        let ie = build_ngap_ie!(
            BroadcastSessionSetupRequest,
            REJECT MBSSessionSetupRequestTransfer(octets.clone())
        );
        assert_eq!(ie.id.0, 315);

        let request =
            BroadcastSessionSetupRequest::new(BroadcastSessionSetupRequestProtocolIEs(vec![ie]));
        extract_ngap_ies!(request, BroadcastSessionSetupRequest,
            req extracted: Vec<u8> = MBSSessionSetupRequestTransfer(value) => value.to_vec(),
        );
        assert_eq!(extracted, octets);
        let decoded: MBSSessionSetupOrModRequestTransfer =
            rasn::aper::decode(&extracted).expect("decode transfer");
        assert_eq!(decoded, transfer);
        Ok(())
    }
}
