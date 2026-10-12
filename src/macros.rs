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
//! )?;
//! assert_eq!(pdu.procedure_name(), "UEContextReleaseRequest");
//! # Ok::<(), oxirush_ngap::Error>(())
//! ```
//!
//! The macro gives a `Result`. A value that does not encode, as an integer outside
//! its constraint, is an [`Error`] of the kind `Encode`, and no panic.
//!
//! Arguments:
//! - `$direction` — `InitiatingMessage`, `SuccessfulOutcome`, or `UnsuccessfulOutcome`
//! - `$proc` — procedure name, used to derive its ASN.1 procedure code
//! - `$outer_crit` — outer criticality: `REJECT`, `IGNORE`, or `NOTIFY`
//! - `$msg` — message struct name, such as `UEContextReleaseRequest`
//! - IEs — `$ie_crit $ie_name($ie_value)`, with the IE ID derived from its name
//!
//! An IE that is written without its criticality does not compile, and the error says
//! which IE it is. Neither does a PDU without its own:
//!
//! ```compile_fail
//! use oxirush_ngap::{build_ngap, ngap::*};
//!
//! let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
//!     IGNORE, UEContextReleaseRequest,
//!     AMF_UE_NGAP_ID(1u64),
//! );
//! ```
//!
//! ```compile_fail
//! use oxirush_ngap::{build_ngap, ngap::*};
//!
//! let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
//!     UEContextReleaseRequest,
//!     REJECT AMF_UE_NGAP_ID(1u64),
//! );
//! ```
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
//! )?;
//! assert_eq!(ie.criticality, Criticality::reject);
//! # Ok::<(), oxirush_ngap::Error>(())
//! ```
//!
//! ```compile_fail
//! use oxirush_ngap::{build_ngap_ie, ngap::*};
//!
//! let ie = build_ngap_ie!(UEContextReleaseRequest, AMF_UE_NGAP_ID(1u64));
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
//! fn downlink(transfer: SONConfigurationTransfer) -> Result<ProtocolIEField, oxirush_ngap::Error> {
//!     build_ngap_ie!(DownlinkRANConfigurationTransfer, IGNORE SONConfigurationTransferDL(transfer))
//! }
//! ```
//!
//! ```compile_fail
//! use oxirush_ngap::{build_ngap_ie, ngap::*};
//!
//! fn downlink(transfer: SONConfigurationTransfer) -> Result<ProtocolIEField, oxirush_ngap::Error> {
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
//! A `req` field is the value, and an `opt` or a `lax` field an `Option` of it:
//!
//! | The IE | A `req` field | An `opt` field | A `lax` field |
//! |--------|---------------|----------------|---------------|
//! | is in the message | the value | `Some` of the value | `Some` of the value |
//! | is not in the message | an error of the kind `MissingIe` | `None` | `None` |
//! | is there and does not decode | an error of the kind `UndecodableIe` | the same error | `None` |
//!
//! Which kind a field is says what the receiver does without the IE:
//!
//! - `req`: it cannot go on. The IE is one that the message always has, or one that
//!   the handling needs.
//! - `opt`: it goes on without an IE that was not sent, and not with one that it
//!   could not read.
//! - `lax`: it goes on as if the IE had not been received, whether it was not sent
//!   or was not read: the field takes the IE if it decodes.
//!
//! A `lax` field is for an IE that the receiver may ignore. TS 38.413 clause 10.3.4.2
//! has a receiver ignore the content of an IE that it does not comprehend and that
//! was received with the criticality `ignore`, and go on as if the IE had not been
//! received. With `notify` it does the same and reports the IE, and with `reject`
//! it rejects the procedure. The macro does not look at the criticality of an IE:
//! the caller reads it in `protocol_ies`, and takes with `lax` what it may ignore.
//!
//! The error is that of the enclosing function, which returns a `Result` whose
//! error is [`Error`] or has a `From` of it, as
//! `Box<dyn std::error::Error>` does: the macro converts the error as `?`
//! converts one. A call with `lax` fields alone has no error, and its function
//! returns what it likes. Without `=> expression`, extraction uses `binding.0`,
//! which unwraps the usual single-field generated newtype.
//!
//! ```
//! use oxirush_ngap::{Error, ErrorKind, build_ngap_ie, extract_ngap_ies, ngap::*};
//!
//! fn handle(msg: &UplinkNASTransport) -> Result<Vec<u8>, Error> {
//!     extract_ngap_ies!(msg, UplinkNASTransport,
//!         req amf_id: u64 = AMF_UE_NGAP_ID(id),
//!         req nas_pdu: Vec<u8> = NAS_PDU(pdu) => pdu.0.to_vec(),
//!         opt ran_id: u32 = RAN_UE_NGAP_ID(id),
//!     );
//!     let _ = (amf_id, ran_id);
//!     Ok(nas_pdu)
//! }
//!
//! let mut message = UplinkNASTransport::new(ProtocolIEContainer(vec![
//!     build_ngap_ie!(UplinkNASTransport, REJECT AMF_UE_NGAP_ID(1u64))?,
//!     build_ngap_ie!(UplinkNASTransport, REJECT NAS_PDU(vec![0x7e, 0x00]))?,
//! ]));
//! assert_eq!(handle(&message)?, [0x7e, 0x00]);
//!
//! // Without the NAS-PDU, which a field requires
//! message.protocol_ies.0.pop();
//! assert_eq!(handle(&message).unwrap_err().kind(), ErrorKind::MissingIe);
//! # Ok::<(), Error>(())
//! ```
//!
//! ## `with_ngap_ie_mut!` — locate and mutate one decoded NGAP IE
//!
//! This decodes the matching open type, passes the concrete value into the
//! expression, then re-encodes it. It gives a `Result<bool, Error>`: what the
//! expression says, `Ok(false)` for an IE that is not in the message, and an error
//! for an IE that does not decode (`UndecodableIe`) or a changed value that does
//! not encode (`Encode`).
//! The setter form `IeName(binding) = value` expands to `binding.0 = value`.
//!
//! When a message has the same IE twice, both macros use the same one, the last:
//! `extract_ngap_ies!` reads it and `with_ngap_ie_mut!` changes it. One of them that
//! does not decode is an error for both, but for a `lax` field, which is `None` when
//! the last one does not decode.
//!
//! ```
//! use oxirush_ngap::{build_ngap_ie, ngap::*, with_ngap_ie_mut};
//!
//! let mut message = UplinkNASTransport::new(ProtocolIEContainer(vec![
//!     build_ngap_ie!(UplinkNASTransport, REJECT AMF_UE_NGAP_ID(1u64))?,
//! ]));
//! let updated = with_ngap_ie_mut!(message, UplinkNASTransport,
//!     AMF_UE_NGAP_ID(id) = 42u64
//! )?;
//! assert!(updated);
//! # Ok::<(), oxirush_ngap::Error>(())
//! ```

