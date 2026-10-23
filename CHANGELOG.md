# Changelog

All notable changes to `oxirush-ngap` are recorded here.

## Unreleased (0.5.0)

The bindings come from TS 38.413 V19.4.0 (Release 19) instead of V16.6.0, and
their codec is [`oxirush-aper`](https://github.com/linouxis9/oxirush-aper),
aligned PER by ITU-T X.691, where 0.4.2 derived the codec of rasn 0.28. It is
not source compatible with 0.4.2.

### Breaking changes relative to 0.4.2

- `Display` of a PDU is the name of its message, its procedure code and its
  direction, then the identifiers of the UE that it has, by the names of
  their IEs: `UEContextReleaseRequest (procedure 42, initiating message),
  AMF-UE-NGAP-ID 42, RAN-UE-NGAP-ID 7`, where 0.4.2 printed
  `InitiatingMessage UEContextReleaseRequest (code=42)`. `Debug` prints octets in
  hexadecimal in lower case.
- The crate no longer depends on `rasn`, and `oxirush_ngap::rasn` is gone.
  The bindings implement `AsnType`, `Decode` and `Encode` of
  `oxirush-aper`, which the crate re-exports as `oxirush_ngap::aper`, and
  have aligned PER alone.
- A field has the words of the ASN.1 identifier of its member: `p_lmnidentity`
  is `plmn_identity`, `i_e_extensions` is `ie_extensions`, `g_nb_id` is
  `gnb_id`, `g_tp_teid` is `gtp_teid`. `build/fields.tsv` of the
  repository has each field with its name in 0.4.2.
- The values of the bindings are native numbers, arrays and the types of
  `oxirush-aper`:

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

- A type that holds octets has a lifetime, as `NASPDU<'a>`, and borrows them
  from what it was decoded from: `into_owned()` gives the `'static` value to
  keep. A type that holds none, as `Cause`, `TAI` or `GUAMI`, has no
  lifetime, as the extension container of a value owns its octets.
  `NGAP_PDU` is `NGAPPDU<'static>`, and a pattern on a PDU that borrows names
  `NGAPPDU`. The message of `decode_value` borrows from the PDU, which is a
  variable of its own: `let pdu = NGAP_PDU::decode(&octets)?;`.
- `decode_value::<T>()` takes the type of a message, and refuses a PDU that
  holds another one (`WrongMessage`): 0.4.2 decoded the octets as any type.
- `decode_open_type` takes `&[u8]` and `encode_open_type` gives `Octets`.
  The `value` of an IE, of an extension and of a PDU is an `Octets`, where it
  was an `Any`: `&ie.value` is what `ie.value.as_bytes()` was.
- One error type, `oxirush_ngap::Error`, whose `kind()` is matched on, for
  every call and every macro, where they returned the errors of rasn.
  `build_ngap!` and `build_ngap_ie!` give a `Result`, where they panicked
  for a value that does not encode. `with_ngap_ie_mut!` gives a
  `Result<bool, Error>`. `extract_ngap_ies!` returns an `Error` from the
  enclosing function, and `MissingIeError` is gone: a `req` field whose IE
  is missing is `MissingIe`, and an IE that does not decode is
  `UndecodableIe` for a `req` and for an `opt` field, where 0.4.2 called it
  missing or `None`. A field that is to go on without an IE that does not
  decode is `lax`.
- When a message has an IE twice, `extract_ngap_ies!` and `with_ngap_ie_mut!`
  take the first one, and do not decode the second; `check()` reports it as
  `Deviation::RepeatedIe`, and `ies::<T>()` gives each.
- The macros, the readers, `check()` and the registry are those of
  `oxirush_aper::protocol`, which oxirush-ngap and oxirush-s1ap share:
  `Message` is `oxirush_aper::protocol::Message`, re-exported, with its
  `Protocol`, and the hidden `registry::ies::NAME` modules are the marker
  types of the module `ie`: `ie::NAME::ID` is `<ie::NAME as Ie>::ID`.
- Each protocol IE and extension container is one type, `ProtocolIEContainer`
  of `ProtocolIEField` and `ProtocolExtensionContainer` of
  `ProtocolExtensionField`, in place of one for each use, as
  `InitialUEMessageProtocolIEs`. The `id` of a field is a `ProtocolIEID` or
  a `ProtocolExtensionID`.
- The macros name an IE by its identifier, and a type name stands for the IE
  of that type only where one IE has it, where 0.4.2's `AMFName` built
  id-OldAMF.
  `build_ngap!` takes the message that the procedure has in that direction,
  and a criticality that a call leaves out is the one that the specification
  assigns: that of the procedure for the PDU (`Message::CRITICALITY`), and that
  of the object set of the message for an IE, which an IE that the set does
  not have has not, an error of the kind `Argument`.
- No helper panics, and none cuts a number short. `plmn`, `guami`, `nr_cgi`
  and `global_gnb_id` return a `Result` with an error of the kind
  `Argument`; `nr_cgi` and `global_gnb_id` take the length of the gNB ID,
  22 to 32 bits (TS 38.413 clause 9.3.1.6), where 0.4.2 had 24; `tai` takes
  `[u8; 3]`, `ue_security_capabilities` its two octets, and `plmn_from`
  returns an `Option`. `int_to_bitvec` and
  `bytes_to_bitvec` are gone: `Bits::new(value, width)?` and
  `BitString::from(octets)` take their place.
- An `INTEGER` is held in 64 bits. A larger number, which only an extension
  marker lets a later release send, is an error of its IE, and the rest of the
  PDU is read.
- `NgapPduKind` names the MBS procedures, its variants are in the order of
  the procedure codes, and `Other` has the `direction` that `direction()`
  gives, as `"UnsuccessfulOutcome"`.
- `PLMNIdentity` prints as its MCC and its MNC, `PLMNIdentity(208-93)`, and a
  type of a fixed number of octets prints them in hexadecimal,
  `TAC(000001)`. `Debug` prints the length of a key, not its octets.
- `helpers::plmn` writes another order for an MNC of three digits: see Fixed.
- The examples are named after the protocol: `ngap_build_pdu`.
- The minimum supported Rust version is 1.88: 0.4.2 declared 1.85 and did not
  build with it.
- The crate depends on one version of `oxirush-aper` exactly, `=0.1.0`: it
  builds on the module `inspect` of `oxirush-aper`, which is hidden and changes
  with the two protocol crates, and the three are released in step.
- The specification renamed types since V16.6.0: `UserLocationInformationN3IWF`
  is `UserLocationInformationN3IWFWithPortNumber`, beside
  `UserLocationInformationN3IWFWithoutPortNumber`; `PagingeDRXInformation`,
  `PagingEDRXCycle` and `PagingTimeWindow` are `EUTRAPagingeDRXInformation`,
  `EUTRAPagingEDRXCycle` and `EUTRAPagingTimeWindow`, beside those of NR and of
  NB-IoT.

### Added

- Each transfer of protocol IEs, as `PDUSessionResourceSetupRequestTransfer`,
  has the object set that TS 38.413 gives it: the macros give its IEs their
  criticality, a tree writes them without it, and `check` and
  `check_structure` of its type compare the octets of a transfer with the
  set as those of a PDU compare its message.

- `ie`: a marker type for each IE, by the names that the macros take it by,
  and `ies::<T>()` and `ie::<T>()` of a PDU, of a message and of a transfer:
  every occurrence of an IE in order, each decoded alone, and the first. On a
  PDU they read the octets of its message where they are, without
  `decode_value` and without allocating.
- `extract_ngap_ies!` takes a PDU as well as a message, as the message that it
  holds: it walks the IEs once and decodes those that a field asks for.
- `check_structure()` of a PDU: the object set of its message, the presence,
  the order, the repetition and the criticality of its IEs, without decoding a
  value, for a receiver that reads the IEs that it needs; `check()` decodes
  every value too.
- `pdu.check()`: how a PDU deviates from the object set of its message, as
  `Deviation`s, refusing nothing. `deviation.cause()` and
  `deviation.diagnostics_item()` give what a receiver answers, as TS 38.413
  clause 10 has it, `pdu.criticality_diagnostics(&deviations)` the whole
  Criticality Diagnostics IE of an ERROR INDICATION, with the procedure code
  and the triggering message, `pdu.criticality_diagnostics_for_response(&deviations)`
  that of a response of the procedure, without them (clause
  9.3.1.3), `pdu.criticality()`, and `error.ie()` the IE of an
  error of the macros. A value outside the root of an extensible constraint
  and one in another form than the canonical one are reported too, and
  `aper::encode_strict` and `aper::decode_strict` refuse them. A procedure
  that the specification does not have is the deviation `UnknownProcedure`,
  which a receiver answers by the criticality of the PDU (clause 10.3.4.1).
- `pdu.protocol_ies()` and `pdu.set_protocol_ies(ies)`: the IEs of any
  message, and `NGAP_PDU::of_message(&message, criticality)`, the PDU of a
  message that is at hand.
- `NGAP_PDU::decode_borrowed`, which copies nothing, `pdu.message()`, which
  lives as long as what the PDU borrows, and `pdu.encode_into(&mut octets)`.
- A field of `extract_ngap_ies!` of the type of the one value of its IE,
  `req id: u64 = AMF_UE_NGAP_ID(id)`, and `build_ngap_ie!` without the name
  of a message, for an IE that is in none.
- `oxirush_ngap::Message`, which each message of a procedure implements. The
  macros also read, change and build the IEs of the seven transfers of
  protocol IEs, as `PDUSessionResourceSetupRequestTransfer`.
- `PLMNIdentity` converts to and from the `Plmn` of `oxirush-utils`, which
  the crate has as `oxirush_ngap::identity`.
- Each type of the bindings says in its documentation what it is in ASN.1:
  `AMF-UE-NGAP-ID ::= INTEGER (0..1099511627775)`, and what the comments of the ASN.1 say of it, without the
  banners of the sections of the modules. The pages of the macros show the
  forms that a caller writes, with an example of `=>`.
- The `inspect` feature: a PDU as JSON, the trees of `oxirush-utils`
  (`oxirush_ngap::tree`), as its tree (`to_tree`, `from_tree`) and as its view
  (`to_view`, `with_view`, `from_view`), with the paths of the values of a
  view (`select`, `paths`, `set`, `insert`, `remove`, `open`, `check_path`,
  `enumerated_at`) and what the ASN.1 says of each message (`message_ies`,
  `message_ie_criticalities`, `item_criticality` and the others). A
  transfer and the container of a handover are read as their types, the
  well-known values as they are usually written, wherever they are, and what
  is not edited keeps the octets received. Every PDU that decodes has a
  tree, which is the same PDU again: a procedure that the protocol does not
  have by its procedure code and its direction, a message that has no list
  of IEs or does not decode by its octets, and an IE or a transfer whose
  typed value has no place for what a later release added by its octets. An
  alternative of the type `NULL` is `{"pLMNWide": null}`, whose `null` a
  path lists and selects. The trees and the paths are those of
  `oxirush-aper`, which oxirush-s1ap shares, on `oxirush-utils`: octets in
  lower case, members in the order of the specification, and the JSON of the
  program that uses the crate as the program has it.
- The `serde` feature: `Serialize` and `Deserialize` through the trees, a
  PDU as the tree of its message and each other type as the tree of a PDU
  shows it, read back in it or in the form of its ASN.1 type. A name that an
  object has twice is refused.
- Tests on 144 messages and 479 vectors that pycrate 0.8.1 encoded, on 177
  PDUs that two open-source cores and a simulator sent, against the codec of
  rasn 0.29 in `oracle/`, with three fuzz targets, and from the published
  package alone. `build/asn1.sha256` records the digests of the ASN.1 modules
  that the bindings are generated from.
- The tools, the programs and the jobs that oxirush-ngap and oxirush-s1ap
  share are those of the repository of `oxirush-aper`, at the revision of
  `build/revisions`, from a checkout beside this repository: its
  `protocols/` has the oracle and its tests, the fuzz targets, the
  benchmarks, the scripts of the fixtures, of the package and of the
  revisions, and the writer of the test of the README, and its workflows
  the jobs of CI and of Reproduce. CONTRIBUTING.md says where each lives.
- `helpers::global_ng_enb_id` and `helpers::eutra_cgi`, of a kind of ng-eNB ID,
  `NgEnbKind`: macro (20 bits), short macro (18) or long macro (21).
- In a tree and through serde, a string of decimal digits is never read as
  octets: a PLMN identity or a number written so (`"310410"`, a TAC
  `"123456"`) is refused with the two values that it could be, and octets whose
  hexadecimal has no letter are written after `0x`. A YAML `true` is the name
  `true` of an ENUMERATED that has it. A member of a tree written twice in two
  spellings is refused. A refusal of a key shows nothing of what is written,
  and a misspelled name of an IE or of a message is refused with the nearest
  names.

### Changed

- The codec reads what a peer may send as rasn read it where the value is the
  same, as padding bits that are not zero or a length in a longer form than
  it needs, and refuses a number of no octet.
- The crate compiles to less code: a type reads and writes its fields with
  the functions of `oxirush-aper`, compiled once, where the derived codec of
  rasn was compiled for each type in each of its encoding rules. It no longer
  depends on `paste`.

### Fixed

- A view and a tree read the value of an IE, and a transfer, alike. Octets in
  another form than the one of X.691, as padding bits that are not zero, are
  their octets with a `_decode_error` in a view, which a path list lists, where
  the view had the value that a tolerant reader gives and the tree the octets.
  A path list lists `_lossy`, where a typed value has no place for all its octets.
- The octets of a PLMN identity with an MNC of three digits. TS 38.413 clause
  9.3.3.5 has the digits of the MCC, then those of the MNC: 310-410 is
  `13 40 01`. `helpers::plmn` wrote `13 00 14`, the order of TS 24.008,
  which a decoder of NGAP reads as 310-041. An MNC of two digits has the same
  octets in both orders.
- Aligned PER of constrained `SEQUENCE OF` lists, size-constrained
  `UTF8String` values, fixed-size `BIT STRING` values longer than 16 bits,
  strings with a two-octet length, and empty open types.
- Decoding skips the unknown additions of an extensible `SEQUENCE`, refuses
  incomplete values and trailing octets, and reserves no memory for a count
  that a length claims before its elements are read.

Earlier releases are described by their tags.
