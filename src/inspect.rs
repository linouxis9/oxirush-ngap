//! Decoded PDUs as JSON, in two forms: a view, which is read and edited by paths and
//! encoded again, and a tree, the short form in which a message is shown and written.
//!
//! The tree of a PDU is the name of its message and its IEs by name. [`message_tree`]
//! shows it, and [`message_from_tree`] gives the PDU of one that is written. The view
//! of a PDU keeps what was received, and the rest of this page is about it.
//!
//! [`message_view`] returns `procedure_code`, `direction`, `criticality` and `message`: the
//! message in the ASN.1 JSON encoding (JER). An IE whose identifier is known has its typed
//! `value` (`extensionValue` in an extension container) in place of its octets. Its type
//! is that of its identifier, and for an IE of an extension container the one that the
//! type which the container extends gives it: id-CurrentQoSParaSetIndex is an
//! `AlternativeQoSParaSetIndex` in a QoS flow that is associated with a tunnel, and an
//! `AlternativeQoSParaSetNotifyIndex` in one that a notification is about. A contained
//! transfer, and the value of an IE declared `OCTET STRING (CONTAINING ...)`, is its
//! `decoded` value beside its `_raw_transfer`. Repeated IEs stay ordered array entries.
//!
//! The values of some well-known types are shown as they are usually written:
//!
//! - a PLMN identity as its MCC and MNC, `"208-93"`;
//! - a transport layer address as an IP address, `"10.0.0.1"` or `"2001:db8::1"`, or as
//!   the two with a comma when it holds both;
//! - an identifier or a counter, such as a TAC, a GTP-TEID, a TMSI, a cell identity, the
//!   parts of a GUAMI or an SST, as a number.
//!
//! [`message_from_view`] takes them in this form, a number also as a `"0x…"` string, and
//! as JER writes them. It takes the name of an ENUMERATED value as any other name: see
//! [Names](#names). A value that does not fit its form, such as a transport layer address
//! of another length, stays as JER writes it. The other strings stay in hexadecimal: an
//! SD, keys and algorithm masks, the NAS-PDU and the other containers, and the node
//! identifiers, whose length is part of their value.
//!
//! The members that start with `_` say what was received:
//!
//! - `_raw_value`, `_raw_transfer` and `_raw_message`: the octets, in hexadecimal;
//! - `_original_id` and `_ie_name`: the identifier that the octets were decoded as, and the
//!   name that ASN.1 gives it. An entry whose `id` was changed is encoded as the type of
//!   its new identifier;
//! - `_decode_error`: why an IE's `value` is still its octets in hexadecimal, or why a
//!   transfer has no `decoded` member. The identifier is unknown, or is one of an
//!   extension where it extends no type that gives it one, the octets do not decode, JER
//!   does not write the value or does not read it back as it is, or the IE is nested
//!   deeper than 64 levels. The rest of the PDU is read all the same;
//! - `_edited`: [`set`], [`remove`] or [`insert`] changed something under the value, whose
//!   octets are no longer those beside it.
//!
//! [`message_from_view`] encodes the view. What was not edited keeps the octets received.
//! To edit:
//!
//! - change a typed `value` or the `decoded` member of a transfer, which is encoded again.
//!   This is refused when the octets received do not decode and encode back to themselves,
//!   as with an extension addition that the typed value does not keep;
//! - to add an IE, write an entry with its `id`, its `criticality` and its `value`, and no
//!   `_raw_value`: the value is encoded as the type of the IE, whatever JSON it is.
//!   The `id` is a number, or the name of the IE. An IE is written this way wherever a
//!   view has IEs: in a message that a transfer contains, in a value that is set whole,
//!   and in a view written by hand;
//! - to send given octets as an IE, write them in hexadecimal as its `octets`, or as
//!   `_raw_value`, and leave `value` out. For a transfer, replace the member with its
//!   octets, or with an object that has them as its `octets` alone;
//! - to add a transfer, or the value of an IE that contains a type, write it as an object
//!   with its `decoded` value alone, which is encoded as the type contained, or as that
//!   value itself: an object that has no member of a transfer, which are `decoded`,
//!   `octets` and those that start with `_`, is its decoded value;
//! - an IE with a `_decode_error` has its octets as `value`: change them there;
//! - the octets of an IE whose content the specification gives by reference are also
//!   written as the type that they carry, by its name: see
//!   [Octets that carry a type](#octets-that-carry-a-type).
//!
//! Beside a `value`, `_raw_value` is what the value is compared with, as `_raw_message` is
//! for the message, so changing one alone sends nothing else. A member that the ASN.1 type
//! does not have is refused, and so is one beside the `decoded` value of a transfer.
//! Hexadecimal is taken in either case.
//!
//! # Paths
//!
//! [`paths`] gives each value of a view with the path that selects it, [`select`] the
//! values at a path, and [`set`], [`remove`] and [`insert`] edit a view at a path:
//!
//! ```text
//! /ngap/AMF-UE-NGAP-ID/criticality = "reject"
//! /ngap/AMF-UE-NGAP-ID/value = 1
//! /ngap/Cause/value/radioNetwork = "user-inactivity"
//! /procedure_code = 42
//! ```
//!
//! A path is a JSON pointer into the view, where `/ngap` stands for the IEs of the
//! message. The segment after it selects among them:
//!
//! - the name of an IE, as ASN.1 has it after `id-`: `/ngap/RAN-UE-NGAP-ID`. An IE that is
//!   there twice is selected twice;
//! - a position, `/ngap/0`, which is how [`paths`] names an IE that has no name or is
//!   there twice. A position is a number as decimal writes it: `0`, `12`;
//! - `@id=N`, the IEs with that identifier, whether it has a name or not, and `*`, each
//!   of them;
//! - `-`, the end of the list, where [`insert`] adds an IE.
//!
//! Under an IE are its `criticality`, its typed `value`, and its `octets`: what it was
//! received as, in hexadecimal. The IEs of a message that the view contains, as the
//! `decoded` value of a transfer, are selected the same way in its place:
//!
//! ```text
//! /ngap/PDUSessionResourceSetupListSUReq/value/0/pDUSessionResourceSetupRequestTransfer/decoded/PDUSessionType/value = "ipv4"
//! ```
//!
//! Any other list selects by position and `*`.
//!
//! `value` and `decoded` may be left out of a path. A segment that names no member of an
//! IE, which has `id`, `criticality`, `value`, `octets` and the members that start with
//! `_`, is taken in its value, and one that names no member of a transfer, which has
//! `decoded`, `octets` and the members that start with `_`, in its decoded value. So
//! `/ngap/Cause/radioNetwork` is `/ngap/Cause/value/radioNetwork`, and these two paths
//! select the same value:
//!
//! ```text
//! /ngap/PDUSessionResourceSetupListSUReq/value/0/pDUSessionResourceSetupRequestTransfer/decoded/QosFlowSetupRequestList/value/0/qosFlowIdentifier
//! /ngap/PDUSessionResourceSetupListSUReq/0/pDUSessionResourceSetupRequestTransfer/QosFlowSetupRequestList/0/qosFlowIdentifier
//! ```
//!
//! What an IE or a transfer has itself comes first: `/ngap/Cause/value` is the value of
//! the IE and `/ngap/Cause/*` each of its members, so the entries of a list that is the
//! value of an IE are `value/*`. A member of a value that is named as one of its IE is
//! reached after `value`: only a string of bits whose size varies has one, its `value`,
//! as in `/ngap/TraceCollectionEntityIPAddress/value/value`. The value of an IE of an
//! extension container is its `extensionValue`, and is left out the same way.
//!
//! After an entry of a list of IEs, or an IE that a value holds alone, a segment may be
//! the name of that IE, and the path goes on from the IE:
//!
//! ```text
//! /ngap/PDUSessionResourceSetupListSUReq/0/pDUSessionResourceSetupRequestTransfer/protocolIEs/0/UL-NGU-UP-TNLInformation/gTPTunnel/gTP-TEID
//! ```
//!
//! So the names and the positions that [`message_tree`] shows, joined after the root,
//! are the path of a value in the view of the same PDU. A path that ends with the name
//! of an IE selects the IE, as its name alone does in a list, and `value` after it its
//! value.
//!
//! Under an IE, a segment is a member of the IE, then the name of the IE, then a member
//! of its value. The value of no IE of TS 38.413 has a member that is named as the IE
//! itself, so the order changes the meaning of no path into a value, and a member that
//! is named as another IE is that member, as the `targetRANNodeID` of a `TargetID`.
//! Were a value to have one, the segment would be the IE where it spells the name as
//! ASN.1 spells that of the IE, and the member otherwise. An IE of the same name that
//! the value holds, in a list or in a message that it contains, is reached after
//! `value`. The name of another IE, where the value has no such member, is an error
//! that says which IE the entry is.
//!
//! ## Names
//!
//! A name is its letters and its digits, whatever their case and whatever is between
//! them: `/ngap/RAN-UE-NGAP-ID`, `/NGAP/ran_ue_ngap_id` and `/ngap/ranuengapid` are one
//! path. The root, the name of an IE, the members of a value, the name of a message
//! ([`message_named`], [`check_path`]) and the name of an ENUMERATED value are all taken
//! this way, and no two of them that could be taken for each other are the same. A
//! member that starts with `_` is named as it is.
//!
//! ## What is not there
//!
//! A path selects nothing where the message could have a value and does not:
//!
//! - an IE that the message does not have;
//! - an OPTIONAL member that is absent, and another alternative of a CHOICE:
//!   `/ngap/Cause/value/nas` when the cause is one of the radio network;
//! - what follows in the path, which is still checked against the type.
//!
//! A name that the type of a value cannot have is an error, which lists the members
//! that the type has: a member that is written wrong is not taken for one that is not
//! there. Under an IE or a transfer, it lists those of its value, then those of the IE
//! or of the transfer itself. So are a member of a value that has none, a path into a
//! value that did not decode, and the name of an IE in a list whose entries are no IEs.
//! After `*`, a segment is an error when none of the values that it is applied to can
//! have it.
//!
//! [`check_path`] says the same of a path without a view: whether it can select
//! anything in a message of a name, whose IEs are those of its object set
//! ([`message_ies`]).
//!
//! ## Edits
//!
//! [`paths`] lists the values of a view without the members that start with `_`, except
//! `_decode_error`. The octets of an IE or of a transfer are selected as its `octets`,
//! which [`paths`] lists only for one that has no value beside them, and set there in
//! place of its value. Those of a value that an edit changed are an error to select:
//! [`message_from_view`] gives the PDU its octets.
//!
//! [`set`] writes a member by the name that ASN.1 gives it, and refuses a name that the
//! type does not have. `null` takes an optional member out, as [`remove`] does; a member
//! that its type always has is not taken out, and an alternative of a CHOICE takes the
//! place of the one that is there. The value of an IE or of a transfer is not taken out
//! either, as its octets would be sent in its place.
//!
//! ```
//! use oxirush_ngap::{build_ngap, inspect, ngap::*};
//! use serde_json::json;
//!
//! let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
//!     IGNORE, UEContextReleaseRequest,
//!     REJECT AMF_UE_NGAP_ID(1u64),
//!     REJECT RAN_UE_NGAP_ID(7u32),
//!     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
//! );
//! let mut view = inspect::message_view(&pdu)?;
//! assert_eq!(inspect::select(&view, "/ngap/RAN-UE-NGAP-ID/value")?, [&json!(7)]);
//!
//! inspect::set(&mut view, "/ngap/Cause/value", json!({"nas": "normal-release"}))?;
//! inspect::remove(&mut view, "/ngap/AMF-UE-NGAP-ID")?;
//! let name = json!({"id": "RANNodeName", "criticality": "ignore", "value": "gnb-1"});
//! inspect::insert(&mut view, "/ngap/-", name)?;
//! let edited = inspect::message_view(&inspect::message_from_view(&view)?)?;
//! assert_eq!(inspect::select(&edited, "/ngap/*/id")?, [&json!(85), &json!(15), &json!(82)]);
//! # Ok::<(), String>(())
//! ```
//!
//! # Octets that carry a type
//!
//! TS 38.413 gives the content of three IEs by reference, where the ASN.1 has an
//! `OCTET STRING`. Between NG-RAN nodes:
//!
//! | The octets of | carry a | Clause |
//! | --- | --- | --- |
//! | `SourceToTarget-TransparentContainer` | `SourceNGRANNode-ToTargetNGRANNode-TransparentContainer` | 9.3.1.20 |
//! | `TargetToSource-TransparentContainer` | `TargetNGRANNode-ToSourceNGRANNode-TransparentContainer` | 9.3.1.21 |
//! | `TargettoSource-Failure-TransparentContainer` | `TargetNGRANNode-ToSourceNGRANNode-FailureTransparentContainer` | 9.3.1.186 |
//!
//! For another target system they are octets of its own specification, and nothing in a
//! message says which they are. So a view and a tree show such an IE by its octets,
//! and the one that writes or reads it names the type, in any case:
//!
//! - written, the value of the IE is an object with one member, the name of the type
//!   with its value, which is encoded as that type:
//!   `{"SourceNGRANNode-ToTargetNGRANNode-TransparentContainer": {"rRCContainer": "…", …}}`.
//!   [`message_from_tree`] takes it, and so does [`message_from_view`] for a value that
//!   [`set`] or [`insert`] wrote and for a view written by hand;
//! - read, a path goes through the name of the type, after the IE or its `value`:
//!   `/ngap/SourceToTarget-TransparentContainer/SourceNGRANNode-ToTargetNGRANNode-TransparentContainer/targetCell-ID`.
//!   [`check_path`] and [`enumerated_at`] read it against the types. In a view,
//!   [`open`] decodes the octets that a path takes so, and nothing else does:
//!   [`select`] gives values of the view, and an edit changes what the view has. On
//!   octets that were not opened, such a path is an error that says so.
//!
//! Once it is opened or written so, the value of the IE is the transfer of that type
//! under its name, with what any transfer has. Its `decoded` may be left out of a
//! path, its `octets` are those that were received, and [`paths`] lists what it has.
//! Octets that are not of the type have a `_decode_error`, which a path into them
//! reports, and what is not edited is sent as the octets received. The `rRCContainer`
//! in it stays octets: it is a message of RRC.

