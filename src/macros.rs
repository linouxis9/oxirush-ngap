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
//! IE IDs and procedure codes are those of `src/registry.rs`, the list generated
//! from the ASN.1 object sets, which the `inspect` feature reads too. Values
//! are converted with `.into()`, so primitive values such as `u32` can be passed
//! directly for generated newtypes.
//!
//! ```
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
//! ```
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
//! fn downlink(transfer: SONConfigurationTransfer) -> ProtocolIEField {
//!     build_ngap_ie!(DownlinkRANConfigurationTransfer, IGNORE SONConfigurationTransferDL(transfer))
//! }
//! ```
//!
//! ```compile_fail
//! use oxirush_ngap::{build_ngap_ie, ngap::*};
//!
//! fn downlink(transfer: SONConfigurationTransfer) -> ProtocolIEField {
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
//! ```
//! use oxirush_ngap::{build_ngap_ie, extract_ngap_ies, macros::MissingIeError, ngap::*};
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
//!
//! let message = UplinkNASTransport::new(ProtocolIEContainer(vec![
//!     build_ngap_ie!(UplinkNASTransport, REJECT AMF_UE_NGAP_ID(1u64)),
//!     build_ngap_ie!(UplinkNASTransport, REJECT NAS_PDU(vec![0x7e, 0x00])),
//! ]));
//! assert_eq!(handle(&message).unwrap(), [0x7e, 0x00]);
//! ```
//!
//! ## `with_ngap_ie_mut!` — locate and mutate one decoded NGAP IE
//!
//! This decodes the matching open type, passes the concrete value into the
//! expression, then re-encodes it. It returns `true` only when the IE was found,
//! decoded and re-encoded successfully, and the expression returned `true`.
//! The setter form `IeName(binding) = value` expands to `binding.0 = value`.
//!
//! Of an IE that a message has twice, both macros take the same one: the last
//! that decodes. `extract_ngap_ies!` reads it and `with_ngap_ie_mut!` changes it.
//!
//! ```
//! use oxirush_ngap::{build_ngap_ie, ngap::*, with_ngap_ie_mut};
//!
//! let mut message = UplinkNASTransport::new(ProtocolIEContainer(vec![
//!     build_ngap_ie!(UplinkNASTransport, REJECT AMF_UE_NGAP_ID(1u64)),
//! ]));
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