use rasn::types::Any;

use crate::ngap::{
    Criticality, InitiatingMessage, NGAP_PDU, ProcedureCode, ProtocolIEContainer, ProtocolIEField,
    ProtocolIEID, SuccessfulOutcome, UnsuccessfulOutcome,
};
use crate::{Error, Message};

/// For the macros that build: one entry of the IEs of a message, which is the
/// identifier of an IE, its criticality and its value in an open type, or why the
/// value does not encode.
#[doc(hidden)]
pub fn __ie(
    id: u16,
    criticality: Criticality,
    value: Result<Any, Error>,
) -> Result<ProtocolIEField, Error> {
    Ok(ProtocolIEField {
        id: ProtocolIEID(id),
        criticality,
        value: value?,
    })
}

/// For the macro that builds a PDU: the PDU of the message `M` that `new` makes of
/// these IEs, with the direction and the procedure code of that message, or the
/// first error of an IE that was not built.
#[doc(hidden)]
pub fn __pdu<M: Message, const N: usize>(
    criticality: Criticality,
    ies: [Result<ProtocolIEField, Error>; N],
    new: fn(ProtocolIEContainer) -> M,
) -> Result<NGAP_PDU, Error> {
    let ies = ies.into_iter().collect::<Result<Vec<_>, Error>>()?;
    let value = crate::per::encode_open_type(&new(ProtocolIEContainer(ies)))?;
    let procedure_code = ProcedureCode(M::PROCEDURE_CODE);
    Ok(match M::DIRECTION {
        "InitiatingMessage" => NGAP_PDU::initiatingMessage(InitiatingMessage {
            procedure_code,
            criticality,
            value,
        }),
        "SuccessfulOutcome" => NGAP_PDU::successfulOutcome(SuccessfulOutcome {
            procedure_code,
            criticality,
            value,
        }),
        _ => NGAP_PDU::unsuccessfulOutcome(UnsuccessfulOutcome {
            procedure_code,
            criticality,
            value,
        }),
    })
}