use serde_json::{Value, json};

use crate::inspect_paths::{
    EDITED, Shape, carried_shape, entry_shape, extended, listed_ies, member_shape, member_shapes,
    message_shape, named_ies, of_a_transfer, open_with, same_name, written_ie,
};
pub use crate::inspect_paths::{check_path, enumerated_at, insert, paths, remove, select, set};
use crate::inspect_registry as registry;
use crate::ngap::NGAP_PDU;

/// The identifier and the name of each IE that an object set of the ASN.1 has, those of
/// the extension containers included. An identifier that the ASN.1 defines and that no
/// object set uses has no type, and is not among them.
pub fn ie_names() -> &'static [(u16, &'static str)] {
    registry::IE_NAMES
}

/// The messages, each as the `direction` and the `procedure_code` that a view has for it
/// and the name that ASN.1 gives it.
pub fn message_names() -> impl Iterator<Item = (&'static str, u8, &'static str)> {
    let messages = registry::MESSAGES.iter();
    messages.map(|(direction, code, name, ..)| (*direction, *code, *name))
}

/// The name that ASN.1 gives the message of a PDU, as `InitialContextSetupResponse`; `None`
/// for a procedure that the specification does not have in that direction.
pub fn message_name(pdu: &NGAP_PDU) -> Option<&'static str> {
    let (direction, code) = (pdu.direction(), pdu.procedure_code());
    message_names()
        .find(|(of, procedure, _)| (*of, *procedure) == (direction, code))
        .map(|(.., name)| name)
}

