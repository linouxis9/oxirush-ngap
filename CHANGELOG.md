# Changelog

All notable changes to `oxirush-ngap` are recorded here.

## Unreleased (0.5.0)

The bindings come from TS 38.413 V19.4.0 (Release 19) instead of V16.6.0,
and the APER codec follows ITU-T X.691 where rasn 0.28 departs from it. It
is not source compatible with 0.4.2.

### Breaking changes relative to 0.4.2

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
- The minimum supported Rust version is 1.88, which rasn 0.28.15 needs:
  0.4.2 declared 1.85 and did not build with it.

### Added

- The optional `inspect` feature: a decoded PDU as a `serde_json` tree in the
  ASN.1 JSON encoding, which can be edited and encoded again. What is not
  edited keeps the octets received, and an IE is added by its typed value or
  by its octets. The transfers that an `OCTET STRING` contains are decoded in
  place, as fields and as IEs, and one that is added is written by its
  `decoded` value alone. A PLMN identity, a transport layer address
  and the usual identifiers read and write as they are usually written, and
  an `ENUMERATED` value is named in any case.
- With the `inspect` feature, the values of a tree by their paths:
  `inspect::paths` gives each value with the path that selects it,
  `inspect::select` the values at a path, and `inspect::set`, `remove` and
  `insert` edit a tree at a path. `/ngap` stands for the IEs of the message,
  and an IE goes by its name, as in `/ngap/RAN-UE-NGAP-ID/value`, by its
  position or by its identifier, in the message of the PDU and in a message
  that a transfer contains.
- With the `inspect` feature, `inspect::message_name` gives the name that
  ASN.1 has for the message of a PDU, such as `InitialContextSetupResponse`,
  `inspect::message_named` the direction and the procedure code of the
  message of a name, in any case, and `inspect::message_names` all of them.
- `NgapPduKind` names the seven MBS procedures, whose fourteen messages were
  `Other`.
- Identifiers, builders and extractors for the IEs whose type is an
  `OCTET STRING (CONTAINING ...)`, and names for the four IEs whose type is
  a plain `OCTET STRING`.
- The `sized` module: the `OCTET STRING` and `BIT STRING` types whose length
  determinant takes two octets.

### Changed

- The macros and the `inspect` feature read one generated list,
  `src/registry.rs`, with one line for each procedure and for each IE. An IE
  or a procedure that a macro does not know is reported by its name, with
  the names that are close to it, where the error named the first IE of the
  specification. The variants of `NgapPduKind` are in the order of the procedure
  codes, and each says its message.
- The crate no longer depends on `paste`: the macros paste no name.
- `build_ngap!` takes the message that the procedure has in that direction, and
  the other macros a type that has IEs: another name does not compile, where
  it was not looked at. `NgapPduKind::Other` has the `direction` that
  `direction()` gives, as `"UnsuccessfulOutcome"`, where it had
  `"Unsuccessful"`.
- With the `inspect` feature, `inspect::message_ies`: the IEs that a message
  can have, as its ASN.1 object set lists them, each with its identifier and
  whether its presence is mandatory. The list of `src/registry.rs` has them
  on the line of each message.
- An edit at a path of an inspection tree changes what is sent or is refused.
  `set` with `null` takes an optional member out, as its documentation said,
  where it wrote a `null` that did not encode. The value of an IE and the
  decoded value of a transfer are not taken out, and the `octets` of a
  transfer are selected and set as those of an IE: these edits returned
  without an error and sent the octets received. A transfer with a member
  that it does not have is refused. The `octets` of an IE or a transfer whose
  value an edit changed are an error to select, and `*` leaves out the
  members that start with `_`.
- The name of an ENUMERATED value of the PDU itself, its `criticality`, is
  taken whatever its case, as those of its IEs are.
- What is nested deeper than 64 levels stays as its octets beside a
  `_decode_error`: `inspect_pdu` refused the whole PDU.

### Fixed

- APER encoding and decoding of constrained `SEQUENCE OF` lists,
  size-constrained `UTF8String` values, fixed-size `BIT STRING` values
  longer than 16 bits, strings with a two-octet length, and empty open type
  values.
- Decoding consumes the unknown extension additions of an extensible
  `SEQUENCE`, and rejects incomplete values and trailing octets.
- A claimed list length no longer reserves memory before its elements are
  read.

Earlier releases are described by their tags.
