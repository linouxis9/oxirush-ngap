//! Decoded PDUs as JSON, in two forms: a view, which is read and edited by paths and
//! encoded again, and a tree, the short form in which a message is shown and written.
//!
//! The two forms are methods of the PDU, [`NGAP_PDU`], and the functions of this module
//! read and edit a view by its paths, and say what the messages of the protocol have.
//!
//! | | Tree | View |
//! | --- | --- | --- |
//! | From a PDU | [`to_tree`] | [`to_view`] |
//! | Back to a PDU | [`from_tree`]: a message written from the tree alone | [`with_view`]: this PDU, edited; [`from_view`]: the PDU of any view |
//!
//! The tree of a PDU is the name of its message and its IEs by name. The view of a PDU
//! keeps what was received, and the rest of this page is about it.
//!
//! [`to_view`] returns `procedure_code`, `direction`, `criticality` and `message`: the
//! message in JSON, each value in the form of its ASN.1 type, which [Values](#values)
//! lists. An IE whose identifier is known has its typed
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
//! [`from_view`] takes them in this form, a number also as a `"0x…"` string, and
//! in the form of their ASN.1 type. It takes the name of an ENUMERATED value as any other name: see
//! [Names](#names). A value that does not fit its form, such as a transport layer address
//! of another length, stays in the form of its type. The other strings stay in hexadecimal: an
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
//!   extension where it extends no type that gives it one, the octets do not decode, or
//!   the IE is nested deeper than 64 levels. The rest of the PDU is read all the same;
//! - `_edited`: [`set`], [`remove`] or [`insert`] changed something under the value, whose
//!   octets are no longer those beside it.
//!
//! [`from_view`] encodes the view, and [`with_view`] the view of a PDU that is at hand.
//! What was not edited keeps the octets received. To edit:
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
//! value of an IE are `value/*`. If a value has a member with the same name as a member
//! of its IE, write `value` before it: only a bit string whose size varies has such a
//! member, its own `value`, as in `/ngap/TraceCollectionEntityIPAddress/value/value`.
//! The value of an IE of an extension container is its `extensionValue`, and is left
//! out the same way.
//!
//! After an entry of a list of IEs, or an IE that a value holds alone, a segment may be
//! the name of that IE, and the path goes on from the IE:
//!
//! ```text
//! /ngap/PDUSessionResourceSetupListSUReq/0/pDUSessionResourceSetupRequestTransfer/protocolIEs/0/UL-NGU-UP-TNLInformation/gTPTunnel/gTP-TEID
//! ```
//!
//! So the names and the positions that [`to_tree`] shows, joined after the root,
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
//! [`from_view`] gives the PDU its octets.
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
//! )?;
//! let mut view = pdu.to_view()?;
//! assert_eq!(inspect::select(&view, "/ngap/RAN-UE-NGAP-ID/value")?, [&json!(7)]);
//!
//! inspect::set(&mut view, "/ngap/Cause/value", json!({"nas": "normal-release"}))?;
//! inspect::remove(&mut view, "/ngap/AMF-UE-NGAP-ID")?;
//! let name = json!({"id": "RANNodeName", "criticality": "ignore", "value": "gnb-1"});
//! inspect::insert(&mut view, "/ngap/-", name)?;
//! let edited = pdu.with_view(&view)?.to_view()?;
//! assert_eq!(inspect::select(&edited, "/ngap/*/id")?, [&json!(85), &json!(15), &json!(82)]);
//! # Ok::<(), oxirush_ngap::Error>(())
//! ```
//!
//! # Values
//!
//! A value has the JSON form of its ASN.1 type. It is that of the JSON Encoding Rules,
//! ITU-T X.697 (02/2021), but for what the rows of an `INTEGER`, of a `BIT STRING` and
//! of an open type say:
//!
//! | ASN.1 | JSON |
//! | --- | --- |
//! | `BOOLEAN`, `NULL` | `true` or `false`, and `null` |
//! | `INTEGER` | A number, which 64 bits hold, as the bindings hold it: X.697 has a number of any size |
//! | `ENUMERATED` | The name of the value |
//! | `OCTET STRING` | Its octets in hexadecimal |
//! | `BIT STRING` of one size | Its octets in hexadecimal, the last one padded with zero bits |
//! | `BIT STRING` whose size varies | `{"value": HEXADECIMAL, "length": BITS}`. Where the type gives one size and an extension marker, a string of that size is its octets in hexadecimal alone, and one of another size has its length: X.697 gives every string of such a type its length |
//! | A character string, an `OBJECT IDENTIFIER` | A string: the arcs of an identifier in decimal, with a full stop between them |
//! | `SEQUENCE` | An object with a member for each component that is there, by the name that ASN.1 gives it |
//! | `SEQUENCE OF` | A list |
//! | `CHOICE` | An object with one member, the name of the alternative with its value |
//! | An open type | Its octets in hexadecimal, where X.697 has the value that they hold: a view shows that value in their place |
//!
//! A `BIT STRING` member that may be absent and that has its size where the member is
//! declared, and not in a type of its own, has its length whatever that size is, as
//! the `dayofWeek` of a `ScheduledCommunicationTime`.
//!
//! A value that is written is read strictly: a member that its type does not have, a
//! second alternative of a `CHOICE`, an integer outside the constraint of its type or
//! written as its digits where JSON has a number for it, a bit string with other octets
//! than its length has and padding bits that are not zero are each an error, which says
//! where the fault is by the names of the value.
//!
//! # Errors
//!
//! Each function that can fail returns the [`Error`] of the crate, and the kind of
//! the error says what went wrong:
//!
//! | Kind | When |
//! | --- | --- |
//! | `UnknownMessage` | A name of a message, or a direction and a procedure code, that the protocol does not have |
//! | `UnknownIe` | A name or an identifier of an IE that the protocol does not have, or has no type for where it is: its octets are written in its place |
//! | `Path` | A path that is not well formed: no `/` before its first segment, `~` before what is no `0` and no `1`, `@id=` before what is no identifier |
//! | `NoSuchMember` | A name that the type at that place cannot have: in a path, a member of a value, an IE of the object set of a message, a name where a list goes by position; in a value that is written, a member that its type does not have |
//! | `NothingSelected` | An edit whose path selects nothing: an IE that the message does not have, or a position after the end of a list |
//! | `NotOpened` | A path through the type that octets carry, before [`open`] decoded them: open, then select again |
//! | `Value` | A JSON value that is not a value of its ASN.1 type: of another form than [Values](#values) lists, a name that an `ENUMERATED` does not have, a number outside its constraint, octets that are no hexadecimal. The message says at which member, by the names of the value |
//! | `Form` | JSON that is not written as a tree or as a view: no name of a message, a member that neither has, an IE with its value and its octets, an IE that says no criticality where no table has one |
//! | `Edit` | An edit that is refused: a member that its type always has, the value of an IE or the alternative of a `CHOICE` taken out, an insertion where there is no list, and the octets of a value that was edited since |
//! | `Limit` | A path of more than 64 segments or 4096 octets, a selection of more than 4096 values, a tree nested deeper than 256 levels, and the tree of a message without a list of IEs |
//! | `UndecodableIe` | A path into an IE or a transfer that did not decode |
//! | `Decode` | A PDU whose message does not decode, and octets of a view that are not what they were decoded as |
//!
//! The view is as it was after an edit that returns an error.
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
//!   [`from_tree`] takes it, and so does [`from_view`] for a value that [`set`] or
//!   [`insert`] wrote and for a view written by hand;
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
//!
//! [`to_tree`]: NGAP_PDU::to_tree
//! [`from_tree`]: NGAP_PDU::from_tree
//! [`to_view`]: NGAP_PDU::to_view
//! [`from_view`]: NGAP_PDU::from_view
//! [`with_view`]: NGAP_PDU::with_view