/// The `direction` and the `procedure_code` of the message that `name` names: a name is
/// its letters and its digits, whatever their case and whatever is between them.
pub fn message_named(name: &str) -> Option<(&'static str, u8)> {
    message_names()
        .find(|(.., known)| same_name(known, name))
        .map(|(direction, code, _)| (direction, code))
}

/// The IEs that the message `name` can have, as its ASN.1 object set lists them: the
/// identifier of each and whether its presence is mandatory, in the order of the set. The
/// name is the one that ASN.1 gives the message, taken as [`message_named`] takes it.
/// `None` for a name that is no message; a message without an object set of IEs, as a
/// private message, has none.
pub fn message_ies(name: &str) -> Option<&'static [(u16, bool)]> {
    let mut messages = registry::MESSAGES.iter();
    let message = messages.find(|(_, _, known, ..)| same_name(known, name));
    message.map(|(_, _, _, _, ies, _)| *ies)
}

/// The criticality that the ASN.1 object set of the message `name` assigns to each of its
/// IEs: the identifier of each and `"reject"`, `"ignore"` or `"notify"`, in the order of
/// the set, which is that of [`message_ies`]. The name is the one that ASN.1 gives the
/// message, taken as [`message_named`] takes it. `None` for a name that is no message; a
/// message without an object set of IEs, as a private message, has none.
pub fn message_ie_criticalities(name: &str) -> Option<&'static [(u16, &'static str)]> {
    let mut messages = registry::MESSAGE_IE_CRITICALITIES.iter();
    let message = messages.find(|(known, _)| same_name(known, name));
    message.map(|(_, ies)| *ies)
}

/// The criticality that ASN.1 assigns to the procedure of the message `name`, which a PDU
/// has beside its procedure code: `"reject"`, `"ignore"` or `"notify"`. The name is the
/// one that ASN.1 gives the message, taken as [`message_named`] takes it. `None` for a
/// name that is no message.
pub fn message_criticality(name: &str) -> Option<&'static str> {
    let (_, code) = message_named(name)?;
    let mut procedures = registry::PROCEDURE_CRITICALITIES.iter();
    let procedure = procedures.find(|(known, _)| *known == code);
    procedure.map(|(_, criticality)| *criticality)
}

/// The criticality that ASN.1 assigns to the IE `id` where a value holds it alone, in a
/// `ProtocolIE-SingleContainer`, as the alternative that extends a CHOICE: `"reject"`,
/// `"ignore"` or `"notify"`. `None` for an IE that no object set of a single container
/// has, as the IEs of a message or of a transfer, and for one that two of these sets give
/// two criticalities.
pub fn item_criticality(id: u16) -> Option<&'static str> {
    let mut items = registry::ITEM_CRITICALITIES.iter();
    let item = items.find(|(known, _)| *known == id);
    item.map(|(_, criticality)| *criticality)
}

/// The functions of the type of the message of a direction and a procedure code.
fn message_type(direction: &str, code: u8) -> Result<Typed, String> {
    let mut messages = registry::MESSAGES.iter();
    let message = messages.find(|(of, procedure, ..)| (*of, *procedure) == (direction, code));
    let protocol = registry::PROTOCOL;
    message
        .map(|(_, _, _, typed, ..)| typed())
        .ok_or_else(|| format!("unknown {protocol} direction/procedure {direction}/{code}"))
}

