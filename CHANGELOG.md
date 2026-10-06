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
  edited keeps the octets received. The transfers that an `OCTET STRING`
  contains are decoded in place, as fields and as IEs.
- `NgapPduKind` names the seven MBS procedures, whose fourteen messages were
  `Other`.
- Identifiers, builders and extractors for the IEs whose type is an
  `OCTET STRING (CONTAINING ...)`, and names for the four IEs whose type is
  a plain `OCTET STRING`.
- The `sized` module: the `OCTET STRING` and `BIT STRING` types whose length
  determinant takes two octets.

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