use serde_json::{Map, Value, json};

use crate::Error;
use crate::inspect_paths::{
    EDITED, Shape, carried_shape, entry_shape, extended, listed_ies, member_shape, member_shapes,
    message_shape, named_ies, of_a_transfer, open_with, same_name, written_ie,
};
pub use crate::inspect_paths::{check_path, enumerated_at, insert, paths, remove, select, set};
use crate::inspect_registry as registry;
use crate::ngap::{NGAP_PDU, NGAPPDU};
use oxirush_aper::json::{self, Json, hex};
use oxirush_aper::{AsnType, Decode, Encode, Octets};

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
pub fn message_name(pdu: &NGAPPDU<'_>) -> Option<&'static str> {
    let (direction, code) = (pdu.direction(), pdu.procedure_code());
    message_names()
        .find(|(of, procedure, _)| *procedure == code && *of == direction)
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
fn message_type(direction: &str, code: u8) -> Result<Typed, Error> {
    let mut messages = registry::MESSAGES.iter();
    let message = messages.find(|(of, procedure, ..)| *procedure == code && *of == direction);
    let protocol = registry::PROTOCOL;
    message.map(|(_, _, _, typed, ..)| typed()).ok_or_else(|| {
        Error::unknown_message(format!("{protocol} has no {direction} of procedure {code}"))
    })
}

/// The value of the type `T` that the octets `raw` are, in its JSON form. `name` is the
/// name of the type in the bindings.
pub(crate) fn decode_typed<'a, T>(name: &'static str, raw: &'a [u8]) -> Result<Value, Error>
where
    T: Decode<'a> + Json,
{
    let read: Result<T, Error> = oxirush_aper::decode(raw);
    let shown = read.map(|value| value.to_json());
    #[cfg(oxirush_ngap_hooks)]
    if let Some(observer) = OBSERVER.get() {
        observer(name, raw, &shown);
    }
    let _ = name;
    shown
}

/// The octets of the value of the type `T` that `value` writes in its JSON form.
pub(crate) fn encode_typed<T>(value: &Value) -> Result<Vec<u8>, Error>
where
    T: Encode + Json + AsnType,
{
    let typed: T = json::from_value(value)?;
    // A value that is read and does not encode is outside a constraint of its type.
    oxirush_aper::encode(&typed).map_err(|error| error.outside_its_type(T::NAME))
}

#[cfg(oxirush_ngap_hooks)]
thread_local! {
    /// What is told of each value that this thread decodes as a type of the registry:
    /// the name of the type in the bindings, the octets, and the JSON form of the
    /// value or why the octets are none. The tests of the oracle and the fuzz targets
    /// compare them with what another codec makes of the same octets.
    pub(crate) static OBSERVER: core::cell::Cell<Option<Observer>> =
        const { core::cell::Cell::new(None) };
}

/// What observes the values that are decoded as a type of the registry.
#[cfg(oxirush_ngap_hooks)]
pub(crate) type Observer = fn(&'static str, &[u8], &Result<Value, Error>);

/// The transfers of the registry: the member that holds each, and the type that its
/// octets contain. A member that has several types has its name alone.
macro_rules! transfers {
    ($($field:literal $($transfer:path)?;)*) => {
        pub(crate) const TRANSFER_FIELDS: &[&str] = &[$($field,)*];
        #[allow(clippy::match_single_binding)]
        pub(crate) fn transfer(field: &str) -> Result<Typed, $crate::Error> {
            match field {
                $($($field => Ok($crate::inspect::typed!($transfer)),)?)*
                _ => Err($crate::Error::form(format!(
                    "{PROTOCOL} has several types for {field}, or none: it is read and \
                     written as its octets"
                ))),
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
                return Some($crate::inspect::typed!($carried));
            })*
            None
        }
    };
}
pub(crate) use carried;

/// The functions of a type of the registry.
pub(crate) struct Typed {
    pub(crate) decode: fn(&[u8]) -> Result<Value, Error>,
    pub(crate) encode: fn(&Value) -> Result<Vec<u8>, Error>,
}

/// The functions of a type of the bindings, which is written with the lifetime `'a`
/// when it borrows, or without one in a function.
macro_rules! typed {
    ($type:ty) => {{
        // A type that borrows nothing leaves the lifetime without a use.
        #[allow(clippy::needless_lifetimes)]
        fn decode<'a>(raw: &'a [u8]) -> Result<serde_json::Value, $crate::Error> {
            let name = $crate::macros::type_name(stringify!($type));
            $crate::inspect::decode_typed::<$type>(name, raw)
        }
        #[allow(clippy::needless_lifetimes)]
        fn encode<'a>(value: &'a serde_json::Value) -> Result<Vec<u8>, $crate::Error> {
            $crate::inspect::encode_typed::<$type>(value)
        }
        $crate::inspect::Typed { decode, encode }
    }};
}
pub(crate) use typed;