/// The value of the type `T` that the octets `raw` are, as JER writes it. An error for
/// a value that JER does not read back as it is: it is not shown as what it is not.
fn decode_typed<T>(raw: &[u8]) -> Result<Value, String>
where
    T: rasn::Decode + rasn::Encode + RepairOpenTypes + PartialEq,
{
    let value = crate::ngap::decode_open_type::<T>(&rasn::types::Any::new(raw.to_vec()))
        .map_err(|e| e.to_string())?;
    let shown: Value = serde_json::from_str(&rasn::jer::encode(&value).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    // rasn's JER writes a string of bits of one extensible size as its octets, without
    // its length: one of another size would be shown as a string that it is not.
    const LOST: &str = "JER does not represent this value";
    let (mut read, _) = decode_jer::<T>(&shown).map_err(|error| format!("{LOST}: {error}"))?;
    read.repair_open_types()?;
    match read == value {
        true => Ok(shown),
        false => Err(LOST.into()),
    }
}

pub(crate) trait RepairOpenTypes {
    fn repair_open_types(&mut self) -> Result<(), String>;
}

/// Implements [`RepairOpenTypes`] for a generated type, from the fields, or the CHOICE
/// alternatives, through which it reaches an open type.
macro_rules! open_types {
    ($type:path { $($open:ident),* } else { $($closed:ident),* }) => {
        impl $crate::inspect::RepairOpenTypes for $type {
            fn repair_open_types(&mut self) -> Result<(), String> {
                match self {
                    $(Self::$open(value) => {
                        $crate::inspect::RepairOpenTypes::repair_open_types(value)
                    })*
                    $(Self::$closed { .. } => Ok(()),)*
                }
            }
        }
    };
    ($type:path { $($field:tt),* }) => {
        impl $crate::inspect::RepairOpenTypes for $type {
            fn repair_open_types(&mut self) -> Result<(), String> {
                $($crate::inspect::RepairOpenTypes::repair_open_types(&mut self.$field)?;)*
                Ok(())
            }
        }
    };
}
pub(crate) use open_types;

impl RepairOpenTypes for rasn::types::Any {
    fn repair_open_types(&mut self) -> Result<(), String> {
        // JER Any retains JSON text. APER Any needs decoded open-type bytes.
        let json: Value = serde_json::from_slice(self.as_bytes()).map_err(|e| e.to_string())?;
        *self = Self::new(unhex(&json)?);
        Ok(())
    }
}
impl RepairOpenTypes for rasn::types::OctetString {
    fn repair_open_types(&mut self) -> Result<(), String> {
        Ok(())
    }
}
impl<T: RepairOpenTypes> RepairOpenTypes for Option<T> {
    fn repair_open_types(&mut self) -> Result<(), String> {
        if let Some(value) = self {
            value.repair_open_types()?;
        }
        Ok(())
    }
}
impl<T: RepairOpenTypes> RepairOpenTypes for Vec<T> {
    fn repair_open_types(&mut self) -> Result<(), String> {
        for value in self {
            value.repair_open_types()?;
        }
        Ok(())
    }
}
impl<T: RepairOpenTypes> RepairOpenTypes for Box<T> {
    fn repair_open_types(&mut self) -> Result<(), String> {
        self.as_mut().repair_open_types()
    }
}

/// The value of the type `T` that JER reads `value` as, and `value` with the names of
/// ENUMERATED values as the specification spells them.
fn decode_jer<T: rasn::Decode>(value: &Value) -> Result<(T, std::borrow::Cow<'_, Value>), String> {
    let mut value = std::borrow::Cow::Borrowed(value);
    let mut respellings = 0;
    loop {
        match rasn::jer::decode::<T>(&value.to_string()) {
            Ok(typed) => return Ok((typed, value)),
            Err(error) => match respelled(&value, &error).filter(|_| respellings < 64) {
                Some(respelled) => {
                    value = std::borrow::Cow::Owned(respelled);
                    respellings += 1;
                }
                None => return Err(error.to_string()),
            },
        }
    }
}

fn encode_typed<T: rasn::Decode + rasn::Encode + RepairOpenTypes>(
    value: &Value,
) -> Result<Vec<u8>, String> {
    let (mut typed, value) = decode_jer::<T>(value)?;
    typed.repair_open_types()?;
    let checked: Value =
        serde_json::from_str(&rasn::jer::encode(&typed).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    if !written_as(&checked, &value) {
        return Err("the value has a member that its ASN.1 type does not have, or is not written as JER writes it".into());
    }
    Ok(crate::ngap::encode_open_type(&typed)
        .map_err(|e| e.to_string())?
        .as_bytes()
        .to_vec())
}

/// Whether `written` is the value that JER writes as `checked`: the same, with
/// hexadecimal in either case.
fn written_as(checked: &Value, written: &Value) -> bool {
    match (checked, written) {
        (Value::String(checked), Value::String(written)) => {
            checked == written
                || (checked.eq_ignore_ascii_case(written)
                    && checked.bytes().all(|digit| digit.is_ascii_hexdigit()))
        }
        (Value::Array(checked), Value::Array(written)) => {
            checked.len() == written.len()
                && checked.iter().zip(written).all(|(c, w)| written_as(c, w))
        }
        (Value::Object(checked), Value::Object(written)) => {
            checked.len() == written.len()
                && (checked.iter())
                    .all(|(name, c)| written.get(name).is_some_and(|w| written_as(c, w)))
        }
        _ => checked == written,
    }
}

/// The transfers of the registry: the member that holds each, and the type that its
/// octets contain. A member that has several types has its name alone.
macro_rules! transfers {
    ($($field:literal $($transfer:path)?;)*) => {
        pub(crate) const TRANSFER_FIELDS: &[&str] = &[$($field,)*];
        #[allow(clippy::match_single_binding)]
        pub(crate) fn transfer(field: &str) -> Result<Typed, String> {
            match field {
                $($($field => Ok(Typed::of::<$transfer>()),)?)*
                _ => Err(format!(
                    "{PROTOCOL} contained transfer {field} is unknown or has several types"
                )),
            }
        }
    };
}
pub(crate) use transfers;

/// The IEs of the registry whose octets may carry a type that the specification names
/// and the ASN.1 does not: the identifier of each, the name that ASN.1 gives the type,
/// and the type.
macro_rules! carried {
    ($($id:literal $name:literal $carried:path;)*) => {
        pub(crate) const CARRIED: &[(u16, &str, &str)] =
            &[$(($id, $name, stringify!($carried)),)*];
        pub(crate) fn carried(name: &str) -> Option<Typed> {
            $(if $crate::inspect_paths::same_name($name, name) {
                return Some(Typed::of::<$carried>());
            })*
            None
        }
    };
}
pub(crate) use carried;

/// The functions of a type of the registry.
pub(crate) struct Typed {
    pub(crate) decode: fn(&[u8]) -> Result<Value, String>,
    pub(crate) encode: fn(&Value) -> Result<Vec<u8>, String>,
}

impl Typed {
    pub(crate) fn of<T: rasn::Decode + rasn::Encode + RepairOpenTypes + PartialEq>() -> Self {
        Self {
            decode: decode_typed::<T>,
            encode: encode_typed::<T>,
        }
    }
}

fn hex(raw: &[u8]) -> String {
    const DIGITS: &[u8] = b"0123456789ABCDEF";
    raw.iter()
        .flat_map(|b| {
            [
                DIGITS[(b >> 4) as usize] as char,
                DIGITS[(b & 15) as usize] as char,
            ]
        })
        .collect()
}

fn unhex(value: &Value) -> Result<Vec<u8>, String> {
    let s = value.as_str().ok_or("open type must be a hex string")?;
    if !s.len().is_multiple_of(2) {
        return Err("odd hex length".into());
    }
    s.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let a = (pair[0] as char).to_digit(16).ok_or("invalid hex")?;
            let b = (pair[1] as char).to_digit(16).ok_or("invalid hex")?;
            Ok(((a << 4) | b) as u8)
        })
        .collect()
}

