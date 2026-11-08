//! Decoded PDUs as JSON trees, which paths read and edit.
//!
//! The tree of a PDU, which [`to_tree`] gives and [`from_tree`] reads, is the name of its
//! message and its IEs by name, in their order:
//!
//! ```json
//! {"message": "UEContextReleaseRequest",
//!  "ies": [{"AMF-UE-NGAP-ID": 42}, {"RAN-UE-NGAP-ID": 7}, {"Cause": {"radioNetwork": "user-inactivity"}}]}
//! ```
//!
//! The functions of this module select and edit the values of a tree by their paths, and
//! say what the messages of the protocol have. With the feature `serde`, a PDU is
//! serialized as its tree in every format of serde, and each other type of the bindings
//! as its own tree, the form in which a tree of a PDU shows it. A tree is a [`Value`] of
//! `oxirush-utils`, which the crate has as [`tree::Value`](crate::tree::Value), with
//! `tree!` to write one.
//!
//! A tree and the serde form show each value as it is, keys too: the `SecurityKey` of an
//! INITIAL CONTEXT SETUP REQUEST, KgNB, and the next hop NH are strings of hexadecimal
//! digits in them, so that they are encoded back to the same octets. They are no form for
//! a log: `Debug` and `Display` of the bindings print no key, and the length of the octets
//! of a message that may hold one.
//!
//! Every PDU that decodes has a tree, which [`from_tree`] makes the same octets again. An
//! IE whose value does not encode back to the octets received is its octets in the tree,
//! `{"octets": "…"}`: one that did not decode, one that holds what a later version of its
//! type adds, which the value has no place for, and one in another form than X.691 gives
//! it. An edit of a tree changes the octets of the IEs that it changes, and of no other:
//! there is no `with_tree` that keeps the octets of the others, as there is none in
//! oxirush-nas, since [`from_tree`] writes them again.
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
//! use oxirush_ngap::{inspect, ngap::NGAPPDU};
//! use oxirush_ngap::tree;
//!
//! let bytes = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
//! let mut tree = NGAPPDU::decode(&bytes).unwrap().to_tree();
//!
//! // An IE is added at the end, by its name and its value
//! let name = tree!({"RANNodeName": "gnb-1", "criticality": "ignore"});
//! inspect::insert(&mut tree, "/ngap/-", name).unwrap();
//!
//! // Before another IE, by its number and its octets
//! let unknown = tree!({"60000": {"octets": "c0ffee"}, "criticality": "ignore"});
//! inspect::insert(&mut tree, "/ngap/Cause", unknown).unwrap();
//!
//! // An IE is taken out by its name
//! inspect::remove(&mut tree, "/ngap/AMF-UE-NGAP-ID").unwrap();
//!
//! let edited = NGAPPDU::from_tree(&tree).unwrap();
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
//! use oxirush_ngap::{inspect, ngap::NGAPPDU};
//! use oxirush_ngap::tree;
//!
//! // A message that has one IE twice
//! let mut tree = tree!({"message": "UEContextReleaseRequest", "ies": [
//!     {"AMF-UE-NGAP-ID": 1},
//!     {"RAN-UE-NGAP-ID": 7},
//!     {"AMF-UE-NGAP-ID": 2},
//! ]});
//!
//! // The name selects both, in the order of the message
//! let both = inspect::select(&tree, "/ngap/AMF-UE-NGAP-ID").unwrap();
//! assert_eq!(both, [&tree!(1), &tree!(2)]);
//!
//! // Each has its position, which is how `paths` names it, and its name may follow
//! inspect::set(&mut tree, "/ngap/2/AMF-UE-NGAP-ID", tree!(3)).unwrap();
//!
//! // After a position, the name of another IE is an error that says which IE is there
//! println!("{}", inspect::select(&tree, "/ngap/2/RAN-UE-NGAP-ID").unwrap_err());
//!
//! // The values, each with its path
//! for (path, value) in inspect::paths(&tree) {
//!     println!("{path} = {value}");
//! }
//! # assert_eq!(printed, r##"this IE is AMF-UE-NGAP-ID, not RAN-UE-NGAP-ID: "RAN-UE-NGAP-ID" is not a member of this value, which has none, at /ngap/2/RAN-UE-NGAP-ID
//! # /ngap/0 = 1
//! # /ngap/RAN-UE-NGAP-ID = 7
//! # /ngap/2 = 3
//! # "##);
//! ```
//!
//! ```text
//! this IE is AMF-UE-NGAP-ID, not RAN-UE-NGAP-ID: "RAN-UE-NGAP-ID" is not a member of this value, which has none, at /ngap/2/RAN-UE-NGAP-ID
//! /ngap/0 = 1
//! /ngap/RAN-UE-NGAP-ID = 7
//! /ngap/2 = 3
//! ```
//!
//! ## An item of a list
//!
//! ```
//! # use std::fmt::Write as _;
//! # let mut printed = String::new();
//! # macro_rules! println { ($($argument:tt)*) => { writeln!(printed, $($argument)*).unwrap() } }
//! use oxirush_ngap::{inspect, ngap::NGAPPDU};
//! use oxirush_ngap::{tree, tree::Value};
//!
//! // A message whose one IE is a list of two items
//! let mut tree = tree!({"message": "InitialContextSetupRequest", "ies": [
//!     {"AllowedNSSAI": [{"s-NSSAI": {"sST": 1}}, {"s-NSSAI": {"sST": 2, "sD": "000001"}}]},
//! ]});
//!
//! // An item by its position, and each of them
//! let select = |tree, path| -> Vec<Value> {
//!     let selected = inspect::select(tree, path).unwrap();
//!     selected.into_iter().cloned().collect()
//! };
//! assert_eq!(select(&tree, "/ngap/AllowedNSSAI/1/s-NSSAI/sD"), [tree!("000001")]);
//! assert_eq!(select(&tree, "/ngap/AllowedNSSAI/*/s-NSSAI/sST"), [tree!(1), tree!(2)]);
//!
//! // An optional member that an item does not have selects nothing
//! assert!(select(&tree, "/ngap/AllowedNSSAI/0/s-NSSAI/sD").is_empty());
//!
//! // An item is added at the end, and one is taken out
//! inspect::insert(&mut tree, "/ngap/AllowedNSSAI/-", tree!({"s-NSSAI": {"sST": 3}})).unwrap();
//! inspect::remove(&mut tree, "/ngap/AllowedNSSAI/0").unwrap();
//!
//! // `null` takes an optional member out
//! inspect::set(&mut tree, "/ngap/AllowedNSSAI/0/s-NSSAI/sD", Value::Null).unwrap();
//!
//! let edited = NGAPPDU::from_tree(&tree).unwrap();
//! println!("{}", edited.to_tree());
//! # assert_eq!(printed, r##"{"message":"InitialContextSetupRequest","ies":[{"AllowedNSSAI":[{"s-NSSAI":{"sST":2}},{"s-NSSAI":{"sST":3}}]}]}
//! # "##);
//! ```
//!
//! ```text
//! {"message":"InitialContextSetupRequest","ies":[{"AllowedNSSAI":[{"s-NSSAI":{"sST":2}},{"s-NSSAI":{"sST":3}}]}]}
//! ```
//!
//! ## A transfer and its IEs
//!
//! A tree shows a transfer as its decoded value, and the IEs of the message that it
//! contains by name.
//!
//! ```
//! # use std::fmt::Write as _;
//! # let mut printed = String::new();
//! # macro_rules! println { ($($argument:tt)*) => { writeln!(printed, $($argument)*).unwrap() } }
//! use oxirush_ngap::{inspect, ngap::NGAPPDU};
//! use oxirush_ngap::tree;
//!
//! // A message with a transfer, written as the message that it contains
//! let mut tree = tree!({"message": "PDUSessionResourceSetupRequest", "ies": [
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
//!
//! // The IEs of the message that it contains go by name in its place
//! let transfer = "/ngap/PDUSessionResourceSetupListSUReq/0/pDUSessionResourceSetupRequestTransfer";
//! let kind = format!("{transfer}/PDUSessionType");
//! assert_eq!(inspect::select(&tree, &kind).unwrap(), [&tree!("ipv4")]);
//!
//! // And by the names of the tree, joined: the IE by its position, then its name
//! let joined = format!("{transfer}/protocolIEs/0/PDUSessionType");
//! assert_eq!(inspect::select(&tree, &joined), inspect::select(&tree, &kind));
//!
//! // An edit of the message that it contains is encoded into the transfer
//! inspect::set(&mut tree, &kind, tree!("ipv6")).unwrap();
//! let edited = NGAPPDU::from_tree(&tree).unwrap();
//! println!("{}", edited.to_tree()["ies"][2]);
//! # assert_eq!(printed, r##"{"PDUSessionResourceSetupListSUReq":[{"pDUSessionID":5,"s-NSSAI":{"sST":1},"pDUSessionResourceSetupRequestTransfer":{"protocolIEs":[{"PDUSessionType":"ipv6"}]}}]}
//! # "##);
//! ```
//!
//! ```text
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
//! let choice = inspect::enumerated_at("UEContextReleaseRequest", "/ngap/Cause");
//! assert_eq!(choice, Ok(None));
//! # assert_eq!(printed, r##"["normal-release", "authentication-failure", "deregister", "unspecified", "uE-not-in-PLMN-serving-area", "mobile-IAB-not-authorized", "iAB-not-authorized", "no-aiot-NAS-response"]
//! # "##);
//! ```
//!
//! ```text
//! ["normal-release", "authentication-failure", "deregister", "unspecified", "uE-not-in-PLMN-serving-area", "mobile-IAB-not-authorized", "iAB-not-authorized", "no-aiot-NAS-response"]
//! ```
//!
//! A name is read in any case and with any separators, as `"Normal Release"`, and shown
//! as the specification spells it.
//!
//! ## Check a path before use
//!
//! ```
//! # use std::fmt::Write as _;
//! # let mut printed = String::new();
//! # macro_rules! println { ($($argument:tt)*) => { writeln!(printed, $($argument)*).unwrap() } }
//! use oxirush_ngap::inspect;
//!
//! // Whether a path can select anything in a message of a name, without a tree
//! let check = |path| inspect::check_path("UEContextReleaseRequest", path);
//! assert_eq!(check("/ngap/Cause/nas"), Ok(()));
//! assert_eq!(check("/ngap/*/radioNetwork"), Ok(()));
//!
//! // A name that is written wrong, and an IE that the message does not have
//! for wrong in ["/ngap/Cause/radioNetwrok", "/ngap/RAN-UE-NGAP-ID/value", "/ngap/GUAMI"] {
//!     println!("{}", check(wrong).unwrap_err());
//! }
//! # assert_eq!(printed, r##""radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, choice-Extensions, at /ngap/Cause/radioNetwrok
//! # "value" is not a member of this value, which has none, at /ngap/RAN-UE-NGAP-ID/value
//! # "GUAMI" is not an IE of UEContextReleaseRequest, which has AMF-UE-NGAP-ID, RAN-UE-NGAP-ID, PDUSessionResourceListCxtRelReq, Cause, GW-Context-Release-Indication, at /ngap/GUAMI
//! # "##);
//! ```
//!
//! ```text
//! "radioNetwrok" is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, choice-Extensions, at /ngap/Cause/radioNetwrok
//! "value" is not a member of this value, which has none, at /ngap/RAN-UE-NGAP-ID/value
//! "GUAMI" is not an IE of UEContextReleaseRequest, which has AMF-UE-NGAP-ID, RAN-UE-NGAP-ID, PDUSessionResourceListCxtRelReq, Cause, GW-Context-Release-Indication, at /ngap/GUAMI
//! ```
//!
//! [`Ngap::object_set`](crate::Ngap::object_set) gives what the ASN.1 says of each
//! message, which `check_path` reads.
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
//! use oxirush_ngap::{ErrorKind, inspect, ngap::NGAPPDU};
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
//! // The tree of the PDU shows the octets that the container was encoded as
//! let mut tree = NGAPPDU::from_tree(&required).unwrap().to_tree();
//! println!("{}", tree["ies"][2]);
//!
//! // Read: a path goes through the name of the type
//! let container = format!("/ngap/SourceToTarget-TransparentContainer/{name}");
//! let cell = format!("{container}/targetCell-ID/nR-CGI/nRCellIdentity");
//! assert_eq!(inspect::check_path("HandoverRequired", &cell), Ok(()));
//!
//! // The path is an error until `open` has decoded the octets, and selects the value after that
//! assert_eq!(inspect::select(&tree, &cell).unwrap_err().kind(), ErrorKind::Path);
//! inspect::open(&mut tree, &cell).unwrap();
//! assert_eq!(inspect::select(&tree, &cell).unwrap(), [&tree!(4660)]);
//! println!("{}", inspect::select(&tree, &container).unwrap()[0]);
//!
//! // An edit goes into it
//! inspect::set(&mut tree, &cell, tree!(7)).unwrap();
//! let edited = NGAPPDU::from_tree(&tree).unwrap();
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
//! [`paths`] gives each value of a tree with the path that selects it, [`select`] the
//! values at a path, and [`set`], [`remove`] and [`insert`] edit a tree at a path:
//!
//! ```text
//! /ngap/AMF-UE-NGAP-ID = 1
//! /ngap/Cause/radioNetwork = "user-inactivity"
//! /ngap/Cause/criticality = "reject"
//! ```
//!
//! A path is a JSON pointer into the tree, where `/ngap` stands for `/ies`, the IEs of
//! the message. The segment after it selects the value of an IE:
//!
//! - by its name, as ASN.1 has it after `id-`: `/ngap/RAN-UE-NGAP-ID`. An IE that is
//!   there twice is selected twice;
//! - by its position, `/ngap/0`, which is how [`paths`] names an IE that has no name or
//!   is there twice. A position is a number as decimal writes it: `0`, `12`;
//! - by `@id=N`, the IEs with that identifier, whether it has a name or not, and `*`,
//!   each of them;
//! - `-` is the end of the list, where [`insert`] adds an IE.
//!
//! After the value of an IE, `criticality` is the criticality beside it, which a tree has
//! where it is not the one that the message assigns, and the name of the IE may follow
//! once: `/ngap/0/AMF-UE-NGAP-ID` is the first IE when it is that one, and the name of
//! another IE is an error that says which IE it is. A member of the value that is spelled
//! as the name of the IE is that member. The IEs of a container, as the message that a
//! transfer contains, go by name in its place, and those of an extension container or of
//! a list of single containers by name in the list:
//!
//! ```text
//! /ngap/PDUSessionResourceSetupListSUReq/0/pDUSessionResourceSetupRequestTransfer/PDUSessionType
//! /ngap/PDUSessionResourceSetupListSUReq/0/pDUSessionResourceSetupRequestTransfer/protocolIEs/0/PDUSessionType
//! ```
//!
//! So the names and the positions that a tree shows, joined after the root, are the path
//! of a value in it. Any other list selects by position and `*`.
//!
//! ## Names
//!
//! A name is its letters and its digits, whatever their case and whatever is between
//! them: `/ngap/RAN-UE-NGAP-ID`, `/NGAP/ran_ue_ngap_id` and `/ngap/ranuengapid` are one
//! path. The root, the name of an IE, the members of a value, the name of a message
//! ([`Ngap::object_set_named`](crate::Ngap::object_set_named), [`check_path`]) and the
//! name of an ENUMERATED value are all taken this way, and no two of them that could be
//! taken for each other are the same.
//!
//! ## What is not there
//!
//! A path selects nothing where the message could have a value and does not:
//!
//! - an IE that the message does not have;
//! - an OPTIONAL member that is absent, and another alternative of a CHOICE (one of the
//!   type `NULL` that is there has its `null`, which a path selects):
//!   `/ngap/Cause/nas` when the cause is one of the radio network;
//! - what follows in the path, which is still checked against the type.
//!
//! A name that the type of a value cannot have is an error, which lists the members
//! that the type has: a member that is written wrong is not taken for one that is not
//! there. So are a member of a value that has none, a path into a value that is its
//! octets, and the name of an IE in a list whose entries are no IEs. After `*`, a
//! segment is an error when none of the values that it is applied to can have it.
//!
//! [`check_path`] says the same of a path without a tree: whether it can select
//! anything in a message of a name, whose IEs are those of its object set
//! ([`Ngap::object_set_named`](crate::Ngap::object_set_named)).
//!
//! ## Edits
//!
//! [`set`] writes a member by the name that ASN.1 gives it, and refuses a name that the
//! type does not have. `null` takes an optional member out, as [`remove`] does; a member
//! that its type always has is not taken out, and an alternative of a CHOICE takes the
//! place of the one that is there. The value of an IE is set whole at its name, as a
//! tree writes it or as its `{"octets": "…"}`, and [`remove`] at its name takes the IE
//! out. An IE that the message does not have is inserted, as `{"NAME": VALUE}` with its
//! `criticality` where the message does not assign one. A path that selects nothing is an
//! error, and the tree is then as it was.
//!
//! ```
//! use oxirush_ngap::{build_ngap, inspect, ngap::*};
//! use oxirush_ngap::tree;
//!
//! let pdu = build_ngap!(IGNORE UEContextReleaseRequest,
//!     REJECT AMF_UE_NGAP_ID(1u64),
//!     REJECT RAN_UE_NGAP_ID(7u32),
//!     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
//! )?;
//! let mut tree = pdu.to_tree();
//! assert_eq!(inspect::select(&tree, "/ngap/RAN-UE-NGAP-ID")?, [&tree!(7)]);
//!
//! inspect::set(&mut tree, "/ngap/Cause", tree!({"nas": "normal-release"}))?;
//! inspect::remove(&mut tree, "/ngap/AMF-UE-NGAP-ID")?;
//! let name = tree!({"RANNodeName": "gnb-1", "criticality": "ignore"});
//! inspect::insert(&mut tree, "/ngap/-", name)?;
//! let edited = NGAPPDU::from_tree(&tree)?.to_tree();
//! assert_eq!(
//!     edited["ies"],
//!     tree!([
//!         {"RAN-UE-NGAP-ID": 7},
//!         {"Cause": {"nas": "normal-release"}},
//!         {"RANNodeName": "gnb-1", "criticality": "ignore"},
//!     ])
//! );
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
//! | An open type | Its octets in hexadecimal, where X.697 has the value that they hold: a tree shows that value in their place |
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
//! A tree and a value alone show the values of some types as they are usually written,
//! and take them so or in the form of their ASN.1 type. A string
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
//! | `Path` | A path that is not well formed (no `/` before its first segment, `~` before what is no `0` and no `1`, `@id=` before what is no identifier), a name that the type at that place cannot have (a member of a value, an IE of the object set of a message, a name where a list goes by position), an edit whose path selects nothing, a path through the type that octets carry before [`open`] decoded them, a path of more than 64 segments or 4096 octets, a selection of more than 65,536 values |
//! | `Value` | A JSON value that is not a value of its ASN.1 type (of another form than [Values](#values) lists, a name that an `ENUMERATED` does not have, a number outside its constraint, octets that are no hexadecimal: the message says at which member), JSON that is not written as a tree, a member that its type does not have, a tree nested deeper than 256 levels, and an edit that is refused: a member that its type always has or the alternative of a `CHOICE` taken out, an insertion where there is no list |
//! | `UndecodableIe` | A path into a value that is its octets |
//! | `Decode` | A PDU whose message does not decode |
//!
//! The tree is as it was after an edit that returns an error.
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
//! At a handover to EPS they carry the containers of TS 36.413, which the tables of
//! oxirush-s1ap read once the program calls `oxirush_s1ap::inspect::register()`:
//!
//! | The octets of | carry a | Clause |
//! | --- | --- | --- |
//! | `SourceToTarget-TransparentContainer` | `SourceeNB-ToTargeteNB-TransparentContainer` | 9.3.1.20 |
//! | `TargetToSource-TransparentContainer` | `TargeteNB-ToSourceeNB-TransparentContainer` | 9.3.1.21 |
//!
//! Before that call such a name is an error that says to make it. For another target
//! system the octets are those of its own specification, and nothing in a message says
//! which they are. So a tree shows such an IE by its octets, and the one that writes or
//! reads it names the type, in any case:
//!
//! - written, the value of the IE is an object with one member, the name of the type
//!   with its value, which is encoded as that type:
//!   `{"SourceNGRANNode-ToTargetNGRANNode-TransparentContainer": {"rRCContainer": "…", …}}`.
//!   [`from_tree`] takes it;
//! - read, a path goes through the name of the type, after the IE:
//!   `/ngap/SourceToTarget-TransparentContainer/SourceNGRANNode-ToTargetNGRANNode-TransparentContainer/targetCell-ID`.
//!   [`check_path`] and [`enumerated_at`] read it against the types. In a tree, [`open`]
//!   decodes the octets that a path takes so, and nothing else does. On octets that were
//!   not opened, such a path is an error that says so.
//!
//! Once it is opened or written so, the value of the IE is that type under its name, or
//! `{"octets": "…"}` under it for octets that are not of the type. The `rRCContainer` in
//! it stays octets: it is a message of RRC.
//!
//! [`to_tree`]: NGAPPDU::to_tree
//! [`from_tree`]: NGAPPDU::from_tree