/// The procedures of the protocol, which `src/registry.rs` lists from the ASN.1: the
/// code, the name and the criticality of each, then for each of its messages the
/// direction, the kind, the name that ASN.1 gives it, its type and the IEs of its object
/// set, each with its identifier, its criticality and its presence.
///
/// The codes that the macros take by the names of the procedures, the type of the
/// message that they build, the kinds of a PDU and, with the `inspect` feature, the
/// messages of a tree, the IEs that each can have and the criticality that ASN.1 assigns
/// all come from these lines.
macro_rules! procedures {
    ($($code:literal $procedure:ident $($criticality:ident)? {
        $(InitiatingMessage $initiating:ident $initiating_name:literal $initiating_type:path
            [$($initiating_ie:literal $initiating_criticality:ident
                $initiating_presence:ident),*];)?
        $(SuccessfulOutcome $successful:ident $successful_name:literal $successful_type:path
            [$($successful_ie:literal $successful_criticality:ident
                $successful_presence:ident),*];)?
        $(UnsuccessfulOutcome $unsuccessful:ident $unsuccessful_name:literal
            $unsuccessful_type:path
            [$($unsuccessful_ie:literal $unsuccessful_criticality:ident
                $unsuccessful_presence:ident),*];)?
    })*) => {
        /// The procedure codes, by the names of the procedures.
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        pub mod procedures {
            $(pub const $procedure: u8 = $code;)*
        }

        /// The type of the message of each procedure, by its direction.
        #[doc(hidden)]
        #[allow(non_snake_case, non_camel_case_types)]
        pub mod messages {
            pub mod InitiatingMessage {
                $($(pub type $procedure = $initiating_type;)?)*
            }
            pub mod SuccessfulOutcome {
                $($(pub type $procedure = $successful_type;)?)*
            }
            pub mod UnsuccessfulOutcome {
                $($(pub type $procedure = $unsuccessful_type;)?)*
            }
        }

        /// The direction and the procedure of a PDU: each variant is one message.
        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum NgapPduKind {
            $($(#[doc = concat!("`", $initiating_name, "`.")] $initiating,)?)*
            $($(#[doc = concat!("`", $successful_name, "`.")] $successful,)?)*
            $($(#[doc = concat!("`", $unsuccessful_name, "`.")] $unsuccessful,)?)*
            /// A procedure that the specification does not have in that direction, which
            /// is the one that `direction()` of the PDU gives.
            Other { direction: &'static str, procedure_code: u8 },
        }

        impl NgapPduKind {
            /// Return this kind's NGAP procedure code.
            pub fn procedure_code(&self) -> u8 {
                match self {
                    $(
                        $(Self::$initiating => $code,)?
                        $(Self::$successful => $code,)?
                        $(Self::$unsuccessful => $code,)?
                    )*
                    Self::Other { procedure_code, .. } => *procedure_code,
                }
            }
        }

        impl $crate::ngap::NGAPPDU {
            /// Return the ASN.1 procedure name.
            pub fn procedure_name(&self) -> &'static str {
                match self.procedure_code() {
                    $($code => stringify!($procedure),)*
                    _ => "Unknown",
                }
            }

            /// Return the canonical direction/procedure kind.
            pub fn kind(&self) -> NgapPduKind {
                let procedure_code = self.procedure_code();
                let kind = match self {
                    Self::initiatingMessage(_) => match procedure_code {
                        $($($code => Some(NgapPduKind::$initiating),)?)*
                        _ => None,
                    },
                    Self::successfulOutcome(_) => match procedure_code {
                        $($($code => Some(NgapPduKind::$successful),)?)*
                        _ => None,
                    },
                    Self::unsuccessfulOutcome(_) => match procedure_code {
                        $($($code => Some(NgapPduKind::$unsuccessful),)?)*
                        _ => None,
                    },
                };
                let direction = self.direction();
                kind.unwrap_or(NgapPduKind::Other { direction, procedure_code })
            }
        }

        /// The messages: the direction, the procedure code, the name that ASN.1 gives it
        /// and the type of each, the IEs of its object set, each with its identifier and
        /// whether its presence is mandatory, and the path of its type.
        #[cfg(feature = "inspect")]
        #[allow(clippy::type_complexity)]
        pub(crate) const MESSAGES: &[(
            &str,
            u8,
            &str,
            fn() -> $crate::inspect::Typed,
            &[(u16, bool)],
            &str,
        )] = &[
            $($(("InitiatingMessage", $code, $initiating_name,
                $crate::inspect::Typed::of::<$initiating_type>,
                &[$(($initiating_ie, $crate::macros::presence!($initiating_presence))),*],
                stringify!($initiating_type)),)?)*
            $($(("SuccessfulOutcome", $code, $successful_name,
                $crate::inspect::Typed::of::<$successful_type>,
                &[$(($successful_ie, $crate::macros::presence!($successful_presence))),*],
                stringify!($successful_type)),)?)*
            $($(("UnsuccessfulOutcome", $code, $unsuccessful_name,
                $crate::inspect::Typed::of::<$unsuccessful_type>,
                &[$(($unsuccessful_ie, $crate::macros::presence!($unsuccessful_presence))),*],
                stringify!($unsuccessful_type)),)?)*
        ];

        /// The criticality that ASN.1 assigns to each procedure, by its code.
        #[cfg(feature = "inspect")]
        pub(crate) const PROCEDURE_CRITICALITIES: &[(u8, &str)] =
            &[$($(($code, stringify!($criticality)),)?)*];

        /// The criticality that ASN.1 assigns to the IEs of each message: the name that
        /// ASN.1 gives the message, then each IE of its object set with its identifier and
        /// its criticality.
        #[cfg(feature = "inspect")]
        pub(crate) const MESSAGE_IE_CRITICALITIES: &[(&str, &[(u16, &str)])] = &[
            $($(($initiating_name,
                &[$(($initiating_ie, stringify!($initiating_criticality))),*]),)?)*
            $($(($successful_name,
                &[$(($successful_ie, stringify!($successful_criticality))),*]),)?)*
            $($(($unsuccessful_name,
                &[$(($unsuccessful_ie, stringify!($unsuccessful_criticality))),*]),)?)*
        ];
    };
}
pub(crate) use procedures;

/// Whether an IE of an object set is always there: its `PRESENCE`, as ASN.1 words it.
#[cfg(feature = "inspect")]
macro_rules! presence {
    (mandatory) => {
        true
    };
    (optional) => {
        false
    };
    (conditional) => {
        false
    };
}
#[cfg(feature = "inspect")]
pub(crate) use presence;

/// The IEs of the protocol, which `src/registry.rs` lists from the ASN.1: the identifier,
/// the name that ASN.1 gives it and the type of each, the type that its octets contain,
/// then the names that the macros take it by. An identifier that has several types has
/// its name alone.
///
/// The identifier and the type that a name stands for in the macros and, with the
/// `inspect` feature, the names and the types of a tree all come from these lines.
macro_rules! ies {
    ($($id:literal $name:literal
        $($ie:path $(, $contents:path)? $(=> $own:ident $($alias:ident)?)?)?;
    )*) => {
        /// The identifier and the type of each IE, by the names that the macros take.
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub mod ies {
            use $crate::macros::Ie;
            $($($(
                pub type $own = Ie<$id, $ie>;
                $(pub type $alias = Ie<$id, $ie>;)?
            )?)?)*
        }

        #[cfg(feature = "inspect")]
        pub(crate) const IE_NAMES: &[(u16, &str)] = &[$(($id, $name),)*];

        /// The path of the type of each IE that has one type, and that of the type that
        /// its octets contain.
        #[cfg(feature = "inspect")]
        pub(crate) const IE_TYPES: &[(u16, &str, &str)] = &[
            $($(($id, stringify!($ie), concat!($(stringify!($contents))?)),)?)*
        ];

        #[cfg(feature = "inspect")]
        pub(crate) fn ie(id: u16) -> Result<$crate::inspect::Typed, String> {
            match id {
                $($($id => Ok($crate::inspect::Typed::of::<$ie>()),)?)*
                _ => Err(format!("NGAP IE {id} is unknown or has several types")),
            }
        }

        #[cfg(feature = "inspect")]
        #[allow(clippy::match_single_binding)]
        pub(crate) fn ie_contents(id: u16) -> Option<$crate::inspect::Typed> {
            match id {
                $($($($id => Some($crate::inspect::Typed::of::<$contents>()),)?)?)*
                _ => None,
            }
        }
    };
}
pub(crate) use ies;

/// An IE as the macros name it: a type of the `ies` of the registry, which has the
/// identifier of the IE and the type of its value.
#[doc(hidden)]
pub struct Ie<const ID: u16, T>(core::marker::PhantomData<T>);

/// What a name of the macros stands for.
#[doc(hidden)]
pub trait Named {
    /// The identifier of the IE.
    const ID: u16;
    /// The type of its value.
    type Type;
}

impl<const ID: u16, T> Named for Ie<ID, T> {
    const ID: u16 = ID;
    type Type = T;
}

/// The identifier of the IE that the macros take by this name.
#[macro_export]
#[doc(hidden)]
macro_rules! __ngap_ie_id {
    ($name:ident) => {
        <$crate::registry::ies::$name as $crate::macros::Named>::ID
    };
}

/// The open type of a value of the IE that the macros take by this name.
#[macro_export]
#[doc(hidden)]
macro_rules! __ngap_encode_ie {
    ($name:ident, $value:expr) => {{
        let value: <$crate::registry::ies::$name as $crate::macros::Named>::Type = ($value).into();
        $crate::ngap::encode_open_type(&value)
    }};
}

/// The value in an open type of the IE that the macros take by this name.
#[macro_export]
#[doc(hidden)]
macro_rules! __ngap_decode_ie {
    ($name:ident, $value:expr) => {
        $crate::ngap::decode_open_type::<
            <$crate::registry::ies::$name as $crate::macros::Named>::Type,
        >($value)
    };
}

/// The code of the procedure of this name.
#[macro_export]
#[doc(hidden)]
macro_rules! __ngap_proc_code {
    ($name:ident) => {
        $crate::registry::procedures::$name
    };
}

/// Extract typed NGAP protocol IEs with `req` and `opt` semantics.
#[macro_export]
macro_rules! extract_ngap_ies {
    ($msg_var:expr, $msg:ident,
        $($kind:ident $name:ident : $ty:ty = $ie_name:ident ($bind:ident) $(=> $expr:expr)?),+
        $(,)?
    ) => {
        // The name is the type of what the IEs are taken from.
        let _: &$crate::ngap::$msg = &$msg_var;
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
        // The name is the type of what the IE is in.
        let _: &$crate::ngap::$msg = &$msg_var;
        let mut matched = false;
        // The IE that `extract_ngap_ies!` reads: the last one that decodes.
        for ie in $msg_var.protocol_ies.0.iter_mut().rev() {
            if ie.id.0 == $crate::__ngap_ie_id!($ie_name) {
                if let Ok(mut $bind) = $crate::__ngap_decode_ie!($ie_name, &ie.value) {
                    let result = $expr;
                    if let Ok(value) = $crate::ngap::encode_open_type(&$bind) {
                        ie.value = value;
                        matched = result;
                    }
                    break;
                }
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
        {
            let ies = vec![
                $( $crate::ngap::ProtocolIEField {
                    id: $crate::ngap::ProtocolIEID($crate::__ngap_ie_id!($ie_name)),
                    criticality: $crate::build_ngap!(@criticality $ie_crit),
                    value: $crate::__ngap_encode_ie!($ie_name, ($($ie_value)+))
                        .expect("failed to APER-encode NGAP IE open type"),
                }, )*
            ];
            // The message is the one that the procedure has in that direction.
            let message: $crate::registry::messages::$direction::$proc =
                $crate::ngap::$msg::new($crate::ngap::ProtocolIEContainer(ies));
            let value = $crate::ngap::encode_open_type(&message)
                .expect("failed to APER-encode NGAP message open type");
            $crate::build_ngap!(@pdu $direction, $proc, $outer_crit, value)
        }
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

/// Build one NGAP protocol IE entry. Every message takes the same
/// `ProtocolIEField`: the name says where the IE goes, and is that of a message
/// or of another type that has IEs.
///
/// # Panics
///
/// Panics if the value cannot be APER-encoded into its open type.
#[macro_export]
macro_rules! build_ngap_ie {
    ($msg:ident, $criticality:ident $ie_name:ident ($($value:tt)+)) => {{
        // The name is a type that has IEs.
        let _ = |of: &$crate::ngap::$msg| {
            let _: &$crate::ngap::ProtocolIEContainer = &of.protocol_ies;
        };
        $crate::ngap::ProtocolIEField {
            id: $crate::ngap::ProtocolIEID($crate::__ngap_ie_id!($ie_name)),
            criticality: $crate::build_ngap!(@criticality $criticality),
            value: $crate::__ngap_encode_ie!($ie_name, ($($value)+))
                .expect("failed to APER-encode NGAP IE open type"),
        }
    }};
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
    fn an_ie_that_is_there_twice_is_read_and_changed_at_the_same_place() {
        use crate::macros::MissingIeError;
        fn id(request: &UEContextReleaseRequest) -> Result<u64, MissingIeError> {
            extract_ngap_ies!(request, UEContextReleaseRequest,
                req id: u64 = AMF_UE_NGAP_ID(id),
            );
            Ok(id)
        }
        let ids = |request: &UEContextReleaseRequest| -> Vec<Vec<u8>> {
            let ies = request.protocol_ies.0.iter();
            ies.map(|ie| ie.value.as_bytes().to_vec()).collect()
        };
        let entry = |id: u64| build_ngap_ie!(UEContextReleaseRequest, REJECT AMF_UE_NGAP_ID(id));
        let mut request = UEContextReleaseRequest::new(ProtocolIEContainer(vec![
            entry(1),
            entry(2),
            // An identifier of this IE that does not decode as one.
            ProtocolIEField::new(
                10u16,
                Criticality::reject,
                rasn::types::Any::new(vec![0xff]),
            ),
        ]));
        assert_eq!(id(&request).unwrap(), 2);
        let before = ids(&request);
        assert!(with_ngap_ie_mut!(
            request,
            UEContextReleaseRequest,
            AMF_UE_NGAP_ID(id) = 42u64
        ));
        assert_eq!(id(&request).unwrap(), 42);
        let after = ids(&request);
        assert_eq!((&after[0], &after[2]), (&before[0], &before[2]));
        assert_ne!(after[1], before[1]);
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
        let transfer = MBSSessionSetupOrModRequestTransfer::new(ProtocolIEContainer(vec![]));
        let octets = rasn::aper::encode(&transfer).expect("encode transfer");
        let ie = build_ngap_ie!(
            BroadcastSessionSetupRequest,
            REJECT MBSSessionSetupRequestTransfer(octets.clone())
        );
        assert_eq!(ie.id.0, 315);

        let request = BroadcastSessionSetupRequest::new(ProtocolIEContainer(vec![ie]));
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