/// How a view and a tree show the values of a well-known type, and take them back.
#[derive(Clone, Copy)]
pub(crate) enum Form {
    /// A PLMN identity as its MCC and MNC: `"208-93"`.
    Plmn,
    /// The digits of an identity in TBCD, as an IMSI has them. A protocol may have no
    /// type of this form.
    #[allow(dead_code)]
    Digits,
    /// An identifier or a counter of this many bits, as a number.
    Number(u32),
    /// A transport layer address: an IPv4 address, an IPv6 address, or both with a comma.
    Address,
}

impl Form {
    /// The readable form of a value as JER has it; `None` when it has none.
    fn read(self, jer: &Value) -> Option<Value> {
        // The decimal digits that these half octets are; none when one of them is no digit.
        let digits = |digits: &[u8]| {
            let text = digits
                .iter()
                .map(|digit| char::from_digit((*digit).into(), 10));
            text.collect::<Option<String>>()
        };
        match self {
            Self::Plmn => {
                // The order of the digits is that of the helpers.
                let octets = <[u8; 3]>::try_from(unhex(jer).ok()?).ok()?;
                let (mcc, mnc) = crate::helpers::plmn_digits(octets);
                Some(json!(format!("{}-{}", digits(&mcc)?, digits(&mnc)?)))
            }
            Self::Digits => {
                let octets = unhex(jer).ok()?;
                let nibbles: Vec<u8> = octets.iter().flat_map(|o| [o & 15, o >> 4]).collect();
                // A filler takes the place of a last digit that is not there.
                let nibbles = nibbles.strip_suffix(&[15]).unwrap_or(&nibbles);
                digits(nibbles).map(Value::from)
            }
            Self::Number(bits) => {
                let (text, octets) = (jer.as_str()?, bits.div_ceil(8));
                let number = u64::from_str_radix(text, 16).ok()?;
                (text.len() as u32 == octets * 2).then(|| json!(number >> (octets * 8 - bits)))
            }
            Self::Address => {
                let octets = unhex(jer.get("value")?).ok()?;
                let v4 = |octets: &[u8]| {
                    Some(std::net::Ipv4Addr::from(<[u8; 4]>::try_from(octets).ok()?))
                };
                let v6 = |octets: &[u8]| {
                    Some(std::net::Ipv6Addr::from(<[u8; 16]>::try_from(octets).ok()?))
                };
                Some(json!(match (jer.get("length")?.as_u64()?, octets.len()) {
                    (32, 4) => v4(&octets)?.to_string(),
                    (128, 16) => v6(&octets)?.to_string(),
                    (160, 20) => format!("{},{}", v4(&octets[..4])?, v6(&octets[4..])?),
                    _ => return None,
                }))
            }
        }
    }

    /// The value as JER has it, from its readable form. A value in another form is kept
    /// as it is: the JER form is still taken, and the codec refuses what is neither.
    fn write(self, shown: &Value) -> Result<Value, String> {
        let number = |bits: u32, number: u64| {
            if bits < 64 && number >> bits != 0 {
                return Err(format!("{number} does not fit in {bits} bits"));
            }
            let octets = bits.div_ceil(8);
            let width = octets as usize * 2;
            Ok(json!(format!("{:0width$X}", number << (octets * 8 - bits))))
        };
        let Some(text) = shown.as_str() else {
            return match (self, shown.as_u64()) {
                (Self::Number(bits), Some(value)) => number(bits, value),
                _ => Ok(shown.clone()),
            };
        };
        let bcd = |digits: &str| digits.bytes().map(|digit| digit & 15).collect::<Vec<_>>();
        let all_digits = |text: &str| text.bytes().all(|b| b.is_ascii_digit());
        match self {
            Self::Plmn if text.contains('-') => {
                let (mcc, mnc) = text.split_once('-').unwrap_or_default();
                if mcc.len() != 3
                    || !(2..=3).contains(&mnc.len())
                    || !all_digits(&text.replacen('-', "", 1))
                {
                    return Err(format!("{text}: a PLMN identity is MCC-MNC, as 208-93"));
                }
                let (mcc, mnc) = (bcd(mcc), bcd(mnc));
                let octets = crate::helpers::plmn_octets([mcc[0], mcc[1], mcc[2]], &mnc);
                Ok(json!(hex(&octets)))
            }
            Self::Digits if all_digits(text) => {
                let pairs = bcd(text);
                let pairs = pairs.chunks(2);
                let octets = pairs.map(|pair| pair.get(1).copied().unwrap_or(15) << 4 | pair[0]);
                Ok(json!(hex(&octets.collect::<Vec<_>>())))
            }
            Self::Number(bits) if text.starts_with("0x") => {
                let value = u64::from_str_radix(&text[2..], 16);
                number(bits, value.map_err(|error| format!("{text}: {error}"))?)
            }
            Self::Address if text.contains(['.', ':']) => {
                let mut octets = Vec::new();
                for part in text.split(',') {
                    match (octets.len(), part.trim().parse()) {
                        (0, Ok(std::net::IpAddr::V4(address))) => octets.extend(address.octets()),
                        (0 | 4, Ok(std::net::IpAddr::V6(address))) => {
                            octets.extend(address.octets())
                        }
                        _ => {
                            return Err(format!(
                                "{text}: a transport layer address is an IPv4 address, an IPv6 address, or the two with a comma"
                            ));
                        }
                    }
                }
                Ok(json!({"value": hex(&octets), "length": octets.len() * 8}))
            }
            _ => Ok(shown.clone()),
        }
    }
}

/// Show `value`, or each value of a list, in the readable form that it has: only a value
/// that the form gives back as it was.
fn show(form: Form, value: &mut Value) {
    match value {
        Value::Array(values) => values.iter_mut().for_each(|value| show(form, value)),
        value => {
            let shown = form.read(value);
            if let Some(shown) =
                shown.filter(|shown| form.write(shown).is_ok_and(|jer| jer == *value))
            {
                *value = shown;
            }
        }
    }
}

/// `value`, or each value of a list, as JER has it again.
fn unshow(form: Form, value: &mut Value) -> Result<(), String> {
    match value {
        Value::Array(values) => values.iter_mut().try_for_each(|value| unshow(form, value)),
        value => {
            *value = form.write(value)?;
            Ok(())
        }
    }
}