// The tables, the paths and the functions of the module, which oxirush-aper writes for
// both protocols.
oxirush_aper::__inspect! {
    bindings ngap, pdu NGAPPDU, hooks oxirush_ngap_hooks, any crate::ngap::ErrorIndication<'a>
}

use crate::ngap::NGAPPDU;
use oxirush_aper::Octets;
use oxirush_aper::protocol::Pdu;

/// The tree of a PDU, with the `inspect` feature: the short form in which a message is
/// shown and written, which the paths of [`inspect`](crate::inspect) read and edit.
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
    /// use oxirush_ngap::{inspect, ngap::NGAPPDU};
    /// use oxirush_ngap::tree;
    ///
    /// // A UE CONTEXT RELEASE REQUEST
    /// let octets = hex::decode("002a4015000003000a0002002a005500020007000f40020500").unwrap();
    /// let pdu = NGAPPDU::decode(&octets)?;
    /// let tree = pdu.to_tree();
    /// assert_eq!(tree["message"], tree!("UEContextReleaseRequest"));
    /// assert_eq!(inspect::select(&tree, "/ngap/Cause/radioNetwork")?, [&tree!("user-inactivity")]);
    ///
    /// // The tree is a PDU again, of the same octets
    /// assert_eq!(NGAPPDU::from_tree(&tree)?.encode()?, octets);
    /// # Ok::<(), oxirush_ngap::Error>(())
    /// ```
    ///
    /// An IE is one member: the name that ASN.1 gives it, or its identifier in digits when
    /// it has none, with its value in the form of its type, which [Values](crate::inspect#values)
    /// lists. Its type is that of its identifier, and for an IE of an extension container
    /// the one that the type which the container extends gives it:
    /// id-CurrentQoSParaSetIndex is an `AlternativeQoSParaSetIndex` in a QoS flow that is
    /// associated with a tunnel, and an `AlternativeQoSParaSetNotifyIndex` in one that a
    /// notification is about. One that did not decode, and one whose typed value has no
    /// place for what it was received with, which a later version of its type adds, has its
    /// octets, as `{"octets": "…"}`. What the specification assigns is left out: the
    /// `criticality` of the PDU is there when it is not that of the procedure, and that of
    /// an IE, beside its name, when it is not the one that the message assigns it.
    ///
    /// A transfer, and the value of an IE declared `OCTET STRING (CONTAINING ...)`, is its
    /// decoded value, or `{"octets": "…"}` when it did not decode or its value has no place
    /// for all of it. The IEs that a value holds, alone, as the entries of a list, in an
    /// extension container or as the message that a transfer contains, are each a name
    /// with a value too:
    ///
    /// ```json
    /// {"PDUSessionResourceSetupListSUReq": [{"pDUSessionID": 5, "s-NSSAI": {"sST": 1},
    ///   "pDUSessionResourceSetupRequestTransfer": {"protocolIEs": [
    ///     {"PDUSessionType": "ipv4"}]}}]}
    /// ```
    ///
    /// The `criticality` of an IE that a value holds alone or as an entry of a list is
    /// left out when it is the one that the single containers gives, and that of an IE of
    /// a message that a transfer contains when it is the one that the object set of the
    /// transfer assigns it. That of an IE of an extension container is always there: the
    /// tables have the criticality that an object set assigns for the messages, the
    /// transfers and the single containers only. An IE nested deeper than 64 levels has its
    /// octets.
    ///
    /// Every PDU that decodes has a tree, which [`from_tree`](Self::from_tree) makes the
    /// same PDU again. A procedure that the specification does not have in that direction
    /// has its `procedure_code` and its `direction` in the place of the name of a message,
    /// and its `criticality`, which no table assigns: its message is read as a container
    /// of protocol IEs, which every message of the protocol but the private one is.
    ///
    /// ```json
    /// {"procedure_code": 200, "direction": "InitiatingMessage", "ies": [...], "criticality": "ignore"}
    /// ```
    ///
    /// A message that has no list of IEs, as the private one, one that does not decode,
    /// and one whose typed value has no place for what it was received with, has its
    /// `octets` in the place of its IEs: `{"message": "PrivateMessage", "octets": "…"}`.
    ///
    /// A tree takes up to 142 octets of memory for each octet of the PDU while it is made:
    /// that of a message of 65,535 IEs of an identifier that the crate does not know, each
    /// of no octet, 262 KB, which takes 37 MB. IEs that decode take less for each octet: 93
    /// for 65,535 AMF UE NGAP IDs. The crate has no limit on the octets of a PDU: the caller
    /// bounds them.
    pub fn to_tree(&self) -> Value {
        let (direction, code) = (self.direction(), self.procedure_code());
        let criticality = self.criticality().to_tree();
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
    /// in an extension container, and where the object set of the transfer, or the single
    /// containers, has none for it. An edit of the tree of a PDU changes the octets of the
    /// IEs that it changes and of no other.
    ///
    /// ```
    /// use oxirush_ngap::ngap::NGAPPDU;
    /// use oxirush_ngap::tree;
    ///
    /// let request = tree!({"message": "UEContextReleaseRequest", "ies": [
    ///     {"AMF-UE-NGAP-ID": 42},
    ///     {"RAN-UE-NGAP-ID": 7},
    ///     {"Cause": {"radioNetwork": "user-inactivity"}},
    /// ]});
    /// let octets = NGAPPDU::from_tree(&request)?.encode()?;
    /// assert_eq!(hex::encode(octets), "002a4015000003000a0002002a005500020007000f40020500");
    /// # Ok::<(), oxirush_ngap::Error>(())
    /// ```
    pub fn from_tree(tree: &Value) -> Result<NGAPPDU<'static>, Error> {
        let parts = shared::tree_parts(&TABLES, tree)?;
        let criticality = json::from_value(&parts.criticality)?;
        let value = Octets::owned(parts.message);
        Ok(NGAPPDU::__of(
            parts.direction,
            parts.code,
            criticality,
            value,
        ))
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
        NGAPPDU::from_tree(&tree).map_err(D::Error::custom)
    }
}