/// The procedures of the protocol, which `src/registry.rs` lists from the ASN.1: the
/// code, the name and the criticality of each, then for each of its messages the
/// direction, the kind, the name that ASN.1 gives it, its type and the IEs of its object
/// set, each with its identifier, its criticality and its presence. A message that has
/// no container of protocol IEs, as the private message, has no object set.
///
/// The codes that the macros take by the names of the procedures, the type of the
/// message that they build, the kinds of a PDU, the messages whose protocol IEs a PDU
/// gives and, with the `inspect` feature, the messages of a view and of a tree, the IEs
/// that each can have and the criticality that ASN.1 assigns all come from these lines.
macro_rules! procedures {
    ($($code:literal $procedure:ident $($criticality:ident)? {
        $(InitiatingMessage $initiating:ident $initiating_name:literal $initiating_type:path
            $([$($initiating_ie:literal $initiating_criticality:ident
                $initiating_presence:ident),*])?;)?
        $(SuccessfulOutcome $successful:ident $successful_name:literal $successful_type:path
            $([$($successful_ie:literal $successful_criticality:ident
                $successful_presence:ident),*])?;)?
        $(UnsuccessfulOutcome $unsuccessful:ident $unsuccessful_name:literal
            $unsuccessful_type:path
            $([$($unsuccessful_ie:literal $unsuccessful_criticality:ident
                $unsuccessful_presence:ident),*])?;)?
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

        // The type of each message is a message, and no other type is one.
        $($(
            impl $crate::message::sealed::Sealed for $initiating_type {}
            impl $crate::Message for $initiating_type {
                const NAME: &'static str = $initiating_name;
                const DIRECTION: &'static str = "InitiatingMessage";
                const PROCEDURE_CODE: u8 = $code;
            }
        )?)*
        $($(
            impl $crate::message::sealed::Sealed for $successful_type {}
            impl $crate::Message for $successful_type {
                const NAME: &'static str = $successful_name;
                const DIRECTION: &'static str = "SuccessfulOutcome";
                const PROCEDURE_CODE: u8 = $code;
            }
        )?)*
        $($(
            impl $crate::message::sealed::Sealed for $unsuccessful_type {}
            impl $crate::Message for $unsuccessful_type {
                const NAME: &'static str = $unsuccessful_name;
                const DIRECTION: &'static str = "UnsuccessfulOutcome";
                const PROCEDURE_CODE: u8 = $code;
            }
        )?)*

        /// The name that ASN.1 gives the message of a direction and a procedure code.
        pub(crate) fn message_name(direction: &str, code: u8) -> Option<&'static str> {
            match (direction, code) {
                $($(("InitiatingMessage", $code) => Some($initiating_name),)?)*
                $($(("SuccessfulOutcome", $code) => Some($successful_name),)?)*
                $($(("UnsuccessfulOutcome", $code) => Some($unsuccessful_name),)?)*
                _ => None,
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

        /// The criticality of the bindings, for the code that the two protocols share.
        pub(crate) use $crate::ngap::Criticality;

        /// The criticality that ASN.1 assigns to the procedure of a code.
        pub(crate) fn procedure_criticality(code: u8) -> Option<Criticality> {
            match code {
                $($($code => Some(Criticality::$criticality),)?)*
                _ => None,
            }
        }

        /// The object set of each message: the direction and the procedure code of its
        /// PDU, the name that ASN.1 gives it, and the IEs that its container of protocol
        /// IEs can have, in the order of the set.
        pub(crate) const OBJECT_SETS: &[$crate::check::ObjectSet] = &[
            $($($crate::check::ObjectSet {
                direction: "InitiatingMessage",
                procedure_code: $code,
                name: $initiating_name,
                ies: $crate::macros::object_set!($([$(
                    $initiating_ie $initiating_criticality $initiating_presence
                ),*])?),
            },)?)*
            $($($crate::check::ObjectSet {
                direction: "SuccessfulOutcome",
                procedure_code: $code,
                name: $successful_name,
                ies: $crate::macros::object_set!($([$(
                    $successful_ie $successful_criticality $successful_presence
                ),*])?),
            },)?)*
            $($($crate::check::ObjectSet {
                direction: "UnsuccessfulOutcome",
                procedure_code: $code,
                name: $unsuccessful_name,
                ies: $crate::macros::object_set!($([$(
                    $unsuccessful_ie $unsuccessful_criticality $unsuccessful_presence
                ),*])?),
            },)?)*
        ];

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
                &[$($(($initiating_ie, $crate::macros::presence!($initiating_presence))),*)?],
                stringify!($initiating_type)),)?)*
            $($(("SuccessfulOutcome", $code, $successful_name,
                $crate::inspect::Typed::of::<$successful_type>,
                &[$($(($successful_ie, $crate::macros::presence!($successful_presence))),*)?],
                stringify!($successful_type)),)?)*
            $($(("UnsuccessfulOutcome", $code, $unsuccessful_name,
                $crate::inspect::Typed::of::<$unsuccessful_type>,
                &[$($(($unsuccessful_ie, $crate::macros::presence!($unsuccessful_presence))),*)?],
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
                &[$($(($initiating_ie, stringify!($initiating_criticality))),*)?]),)?)*
            $($(($successful_name,
                &[$($(($successful_ie, stringify!($successful_criticality))),*)?]),)?)*
            $($(($unsuccessful_name,
                &[$($(($unsuccessful_ie, stringify!($unsuccessful_criticality))),*)?]),)?)*
        ];
    };
}
pub(crate) use procedures;