/// `value` with the name that JER did not find among those of an ENUMERATED type as the
/// specification spells it: a name is its letters and its digits, whatever their case
/// and whatever is between them.
fn respelled(value: &Value, error: &rasn::error::DecodeError) -> Option<Value> {
    use rasn::error::{CodecDecodeError, DecodeErrorKind, JerDecodeErrorKind};
    // The error of a member is in the errors of the fields that lead to it.
    let mut error = error;
    while let DecodeErrorKind::FieldError { nested, .. } = &*error.kind {
        error = nested;
    }
    let DecodeErrorKind::CodecSpecific {
        inner: CodecDecodeError::Jer(JerDecodeErrorKind::InvalidEnumDiscriminant { discriminant }),
    } = &*error.kind
    else {
        return None;
    };
    let same = |name: &&&str| same_name(name, discriminant);
    // Two types may spell a name differently: the next decoding tries the other.
    let mut names = registry::ENUMERATED.iter().filter(same);
    let name = names.find(|name| **name != discriminant.as_str())?;
    fn rename(value: &mut Value, from: &str, to: &str) {
        match value {
            Value::String(text) if text == from => *text = to.into(),
            Value::Array(values) => values.iter_mut().for_each(|value| rename(value, from, to)),
            Value::Object(members) => members
                .values_mut()
                .for_each(|value| rename(value, from, to)),
            _ => {}
        }
    }
    let mut value = value.clone();
    rename(&mut value, discriminant, name);
    Some(value)
}

/// The value that the octets `raw` contain, beside them, or why they did not decode.
fn contained(raw: Value, typed: Result<Typed, String>) -> Value {
    match typed.and_then(|typed| (typed.decode)(&unhex(&raw)?)) {
        Ok(decoded) => json!({"_raw_transfer": raw, "decoded": decoded}),
        Err(error) => json!({"_raw_transfer": raw, "_decode_error": error}),
    }
}

/// The octets `raw` as the transfer that they are when they carry the type that ASN.1
/// names `name`: its decoded value beside them, or why they are not of that type.
fn opened(raw: Value, name: &str) -> Value {
    let typed = registry::carried(name);
    let mut transfer = contained(raw, typed.ok_or(format!("octets carry no {name}")));
    expand(&mut transfer, 0, carried_shape(name));
    transfer
}

/// Decode, in `view`, the octets that `path` takes as a type that they may carry: those
/// of an IE whose content the specification gives by reference, as the transparent
/// container of a handover, where the segment after the IE or its `value` is the name
/// that ASN.1 gives one of these types. See
/// [Octets that carry a type](self#octets-that-carry-a-type).
///
/// The IE then has, in place of its octets, the transfer of that type under that name,
/// which [`select`], [`paths`] and the edits go into and [`message_from_view`] takes
/// back: its decoded value beside the octets, or why they are not of that type. Nothing
/// in a message says which type octets carry, so [`message_view`] leaves them as they
/// are, and nothing but this function decodes them: on octets that were not opened, a
/// path through the name of a type is an error that says so.
///
/// A path that goes through no such name changes nothing. A name that the type of a
/// value cannot have is an error, and the view is then as it was.
pub fn open(view: &mut Value, path: &str) -> Result<(), String> {
    open_with(view, path, &opened)
}

/// The octets of the IE `id`, which is written as a type that they may carry: one
/// member, the name of that type with its value, which is its octets by now.
fn carried_octets(written: &Value, id: u16) -> Result<Value, String> {
    let carried = registry::CARRIED
        .iter()
        .filter(|(carrier, ..)| *carrier == id);
    let names: Vec<_> = carried.map(|(_, name, _)| *name).collect();
    let mut members = written.as_object().into_iter().flatten();
    match (members.next(), members.next()) {
        (Some((name, octets)), None) if names.iter().any(|known| same_name(known, name)) => {
            Ok(octets.clone())
        }
        _ => Err(format!(
            "IE {id} is written as its octets, or as {{NAME: VALUE}} for a type that they carry: {}",
            names.join(", ")
        )),
    }
}

/// How deep in a view an IE or a transfer is still decoded.
const NESTING: usize = 64;

/// Why what is deeper keeps its octets.
fn too_deep<T>() -> Result<T, String> {
    Err(format!("inspection nesting exceeds {NESTING}"))
}

/// The functions of the type of the IE `id` in a container of `shape`: the type of its
/// identifier, or the one that the type which an extension container extends gives it.
fn ie_type(id: u16, shape: Shape) -> Result<Typed, String> {
    match crate::registry::extension(id, extended(shape)) {
        Some(typed) => Ok(typed),
        None => registry::ie(id),
    }
}

/// Decode the IEs and the transfers of `value`, a value of `shape` which is `depth` deep
/// in its view. What is deeper than [`NESTING`] keeps its octets, with why.
fn expand(value: &mut Value, depth: usize, shape: Shape) {
    let deep = depth > NESTING;
    if let Some(object) = value.as_object_mut() {
        let field = if object.contains_key("value") {
            "value"
        } else {
            "extensionValue"
        };
        if let Some(id) = object
            .get("id")
            .and_then(Value::as_u64)
            .and_then(|v| u16::try_from(v).ok())
            && let Some(raw) = object.get(field).filter(|v| v.is_string()).cloned()
        {
            object.insert("_raw_value".into(), raw.clone());
            object.insert("_original_id".into(), json!(id));
            if let Some((_, name)) = ie_names().iter().find(|(key, _)| *key == id) {
                object.insert("_ie_name".into(), json!(name));
            }
            let decoded = match deep {
                true => too_deep(),
                false => unhex(&raw).and_then(|bytes| (ie_type(id, shape)?.decode)(&bytes)),
            };
            match decoded {
                Ok(decoded) => {
                    // An OCTET STRING (CONTAINING ...) has the form of a transfer.
                    let mut decoded = match registry::ie_contents(id) {
                        Some(contents) => contained(decoded, Ok(contents)),
                        None => decoded,
                    };
                    if let Some(form) = registry::ie_form(id) {
                        show(form, &mut decoded);
                    }
                    object.insert(field.into(), decoded);
                }
                Err(error) => {
                    object.insert("_decode_error".into(), json!(error));
                }
            }
        }
        // What each member is, once the entry of an IE says what it was decoded as.
        let members = member_shapes(shape, object);
        for (key, child) in object {
            if !key.starts_with('_') {
                if registry::TRANSFER_FIELDS.contains(&key.as_str()) && child.is_string() {
                    let typed = match deep {
                        true => too_deep(),
                        false => registry::transfer(key),
                    };
                    *child = contained(child.take(), typed);
                }
                expand(child, depth + 1, member_shape(&members, key));
                if let Some(form) = registry::member_form(key) {
                    show(form, child);
                }
            }
        }
    } else if let Some(array) = value.as_array_mut() {
        for child in array {
            expand(child, depth + 1, entry_shape(shape));
        }
    }
}

