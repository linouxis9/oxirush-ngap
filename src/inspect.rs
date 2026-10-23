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
//! keeps what was received, and the rest of this page is about it. With the feature
//! `serde`, a PDU is serialized as its tree in every format of serde, and each other
//! type of the bindings as its own tree, the form in which a tree of a PDU shows it.
//! A tree and a view are a [`Value`] of `oxirush-utils`, which the crate has as
//! [`tree::Value`](crate::tree::Value), with `tree!` to write one.
//!
//! A tree, a view and the serde form show each value as it is, keys too: the
//! `SecurityKey` of an INITIAL CONTEXT SETUP REQUEST, KgNB, and the next hop NH are
//! strings of hexadecimal digits in them, so that they are encoded back to the same
//! octets. They are no form for a log: `Debug` and `Display` of the bindings print no
//! key, and the length of the octets of a message that may hold one.
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
//! The values of some well-known types are shown as they are usually written, in a view,
//! in a tree and alone:
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
//!   octets are no longer those beside it;
//! - `_lossy`: the typed value encodes to other octets than those beside it: they hold
//!   what a later version of its type adds, which the value has no place for, or, for
//!   the container of the message, are in another form than the canonical one. A tree
//!   has the octets in the place of the value, so that it gives them back.
//!
//! The value of an IE and a transfer in another form than the one that X.691 gives them,
//! as with padding bits that are not zero, are their octets, with the `_decode_error` of
//! a reader of that one form: the tree has their octets too. A path list has
//! `_decode_error` and `_lossy`, which say why the tree has octets.
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
//! Hexadecimal is written in lower case and taken in either.
//!
//! # Examples
//!
//! The README has the main tasks of a PDU as JSON; these are the others.
//!
//! ## Add or remove an IE
//!
//! ```
//! # use std::fmt::Write as _;
//! # let mut printed = String::new();
//! # macro_rules! println { ($($argument:tt)*) => { writeln!(printed, $($argument)*).unwrap() } }
//! use oxirush_ngap::{inspect, ngap::NGAP_PDU};
//! use oxirush_ngap::tree;
//!
//! let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
//! let pdu = NGAP_PDU::decode(&bytes).unwrap();
//! let mut view = pdu.to_view().unwrap();
//!
//! // An IE is added at the end, by its name and its typed value
//! let name = tree!({"id": "RANNodeName", "criticality": "ignore", "value": "gnb-1"});
//! inspect::insert(&mut view, "/ngap/-", name).unwrap();
//!
//! // Before another IE, by its number and its octets
//! let unknown = tree!({"id": 60000, "criticality": "ignore", "octets": "c0ffee"});
//! inspect::insert(&mut view, "/ngap/Cause", unknown).unwrap();
//!
//! // An IE is taken out by its name
//! inspect::remove(&mut view, "/ngap/AMF-UE-NGAP-ID").unwrap();
//!
//! let edited = pdu.with_view(&view).unwrap();
//! println!("{}", edited.to_tree()["ies"]);
//! # assert_eq!(printed, r##"[{"RAN-UE-NGAP-ID":7},{"60000":{"octets":"c0ffee"},"criticality":"ignore"},{"Cause":{"radioNetwork":"user-inactivity"}},{"RANNodeName":"gnb-1","criticality":"ignore"}]
//! # "##);
//! ```
//!
//! ```text
//! [{"RAN-UE-NGAP-ID":7},{"60000":{"octets":"c0ffee"},"criticality":"ignore"},{"Cause":{"radioNetwork":"user-inactivity"}},{"RANNodeName":"gnb-1","criticality":"ignore"}]
//! ```
//!
//! ## A repeated IE
//!
//! ```
//! # use std::fmt::Write as _;
//! # let mut printed = String::new();
//! # macro_rules! println { ($($argument:tt)*) => { writeln!(printed, $($argument)*).unwrap() } }
//! use oxirush_ngap::{inspect, ngap::NGAP_PDU};
//! use oxirush_ngap::tree;
//!
//! // A message that has one IE twice
//! let twice = tree!({"message": "UEContextReleaseRequest", "ies": [
//!     {"AMF-UE-NGAP-ID": 1},
//!     {"RAN-UE-NGAP-ID": 7},
//!     {"AMF-UE-NGAP-ID": 2},
//! ]});
//! let pdu = NGAP_PDU::from_tree(&twice).unwrap();
//! let mut view = pdu.to_view().unwrap();
//!
//! // The name selects both, in the order of the message
//! let both = inspect::select(&view, "/ngap/AMF-UE-NGAP-ID/value").unwrap();
//! assert_eq!(both, [&tree!(1), &tree!(2)]);
//!
//! // Each has its position, which is how `paths` names it, and its name may follow
//! inspect::set(&mut view, "/ngap/2/AMF-UE-NGAP-ID/value", tree!(3)).unwrap();
//!
//! // After a position, the name of another IE is an error that says which IE is there
//! println!("{}", inspect::select(&view, "/ngap/2/RAN-UE-NGAP-ID/value").unwrap_err());
//!
//! // The values, each with its path
//! for (path, value) in inspect::paths(&view) {
//!     if path.ends_with("/value") {
//!         println!("{path} = {value}");
//!     }
//! }
//! # assert_eq!(printed, r##"this IE is AMF-UE-NGAP-ID, not RAN-UE-NGAP-ID: "RAN-UE-NGAP-ID" is not a member of this value, which has none, at /ngap/2/RAN-UE-NGAP-ID/value
//! # /ngap/0/value = 1
//! # /ngap/RAN-UE-NGAP-ID/value = 7
//! # /ngap/2/value = 3
//! # "##);
//! ```
//!
//! ```text
//! this IE is AMF-UE-NGAP-ID, not RAN-UE-NGAP-ID: "RAN-UE-NGAP-ID" is not a member of this value, which has none, at /ngap/2/RAN-UE-NGAP-ID/value
//! /ngap/0/value = 1
//! /ngap/RAN-UE-NGAP-ID/value = 7
//! /ngap/2/value = 3
//! ```
//!
//! ## An item of a list
//!
//! ```
//! # use std::fmt::Write as _;
//! # let mut printed = String::new();
//! # macro_rules! println { ($($argument:tt)*) => { writeln!(printed, $($argument)*).unwrap() } }
//! use oxirush_ngap::{inspect, ngap::NGAP_PDU};
//! use oxirush_ngap::{tree, tree::Value};
//!
//! // A message whose one IE is a list of two items
//! let request = tree!({"message": "InitialContextSetupRequest", "ies": [
//!     {"AllowedNSSAI": [{"s-NSSAI": {"sST": 1}}, {"s-NSSAI": {"sST": 2, "sD": "000001"}}]},
//! ]});
//! let pdu = NGAP_PDU::from_tree(&request).unwrap();
//! let mut view = pdu.to_view().unwrap();
//!
//! // An item by its position, and each of them after `value`
//! let select = |view, path| -> Vec<Value> {
//!     let selected = inspect::select(view, path).unwrap();
//!     selected.into_iter().cloned().collect()
//! };
//! assert_eq!(select(&view, "/ngap/AllowedNSSAI/1/s-NSSAI/sD"), [tree!("000001")]);
//! assert_eq!(select(&view, "/ngap/AllowedNSSAI/value/*/s-NSSAI/sST"), [tree!(1), tree!(2)]);
//!
//! // An optional member that an item does not have selects nothing
//! assert!(select(&view, "/ngap/AllowedNSSAI/0/s-NSSAI/sD").is_empty());
//!
//! // An item is added at the end, and one is taken out
//! inspect::insert(&mut view, "/ngap/AllowedNSSAI/-", tree!({"s-NSSAI": {"sST": 3}})).unwrap();
//! inspect::remove(&mut view, "/ngap/AllowedNSSAI/0").unwrap();
//!
//! // `null` takes an optional member out
//! inspect::set(&mut view, "/ngap/AllowedNSSAI/0/s-NSSAI/sD", Value::Null).unwrap();
//!
//! let edited = pdu.with_view(&view).unwrap();
//! println!("{}", edited.to_tree());
//! # assert_eq!(printed, r##"{"message":"InitialContextSetupRequest","ies":[{"AllowedNSSAI":[{"s-NSSAI":{"sST":2}},{"s-NSSAI":{"sST":3}}]}]}
//! # "##);
//! ```
//!
//! ```text
//! {"message":"InitialContextSetupRequest","ies":[{"AllowedNSSAI":[{"s-NSSAI":{"sST":2}},{"s-NSSAI":{"sST":3}}]}]}
//! ```
//!
//! ## A transfer and its members
//!
//! A view shows the decoded value of a transfer beside its octets, and a tree
//! its decoded value alone.
//!
//! ```
//! # use std::fmt::Write as _;
//! # let mut printed = String::new();
//! # macro_rules! println { ($($argument:tt)*) => { writeln!(printed, $($argument)*).unwrap() } }
//! use oxirush_ngap::{inspect, ngap::NGAP_PDU};
//! use oxirush_ngap::tree;
//!
//! // A message with a transfer, written as the message that it contains
//! let request = tree!({"message": "PDUSessionResourceSetupRequest", "ies": [
//!     {"AMF-UE-NGAP-ID": 42},
//!     {"RAN-UE-NGAP-ID": 7},
//!     {"PDUSessionResourceSetupListSUReq": [{
//!         "pDUSessionID": 5,
//!         "s-NSSAI": {"sST": 1},
//!         "pDUSessionResourceSetupRequestTransfer": {"protocolIEs": [
//!             {"PDUSessionType": "ipv4"},
//!         ]},
//!     }]},
//! ]});
//! let pdu = NGAP_PDU::from_tree(&request).unwrap();
//! let mut view = pdu.to_view().unwrap();
//!
//! // In a view, the transfer is its octets beside its decoded value
//! let transfer = "/ngap/PDUSessionResourceSetupListSUReq/0/pDUSessionResourceSetupRequestTransfer";
//! println!("{}", inspect::select(&view, transfer).unwrap()[0]);
//!
//! // The IEs of the message that it contains go by name in its place
//! let kind = format!("{transfer}/PDUSessionType/value");
//! assert_eq!(inspect::select(&view, &kind).unwrap(), [&tree!("ipv4")]);
//!
//! // The same value by the path that says `value` and `decoded`
//! let long = "/ngap/PDUSessionResourceSetupListSUReq/value/0\
//!     /pDUSessionResourceSetupRequestTransfer/decoded/PDUSessionType/value";
//! assert_eq!(inspect::select(&view, long), inspect::select(&view, &kind));
//!
//! // And by the names that the tree shows, joined: the IE by its position, then its name
//! let joined = format!("{transfer}/protocolIEs/0/PDUSessionType/value");
//! assert_eq!(inspect::select(&view, &joined), inspect::select(&view, &kind));
//!
//! // An edit of the message that it contains is encoded into the transfer
//! inspect::set(&mut view, &kind, tree!("ipv6")).unwrap();
//! let edited = pdu.with_view(&view).unwrap();
//! println!("{}", edited.to_tree()["ies"][2]);
//! # assert_eq!(printed, r##"{"_raw_transfer":"0000010086000100","decoded":{"protocolIEs":[{"id":134,"criticality":"reject","value":"ipv4","_raw_value":"00","_original_id":134,"_ie_name":"PDUSessionType"}]}}
//! # {"PDUSessionResourceSetupListSUReq":[{"pDUSessionID":5,"s-NSSAI":{"sST":1},"pDUSessionResourceSetupRequestTransfer":{"protocolIEs":[{"PDUSessionType":"ipv6"}]}}]}
//! # "##);
//! ```
//!
//! ```text
//! {"_raw_transfer":"0000010086000100","decoded":{"protocolIEs":[{"id":134,"criticality":"reject","value":"ipv4","_raw_value":"00","_original_id":134,"_ie_name":"PDUSessionType"}]}}
//! {"PDUSessionResourceSetupListSUReq":[{"pDUSessionID":5,"s-NSSAI":{"sST":1},"pDUSessionResourceSetupRequestTransfer":{"protocolIEs":[{"PDUSessionType":"ipv6"}]}}]}
//! ```
//!
//! ## The names of an ENUMERATED
//!
//! ```
//! # use std::fmt::Write as _;
//! # let mut printed = String::new();
//! # macro_rules! println { ($($argument:tt)*) => { writeln!(printed, $($argument)*).unwrap() } }
//! use oxirush_ngap::inspect;
//!
//! // The names that the value at a path can have, as the specification spells them
//! let names = inspect::enumerated_at("UEContextReleaseRequest", "/ngap/Cause/nas").unwrap();
//! println!("{:?}", names.unwrap());
//!
//! // A value of another type has none
//! let choice = inspect::enumerated_at("UEContextReleaseRequest", "/ngap/Cause/value");
//! assert_eq!(choice, Ok(None));
//! # assert_eq!(printed, r##"["normal-release", "authentication-failure", "deregister", "unspecified", "uE-not-in-PLMN-serving-area", "mobile-IAB-not-authorized", "iAB-not-authorized", "no-aiot-NAS-response"]
//! # "##);
//! ```
//!
//! ```text
//! ["normal-release", "authentication-failure", "deregister", "unspecified", "uE-not-in-PLMN-serving-area", "mobile-IAB-not-authorized", "iAB-not-authorized", "no-aiot-NAS-response"]
//! ```
//!
//! A name is read in any case and with any separators, as `"Normal Release"`
//! above, and shown as the specification spells it.
//!
//! ## Check a path before use
//!
//! ```
//! # use std::fmt::Write as _;
//! # let mut printed = String::new();
//! # macro_rules! println { ($($argument:tt)*) => { writeln!(printed, $($argument)*).unwrap() } }
//! use oxirush_ngap::inspect;
//!
//! // Whether a path can select anything in a message of a name, without a view
//! let check = |path| inspect::check_path("UEContextReleaseRequest", path);
//! assert_eq!(check("/ngap/Cause/nas"), Ok(()));
//! assert_eq!(check("/ngap/*/radioNetwork"), Ok(()));
//!
//! // A name that is written wrong, and an IE that the message does not have
//! for wrong in ["/ngap/Cause/radioNetwrok", "/ngap/RAN-UE-NGAP-ID/valeu", "/ngap/GUAMI"] {
//!     println!("{}", check(wrong).unwrap_err());
//! }
//! # assert_eq!(printed, r##""radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, choice-Extensions; an IE itself has id, criticality, value, octets, at /ngap/Cause/radioNetwrok
//! # "valeu" is not a member of this value, which has none; an IE itself has id, criticality, value, octets, at /ngap/RAN-UE-NGAP-ID/valeu
//! # "GUAMI" is not an IE of UEContextReleaseRequest, which has AMF-UE-NGAP-ID, RAN-UE-NGAP-ID, PDUSessionResourceListCxtRelReq, Cause, GW-Context-Release-Indication, at /ngap/GUAMI
//! # "##);
//! ```
//!
//! ```text
//! "radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, choice-Extensions; an IE itself has id, criticality, value, octets, at /ngap/Cause/radioNetwrok
//! "valeu" is not a member of this value, which has none; an IE itself has id, criticality, value, octets, at /ngap/RAN-UE-NGAP-ID/valeu
//! "GUAMI" is not an IE of UEContextReleaseRequest, which has AMF-UE-NGAP-ID, RAN-UE-NGAP-ID, PDUSessionResourceListCxtRelReq, Cause, GW-Context-Release-Indication, at /ngap/GUAMI
//! ```
//!
//! The functions `message_ies`, `message_ie_criticalities`, `item_criticality`
//! and the others of this module give what the ASN.1 says of each message, which
//! `check_path` reads.
//!
//! ## The container of a handover
//!
//! Written, read through [`open`], and edited, as
//! [Octets that carry a type](#octets-that-carry-a-type) says:
//!
//! ```
//! # use std::fmt::Write as _;
//! # let mut printed = String::new();
//! # macro_rules! println { ($($argument:tt)*) => { writeln!(printed, $($argument)*).unwrap() } }
//! use oxirush_ngap::{ErrorKind, inspect, ngap::NGAP_PDU};
//! use oxirush_ngap::tree;
//!
//! // Written: the name of the type with its value, in place of the octets
//! let name = "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer";
//! let required = tree!({"message": "HandoverRequired", "ies": [
//!     {"AMF-UE-NGAP-ID": 42},
//!     {"RAN-UE-NGAP-ID": 7},
//!     {"SourceToTarget-TransparentContainer": {name: {
//!         "rRCContainer": "0102",
//!         "targetCell-ID": {"nR-CGI": {"pLMNIdentity": "208-93", "nRCellIdentity": 4660}},
//!         "uEHistoryInformation": [{"lastVisitedCellInformation": {"eUTRANCell": "00"}}],
//!     }}},
//! ]});
//!
//! // A tree shows the octets that the container was encoded as
//! let pdu = NGAP_PDU::from_tree(&required).unwrap();
//! println!("{}", pdu.to_tree()["ies"][2]);
//!
//! // Read: a path goes through the name of the type
//! let mut view = pdu.to_view().unwrap();
//! let container = format!("/ngap/SourceToTarget-TransparentContainer/{name}");
//! let cell = format!("{container}/targetCell-ID/nR-CGI/nRCellIdentity");
//! assert_eq!(inspect::check_path("HandoverRequired", &cell), Ok(()));
//!
//! // The path is an error until `open` has decoded the octets, and selects the value after that
//! assert_eq!(inspect::select(&view, &cell).unwrap_err().kind(), ErrorKind::NotOpened);
//! inspect::open(&mut view, &cell).unwrap();
//! assert_eq!(inspect::select(&view, &cell).unwrap(), [&tree!(4660)]);
//!
//! // What the octets carry, decoded
//! let decoded = format!("{container}/decoded");
//! println!("{}", inspect::select(&view, &decoded).unwrap()[0]);
//!
//! // An edit goes into it, and what is not edited is sent as it was received
//! inspect::set(&mut view, &cell, tree!(7)).unwrap();
//! let edited = pdu.with_view(&view).unwrap();
//! println!("{}", edited.to_tree()["ies"][2]);
//! # assert_eq!(printed, r##"{"SourceToTarget-TransparentContainer":"000201020002f8390000012340080100"}
//! # {"rRCContainer":"0102","targetCell-ID":{"nR-CGI":{"pLMNIdentity":"208-93","nRCellIdentity":4660}},"uEHistoryInformation":[{"lastVisitedCellInformation":{"eUTRANCell":"00"}}]}
//! # {"SourceToTarget-TransparentContainer":"000201020002f8390000000070080100"}
//! # "##);
//! ```
//!
//! ```text
//! {"SourceToTarget-TransparentContainer":"000201020002f8390000012340080100"}
//! {"rRCContainer":"0102","targetCell-ID":{"nR-CGI":{"pLMNIdentity":"208-93","nRCellIdentity":4660}},"uEHistoryInformation":[{"lastVisitedCellInformation":{"eUTRANCell":"00"}}]}
//! {"SourceToTarget-TransparentContainer":"000201020002f8390000000070080100"}
//! ```
//!
//! # Paths
//!
//! [`paths`] gives each value of a view with the path that selects it, [`select`] the
//! values at a path, and [`set`], [`remove`] and [`insert`] edit a view at a path:
//!
//! ```text
//! /procedure_code = 42
//! /ngap/AMF-UE-NGAP-ID/criticality = "reject"
//! /ngap/AMF-UE-NGAP-ID/value = 1
//! /ngap/Cause/value/radioNetwork = "user-inactivity"
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
//! - an OPTIONAL member that is absent, and another alternative of a CHOICE (one of the
//!   type `NULL` that is there has its `null`, which a path selects):
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
//! use oxirush_ngap::tree;
//!
//! let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
//!     IGNORE, UEContextReleaseRequest,
//!     REJECT AMF_UE_NGAP_ID(1u64),
//!     REJECT RAN_UE_NGAP_ID(7u32),
//!     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
//! )?;
//! let mut view = pdu.to_view()?;
//! assert_eq!(inspect::select(&view, "/ngap/RAN-UE-NGAP-ID/value")?, [&tree!(7)]);
//!
//! inspect::set(&mut view, "/ngap/Cause/value", tree!({"nas": "normal-release"}))?;
//! inspect::remove(&mut view, "/ngap/AMF-UE-NGAP-ID")?;
//! let name = tree!({"id": "RANNodeName", "criticality": "ignore", "value": "gnb-1"});
//! inspect::insert(&mut view, "/ngap/-", name)?;
//! let edited = pdu.with_view(&view)?.to_view()?;
//! assert_eq!(inspect::select(&edited, "/ngap/*/id")?, [&tree!(85), &tree!(15), &tree!(82)]);
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
//! | `BOOLEAN`, `NULL` | `true` or `false`, and `null`: an alternative of the type `NULL` is `{"pLMNWide": null}`, whose `null` a path lists and selects |
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
//! ## Values as they are usually written
//!
//! A tree, a view and a value alone show the values of some types as they are
//! usually written, and take them so or in the form of their ASN.1 type. A string
//! of decimal digits is never read as octets: octets whose hexadecimal has no
//! letter are written after `0x`.
//!
//! | Type | Notation |
//! |------|----------|
//! | `PLMNIdentity` | `"208-93"`, the MCC and the MNC, or its octets: `"02f839"`, `"0x134001"`. `"310410"` is refused, as it is read as 310-410 and as the octets 0x310410 |
//! | `TransportLayerAddress` | `"10.0.0.1"`, `"2001:db8::1"`, or the two with a comma |
//! | `TAC`, `EPS-TAC`, `LAC`, `GTP-TEID`, `FiveG-TMSI`, `SST`, `PortNumber`, `AMFRegionID`, `AMFSetID`, `AMFPointer`, `NRCellIdentity`, `EUTRACellIdentity`, `UL-NAS-Count` | a number, also read in hexadecimal as `"0x…"`. A string of digits, as `"123456"`, is refused: it is the number 123456 and the octets 0x123456 |
//! | `ENUMERATED` | the name of the value, read in any case |
//! | `INTEGER` | a number, which 64 bits hold, as the bindings hold it |
//! | a bit string whose size varies, as the identifier of a gNB | `{"value": "000004", "length": 22}`: its octets in hexadecimal, from its first bit, and its length in bits |
//! | a bit string of another size than the one that its type gives beside an extension marker | `{"value": "c00000", "length": 24}`, where a mask of the 16 bits of its type is `"c000"` |
//! | the other strings: an SD, keys, the NAS-PDU and the other containers | hexadecimal, written in lower case and read in either |
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
//! | `Limit` | A path of more than 64 segments or 4096 octets, a selection of more than 65,536 values, a tree nested deeper than 256 levels |
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