/// The IEs of an object set, each as its identifier, the criticality that the set
/// assigns it and whether its presence is mandatory: none for a message that has no
/// container of protocol IEs.
macro_rules! object_set {
    () => {
        None
    };
    ([$($id:literal $criticality:ident $presence:ident),*]) => {
        Some(&[$((
            $id,
            $crate::ngap::Criticality::$criticality,
            $crate::macros::presence!($presence),
        )),*])
    };
}
pub(crate) use object_set;

/// Whether an IE of an object set is always there: its `PRESENCE`, as ASN.1 words it.
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
pub(crate) use presence;

/// The IEs of the protocol, which `src/registry.rs` lists from the ASN.1: the identifier,
/// the name that ASN.1 gives it and the type of each, the type that its octets contain,
/// then the names that the macros take it by: its own, then those of its type. An IE
/// that a value holds alone, in a single container, has its criticality after its
/// identifier when its object sets give it one. An IE of an extension container that
/// has the type which the type that it extends gives it has its types in brackets, each
/// with the types that it extends so.
///
/// The identifier and the type that a name stands for in the macros and, with the
/// `inspect` feature, the names and the types of a view and of a tree and the
/// criticality of an IE in a single container all come from these lines.
macro_rules! ies {
    ($($id:literal $($item:ident)? $name:literal
        $($ie:path $(, $contents:path)? $(=> $own:ident $($alias:ident)*)?)?
        $([$($extending:path: $($extended:ident)+),+])?;
    )*) => {
        /// The identifier and the type of each IE, by the names that the macros take.
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub mod ies {
            use $crate::macros::Ie;
            $($($(
                pub type $own = Ie<$id, $ie>;
                $(pub type $alias = Ie<$id, $ie>;)*
            )?)?)*
        }

        /// The identifier of each IE with the name that ASN.1 gives it after `id-`.
        pub(crate) const IE_NAMES: &[(u16, &str)] = &[$(($id, $name),)*];

        /// The criticality that ASN.1 assigns to each IE that a value holds alone, in a
        /// single container, when its object sets give it one.
        #[cfg(feature = "inspect")]
        pub(crate) const ITEM_CRITICALITIES: &[(u16, &str)] =
            &[$($(($id, stringify!($item)),)?)*];

        /// The path of the type of each IE that has one type, and that of the type that
        /// its octets contain.
        #[cfg(feature = "inspect")]
        pub(crate) const IE_TYPES: &[(u16, &str, &str)] = &[
            $($(($id, stringify!($ie), concat!($(stringify!($contents))?)),)?)*
        ];

        /// The IEs of an extension container that have the type which the type that
        /// they extend gives them: the identifier, the name of a type that is extended
        /// and the path of the type that the IE has there.
        #[cfg(feature = "inspect")]
        pub(crate) const EXTENSION_TYPES: &[(u16, &str, &str)] = &[
            $($($($(($id, stringify!($extended), stringify!($extending)),)+)+)?)*
        ];

        /// The functions of the type of the IE `id` where it extends the type that the
        /// bindings name `extended`; `None` for an IE that has its type whatever it
        /// extends.
        #[cfg(feature = "inspect")]
        #[allow(clippy::match_single_binding)]
        pub(crate) fn extension(id: u16, extended: &str) -> Option<$crate::inspect::Typed> {
            match (id, extended) {
                $($($($(($id, stringify!($extended)) => {
                    Some($crate::inspect::Typed::of::<$extending>())
                })+)+)?)*
                _ => None,
            }
        }

        #[cfg(feature = "inspect")]
        pub(crate) fn ie(id: u16) -> Result<$crate::inspect::Typed, $crate::Error> {
            match id {
                $($($id => Ok($crate::inspect::Typed::of::<$ie>()),)?)*
                _ => {
                    // An IE that has a type for each type that it extends has none
                    // elsewhere.
                    let extensions = EXTENSION_TYPES.iter().filter(|(known, ..)| *known == id);
                    let extended: Vec<_> = extensions.map(|(_, extended, _)| *extended).collect();
                    Err($crate::Error::unknown_ie(match extended.is_empty() {
                        true => format!("NGAP IE {id} is unknown"),
                        false => format!(
                            "NGAP IE {id} has a type where it extends {}, and none here",
                            extended.join(", ")
                        ),
                    }))
                }
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

/// Why the IEs of a call of `build_ngap!` are not written as they are read: the one
/// that has no criticality, by its name.
#[macro_export]
#[doc(hidden)]
macro_rules! __ngap_ie_syntax {
    // An IE that has its criticality: the fault is in one that follows.
    ($criticality:ident $name:ident ($($value:tt)*) $(, $($rest:tt)*)?) => {
        $crate::__ngap_ie_syntax!($($($rest)*)?)
    };
    ($name:ident ($($value:tt)*) $($rest:tt)*) => {
        compile_error!(concat!(
            "the criticality of the IE ",
            stringify!($name),
            " is missing: write REJECT, IGNORE or NOTIFY before its name",
        ))
    };
    ($($other:tt)*) => {
        compile_error!(
            "each IE is written `CRITICALITY Name(value)`, with REJECT, IGNORE or NOTIFY \
             as its criticality and a comma after it"
        )
    };
}

/// Extract typed NGAP protocol IEs into `req`, `opt` and `lax` fields.
///
/// | The IE | A `req` field | An `opt` field | A `lax` field |
/// |--------|---------------|----------------|---------------|
/// | is in the message | the value | `Some` of the value | `Some` of the value |
/// | is not in the message | an error of the kind `MissingIe` | `None` | `None` |
/// | is there and does not decode | an error of the kind `UndecodableIe` | the same error | `None` |
///
/// The enclosing function returns the error, converted as `?` converts one. An `opt`
/// field is `None` only when its IE is not in the message. A `lax` field takes the IE
/// if it decodes, and is `None` otherwise: it is for an IE that the receiver may
/// ignore, as TS 38.413 clause 10.3.4.2 has it ignore one that it does not comprehend
/// and that was received with the criticality `ignore`. The macro does not look at
/// the criticality, which the caller reads in `protocol_ies`. A call with `lax`
/// fields alone has no error, and its function need not return a `Result`.
///
/// When the message has the IE twice, the field is the last one, which is the one
/// that `with_ngap_ie_mut!` changes. Any of them that does not decode is the error of a `req`
/// and of an `opt` field, and a `lax` field is `None` when the last one does not
/// decode.
#[macro_export]
macro_rules! extract_ngap_ies {
    ($msg_var:expr, $msg:ident,
        $($kind:ident $name:ident : $ty:ty = $ie_name:ident ($bind:ident) $(=> $expr:expr)?),+
        $(,)?
    ) => {
        // The name is the type of what the IEs are taken from.
        let _: &$crate::ngap::$msg = &$msg_var;
        $(
            let mut $name: ::core::option::Option<::core::result::Result<$ty, $crate::Error>> =
                ::core::option::Option::None;
        )+
        for _ie in &$msg_var.protocol_ies.0 {
            $(
                if _ie.id.0 == $crate::__ngap_ie_id!($ie_name) {
                    // `$bind` is out of scope where `$name` is assigned, as
                    // both may have the same name.
                    let _value: ::core::result::Result<$ty, $crate::Error> =
                        match $crate::__ngap_decode_ie!($ie_name, &_ie.value) {
                            ::core::result::Result::Ok($bind) => ::core::result::Result::Ok(
                                $crate::extract_ngap_ies!(@val $bind $(, $expr)?),
                            ),
                            ::core::result::Result::Err(_error) => ::core::result::Result::Err(
                                _error.__undecodable_ie(stringify!($msg), _ie.id.0),
                            ),
                        };
                    $name = $crate::extract_ngap_ies!(@last $kind $name, _value);
                }
            )+
        }
        $( $crate::extract_ngap_ies!(@check $kind $name, $msg, $ie_name); )+
    };
    (@val $bind:ident) => { $bind.0 };
    (@val $_bind:ident, $expr:expr) => { $expr };
    // The last one decides a `lax` field.
    (@last lax $name:ident, $value:ident) => {
        ::core::option::Option::Some($value)
    };
    // The last one is read, and one that does not decode is an error.
    (@last $kind:ident $name:ident, $value:ident) => {
        match $name {
            ::core::option::Option::Some(::core::result::Result::Err(_error)) => {
                ::core::option::Option::Some(::core::result::Result::Err(_error))
            }
            _ => ::core::option::Option::Some($value),
        }
    };
    (@check req $name:ident, $msg:ident, $ie_name:ident) => {
        let $name = match $name {
            ::core::option::Option::Some(::core::result::Result::Ok(_value)) => _value,
            // The error of the enclosing function, converted as `?` converts one.
            ::core::option::Option::Some(::core::result::Result::Err(_error)) => {
                return ::core::result::Result::Err(::core::convert::From::from(_error));
            }
            ::core::option::Option::None => {
                return ::core::result::Result::Err(::core::convert::From::from(
                    $crate::Error::__missing_ie(
                        stringify!($msg),
                        $crate::__ngap_ie_id!($ie_name),
                        stringify!($name),
                    ),
                ));
            }
        };
    };
    (@check opt $name:ident, $msg:ident, $ie_name:ident) => {
        let $name = match $name {
            ::core::option::Option::Some(::core::result::Result::Ok(_value)) => {
                ::core::option::Option::Some(_value)
            }
            ::core::option::Option::Some(::core::result::Result::Err(_error)) => {
                return ::core::result::Result::Err(::core::convert::From::from(_error));
            }
            ::core::option::Option::None => ::core::option::Option::None,
        };
    };
    // The IE if it decodes: nothing is returned.
    (@check lax $name:ident, $msg:ident, $ie_name:ident) => {
        let $name = match $name {
            ::core::option::Option::Some(::core::result::Result::Ok(_value)) => {
                ::core::option::Option::Some(_value)
            }
            _ => ::core::option::Option::None,
        };
    };
    (@check $kind:ident $name:ident, $msg:ident, $ie_name:ident) => {
        compile_error!(concat!(
            "`",
            stringify!($kind),
            "` is no kind of field: write req, opt or lax before the name of the field ",
            stringify!($name),
        ));
    };
}

/// Locate, decode, mutate, and re-encode one NGAP protocol IE.
///
/// Gives a `Result<bool, Error>`: what the expression says, `Ok(false)` when the IE is
/// not in the message, and an error when the IE does not decode (`UndecodableIe`) or
/// the changed value does not encode (`Encode`). The message is as it was after an
/// error.
///
/// When the message has the IE twice, the last one is changed, which is the one that
/// `extract_ngap_ies!` reads, and any of them that does not decode is the error.
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
        // The IE that `extract_ngap_ies!` reads: the last one, and one that does not
        // decode is an error.
        let mut _found = ::core::result::Result::Ok(::core::option::Option::None);
        for (_index, _ie) in $msg_var.protocol_ies.0.iter().enumerate() {
            if _ie.id.0 == $crate::__ngap_ie_id!($ie_name) && _found.is_ok() {
                _found = match $crate::__ngap_decode_ie!($ie_name, &_ie.value) {
                    ::core::result::Result::Ok(_value) => {
                        ::core::result::Result::Ok(::core::option::Option::Some((_index, _value)))
                    }
                    ::core::result::Result::Err(_error) => ::core::result::Result::Err(
                        _error.__undecodable_ie(stringify!($msg), _ie.id.0),
                    ),
                };
            }
        }
        match _found {
            ::core::result::Result::Err(_error) => ::core::result::Result::Err(_error),
            ::core::result::Result::Ok(::core::option::Option::None) => {
                ::core::result::Result::Ok(false)
            }
            ::core::result::Result::Ok(::core::option::Option::Some((_index, mut $bind))) => {
                let _changed: bool = $expr;
                match $crate::ngap::encode_open_type(&$bind) {
                    ::core::result::Result::Ok(_value) => {
                        // The IE is where it was read, unless the message is another
                        // one each time that its expression is read.
                        match $msg_var.protocol_ies.0.get_mut(_index) {
                            ::core::option::Option::Some(_ie) => {
                                _ie.value = _value;
                                ::core::result::Result::Ok(_changed)
                            }
                            ::core::option::Option::None => ::core::result::Result::Ok(false),
                        }
                    }
                    ::core::result::Result::Err(_error) => ::core::result::Result::Err(_error),
                }
            }
        }
    }};
}

/// Build a complete `NGAP_PDU` from a direction, procedure, message, and IEs.
///
/// Gives a `Result<NGAP_PDU, Error>`: a value that does not encode into its open type,
/// such as an integer outside its constraint, is an error of the kind `Encode`. A call
/// that leaves out a criticality does not compile, and the error says which one.
#[macro_export]
macro_rules! build_ngap {
    ($direction:ident, $proc:ident,
     $outer_crit:ident, $msg:ident,
     $($ie_crit:ident $ie_name:ident ($($ie_value:tt)+)),*
     $(,)?
    ) => {
        $crate::macros::__pdu(
            $crate::build_ngap!(@criticality $outer_crit),
            [$(
                $crate::macros::__ie(
                    $crate::__ngap_ie_id!($ie_name),
                    $crate::build_ngap!(@criticality $ie_crit),
                    $crate::__ngap_encode_ie!($ie_name, ($($ie_value)+)),
                ),
            )*],
            // The message is the one that the procedure has in that direction.
            {
                let new: fn(
                    $crate::ngap::ProtocolIEContainer,
                ) -> $crate::registry::messages::$direction::$proc = $crate::ngap::$msg::new;
                new
            },
        )
    };
    // The criticality of the PDU is left out: the message follows the procedure.
    ($direction:ident, $proc:ident, $msg:ident,
     $($ie_crit:ident $ie_name:ident ($($ie_value:tt)+)),*
     $(,)?
    ) => {
        compile_error!(concat!(
            "the criticality of the PDU is missing: write REJECT, IGNORE or NOTIFY before ",
            stringify!($msg),
        ))
    };
    // An IE is not written as its criticality, its name and its value.
    ($direction:ident, $proc:ident, $outer_crit:ident, $msg:ident, $($ies:tt)*) => {
        $crate::__ngap_ie_syntax!($($ies)*)
    };
    (@criticality REJECT) => { $crate::ngap::Criticality::reject };
    (@criticality IGNORE) => { $crate::ngap::Criticality::ignore };
    (@criticality NOTIFY) => { $crate::ngap::Criticality::notify };
    (@criticality $other:tt) => {
        compile_error!(concat!(
            "`",
            stringify!($other),
            "` is no criticality: write REJECT, IGNORE or NOTIFY",
        ))
    };
}

/// Build one NGAP protocol IE entry. Every message takes the same
/// `ProtocolIEField`: the name says where the IE goes, and is that of a message
/// or of another type that has IEs.
///
/// Gives a `Result<ProtocolIEField, Error>`: a value that does not encode into its
/// open type is an error of the kind `Encode`.
#[macro_export]
macro_rules! build_ngap_ie {
    ($msg:ident, $criticality:ident $ie_name:ident ($($value:tt)+)) => {{
        // The name is a type that has IEs.
        let _ = |of: &$crate::ngap::$msg| {
            let _: &$crate::ngap::ProtocolIEContainer = &of.protocol_ies;
        };
        $crate::macros::__ie(
            $crate::__ngap_ie_id!($ie_name),
            $crate::build_ngap!(@criticality $criticality),
            $crate::__ngap_encode_ie!($ie_name, ($($value)+)),
        )
    }};
    ($msg:ident, $ie_name:ident ($($value:tt)+)) => {
        compile_error!(concat!(
            "the criticality of the IE ",
            stringify!($ie_name),
            " is missing: write REJECT, IGNORE or NOTIFY before its name",
        ))
    };
}

#[cfg(test)]
mod tests {
    use crate::ngap::*;
    use crate::{Error, ErrorKind};

    #[test]
    fn mutating_macro_updates_newtype_payload() -> Result<(), Error> {
        let mut pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
            IGNORE, UEContextReleaseRequest,
            REJECT AMF_UE_NGAP_ID(1u64),
            REJECT RAN_UE_NGAP_ID(7u32),
            IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
        )?;

        let NGAP_PDU::initiatingMessage(message) = &mut pdu else {
            panic!("expected initiating message");
        };
        let request: UEContextReleaseRequest =
            rasn::aper::decode(message.value.as_bytes()).expect("decode request");
        let mut request = request;

        let changed =
            with_ngap_ie_mut!(request, UEContextReleaseRequest, AMF_UE_NGAP_ID(id) = 42u64);
        assert_eq!(changed, Ok(true));

        let ie = request
            .protocol_ies
            .0
            .iter()
            .find(|ie| ie.id.0 == 10)
            .expect("AMF UE ID");
        let id: AMFUENGAPID = rasn::aper::decode(ie.value.as_bytes()).expect("decode ID");
        assert_eq!(id.0, 42);
        Ok(())
    }

    /// The identifier of a request, as the macro reads it.
    fn id(request: &UEContextReleaseRequest) -> Result<u64, Error> {
        extract_ngap_ies!(request, UEContextReleaseRequest,
            req id: u64 = AMF_UE_NGAP_ID(id),
        );
        Ok(id)
    }

    #[test]
    fn an_ie_that_is_there_twice_is_read_and_changed_at_the_same_place() -> Result<(), Error> {
        let ids = |request: &UEContextReleaseRequest| -> Vec<Vec<u8>> {
            let ies = request.protocol_ies.0.iter();
            ies.map(|ie| ie.value.as_bytes().to_vec()).collect()
        };
        let entry = |id: u64| build_ngap_ie!(UEContextReleaseRequest, REJECT AMF_UE_NGAP_ID(id));
        let mut request =
            UEContextReleaseRequest::new(ProtocolIEContainer(vec![entry(1)?, entry(2)?]));
        // The last one is read, and is the one that is changed.
        assert_eq!(id(&request)?, 2);
        let before = ids(&request);
        let changed =
            with_ngap_ie_mut!(request, UEContextReleaseRequest, AMF_UE_NGAP_ID(id) = 42u64);
        assert_eq!(changed, Ok(true));
        assert_eq!(id(&request)?, 42);
        let after = ids(&request);
        assert_eq!(after[0], before[0]);
        assert_ne!(after[1], before[1]);
        Ok(())
    }

    #[test]
    fn one_of_two_that_does_not_decode_is_an_error_for_both_macros() -> Result<(), Error> {
        // An entry with the identifier of this IE whose octets are not its value,
        // after one that decodes and before it.
        let undecodable = || {
            let octets = rasn::types::Any::new(vec![0xff]);
            ProtocolIEField::new(10u16, Criticality::reject, octets)
        };
        let entry = build_ngap_ie!(UEContextReleaseRequest, REJECT AMF_UE_NGAP_ID(1u64))?;
        for ies in [
            vec![entry.clone(), undecodable()],
            vec![undecodable(), entry.clone()],
        ] {
            let mut request = UEContextReleaseRequest::new(ProtocolIEContainer(ies));
            let before = request.clone();
            assert_eq!(id(&request).unwrap_err().kind(), ErrorKind::UndecodableIe);
            let changed =
                with_ngap_ie_mut!(request, UEContextReleaseRequest, AMF_UE_NGAP_ID(id) = 42u64);
            assert_eq!(changed.unwrap_err().kind(), ErrorKind::UndecodableIe);
            // The message is as it was.
            assert_eq!(request, before);
        }
        Ok(())
    }

    #[test]
    fn a_message_that_is_another_one_each_time_it_is_read_is_no_panic() -> Result<(), Error> {
        // The macro reads the expression of its message more than once: one that gives
        // a message with the IE, then one without it, has nowhere to write the IE.
        struct Each {
            messages: [UEContextReleaseRequest; 2],
            reads: usize,
        }
        impl Each {
            fn next(&mut self) -> &mut UEContextReleaseRequest {
                self.reads += 1;
                &mut self.messages[usize::from(self.reads > 2)]
            }
        }
        let with = build_ngap_ie!(UEContextReleaseRequest, REJECT AMF_UE_NGAP_ID(1u64))?;
        let mut each = Each {
            messages: [
                UEContextReleaseRequest::new(ProtocolIEContainer(vec![with])),
                UEContextReleaseRequest::new(ProtocolIEContainer(vec![])),
            ],
            reads: 0,
        };
        let changed = with_ngap_ie_mut!(
            each.next(),
            UEContextReleaseRequest,
            AMF_UE_NGAP_ID(id) = 2u64
        );
        assert_eq!(changed, Ok(false));
        assert_eq!(each.reads, 3);
        Ok(())
    }

    #[test]
    fn builder_macros_cover_ie_and_outcome_forms() -> Result<(), Error> {
        let ie = build_ngap_ie!(
            UEContextReleaseRequest,
            REJECT AMF_UE_NGAP_ID(1u64)
        )?;
        assert_eq!(ie.id.0, 10);

        // The direction and the procedure code of a PDU are those of its message.
        let initiating = build_ngap!(InitiatingMessage, NGSetup, REJECT, NGSetupRequest,)?;
        assert!(initiating.is_initiating());
        assert_eq!(initiating.procedure_code(), 21);

        let successful = build_ngap!(SuccessfulOutcome, NGSetup, REJECT, NGSetupResponse,)?;
        assert!(successful.is_successful());
        assert_eq!(successful.procedure_code(), 21);

        let unsuccessful = build_ngap!(UnsuccessfulOutcome, NGSetup, REJECT, NGSetupFailure,)?;
        assert!(unsuccessful.is_unsuccessful());
        assert_eq!(unsuccessful.procedure_code(), 21);
        Ok(())
    }

    /// The IEs whose type is an OCTET STRING (CONTAINING ...) carry the
    /// encoded transfer as octets.
    #[test]
    fn octet_string_containing_ies_have_macros() -> Result<(), Error> {
        let transfer = MBSSessionSetupOrModRequestTransfer::new(ProtocolIEContainer(vec![]));
        let octets = rasn::aper::encode(&transfer).expect("encode transfer");
        let ie = build_ngap_ie!(
            BroadcastSessionSetupRequest,
            REJECT MBSSessionSetupRequestTransfer(octets.clone())
        )?;
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