/// `member` is the name of the member that holds `value`, a value of `shape`, or the
/// list it is in.
fn collapse(value: &mut Value, member: &str, depth: usize, shape: Shape) -> Result<(), String> {
    // What is decoded at the limit of the inspection has values of its own under it.
    if depth > 4 * NESTING {
        return Err(format!("inspection nesting exceeds {}", 4 * NESTING));
    }
    if let Some(object) = value.as_object_mut() {
        // An IE may be written with its name as `id`, and with its `octets`.
        written_ie(object)?;
        let id = object
            .get("id")
            .and_then(Value::as_u64)
            .and_then(|v| u16::try_from(v).ok());
        let members = member_shapes(shape, object);
        for (key, child) in object.iter_mut() {
            if !key.starts_with('_') {
                collapse(child, key, depth + 1, member_shape(&members, key))?;
                if let Some(form) = registry::member_form(key) {
                    unshow(form, child)?;
                }
                if let Some(raw) = child.get("_raw_transfer").cloned() {
                    // Nothing that is written is left out: a transfer has its decoded
                    // value beside the octets that it was received as, and no other.
                    let known = ["_raw_transfer", "decoded", "_decode_error", EDITED];
                    let mut members = child.as_object().into_iter().flatten();
                    if let Some((other, _)) =
                        members.find(|(name, _)| !known.contains(&name.as_str()))
                    {
                        return Err(format!(
                            "{key} has no member {other:?}: a transfer has its decoded value"
                        ));
                    }
                    if let Some(edited) = child.get("decoded") {
                        let bytes = unhex(&raw)?;
                        // An IE's identifier gives the type of what it contains, and
                        // the name of a transfer field the type of the transfer.
                        let transfer = match key.as_str() {
                            "value" | "extensionValue" => id
                                .and_then(registry::ie_contents)
                                .ok_or("this IE does not contain a type")?,
                            key => match registry::carried(key) {
                                Some(carried) => carried,
                                None => registry::transfer(key)?,
                            },
                        };
                        let original = (transfer.decode)(&bytes)?;
                        if &original == edited {
                            *child = raw;
                        } else {
                            if (transfer.encode)(&original)? != bytes {
                                return Err(format!(
                                    "{key} does not encode back to the octets received"
                                ));
                            }
                            *child = json!(hex(&(transfer.encode)(edited)?));
                        }
                    } else {
                        *child = raw;
                    }
                } else if let Some(members) = child.as_object() {
                    // What was not received is encoded from the value written.
                    let transfer = match key.as_str() {
                        "value" | "extensionValue" => id.and_then(registry::ie_contents),
                        key if registry::TRANSFER_FIELDS.contains(&key) => {
                            Some(registry::transfer(key)?)
                        }
                        // A type that the octets of an IE may carry, by its name.
                        key => registry::carried(key),
                    };
                    if let Some(transfer) = transfer {
                        // The decoded value, as the object itself when it has no member
                        // of a transfer or as its `decoded` member, or the `octets`.
                        let alone = |member| members.get(member).filter(|_| members.len() == 1);
                        let wire = if !members.keys().any(|name| of_a_transfer(name)) {
                            json!(hex(&(transfer.encode)(child)?))
                        } else if let Some(written) = alone("decoded") {
                            json!(hex(&(transfer.encode)(written)?))
                        } else if let Some(octets) = alone("octets") {
                            octets.clone()
                        } else {
                            return Err(format!(
                                "{key} is written with its decoded value alone, or as its octets"
                            ));
                        };
                        *child = wire;
                    }
                }
            }
        }
        let raw = object.remove("_raw_value");
        let original_id = object.remove("_original_id");
        let undecoded = object.remove("_decode_error").is_some();
        object.remove("_ie_name");
        object.remove(EDITED);
        if let Some(id) = id {
            // The value of an IE of an extension container has another name.
            let field = match member.starts_with("iE-Extension") {
                true => "extensionValue",
                false => "value",
            };
            // Octets that are written as a type that they may carry.
            if let Some(written) = object.get_mut(field).filter(|value| value.is_object())
                && registry::CARRIED.iter().any(|(carrier, ..)| *carrier == id)
            {
                *written = carried_octets(written, id)?;
            }
            let typed = |id: u16| {
                let octets = format!("give its octets as _raw_value, without {field}");
                ie_type(id, shape).map_err(|error| format!("{error}: {octets}"))
            };
            if !undecoded
                && let Some(form) = registry::ie_form(id)
                && let Some(value) = object.get_mut(field)
            {
                unshow(form, value)?;
            }
            let wire = match (raw, object.get(field)) {
                // The octets alone are sent as they are.
                (Some(raw), None) => raw,
                // The IE was shown as its octets.
                (_, Some(edited)) if undecoded => {
                    if !edited.is_string() {
                        return Err(format!(
                            "IE {id} did not decode: its value is its octets in hexadecimal"
                        ));
                    }
                    edited.clone()
                }
                (Some(raw), Some(edited)) => {
                    // The identifier that the octets were decoded as, when a view has
                    // one: without it they are those of the identifier of the entry.
                    let original_id = match &original_id {
                        Some(original) => (original.as_u64())
                            .and_then(|v| u16::try_from(v).ok())
                            .ok_or("_original_id is the identifier that the octets were decoded as, a number")?,
                        None => id,
                    };
                    let bytes = unhex(&raw)?;
                    let received = ie_type(original_id, shape)?;
                    let original = (received.decode)(&bytes)?;
                    if &original == edited && id == original_id {
                        raw
                    } else if (received.encode)(&original)? != bytes {
                        return Err(format!(
                            "IE {original_id} does not encode back to the octets received; replace its octets instead"
                        ));
                    } else {
                        json!(hex(&(typed(id)?.encode)(edited)?))
                    }
                }
                // An IE that was not received.
                (None, Some(new)) => json!(hex(&(typed(id)?.encode)(new)?)),
                (None, None) => return Err(format!("IE {id} has no {field} and no _raw_value")),
            };
            object.insert(field.into(), wire);
        }
    } else if let Some(array) = value.as_array_mut() {
        for child in array {
            collapse(child, member, depth + 1, entry_shape(shape))?;
        }
    }
    Ok(())
}

