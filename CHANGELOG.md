# Changelog

All notable changes to `oxirush-ngap` are recorded here.

## Unreleased (0.5.0)

The bindings come from TS 38.413 V19.4.0 (Release 19) instead of V16.6.0, and
their codec is [`oxirush-aper`](https://github.com/linouxis9/oxirush-aper),
aligned PER by ITU-T X.691, where 0.4.2 derived the codec of rasn 0.28. It is
not source compatible with 0.4.2.

### Breaking changes relative to 0.4.2

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
  take the last one, and one of them that does not decode is an error.
- Each protocol IE and extension container is one type, `ProtocolIEContainer`
  of `ProtocolIEField` and `ProtocolExtensionContainer` of
  `ProtocolExtensionField`, in place of one for each use, as
  `InitialUEMessageProtocolIEs`. The `id` of a field is a `ProtocolIEID` or
  a `ProtocolExtensionID`.
- The macros name an IE by its identifier, and a type name stands for the IE
  of that type only where one IE has it, where 0.4.2's `AMFName` built
  id-OldAMF.
  `build_ngap!` takes the message that the procedure has in that direction,
  and a missing criticality is a compile error that says which.
- No helper panics, and none cuts a number short. `plmn`, `guami`, `nr_cgi`
  and `global_gnb_id` return a `Result` with an error of the kind
  `Argument`; `tai` takes `[u8; 3]`, `ue_security_capabilities` its two
  octets, and `plmn_from` returns an `Option`. `int_to_bitvec` and
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

### Added

- `pdu.check()`: how a PDU deviates from the object set of its message, as
  `Deviation`s, refusing nothing. `deviation.cause()` and
  `deviation.diagnostics_item()` give what a receiver answers, as TS 38.413
  clause 10 has it, and `error.ie()` the IE of an error of the macros. A
  value outside the root of an extensible constraint and one in another form
  than the canonical one are reported too, and `aper::encode_strict` and
  `aper::decode_strict` refuse them.
- `pdu.protocol_ies()` and `pdu.set_protocol_ies(ies)`: the IEs of any
  message.
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
  `AMF-UE-NGAP-ID ::= INTEGER (0..1099511627775)`.
- The `inspect` feature: a PDU as a `serde_json::Value`, as its tree
  (`to_tree`, `from_tree`) and as its view (`to_view`, `with_view`,
  `from_view`), with the paths of the values of a view (`select`, `paths`,
  `set`, `insert`, `remove`, `open`, `check_path`, `enumerated_at`) and
  what the ASN.1 says of each message (`message_ies`,
  `message_ie_criticalities`, `item_criticality` and the others). A
  transfer and the container of a handover are read as their types, the
  usual values as they are written, and what is not edited keeps the octets
  received. The trees and the paths are those of `oxirush-aper`, which
  oxirush-s1ap shares, on `oxirush-utils`: octets in lower case, members in
  the order of the specification.
- The `serde` feature: `Serialize` and `Deserialize` through the trees, a
  PDU as the tree of its message.
- Tests on 144 messages and 479 vectors that pycrate 0.8.1 encoded, on 177
  PDUs that two open-source cores and a simulator sent, against the codec of
  rasn 0.29 in `oracle/`, with three fuzz targets, and from the published
  package alone. `build/asn1.sha256` records the digests of the ASN.1 modules
  that the bindings are generated from.

### Changed

- The codec reads what a peer may send as rasn read it where the value is the
  same, as padding bits that are not zero or a length in a longer form than
  it needs, and refuses a number of no octet.
- The crate compiles to less code: a type reads and writes its fields with
  the functions of `oxirush-aper`, compiled once, where the derived codec of
  rasn was compiled for each type in each of its encoding rules. It no longer
  depends on `paste`.

### Fixed

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