// The tables, the paths and the functions of the module, which oxirush-aper writes for
// both protocols.
oxirush_aper::__inspect! {
    bindings ngap, pdu NGAPPDU, hooks oxirush_ngap_hooks, any crate::ngap::ErrorIndication<'a>;
/// The criticality that ASN.1 assigns to the IE `id` where a value holds it alone, in a
/// `ProtocolIE-SingleContainer`, as the alternative that extends a CHOICE: `"reject"`,
/// `"ignore"` or `"notify"`. `None` for an IE that no object set of a single container
/// has, as the IEs of a message or of a transfer, and for one that two of these sets give
/// two criticalities.
}

use crate::ngap::{NGAP_PDU, NGAPPDU};
use oxirush_aper::Octets;
use oxirush_aper::inspect::Parts;

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
    /// let tree = pdu.to_tree();
    /// assert_eq!(tree["message"], "UEContextReleaseRequest");
    /// assert_eq!(tree["ies"][2]["Cause"]["radioNetwork"], "user-inactivity");
    ///
    /// // The tree is a PDU again, of the same octets
    /// assert_eq!(NGAP_PDU::from_tree(&tree)?.encode()?, octets);
    /// # Ok::<(), oxirush_ngap::Error>(())
    /// ```
    ///
    /// An IE is one member: the name that ASN.1 gives it, or its identifier in digits when
    /// it has none, with its value. One that did not decode, and one whose typed value
    /// has no place for what it was received with, which a later version of its type
    /// adds, has its octets, as `{"octets": "…"}`. What the specification assigns is left out: the `criticality` of
    /// the PDU is there when it is not that of the procedure, and that of an IE, beside its
    /// name, when it is not the one that the message assigns it.
    /// [`from_tree`](Self::from_tree) reads this form back.
    ///
    /// A value is as [`to_view`](Self::to_view) shows it, without what a view keeps of
    /// what was received, to any depth. A transfer is its decoded value, or
    /// `{"octets": "…"}` when it did not decode or its value has no place for all of it. The IEs that a value holds, alone, as the
    /// entries of a list, in an extension container or as the message that a transfer
    /// contains, are each a name with a value too:
    ///
    /// ```json
    /// {"PDUSessionResourceSetupListSUReq": [{"pDUSessionID": 5, "s-NSSAI": {"sST": 1},
    ///   "pDUSessionResourceSetupRequestTransfer": {"protocolIEs": [
    ///     {"PDUSessionType": "ipv4"}]}}]}
    /// ```
    ///
    /// The `criticality` of an IE that a value holds alone or as an entry of a list is
    /// left out when it is the one that [`item_criticality`] gives, and that of an IE of
    /// a message that a transfer contains when it is the one that the object set of the
    /// transfer assigns it. That of an IE of an extension container is always there: the
    /// tables have the criticality that an object set assigns for the messages, the
    /// transfers and the single containers only.
    ///
    /// Every PDU that decodes has a tree, which [`from_tree`](Self::from_tree) makes the
    /// same PDU again. A procedure that the specification does not have in that direction
    /// has its `procedure_code` and its `direction` in the place of the name of a message,
    /// and its `criticality`, which no table assigns:
    ///
    /// ```json
    /// {"procedure_code": 200, "direction": "InitiatingMessage", "ies": [...], "criticality": "ignore"}
    /// ```
    ///
    /// A message that has no list of IEs, as the private one, one that does not decode,
    /// and one whose typed value has no place for what it was received with, has its
    /// `octets` in the place of its IEs: `{"message": "PrivateMessage", "octets": "…"}`.
    pub fn to_tree(&self) -> Value {
        let (direction, code) = (self.direction(), self.procedure_code());
        let criticality = json::to_value(&self.criticality());
        shared::tree(&TABLES, direction, code, criticality, self.value())
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
    /// octets of the IE as they are sent, whatever they are. A procedure that the protocol
    /// does not have in that direction is written by its `procedure_code` and its
    /// `direction`, with its `criticality`, and a message as it is sent by its `octets`
    /// in the place of its `ies`.
    ///
    /// A value is written as [`to_tree`](Self::to_tree) shows it: a transfer as its
    /// decoded value, or as its `octets`, and each IE that the value holds as its name
    /// with its value. Such an IE says its `criticality` where no table has one for it:
    /// in an extension container, and where the object set of the transfer, or
    /// [`item_criticality`], has none for it. A value is also written as [`to_view`](Self::to_view) shows it, with
    /// `decoded` for the value of a transfer and an entry with its `id`, its
    /// `criticality` and its `value` for an IE, and the two forms are taken in one tree.
    ///
    /// ```
    /// use oxirush_ngap::ngap::NGAP_PDU;
    /// use oxirush_ngap::tree;
    ///
    /// let request = tree!({"message": "UEContextReleaseRequest", "ies": [
    ///     {"AMF-UE-NGAP-ID": 42},
    ///     {"RAN-UE-NGAP-ID": 7},
    ///     {"Cause": {"radioNetwork": "user-inactivity"}},
    /// ]});
    /// let octets = NGAP_PDU::from_tree(&request)?.encode()?;
    /// assert_eq!(hex::encode(octets), "002a4015000003000a0002002a005500020007000f40020500");
    /// # Ok::<(), oxirush_ngap::Error>(())
    /// ```
    pub fn from_tree(tree: &Value) -> Result<NGAP_PDU, Error> {
        NGAP_PDU::of_parts(shared::tree_parts(&TABLES, tree)?)
    }

    /// The PDU of what a view or a tree writes.
    fn of_parts(parts: Parts) -> Result<NGAP_PDU, Error> {
        let criticality = json::from_value(&parts.criticality)?;
        let value = Octets::owned(parts.message);
        NGAP_PDU::of(parts.direction, parts.code, criticality, value).ok_or_else(|| {
            Error::form(
                "the direction of a view is InitiatingMessage, SuccessfulOutcome or \
                 UnsuccessfulOutcome",
            )
        })
    }

    /// The view of this PDU: its `procedure_code`, its `direction`, its `criticality` and
    /// its `message`, each IE of which has its typed value and what it was received as.
    /// The functions of [`inspect`](crate::inspect) read and edit it by paths, and its
    /// documentation says what a view has.
    ///
    /// An error when the message does not decode. A procedure that the specification
    /// does not have in that direction has a view too: its message is read as a container
    /// of protocol IEs, which every message of the protocol but the private one is, and
    /// each IE with the type of its identifier, as
    /// [`protocol_ies`](Self::protocol_ies) reads them whatever the message is. Its tree
    /// has its procedure code and its direction in the place of the name of a message.
    ///
    /// A view takes up to 143 octets of memory for each octet of the NGAP_PDU, and
    /// [`with_view`](Self::with_view) 177: those of a message of 65,535 IEs of an
    /// identifier that the crate does not know, each of no octet, 262 KB, which takes 37
    /// MB and 46 MB. IEs that decode take less for each octet: 94 and 120 octets for
    /// 65,535 AMF UE NGAP IDs. The crate has no limit on the octets of a NGAP_PDU: the
    /// caller bounds them.
    ///
    /// ```
    /// use oxirush_ngap::{inspect, ngap::NGAP_PDU};
    /// use oxirush_ngap::tree;
    ///
    /// // A UE CONTEXT RELEASE REQUEST
    /// let octets = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    /// let view = NGAP_PDU::decode(&octets)?.to_view()?;
    /// assert_eq!(view["procedure_code"], 42);
    /// assert_eq!(inspect::select(&view, "/ngap/RAN-UE-NGAP-ID/value")?, [&tree!(7)]);
    /// # Ok::<(), oxirush_ngap::Error>(())
    /// ```
    pub fn to_view(&self) -> Result<Value, Error> {
        let (direction, code) = (self.direction(), self.procedure_code());
        let criticality = json::to_value(&self.criticality());
        shared::view(&TABLES, direction, code, criticality, self.value())
    }

    /// The PDU of a view, as [`to_view`](Self::to_view) gives one and the edits leave
    /// it, with the octets received for what was not edited. A view that is written by
    /// hand, without what a view keeps of what was received, is encoded from its values.
    ///
    /// [`with_view`](Self::with_view) is this for the view of a PDU that is at hand, and
    /// refuses the view of another message.
    ///
    /// ```
    /// use oxirush_ngap::ngap::NGAP_PDU;
    ///
    /// // A view, kept as JSON text and read back
    /// let octets = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    /// let view = NGAP_PDU::decode(&octets)?.to_view()?;
    /// let text = view.to_string();
    /// let read: oxirush_ngap::tree::Value = text.parse().unwrap();
    /// assert_eq!(NGAP_PDU::from_view(&read)?.encode()?, octets);
    /// # Ok::<(), oxirush_ngap::Error>(())
    /// ```
    pub fn from_view(view: &Value) -> Result<NGAP_PDU, Error> {
        NGAP_PDU::of_parts(shared::view_parts(&TABLES, view)?)
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
    /// use oxirush_ngap::tree;
    ///
    /// // A UE CONTEXT RELEASE REQUEST, and its view with another cause
    /// let octets = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    /// let pdu = NGAP_PDU::decode(&octets)?;
    /// let mut view = pdu.to_view()?;
    /// inspect::set(&mut view, "/ngap/Cause/nas", tree!("normal-release"))?;
    /// let edited = pdu.with_view(&view)?;
    /// assert_eq!(edited.to_tree()["ies"][2], tree!({"Cause": {"nas": "normal-release"}}));
    ///
    /// // A view that says another procedure is not a view of this PDU
    /// view["procedure_code"] = tree!(21);
    /// assert_eq!(pdu.with_view(&view).unwrap_err().kind(), ErrorKind::Form);
    /// # Ok::<(), oxirush_ngap::Error>(())
    /// ```
    pub fn with_view(&self, view: &Value) -> Result<NGAP_PDU, Error> {
        let (direction, code) = (self.direction(), self.procedure_code());
        shared::of_the_pdu(&TABLES, view, direction, code)?;
        NGAP_PDU::from_view(view)
    }
}

/// With the feature `serde`, a PDU is serialized as its tree, which
/// [`to_tree`](NGAPPDU::to_tree) gives: every PDU that decodes has one.
#[cfg(feature = "serde")]
impl serde_core::Serialize for NGAPPDU<'_> {
    fn serialize<S: serde_core::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde_core::Serialize::serialize(&self.to_tree(), serializer)
    }
}

/// With the feature `serde`, a PDU is deserialized from its tree, as
/// [`from_tree`](NGAPPDU::from_tree) reads it.
#[cfg(feature = "serde")]
impl<'de> serde_core::Deserialize<'de> for NGAPPDU<'_> {
    fn deserialize<D: serde_core::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde_core::de::Error as _;
        let tree = <Value as serde_core::Deserialize>::deserialize(deserializer)?;
        NGAP_PDU::from_tree(&tree).map_err(D::Error::custom)
    }
}