/// The octets that `value` writes in hexadecimal.
fn unhex(value: &Value) -> Result<Vec<u8>, Error> {
    let octets = value.as_str().and_then(json::unhex);
    octets.ok_or_else(|| {
        // The start of what is written says which value it is.
        let written: String = value.to_string().chars().take(40).collect();
        Error::value(format!(
            "{written} is not octets: octets are written in hexadecimal, two digits for each"
        ))
    })
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
    /// The readable form of a value as its ASN.1 type has it; `None` when it has none.
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
                // The order of the digits is that of the helpers, which have none for
                // octets that are no PLMN identity: those stay octets.
                let octets = <[u8; 3]>::try_from(unhex(jer).ok()?).ok()?;
                let (mcc, mnc) = crate::helpers::plmn_digits(octets)?;
                Some(json!(format!("{mcc}-{mnc}")))
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

    /// The value as its ASN.1 type has it, from its readable form. A value in another
    /// form is kept as it is: the form of the type is still taken, and the codec refuses
    /// what is neither.
    fn write(self, shown: &Value) -> Result<Value, Error> {
        let number = |bits: u32, number: u64| {
            if bits < 64 && number >> bits != 0 {
                return Err(Error::value(format!(
                    "{number} does not fit in {bits} bits"
                )));
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
                // The three digits of an MCC, and the two or three of an MNC.
                let digits = |digits: &str| all_digits(digits).then(|| bcd(digits));
                let mcc = digits(mcc).and_then(|mcc| <[u8; 3]>::try_from(mcc).ok());
                let octets = mcc.zip(digits(mnc));
                let octets = octets.and_then(|(mcc, mnc)| crate::helpers::plmn_octets(mcc, &mnc));
                let octets = octets.ok_or_else(|| {
                    Error::value(format!("{text}: a PLMN identity is MCC-MNC, as 208-93"))
                })?;
                Ok(Value::String(hex(&octets)))
            }
            // Without a hyphen it is its three octets: what is neither says both forms,
            // where the codec would say that it is no octets.
            Self::Plmn if text.len() != 6 || !text.bytes().all(|b| b.is_ascii_hexdigit()) => {
                Err(Error::value(format!(
                    "{text}: a PLMN identity is MCC-MNC, as 208-93, or its three octets in \
                     hexadecimal"
                )))
            }
            Self::Digits if all_digits(text) => {
                let pairs = bcd(text);
                let pairs = pairs.chunks(2);
                let octets = pairs.map(|pair| pair.get(1).copied().unwrap_or(15) << 4 | pair[0]);
                Ok(Value::String(hex(&octets.collect::<Vec<_>>())))
            }
            Self::Number(bits) if text.starts_with("0x") => {
                let value = u64::from_str_radix(&text[2..], 16).map_err(|error| {
                    Error::value(format!("{text} is not a number in hexadecimal: {error}"))
                })?;
                number(bits, value)
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
                            return Err(Error::value(format!(
                                "{text}: a transport layer address is an IPv4 address, an IPv6 address, or the two with a comma"
                            )));
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

/// `value`, or each value of a list, as its ASN.1 type has it again.
fn unshow(form: Form, value: &mut Value) -> Result<(), Error> {
    match value {
        Value::Array(values) => values.iter_mut().try_for_each(|value| unshow(form, value)),
        value => {
            *value = form.write(value)?;
            Ok(())
        }
    }
}

/// The value that the octets `raw` contain, beside them, or why they did not decode.
fn contained(raw: Value, typed: Result<Typed, Error>) -> Value {
    let decoded = typed.and_then(|typed| (typed.decode)(&unhex(&raw)?));
    let mut transfer = Map::new();
    transfer.insert("_raw_transfer".into(), raw);
    match decoded {
        Ok(decoded) => transfer.insert("decoded".into(), decoded),
        Err(error) => transfer.insert("_decode_error".into(), error.to_string().into()),
    };
    Value::Object(transfer)
}

/// The octets `raw` as the transfer that they are when they carry the type that ASN.1
/// names `name`: its decoded value beside them, or why they are not of that type.
fn opened(raw: Value, name: &str) -> Value {
    let typed = registry::carried(name);
    let carries_none = || Error::no_such_member(format!("octets carry no {name}"));
    let mut transfer = contained(raw, typed.ok_or_else(carries_none));
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
/// which [`select`], [`paths`] and the edits go into and
/// [`from_view`](NGAP_PDU::from_view) takes back: its decoded value beside the octets, or
/// why they are not of that type. Nothing in a message says which type octets carry, so
/// [`to_view`](NGAP_PDU::to_view) leaves them as they are, and nothing but this function
/// decodes them: on octets that were not opened, a path through the name of a type is an
/// error that says so.
///
/// A path that goes through no such name changes nothing. A name that the type of a
/// value cannot have is an error, and the view is then as it was.
pub fn open(view: &mut Value, path: &str) -> Result<(), Error> {
    open_with(view, path, &opened)
}

/// The octets of the IE `id`, which is written as a type that they may carry: one
/// member, the name of that type with its value, which is its octets by now.
fn carried_octets(written: &Value, id: u16) -> Result<Value, Error> {
    let carried = registry::CARRIED
        .iter()
        .filter(|(carrier, ..)| *carrier == id);
    let names: Vec<_> = carried.map(|(_, name, _)| *name).collect();
    let mut members = written.as_object().into_iter().flatten();
    match (members.next(), members.next()) {
        (Some((name, octets)), None) if names.iter().any(|known| same_name(known, name)) => {
            Ok(octets.clone())
        }
        _ => Err(Error::value(format!(
            "IE {id} is written as its octets, or as {{NAME: VALUE}} for a type that they carry: {}",
            names.join(", ")
        ))),
    }
}

/// A view whose direction is none of the three.
fn no_direction() -> Error {
    Error::form(
        "the direction of a view is InitiatingMessage, SuccessfulOutcome or UnsuccessfulOutcome",
    )
}

/// How deep in a view an IE or a transfer is still decoded.
const NESTING: usize = 64;

/// Why what is deeper keeps its octets.
fn too_deep<T>() -> Result<T, Error> {
    Err(Error::limit(format!(
        "inspection nesting exceeds {NESTING}"
    )))
}

/// The functions of the type of the IE `id` in a container of `shape`: the type of its
/// identifier, or the one that the type which an extension container extends gives it.
fn ie_type(id: u16, shape: Shape) -> Result<Typed, Error> {
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
            && let Some(raw) = object.get_mut(field).filter(|v| v.is_string())
        {
            let raw = raw.take();
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
                // The value stays the octets that it was received as.
                Err(error) => {
                    object.insert(field.into(), raw.clone());
                    object.insert("_decode_error".into(), error.to_string().into());
                }
            }
            object.insert("_raw_value".into(), raw);
            object.insert("_original_id".into(), id.into());
            if let Ok(at) = ie_names().binary_search_by_key(&id, |(known, _)| *known) {
                object.insert("_ie_name".into(), ie_names()[at].1.into());
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
fn collapse(value: &mut Value, member: &str, depth: usize, shape: Shape) -> Result<(), Error> {
    // What is decoded at the limit of the inspection has values of its own under it.
    if depth > 4 * NESTING {
        return Err(Error::limit(format!(
            "inspection nesting exceeds {}",
            4 * NESTING
        )));
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
                        return Err(Error::no_such_member(format!(
                            "{key} has no member {other:?}: a transfer has its decoded value"
                        )));
                    }
                    if let Some(edited) = child.get("decoded") {
                        let bytes = unhex(&raw)?;
                        // An IE's identifier gives the type of what it contains, and
                        // the name of a transfer field the type of the transfer.
                        let transfer = match key.as_str() {
                            "value" | "extensionValue" => {
                                id.and_then(registry::ie_contents).ok_or_else(|| {
                                    Error::form(
                                        "an IE that contains no type has no decoded value \
                                         beside its octets",
                                    )
                                })?
                            }
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
                                return Err(Error::edit(format!(
                                    "{key} does not encode back to the octets received"
                                )));
                            }
                            *child = Value::String(hex(&(transfer.encode)(edited)?));
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
                            Value::String(hex(&(transfer.encode)(child)?))
                        } else if let Some(written) = alone("decoded") {
                            Value::String(hex(&(transfer.encode)(written)?))
                        } else if let Some(octets) = alone("octets") {
                            octets.clone()
                        } else {
                            return Err(Error::form(format!(
                                "{key} is written with its decoded value alone, or as its octets"
                            )));
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
                ie_type(id, shape)
                    .map_err(|error| error.reworded(|what| format!("{what}: {octets}")))
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
                        return Err(Error::edit(format!(
                            "IE {id} did not decode: its value is its octets in hexadecimal"
                        )));
                    }
                    edited.clone()
                }
                (Some(raw), Some(edited)) => {
                    // The identifier that the octets were decoded as, when a view has
                    // one: without it they are those of the identifier of the entry.
                    let original_id = match &original_id {
                        Some(original) => (original.as_u64())
                            .and_then(|v| u16::try_from(v).ok())
                            .ok_or_else(|| {
                            Error::form(
                                "_original_id is the identifier that the octets were \
                                     decoded as, a number",
                            )
                        })?,
                        None => id,
                    };
                    let bytes = unhex(&raw)?;
                    let received = ie_type(original_id, shape)?;
                    let original = (received.decode)(&bytes)?;
                    if &original == edited && id == original_id {
                        raw
                    } else if (received.encode)(&original)? != bytes {
                        return Err(Error::edit(format!(
                            "IE {original_id} does not encode back to the octets received; replace its octets instead"
                        )));
                    } else {
                        Value::String(hex(&(typed(id)?.encode)(edited)?))
                    }
                }
                // An IE that was not received.
                (None, Some(new)) => Value::String(hex(&(typed(id)?.encode)(new)?)),
                (None, None) => {
                    return Err(Error::form(format!(
                        "IE {id} has no {field} and no _raw_value"
                    )));
                }
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

/// The two JSON forms of a PDU, with the `inspect` feature: its tree, the short form in
/// which a message is shown and written, and its view, which keeps what was received and
/// is read and edited by the paths of [`inspect`](crate::inspect).
impl NGAPPDU<'_> {
    /// The tree of this PDU, which is the name of its message and its IEs by name, in
    /// their order:
    ///
    /// ```json
    /// {"message": "UEContextReleaseRequest",
    ///  "ies": [{"AMF-UE-NGAP-ID": 42}, {"RAN-UE-NGAP-ID": 7}, {"Cause": {"radioNetwork": "user-inactivity"}}]}
    /// ```
    ///
    /// ```
    /// use oxirush_ngap::ngap::NGAP_PDU;
    ///
    /// // A UE CONTEXT RELEASE REQUEST
    /// let octets = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    /// let pdu = NGAP_PDU::decode(&octets)?;
    /// let tree = pdu.to_tree()?;
    /// assert_eq!(tree["message"], "UEContextReleaseRequest");
    /// assert_eq!(tree["ies"][2]["Cause"]["radioNetwork"], "user-inactivity");
    ///
    /// // The tree is a PDU again, of the same octets
    /// assert_eq!(NGAP_PDU::from_tree(&tree)?.encode()?, octets);
    /// # Ok::<(), oxirush_ngap::Error>(())
    /// ```
    ///
    /// An IE is one member: the name that ASN.1 gives it, or its identifier in digits when
    /// it has none, with its value. One that did not decode has its octets, as
    /// `{"octets": "…"}`. What the specification assigns is left out: the `criticality` of
    /// the PDU is there when it is not that of the procedure, and that of an IE, beside its
    /// name, when it is not the one that the message assigns it.
    /// [`from_tree`](Self::from_tree) reads this form back.
    ///
    /// A value is as [`to_view`](Self::to_view) shows it, without what a view keeps of
    /// what was received, to any depth. A transfer is its decoded value, or
    /// `{"octets": "…"}` when it did not decode. The IEs that a value holds, alone, as the
    /// entries of a list, in an extension container or as the message that a transfer
    /// contains, are each a name with a value too:
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
    pub fn to_tree(&self) -> Result<Value, Error> {
        let (direction, code) = (self.direction(), self.procedure_code());
        let name = message_name(self).ok_or_else(|| {
            let protocol = registry::PROTOCOL;
            Error::unknown_message(format!("{protocol} has no {direction} of procedure {code}"))
        })?;
        let mut view = self.to_view()?;
        // The tree is made of the view: its IEs are taken out of it, and not copied.
        let listed = view.pointer_mut("/message/protocolIEs").map(Value::take);
        let Some(listed) = listed.filter(Value::is_array) else {
            return Err(Error::limit(format!("a message {name} has no list of IEs")));
        };
        let mut tree = Map::new();
        tree.insert("message".into(), name.into());
        tree.insert("ies".into(), named_ies(name, listed)?);
        if view["criticality"].as_str() != message_criticality(name) {
            tree.insert("criticality".into(), view["criticality"].clone());
        }
        Ok(Value::Object(tree))
    }

    /// The PDU of a tree: a message written as its name and its IEs by name, as
    /// [`to_tree`](Self::to_tree) shows one. The IEs are sent in the order written, and
    /// one that is written twice is sent twice.
    ///
    /// The name of the message gives the procedure code, the direction and the criticality
    /// of the PDU, and the message the criticality of each of its IEs; a `criticality`
    /// beside the name of the message or of an IE is sent in its place. An IE that the
    /// message does not have, by its name or by its identifier in digits, says its
    /// `criticality`. A value is encoded as the type of the IE; `{"octets": "…"}` is the
    /// octets of the IE as they are sent, whatever they are.
    ///
    /// A value is written as [`to_tree`](Self::to_tree) shows it: a transfer as its
    /// decoded value, or as its `octets`, and each IE that the value holds as its name
    /// with its value. Such an IE says its `criticality` where
    /// [`to_tree`](Self::to_tree) always shows it, and where [`item_criticality`] has
    /// none for it. A value is also written as [`to_view`](Self::to_view) shows it, with
    /// `decoded` for the value of a transfer and an entry with its `id`, its
    /// `criticality` and its `value` for an IE, and the two forms are taken in one tree.
    ///
    /// ```
    /// use oxirush_ngap::ngap::NGAP_PDU;
    /// use serde_json::json;
    ///
    /// let request = json!({"message": "UEContextReleaseRequest", "ies": [
    ///     {"AMF-UE-NGAP-ID": 42},
    ///     {"RAN-UE-NGAP-ID": 7},
    ///     {"Cause": {"radioNetwork": "user-inactivity"}},
    /// ]});
    /// let octets = NGAP_PDU::from_tree(&request)?.encode()?;
    /// assert_eq!(hex::encode(octets), "002a4015000003000a0002002a005500020007000f40020500");
    /// # Ok::<(), oxirush_ngap::Error>(())
    /// ```
    pub fn from_tree(tree: &Value) -> Result<NGAP_PDU, Error> {
        const FORM: &str = "a tree is {\"message\": NAME, \"ies\": [{NAME: VALUE}, …]}";
        let form = || Error::form(FORM);
        let written = tree.as_object().ok_or_else(form)?;
        if let Some(other) = written
            .keys()
            .find(|key| !["message", "ies", "criticality"].contains(&key.as_str()))
        {
            return Err(Error::form(format!(
                "{other:?} is no member of a tree: {FORM}"
            )));
        }
        let name = written.get("message").and_then(Value::as_str);
        let name = name.ok_or_else(form)?;
        let (direction, code) = message_named(name)
            .ok_or_else(|| Error::unknown_message(format!("{name:?} is not a message of NGAP")))?;
        let assigned = json!(message_criticality(name));
        let criticality = written.get("criticality").unwrap_or(&assigned);
        let listed = written.get("ies").filter(|ies| ies.is_array());
        let mut message = Map::new();
        message.insert(
            "protocolIEs".into(),
            listed_ies(name, listed.ok_or_else(form)?)?,
        );
        NGAP_PDU::of_view(direction, code, criticality, Value::Object(message), None)
    }

    /// The PDU of a direction and a procedure code that has `criticality` and the
    /// message of a view, `message`, which was received as the octets `received` when
    /// the view has them.
    fn of_view(
        direction: &str,
        code: u8,
        criticality: &Value,
        mut message: Value,
        received: Option<&Value>,
    ) -> Result<NGAP_PDU, Error> {
        collapse(&mut message, "", 0, message_shape(direction, code))?;
        let raw = if let Some(original) = received {
            let original = unhex(original)?;
            let typed = message_type(direction, code)?;
            let decoded = (typed.decode)(&original)?;
            if decoded == message {
                original
            } else {
                if (typed.encode)(&decoded)? != original {
                    return Err(Error::edit(
                        "the message does not encode back to the octets received",
                    ));
                }
                (typed.encode)(&message)?
            }
        } else {
            (message_type(direction, code)?.encode)(&message)?
        };
        let criticality = json::from_value(criticality)?;
        let value = Octets::owned(raw);
        NGAP_PDU::of(direction, code, criticality, value).ok_or_else(no_direction)
    }

    /// The view of this PDU: its `procedure_code`, its `direction`, its `criticality` and
    /// its `message`, each IE of which has its typed value and what it was received as.
    /// The functions of [`inspect`](crate::inspect) read and edit it by paths, and its
    /// documentation says what a view has.
    ///
    /// An error when the message does not decode, and for a procedure that the
    /// specification does not have in that direction.
    ///
    /// A view takes up to 125 octets of memory for each octet of the NGAP_PDU, and
    /// [`with_view`](Self::with_view) twice that: 49 MB and 94 MB for a message of 65,535
    /// IEs, 393 KB. The crate has no limit on the octets of a NGAP_PDU: the caller bounds them.
    ///
    /// ```
    /// use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    /// use serde_json::json;
    ///
    /// // A UE CONTEXT RELEASE REQUEST
    /// let octets = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    /// let view = NGAP_PDU::decode(&octets)?.to_view()?;
    /// assert_eq!(view["procedure_code"], 42);
    /// assert_eq!(inspect::select(&view, "/ngap/RAN-UE-NGAP-ID/value")?, [&json!(7)]);
    /// # Ok::<(), oxirush_ngap::Error>(())
    /// ```
    pub fn to_view(&self) -> Result<Value, Error> {
        let (direction, code) = (self.direction(), self.procedure_code());
        let raw: &[u8] = self.value();
        let mut message = (message_type(direction, code)?.decode)(raw)?;
        expand(&mut message, 0, message_shape(direction, code));
        let mut view = Map::new();
        view.insert("procedure_code".into(), code.into());
        view.insert("direction".into(), direction.into());
        view.insert("criticality".into(), json::to_value(&self.criticality()));
        view.insert("message".into(), message);
        view.insert("_raw_message".into(), hex(raw).into());
        Ok(Value::Object(view))
    }

    /// The PDU of a view, as [`to_view`](Self::to_view) gives one and the edits leave
    /// it, with the octets received for what was not edited. A view that is written by
    /// hand, without what a view keeps of what was received, is encoded from its values.
    ///
    /// [`with_view`](Self::with_view) is this for the view of a PDU that is at hand, and
    /// refuses the view of another message.
    pub fn from_view(view: &Value) -> Result<NGAP_PDU, Error> {
        const MEMBERS: [&str; 5] = [
            "procedure_code",
            "direction",
            "criticality",
            "message",
            "_raw_message",
        ];
        const FORM: &str =
            "a view has its procedure_code, its direction, its criticality and its message";
        let members = view.as_object().ok_or_else(|| Error::form(FORM))?;
        if let Some(other) = members.keys().find(|key| !MEMBERS.contains(&key.as_str())) {
            return Err(Error::form(format!(
                "{other:?} is no member of a view: {FORM}"
            )));
        }
        let code = view["procedure_code"]
            .as_u64()
            .and_then(|c| u8::try_from(c).ok())
            .ok_or_else(|| Error::form("the procedure_code of a view is a number from 0 to 255"))?;
        let direction = view["direction"].as_str().ok_or_else(no_direction)?;
        const DIRECTIONS: [&str; 3] = [
            "InitiatingMessage",
            "SuccessfulOutcome",
            "UnsuccessfulOutcome",
        ];
        if !DIRECTIONS.contains(&direction) {
            return Err(no_direction());
        }
        let message = view.get("message").ok_or_else(|| Error::form(FORM))?;
        let received = view.get("_raw_message");
        NGAP_PDU::of_view(
            direction,
            code,
            &view["criticality"],
            message.clone(),
            received,
        )
    }

    /// The PDU of a view of this PDU, as [`to_view`](Self::to_view) gives it and the
    /// edits leave it: this message, edited. What was not edited keeps the octets
    /// received.
    ///
    /// An error of the kind `Form` when the view is of another message, which has
    /// another direction or another procedure code than this PDU:
    /// [`from_view`](Self::from_view) gives the PDU of any view.
    ///
    /// ```
    /// use oxirush_ngap::{ErrorKind, inspect, ngap::NGAP_PDU};
    /// use serde_json::json;
    ///
    /// // A UE CONTEXT RELEASE REQUEST, and its view with another cause
    /// let octets = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    /// let pdu = NGAP_PDU::decode(&octets)?;
    /// let mut view = pdu.to_view()?;
    /// inspect::set(&mut view, "/ngap/Cause/nas", json!("normal-release"))?;
    /// let edited = pdu.with_view(&view)?;
    /// assert_eq!(edited.to_tree()?["ies"][2], json!({"Cause": {"nas": "normal-release"}}));
    ///
    /// // A view that says another procedure is not a view of this PDU
    /// view["procedure_code"] = json!(21);
    /// assert_eq!(pdu.with_view(&view).unwrap_err().kind(), ErrorKind::Form);
    /// # Ok::<(), oxirush_ngap::Error>(())
    /// ```
    pub fn with_view(&self, view: &Value) -> Result<NGAP_PDU, Error> {
        let (direction, code) = (self.direction(), self.procedure_code());
        let of = (view["direction"].as_str(), view["procedure_code"].as_u64());
        // A view that does not say which message it is of is refused as any view is.
        if let (Some(of), Some(procedure)) = of
            && (of, procedure) != (direction, u64::from(code))
        {
            let message = |direction: &str, code: u64| {
                let known = u8::try_from(code).ok();
                match known.and_then(|code| crate::registry::message_name(direction, code)) {
                    Some(name) => name.to_string(),
                    None => format!("the {direction} of procedure {code}"),
                }
            };
            return Err(Error::form(format!(
                "this view is of {}, and the PDU is {}: with_view takes a view of the PDU \
                 itself, and from_view any view",
                message(of, procedure),
                message(direction, code.into()),
            )));
        }
        NGAP_PDU::from_view(view)
    }
}
