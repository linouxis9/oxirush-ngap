# Changelog

All notable changes to `oxirush-ngap` are recorded here.

## Unreleased (0.5.0)

The bindings come from TS 38.413 V19.4.0 (Release 19) instead of V16.6.0,
and the codec is the crate's own: the bindings read and write aligned PER
with [`oxirush-aper`](https://crates.io/crates/oxirush-aper), by ITU-T X.691,
where 0.4.2 derived the codec of rasn 0.28. It is not source compatible
with 0.4.2: [the values of the bindings](#the-values-of-the-bindings) says
what each type became.

### Breaking changes relative to 0.4.2

- The crate no longer depends on `rasn`, and `oxirush_ngap::rasn` is not
  there. The bindings implement `AsnType`, `Decode` and `Encode` of
  `oxirush-aper`, which the crate re-exports as `oxirush_ngap::aper`, and no
  trait of rasn: they have aligned PER, and none of the other encoding rules
  that the derived codec of rasn gave them. With the `inspect` feature they
  have the JSON forms.
- <a id="the-values-of-the-bindings"></a>The values of the bindings are
  native numbers, arrays and the types of `oxirush-aper`, where they were
  those of rasn:

  | 0.4.2 | Now | To migrate |
  | --- | --- | --- |
  | A native number, for an `INTEGER` whose constraint has no extension marker | The same number | As before |
  | `Integer`, for an `INTEGER` whose constraint has an extension marker | `u64`, or `i64` for a type whose root goes below zero | `BitRate(n)` for `(0..4000000000000, ...)`, where `n` is a `u64`: no `.into()`, and no `Integer` to convert. A number that 64 bits do not hold does not decode: see below |
  | `FixedOctetString<N>` | `[u8; N]` | `PLMNIdentity([0x02, 0xf8, 0x39])` |
  | `OctetString`, `Any` | `Octets<'a>` | `Octets::from(vec)`, `Octets::from(&slice)`, or `.into()` of either. It derefs to `[u8]`: `.as_bytes()` is gone |
  | `FixedBitString<N>` and `BitString` of 64 bits at most, without an extension marker | `Bits`, a number with a length | `AMFSetID::new(1)?`, which a type of one size has, or `AMFSetID(Bits::new(1, 10)?)`, `bits.value()` and `bits.len()` |
  | Another `BitString` | `BitString<'a>` | `BitString::from([0xe0, 0x00])`, `BitString::new(octets, length)?`, `.as_octets()` and `.len()` |
  | `PrintableString`, `VisibleString`, `Utf8String` | `Cow<'a, str>` | `AMFName("amf-1".into())`, and `&name.0` is a `&str` |
  | `SequenceOf<T>` | `Vec<T>` | As before |
  | The `LazyLock<Integer>` statics of the value assignments, as `MAXNOOF…` | `pub const … : u64` | The name is the same |

- A type that holds octets, a long bit string, a character string or an open
  type has a lifetime, as `NASPDU<'a>`, and so has each type that holds one of
  them, which is every message: a value that is decoded borrows from the
  octets that it was decoded from, and copies none. `NGAP_PDU` is the PDU that
  owns its octets, `NGAPPDU<'static>`, and `NGAP_PDU::decode` gives it as
  before. What changes for code that compiled:
  - a function that returns or keeps a value of the bindings writes its
    lifetime: `-> NASPDU<'static>` for a value that it built or that it made
    with `into_owned()`, which every type that borrows has, and
    `fn name<'a>(message: &'a T<'a>) -> U<'a>` for one that it read in a
    message;
  - the PDU is a variable of its own before `decode_value`, as the message
    borrows from it: `let pdu = NGAP_PDU::decode(&octets)?;` then
    `let message: T = pdu.decode_value()?;`. The two in one expression do
    not compile, and the compiler says to bind the first;
  - a pattern on a PDU that borrows names its type, `NGAPPDU::initiatingMessage(..)`:
    `NGAP_PDU::` in a pattern asks for `'static`.
- `ngap::decode_open_type` takes the octets, `&[u8]`, where it took an `&Any`,
  and `ngap::encode_open_type` gives an `Octets<'static>`. The `value` of a
  `ProtocolIEField`, of a `ProtocolExtensionField` and of the three
  alternatives of a PDU is an `Octets`, where it was an `Any`:
  `decode_open_type::<T>(&ie.value)` reads the same, and `&ie.value` is the
  octets where `ie.value.as_bytes()` was.
- The bindings hold an `INTEGER` in 64 bits, where rasn held one of any
  size. Octets with a larger number, which only an extension marker lets a
  later release send, are an error of the kind `Decode` for the value that
  holds it: `the number does not fit in 64 bits`. The PDU and its other IEs
  are read, and the JSON forms show that IE as the octets received.
- The bindings are generated from the Release 19 ASN.1 (`38413-j40`), with
  every procedure, IE and extension up to Release 19.
- Each protocol IE container, each extension container and their fields are
  one type: `ProtocolIEContainer` of `ProtocolIEField`, and
  `ProtocolExtensionContainer` of `ProtocolExtensionField`. The types that
  each use of one had, such as `InitialUEMessageProtocolIEs` and
  `AnonymousInitialUEMessageProtocolIEs`, are gone. The `id` of a field is a
  `ProtocolIEID` or a `ProtocolExtensionID`, where an extension field had a
  `u16`, and every `criticality` is a `Criticality`.
- The macros name each IE by its own identifier before any type alias, and a
  type name is an alias only for the one IE of that type: `AMFName` built
  id-OldAMF, and `SONConfigurationTransfer` the uplink IE in a downlink
  message. A type name shared by several IEs is no longer an IE name.
- The minimum supported Rust version is 1.88: 0.4.2 declared 1.85 and did not build with it.
- `helpers::plmn` writes other octets for an MNC of three digits, and
  `helpers::plmn_from` reads them: see the first entry of Fixed.
- `helpers::plmn_from` returns an `Option<(String, String)>`, where it
  returned the two strings: `None` for octets that are not a PLMN identity,
  a half octet being no digit where it is not the filler before an MNC of
  two digits. `ab cd ef` was the MCC `111013` and the MNC `151412`.
- One error type, `oxirush_ngap::Error`. `NGAP_PDU::decode`, `encode` and
  `decode_value`, `ngap::decode_open_type` and `ngap::encode_open_type` return
  it where they returned a `rasn::error::DecodeError` or an `EncodeError`,
  and so do the four macros. A function that returned one of those errors
  returns `Result<_, oxirush_ngap::Error>`, or keeps a
  `Box<dyn std::error::Error>`. `error.kind()` is an `ErrorKind` to match on,
  and the message says what is wrong and where: `these octets do not decode
  as NGAP-PDU: the octets end before the value does, in its field
  `value``. An error of an encoding names the member that does not encode
  in the same way, and `error.fields()` gives the fields. It is the error of
  `oxirush-aper`, and has no `source()`: no error of rasn is behind it.
- `decode_value::<T>()` takes the type of a message alone, and checks that
  the PDU holds that message: on another one the error has the kind
  `WrongMessage`, where 0.4.2 decoded the octets as whatever type was named,
  and a UE CONTEXT RELEASE REQUEST read as a setup request with three IEs.
  A type that is no message, as that of an IE, does not compile: decode it
  with `ngap::decode_open_type::<T>(&octets)`. Generic code that calls
  `decode_value` asks for `T: oxirush_ngap::Message<'a>` where it asked for
  `T: rasn::Decode`. Code that read the IEs of any PDU as those of one
  message, as in `pdu.decode_value::<ErrorIndication>()`, because every
  message is the same container, and wrote them back with
  `encode_open_type`, calls `pdu.protocol_ies()` and
  `pdu.set_protocol_ies(ies)`: see Added.
- `build_ngap!` gives a `Result<NGAP_PDU, Error>` and `build_ngap_ie!` a
  `Result<ProtocolIEField, Error>`: write `?` after the call, or `.unwrap()`
  where the values are known to encode. A value that does not encode, as an
  integer outside its constraint, is an error of the kind `Encode`, where
  the macros panicked with `failed to APER-encode NGAP IE open type`.
- `extract_ngap_ies!` returns an `Error` from the enclosing function, and
  `macros::MissingIeError` is gone. A `req` field whose IE is not in the
  message is an error of the kind `MissingIe`, whose message names the IE and
  the field, where `MissingIeError` had the name of the field as `ie_name`.
  An IE that is in the message and does not decode is an error of the kind
  `UndecodableIe`, for a `req` and for an `opt` field: 0.4.2 said that a
  `req` field was missing, and gave `None` for an `opt` one. The macro
  converts the error as `?` converts one, which 0.4.2 documented and did
  not do, so the enclosing function returns a `Result` whose error is
  `Error`, `Box<dyn std::error::Error>` or a type with a
  `From<oxirush_ngap::Error>`, where it returned `MissingIeError` itself. A
  call with `opt` fields alone returns the error too: it no longer compiles
  in a function that returns nothing.
  A field that took an IE when it decoded, and went on without it otherwise,
  is a `lax` field now: `lax name: Type = IE(value)` is `None` for an IE that
  is not in the message and for one that does not decode, which is what a
  receiver does with an IE that it may ignore (TS 38.413 clause 10.3.4.2). A
  call with `lax` fields alone returns no error.
- `with_ngap_ie_mut!` gives a `Result<bool, Error>` where it gave a `bool`:
  what the expression says, `Ok(false)` for an IE that is not in the message,
  and an error for an IE that does not decode (`UndecodableIe`) or a changed
  value that does not encode (`Encode`), which were both `false`. Write
  `with_ngap_ie_mut!(…)?` where the `bool` was read.
- When a message has the same IE twice, `extract_ngap_ies!` reads the last
  one and `with_ngap_ie_mut!` changes the last one, and any of them that does
  not decode is an error for both, but for a `lax` field, which the last one
  decides. 0.4.2 read the last one that decoded,
  and changed the first one or, when that did not decode, nothing.
- No function of `helpers` panics, and none cuts a number short. Each of
  these returns a `Result` with an `Error` of the kind `Argument`, so a call
  takes a `?` or an `.unwrap()`:
  - `plmn(mcc, mnc) -> Result<PLMNIdentity, Error>`, for an MCC that is not
    three digits or an MNC that is not two or three, where it panicked;
  - `int_to_bitvec` and `bytes_to_bitvec` are gone with the bit strings of
    rasn: `Bits::new(value, width)?` is a number in a width, which refuses a
    width over 64 bits, where `int_to_bitvec` panicked, and a value that does
    not fit in the width, whose high bits it dropped, and
    `BitString::from(octets)` is the bits of octets;
  - `guami(…) -> Result<GUAMI, Error>`, for an AMF Set ID over 10 bits or an
    AMF Pointer over 6: `guami(plmn, 1, 2000, 70)` built the set 976 and the
    pointer 6;
  - `nr_cgi(…) -> Result<NRCGI, Error>` and
    `global_gnb_id(…) -> Result<GlobalGNBID, Error>`, for a gNB ID over 24
    bits or a cell over 12, which they masked.
- `helpers::tai` takes the TAC as `[u8; 3]`, where it took a slice and
  panicked unless it had three octets: `tai(plmn, [0, 0, 1])`.
- `helpers::ue_security_capabilities(encryption, integrity)` takes the two
  octets, where it took a slice, read its first two octets and took a
  missing one for zero: `ue_security_capabilities(0xe0, 0xe0)`.

### Added

- `pdu.check()`, and `oxirush_ngap::Deviation`: how a PDU deviates from what
  the ASN.1 assigns its message, without any feature. It reports a
  criticality of the PDU that is not that of its procedure
  (`PduCriticality`) and, against the object set of the message, an IE that
  the set does not have (`UnexpectedIe`), one that is there twice
  (`RepeatedIe`), one that comes after an IE that the set has after it
  (`OutOfOrder`), a criticality that is not the assigned one
  (`IeCriticality`), a mandatory IE that is missing (`MissingIe`) and an IE
  whose value does not decode as its type (`UndecodableIe`), each with the
  identifier, the name, the position and the criticality of the IE, and with
  a sentence as its `Display`. It refuses nothing: a message that deviates is
  built, written from a tree, encoded and decoded as before, for the tests
  that send one. A conditional IE is not reported, as its condition is in
  the prose of TS 38.413, and the IEs that a value holds are not looked at.
  0.4.2 left all of it to the caller.
- A value outside the root of an extensible constraint, as `AMFName("")`
  where the type has `SIZE (1..150, ...)`, is written as an extension, which
  is right for a test and no value for an AMF to send. `check()` reports it
  as `Deviation::OutsideRoot`, and `aper::encode_strict` refuses it.
- What a receiver answers a message that deviates with, as clause 10 of
  TS 38.413 has it: `deviation.cause()` is the protocol cause, a transfer
  syntax error for an IE that does not decode, an abstract syntax error by
  its criticality for an IE that the message cannot have or that is missing,
  and a falsely constructed message for an IE that is there twice or in a
  wrong order. `deviation.diagnostics_item()` is the item of the Criticality
  Diagnostics, from `deviation.ie()` and `deviation.type_of_error()`.
- An error of the macros is about an IE, and says which: `error.ie()` has the
  identifier of the IE that is missing, with the criticality that the object
  set assigns it, or of the IE that does not decode, with the criticality
  that it was received with and its position. `Deviation::of(&error)` is the
  deviation that `check()` reports for it. The macro that extracts stops at
  the first such IE, and `check()` has them all.
- `pdu.protocol_ies()` and `pdu.set_protocol_ies(ies)`: the protocol IEs of
  the message of a PDU, whatever the message is, and the same PDU with other
  IEs. The first gives the `ProtocolIEContainer` of the bindings, with each
  IE as it was received: its identifier, its criticality and the octets of
  its value, for an IE that the message cannot have, one that the protocol
  does not have and one that is there twice as for any other. The second
  keeps the direction, the procedure code and the criticality of the PDU,
  and checks nothing. Both are an error of the kind `WrongMessage` for the
  private message, which has no container of protocol IEs.
- The optional `inspect` feature: `pdu.to_view()` gives a decoded PDU as its
  view, a `serde_json` value in the ASN.1 JSON encoding, which is edited and
  which `pdu.with_view(&view)` encodes again; `NGAP_PDU::from_view(&view)`
  gives the PDU of any view, and `with_view` refuses the view of another
  message than the PDU, with an error of the kind `Form`. What is not edited
  keeps the octets received, and an IE is added by its typed value or by its
  octets. The two JSON forms are methods of the PDU, by the names that
  oxirush-nas has for them. Before the release, the revisions of this version
  had them as functions of `inspect`: `message_view`, `message_from_view`,
  `message_tree` and `message_from_tree`, and before that `inspect_pdu` and
  `encode_pdu`, which called the view a tree.
  The transfers that an `OCTET STRING` contains are decoded in place, as
  fields and as IEs, and one that is added is written by its `decoded` value
  alone. A PLMN identity, a transport layer address and the usual identifiers
  read and write as they are usually written, and an `ENUMERATED` value is
  named in any case.
- With the `inspect` feature, the values of a view by their paths:
  `inspect::paths` gives each value with the path that selects it,
  `inspect::select` the values at a path, and `inspect::set`, `remove` and
  `insert` edit a view at a path. `/ngap` stands for the IEs of the message,
  and an IE goes by its name, as in `/ngap/RAN-UE-NGAP-ID/value`, by its
  position or by its identifier, in the message of the PDU and in a message
  that a transfer contains.
- With the `inspect` feature, `inspect::message_name` gives the name that
  ASN.1 has for the message of a PDU, such as `InitialContextSetupResponse`,
  `inspect::message_named` the direction and the procedure code of the
  message of a name, in any case, and `inspect::message_names` all of them.
- With the `inspect` feature, every function that can fail returns the
  `Error` of the crate, and its kind says what a caller can do: `Path` for a
  path that is not well formed, `NoSuchMember` for a name that the type at
  its place cannot have, `NothingSelected` for an edit that selects nothing,
  `NotOpened` for a path through the type that octets carry before
  `inspect::open` decoded them, `Value` for a JSON value that is not a value
  of its ASN.1 type, `Form` for JSON that is no tree and no view, `Edit` for
  an edit that is refused, `Limit` for what is over the limits of the
  inspection, and `UnknownMessage` and `UnknownIe`. The message of a `Value`
  says at which member the value is wrong, by the names that the value has:
  `this is not a value of Cause, at radioNetwork: "no-such-cause" is no
  value of the ENUMERATED`. The revisions of this version before the release
  returned a `String`, which for such a value was the text of rasn.
- `NgapPduKind` names the seven MBS procedures, whose fourteen messages were
  `Other`.
- Identifiers, builders and extractors for the IEs whose type is an
  `OCTET STRING (CONTAINING ...)`, and names for the four IEs whose type is
  a plain `OCTET STRING`.
- `NGAP_PDU::decode_borrowed(&octets)`: the PDU that borrows its message from
  the octets, for a caller that reads a PDU where it was received. It copies
  nothing, where `NGAP_PDU::decode` copies the message once to own it, and
  `into_owned()` gives the PDU that `decode` gives. `pdu.message::<T>()`
  gives the message of a PDU with the lifetime of what the PDU borrows from,
  so that a function which is given octets returns their message, where the
  message of `decode_value` lives as long as the PDU. `pdu.encode_into(&mut
  octets)` appends the octets of a PDU to a buffer that is kept.
- `oxirush_ngap::aper`, the `oxirush-aper` that the crate is built with, and
  at the root of the crate `Octets`, `Bits`, `BitString`, `AsnType`, `Decode`
  and `Encode`: a crate that names the values of the bindings takes them
  from there, and needs no dependency of its own. The examples do.
- `oxirush_ngap::Message`, which the type of each message of a procedure
  implements: its `DIRECTION` and its `PROCEDURE_CODE`. The name that ASN.1
  gives it is the `NAME` of `AsnType`, which every type of the bindings has.
- With the `inspect` feature, `inspect::check_path`: whether a path can select
  anything in a message of a name, without a view. The IE has to be one of
  the object set of the message, and each segment under it a member that the
  type of the value has. `src/inspect_registry.rs` lists what each type has,
  from the bindings: one line for each SEQUENCE and each CHOICE.
- With the `inspect` feature, `inspect::message_ie_criticalities` and
  `inspect::message_criticality`: the criticality that ASN.1 assigns to each
  IE of the object set of a message, and to the procedure of the message.
  The list of `src/registry.rs` has both on the line of each procedure.
- With the `inspect` feature, `inspect::item_criticality`: the criticality
  that ASN.1 assigns to an IE that a value holds alone, in a
  `ProtocolIE-SingleContainer`, by its identifier, when its object sets give
  it one. The list of `src/registry.rs` has it on the line of the IE.
- With the `inspect` feature, `inspect::enumerated_at` gives the names that
  the value at a path of a message can have when its type is an `ENUMERATED`,
  from the bindings, without a view: a caller can refuse a name that no
  message has before it compares one.
- With the `inspect` feature, the tree of a PDU: the short form, in which a
  message is shown and written as its name and its IEs by name.
  `NGAP_PDU::from_tree(&tree)` makes a PDU from `{"message": NAME, "ies":
  [{NAME: VALUE}, …]}`, with the procedure code, the direction and the
  criticalities that the specification assigns, and `pdu.to_tree()` shows a
  PDU in that form.
- With the `inspect` feature, `value` and `decoded` may be left out. In a
  path, a segment that names no member of an IE is taken in its value, and
  one that names no member of a transfer in its decoded value:
  `/ngap/Cause/radioNetwork` is `/ngap/Cause/value/radioNetwork`. A transfer
  is written as its decoded value itself, as well as with `decoded`.
  `to_tree` shows a transfer as its decoded value and each IE that a value
  holds as its name with its value, to any depth, as in
  `{"pDUSessionResourceSetupRequestTransfer": {"protocolIEs":
  [{"PDUSessionType": "ipv4", "criticality": "reject"}]}}`, and `from_tree`
  reads that form. What is written with the two words is taken as it was.
- With the `inspect` feature, the content of a handover's transparent
  container, which TS 38.413 gives by reference where the ASN.1 has an
  `OCTET STRING`. The octets of `SourceToTarget-TransparentContainer`,
  `TargetToSource-TransparentContainer` and
  `TargettoSource-Failure-TransparentContainer` are written by the name of
  the NG-RAN type that they carry, as in
  `{"SourceToTarget-TransparentContainer":
  {"SourceNGRANNode-ToTargetNGRANNode-TransparentContainer": {…}}}`, and a
  path goes through that name. `inspect::open` decodes, in a view, the octets
  that a path takes so, and nothing else does: a view shows them as octets
  until then, as nothing in a message says what they carry.
- With the `inspect` feature, a path may say which IE an entry is. After an
  entry of a list of IEs, or an IE that a value holds alone, the name of that
  IE stays on it, so the names and the positions that `to_tree` shows,
  joined, are the path of a value, as in
  `/ngap/PDUSessionResourceSetupListSUReq/0/pDUSessionResourceSetupRequestTransfer/protocolIEs/0/UL-NGU-UP-TNLInformation/gTPTunnel/gTP-TEID`.
  The name of another IE there is an error that says which IE the entry is,
  and `inspect::check_path` reads what follows a position and a name against
  the type of that IE.
- With the `inspect` feature, an IE of an extension container has the type
  that the type which the container extends gives it, where its identifier
  alone does not say its type. `id-CurrentQoSParaSetIndex` is an
  `AlternativeQoSParaSetIndex` in a QoS flow that is associated, accepted,
  modified or forwarded and an `AlternativeQoSParaSetNotifyIndex` in one that
  a notification is about, and `id-QosFlowAdditionalInfoList` a
  `QosFlowAdditionalInfoListRelCom` in the transfer of a release command and
  a `QosFlowAdditionalInfoListRelRes` in that of a release response. The two
  are read, edited and written in a view and in a tree, and
  `inspect::check_path` and `inspect::enumerated_at` read a path into them
  against their type. The list of `src/registry.rs` has each of their types
  with the types that it extends.
- Vectors with content, in `tests/fixtures/vectors.tsv`: 479 messages that
  pycrate 0.8.1 encoded from the ASN.1 of TS 38.413, beside the 144 of
  `messages.tsv`, which have what is mandatory and nothing else. They hold
  each message with every IE of its object set, each of the 489 IEs that the
  crate names, where the messages of `messages.tsv` hold 111, each
  alternative of each `CHOICE`, each value of each `ENUMERATED` but those of
  the type of handover, and strings and lists around each form of the length
  determinant, in fragments too. The tests read each with its types and
  write it back to the same octets, from its values alone too.
  `tests/fixtures/later.tsv` has messages of a later release, with additions
  that the bindings do not have: they are read as the messages without them.
  `tests/fixtures/generate.py` writes the three files, and
  `tests/fixtures/dissect.py` gives them to tshark.
- PDUs that other implementations sent, in `tests/fixtures/captured.tsv`: 177
  distinct PDUs that were captured on the wire in lab runs, 85 that the AMF
  of Open5GS 2.8.0 sent, 79 that the AMF of free5GC 4.2.3 sent and 13 that
  PacketRusher sent as gNB, each with its sender, its direction and what
  tshark 4.6.4 says the frame is. Nothing of this crate wrote or chose them.
  `tests/captured.rs` reads each and writes it back as the octets that were
  sent, and with `inspect` the message has the name that tshark gives it,
  each of its IEs is read with its type, and its view, its values alone and
  its tree give the octets back. The comparison with the oracle, the corpus
  of the fuzz targets and `tests/fixtures/dissect.py`, which gives them to
  tshark, take them beside the vectors.
- For maintainers: `build/asn1.sha256` records the SHA-256 of the six ASN.1
  modules that the bindings are generated from, `build/extract_asn1.py`
  takes them out of the archive that 3GPP publishes, and the generator reads
  no module that has another digest.
- An oracle for the tests, in `oracle/`: the same types with the codec of
  rasn 0.29, another implementation of aligned PER, which the generator
  writes beside the bindings. The tests give each vector, and what a peer may
  send in its place, to both, and compare what they read and write: the PDU,
  the message, and with `inspect` each IE and each transfer of a view. The
  fuzz targets, in `fuzz/`, which CI runs, make the same comparison on the
  octets that they find. rasn is a dependency of the tests alone.
  The crate depends on the oracle by path, so a workspace that has the crate
  as a member has the oracle as a member too: the oracle has no workspace of
  its own, and is empty without its feature `rasn`, which the tests and the
  fuzz targets ask for, so that such a workspace, with the feature resolver
  2 or 3, builds nothing of rasn until it builds the tests of the crate. `tests/workspace_member.sh` makes such a
  workspace, and CI has cargo load it and clippy and rustdoc accept it.

### Changed

- An identity prints as it is read, and whole. A `PLMNIdentity` prints as its MCC
  and its MNC, `PLMNIdentity(208-93)`, where it printed three numbers, and
  converts to and from the `Plmn` of `oxirush-utils`, which the crate has as
  `oxirush_ngap::identity` and now depends on. A type that is a fixed number of
  octets prints them in hexadecimal, as `FiveGTMSI(deadbeef)` and
  `TAC(000001)`. Each has a `Display`, which is the same without the name of the
  type.
- `Debug` prints no octet of a key. A `SecurityKey`, which is K_gNB or the
  next hop NH, prints as `SecurityKey(256 bits)`, and so in every value that
  holds one, as a `SecurityContext`. The octets of an IE whose type holds a
  key, and those of a message whose object set has such an IE, print as their
  length: `value: 32 octets`. A tree and a view are the data of a message, and
  have the key.
- The examples are named after the protocol, `ngap_build_pdu` where it was
  `build_pdu`, and so are their files: a workspace that has oxirush-ngap and
  oxirush-s1ap as members builds the examples of both, which wrote the
  same files under the same names.
- The macros and the `inspect` feature read one generated list,
  `src/registry.rs`, with one line for each procedure and for each IE. An IE
  or a procedure that a macro does not know is reported by its name, with
  the names that are close to it, where the error named the first IE of the
  specification. The variants of `NgapPduKind` are in the order of the procedure
  codes, and each says its message.
- The crate no longer depends on `paste`: the macros paste no name.
- A call of `build_ngap!` or `build_ngap_ie!` that leaves out a criticality is
  a compile error that says which one: `the criticality of the IE … is
  missing: write REJECT, IGNORE or NOTIFY before its name`, where the compiler
  said ``no rules expected `(` ``. A word that is no criticality is named too.
- `build_ngap!` takes the message that the procedure has in that direction, and
  the other macros a type that has IEs: another name does not compile, where
  it was not looked at. `NgapPduKind::Other` has the `direction` that
  `direction()` gives, as `"UnsuccessfulOutcome"`, where it had
  `"Unsuccessful"`.
- With the `inspect` feature, `inspect::message_ies`: the IEs that a message
  can have, as its ASN.1 object set lists them, each with its identifier and
  whether its presence is mandatory. The list of `src/registry.rs` has them
  on the line of each message.
- An edit at a path of an inspection view changes what is sent or is refused.
  `set` with `null` takes an optional member out, as its documentation said,
  where it wrote a `null` that did not encode. The value of an IE and the
  decoded value of a transfer are not taken out, and the `octets` of a
  transfer are selected and set as those of an IE: these edits returned
  without an error and sent the octets received. A transfer with a member
  that it does not have is refused. The `octets` of an IE or a transfer whose
  value an edit changed are an error to select, and `*` leaves out the
  members that start with `_`.
- The error of an identifier that no object set has is `NGAP IE N is unknown`,
  where it said `is unknown or has several types`. An IE that has a type for
  each type that it extends has none where it extends none of them, and its
  error names those types.
- The name of an ENUMERATED value of the PDU itself, its `criticality`, is
  taken whatever its case, as those of its IEs are.
- What is nested deeper than 64 levels stays as its octets beside a
  `_decode_error`: `to_view` refused the whole PDU.
- A path of an inspection view selects nothing for an OPTIONAL member that
  is absent and for another alternative of a CHOICE, and a name that the type
  of a value cannot have is an error that lists the members of the type: both
  were the same error. What follows an absent value in a path is checked
  against the type, `*` on a value without members selects nothing, and after
  `*` a segment is an error when none of the values can have it. The name of
  an IE in a list whose entries are no IEs is an error, where it selected
  nothing, and so is a path into a value that did not decode.
- One rule for the names of a path: a name is its letters and its digits,
  whatever their case and whatever is between them. The root, the members of
  a value and the names of ENUMERATED values are taken as the names of IEs
  and of messages were: `/ngap/cause/value/radionetwork`. A position is a
  number as decimal writes it: `+1` and `01` are not.
- `set` refuses a name that the type of a value does not have, where the
  error came from `from_view`, and writes a member by the name that
  ASN.1 gives it. A member that its type always has is not taken out, and an
  alternative of a CHOICE takes the place of the one that is there.
- An IE is written with the name of its IE as `id` and with its `octets`
  wherever a view has IEs: `from_view` takes them in any entry, where
  only `set` and `insert` on a path under `/ngap` did, and the `octets` of an
  IE that was added are set at its path. An `_original_id` that is no number
  is refused.
- `paths` lists the `octets` of an IE or of a transfer that has no value
  beside them.
- The JSON forms are written and read by the bindings themselves, which make
  a `serde_json::Value` of a value and back with no text between them, where
  the codec of the JSON Encoding Rules of rasn wrote text that was parsed.
  A value has the form that it had, which the documentation of `inspect`
  lists under Values, and two values that stayed octets beside a
  `_decode_error` have one:
  - an integer of 2^63 or more is a number;
  - a `BIT STRING` of another size than the one that its type gives beside
    an extension marker is `{"value", "length"}`, as an algorithm mask of 24
    bits is `{"value": "C00000", "length": 24}`. One of the size of its type
    is its octets in hexadecimal, as before, and a path goes into the two
    members.

  A value that is written is read strictly, and the error says where the
  fault is by the names of the value: a member that its type does not have,
  a second alternative of a `CHOICE`, an integer outside its constraint or
  written as its digits, a bit string with other octets than its length has
  and padding bits that are not zero. The name of an `ENUMERATED` value is
  taken whatever its case and its separators where it is read.
- The codec reads what a peer may send as rasn read it where the value is
  the same: padding bits that are not zero, a length in a longer form than
  it needs and a number with leading zero octets, up to the octets that a
  number of 64 bits may have. It refuses a number of no octet, where ITU-T
  X.691 has one at least.
- The examples of the documentation and of the README are compiled and run
  by the documentation tests.
- The crate compiles to less code, with `inspect` and without: a type
  reads and writes its fields with the functions of `oxirush-aper`, which
  are compiled once, where the derived codec of rasn was compiled for each
  type in each of its encoding rules.
- A message is found by the code of its procedure before its direction,
  the name of an IE by a binary search of its identifier, and its
  identifier by a hash of its name. `check` names an IE where it reports a
  deviation, and no longer each IE of the message.
- A view and a tree are built without copying what they hold: a value goes
  into the object that holds it by a move, `from_tree` makes the PDU of its
  message without writing a view of it first, and the members of a value
  are read where the list of the types has them.
- Hexadecimal is written and read without a table and without a branch for
  each digit, in loops that the compiler writes for several octets at once.

### Fixed

- The octets of a PLMN identity with an MNC of three digits, which is a fix
  of the wire format of 0.4.2. TS 38.413 clause 9.3.3.5 has the three digits
  of the MCC, then those of the MNC, two in each octet, the second over the
  first: 310-410 is `13 40 01`. `helpers::plmn` wrote `13 00 14`, the order
  that TS 24.008 gives a NAS message, which a decoder of NGAP reads as
  310-041, and `helpers::plmn_from` read 310-410 in those octets. Both have
  the order of the clause now, and so has the `"310-410"` that the `inspect`
  feature writes and shows. An MNC of two digits has the same octets in both
  orders: 208-93 is `02 f8 39` as before.
- APER encoding and decoding of constrained `SEQUENCE OF` lists,
  size-constrained `UTF8String` values, fixed-size `BIT STRING` values
  longer than 16 bits, strings with a two-octet length, and empty open type
  values.
- Decoding consumes the unknown extension additions of an extensible
  `SEQUENCE`, and rejects incomplete values and trailing octets.
- A claimed list length no longer reserves memory before its elements are
  read.

Earlier releases are described by their tags.