/// The tree of a PDU, which is the name of its message and its IEs by name, in their order:
///
/// ```json
/// {"message": "UEContextReleaseRequest",
///  "ies": [{"AMF-UE-NGAP-ID": 1}, {"RAN-UE-NGAP-ID": 2}, {"Cause": {"radioNetwork": "user-inactivity"}}]}
/// ```
///
/// An IE is one member: the name that ASN.1 gives it, or its identifier in digits when
/// it has none, with its value. One that did not decode has its octets, as
/// `{"octets": "…"}`. What the specification assigns is left out: the `criticality` of
/// the PDU is there when it is not that of the procedure, and that of an IE, beside its
/// name, when it is not the one that the message assigns it.
/// [`message_from_tree`] reads this form back.
///
/// A value is as [`message_view`] shows it, without what a view keeps of what was
/// received, to any depth. A transfer is its decoded value, or `{"octets": "…"}` when
/// it did not decode. The IEs that a value holds, alone, as the entries of a list, in
/// an extension container or as the message that a transfer contains, are each a name
/// with a value too:
///
/// ```json
/// {"PDUSessionResourceSetupListSUReq": [{"pDUSessionID": 5, "s-NSSAI": {"sST": 1},
///   "pDUSessionResourceSetupRequestTransfer": {"protocolIEs": [
///     {"PDUSessionType": "ipv4", "criticality": "reject"}]}}]}
/// ```
///
/// The `criticality` of an IE that a value holds alone or as an entry of a list is
/// left out when it is the one that [`item_criticality`] gives. That of an IE of a
/// message that a transfer contains, and that of an IE of an extension container, is
/// always there: the tables have the criticality that an object set assigns for the
/// messages of the procedures and for the single containers only.
///
/// An error for a PDU whose message does not decode, for a procedure that the
/// specification does not have in that direction, and for a message without a list of
/// IEs of the protocol, as a private message.
pub fn message_tree(pdu: &NGAP_PDU) -> Result<Value, String> {
    let name = message_name(pdu).ok_or("the specification has no message of this procedure")?;
    let view = message_view(pdu)?;
    let listed = &view["message"]["protocolIEs"];
    if !listed.is_array() {
        return Err(format!("a message {name} has no list of IEs"));
    }
    let mut tree = json!({"message": name, "ies": named_ies(name, listed)?});
    if view["criticality"].as_str() != message_criticality(name) {
        tree["criticality"] = view["criticality"].clone();
    }
    Ok(tree)
}

/// The PDU of a tree: a message written as its name and its IEs by name, as
/// [`message_tree`] shows one. The IEs are sent in the order written, and one that is
/// written twice is sent twice.
///
/// The name of the message gives the procedure code, the direction and the criticality
/// of the PDU, and the message the criticality of each of its IEs; a `criticality`
/// beside the name of the message or of an IE is sent in its place. An IE that the
/// message does not have, by its name or by its identifier in digits, says its
/// `criticality`. A value is encoded as the type of the IE; `{"octets": "…"}` is the
/// octets of the IE as they are sent, whatever they are.
///
/// A value is written as [`message_tree`] shows it: a transfer as its decoded value, or
/// as its `octets`, and each IE that the value holds as its name with its value. Such
/// an IE says its `criticality` where [`message_tree`] always shows it, and where
/// [`item_criticality`] has none for it. A value is also written as [`message_view`]
/// shows it, with `decoded` for the value of a transfer and an entry with its `id`,
/// its `criticality` and its `value` for an IE, and the two forms are taken in one
/// tree.
pub fn message_from_tree(tree: &Value) -> Result<NGAP_PDU, String> {
    const FORM: &str = "a tree is {\"message\": NAME, \"ies\": [{NAME: VALUE}, …]}";
    let written = tree.as_object().ok_or(FORM)?;
    if let Some(other) = written
        .keys()
        .find(|key| !["message", "ies", "criticality"].contains(&key.as_str()))
    {
        return Err(format!("{other:?} is no member of a tree: {FORM}"));
    }
    let name = written.get("message").and_then(Value::as_str).ok_or(FORM)?;
    let (direction, code) =
        message_named(name).ok_or_else(|| format!("{name:?} is not a message of NGAP"))?;
    let criticality = match written.get("criticality") {
        Some(criticality) => criticality.clone(),
        None => json!(message_criticality(name)),
    };
    let listed = written.get("ies").filter(|ies| ies.is_array());
    let ies = listed_ies(name, listed.ok_or(FORM)?)?;
    message_from_view(&json!({
        "procedure_code": code,
        "direction": direction,
        "criticality": criticality,
        "message": {"protocolIEs": ies},
    }))
}

/// The view of a PDU: its `procedure_code`, its `direction`, its `criticality` and its
/// `message`, each IE of which has its typed value and what it was received as. An
/// error when its message does not decode.
pub fn message_view(pdu: &NGAP_PDU) -> Result<Value, String> {
    let top: Value = serde_json::from_str(&rasn::jer::encode(pdu).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let body = top
        .as_object()
        .and_then(|o| o.values().next())
        .ok_or("invalid PDU choice")?;
    let code = pdu.procedure_code();
    let direction = pdu.direction();
    let raw = body["value"].clone();
    let mut message = (message_type(direction, code)?.decode)(&unhex(&raw)?)?;
    expand(&mut message, 0, message_shape(direction, code));
    Ok(
        json!({"procedure_code": code, "direction": direction, "criticality": body["criticality"], "message": message, "_raw_message": raw}),
    )
}

/// The PDU of a view, as [`message_view`] gives one and the edits leave it, with the
/// octets received for what was not edited.
pub fn message_from_view(view: &Value) -> Result<NGAP_PDU, String> {
    if view.as_object().is_none_or(|o| {
        o.keys().any(|key| {
            ![
                "procedure_code",
                "direction",
                "criticality",
                "message",
                "_raw_message",
            ]
            .contains(&key.as_str())
        })
    }) {
        return Err("unknown PDU inspection root field".into());
    }
    let code = view["procedure_code"]
        .as_u64()
        .and_then(|c| u8::try_from(c).ok())
        .ok_or("procedure_code must be u8")?;
    let direction = view["direction"]
        .as_str()
        .ok_or("direction must be a string")?;
    let variant = match direction {
        "InitiatingMessage" => "initiatingMessage",
        "SuccessfulOutcome" => "successfulOutcome",
        "UnsuccessfulOutcome" => "unsuccessfulOutcome",
        _ => return Err("invalid direction".into()),
    };
    let mut message = view.get("message").ok_or("missing message")?.clone();
    collapse(&mut message, "", 0, message_shape(direction, code))?;
    let raw = if let Some(original) = view.get("_raw_message") {
        let original = unhex(original)?;
        let typed = message_type(direction, code)?;
        let decoded = (typed.decode)(&original)?;
        if decoded == message {
            original
        } else {
            if (typed.encode)(&decoded)? != original {
                return Err("the message does not encode back to the octets received".into());
            }
            (typed.encode)(&message)?
        }
    } else {
        (message_type(direction, code)?.encode)(&message)?
    };
    let top = json!({variant: {"procedureCode": code, "criticality": view["criticality"], "value": hex(&raw)}});
    let (mut pdu, _) = decode_jer::<NGAP_PDU>(&top)?;
    pdu.repair_open_types()?;
    Ok(pdu)
}
