# oxirush-ngap

[![Crates.io](https://img.shields.io/crates/v/oxirush-ngap.svg)](https://crates.io/crates/oxirush-ngap)
[![Documentation](https://docs.rs/oxirush-ngap/badge.svg)](https://docs.rs/oxirush-ngap)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

A complete 5G NG Application Protocol (NGAP) APER codec generated from the
3GPP TS 38.413 ASN.1 modules with
[`rasn-compiler`](https://crates.io/crates/rasn-compiler) and encoded with
[`rasn`](https://crates.io/crates/rasn).

Part of [OxiRush](https://github.com/linouxis9/oxirush), a mobile-core testing
framework. The LTE companion crate,
[`oxirush-s1ap`](https://crates.io/crates/oxirush-s1ap), exposes the same API
shape, macros, generated-source release model, and example workflow for TS 36.413.

## Features

- Bindings generated from the six normative TS 38.413 ASN.1 modules sourced
  from the official 3GPP 38.413 v19.4.0 (`38413-j40.zip`) archive.
- Checked-in Rust bindings; normal builds and docs.rs do not run a generator,
  and the published crate excludes its raw ASN.1 inputs.
- Aligned PER encoding and decoding through `rasn`.
- Flat re-exports for generated NGAP types and compatibility aliases such as
  `NGAP_PDU`, `AMF_UE_NGAP_ID`, and `RAN_UE_NGAP_ID`.
- `build_ngap!`, `build_ngap_ie!`, `extract_ngap_ies!`, and
  `with_ngap_ie_mut!` helpers for typed open types.
- ASN.1-derived IE IDs, procedure codes, procedure names, directions, and PDU
  kinds.
- Protocol helpers for PLMN, core-network, tracking-area, cell, and radio-node
  identities, plus bit strings and UE security capabilities.

## Quick start

```toml
[dependencies]
oxirush-ngap = "0.5"
```

```rust
use oxirush_ngap::{build_ngap, ngap::*};

let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
    IGNORE, UEContextReleaseRequest,
    REJECT AMF_UE_NGAP_ID(42u64),
    REJECT RAN_UE_NGAP_ID(7u32),
    IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
);

let bytes = pdu.encode().unwrap();
let decoded = NGAP_PDU::decode(&bytes).unwrap();
assert_eq!(decoded.procedure_code(), 42);
assert_eq!(decoded.procedure_name(), "UEContextReleaseRequest");
assert!(decoded.is_initiating());

let request: UEContextReleaseRequest = decoded.decode_value().unwrap();
assert_eq!(request.protocol_ies.0.len(), 3);
```

`build_ngap!` takes a direction, procedure name, outer criticality, message
name, and zero or more IEs. Each IE is written as
`CRITICALITY IeName(value)`. Numeric IE IDs and procedure codes are generated
from the ASN.1 object sets rather than maintained by hand.

## Extracting IEs

```rust
use oxirush_ngap::{extract_ngap_ies, macros::MissingIeError, ngap::*};

fn handle(request: &UEContextReleaseRequest) -> Result<(), MissingIeError> {
    extract_ngap_ies!(request, UEContextReleaseRequest,
        req amf_id: u64 = AMF_UE_NGAP_ID(id),
        req ran_id: u32 = RAN_UE_NGAP_ID(id),
        req cause: Cause = Cause(value) => value,
    );

    println!("AMF={amf_id} RAN={ran_id} cause={cause:?}");
    Ok(())
}
```

A `req` field returns `MissingIeError` from the enclosing function when the IE
is absent or invalid. An `opt` field remains `Option<T>`. Without a custom
`=> expression`, extraction unwraps the generated newtype's `.0` field.

## Common helpers

```rust
use oxirush_ngap::helpers::*;

let network = plmn("208", "93");
let amf = guami(network.clone(), 1, 1, 0);
let tracking_area = tai(network.clone(), &[0x00, 0x00, 0x01]);
let cell = nr_cgi(network.clone(), 0x123456, 1);
let gnb = global_gnb_id(network, 0x123456);
let algorithm_mask = bytes_to_bitvec(&[0xe0]);
let capabilities = ue_security_capabilities(&[0xe0, 0xe0]);
```

## Code generation

Cargo compiles the checked-in `src/ngap.rs` module directly. Normal builds,
docs.rs, and crates.io package verification do not run a generator. The
published crate deliberately excludes both the generator and the raw ASN.1
inputs.

For maintainers, `build/main.rs` contains the `rasn-compiler` generation
pipeline. It reads the six locally supplied `.asn` modules in `ngap/`, then
adds the flat API, typed-open-type macros, procedure metadata, convenience
methods, and `Display` implementation. The `.asn` and `.asn1` inputs are
ignored by Git and must be obtained directly from the official 3GPP 38.413
v19.4.0 (`38413-j40.zip`) archive before regeneration.

Commit `src/ngap.rs` after regenerating it. Do not edit generated bindings by
hand.

## rasn integration

`rasn-compiler`'s stable opaque-open-type mode is used because TS 38.413 relies
heavily on parameterized information object classes. Its experimental typed
mode emits unresolved object-set warnings and uncompilable bindings for these
six modules. `Any` stays inside the generated representation; the public macros
provide typed construction, extraction, and mutation.

The crate enables rasn's efficient `bytes` storage and leaves its unused `f32`
and `f64` features disabled. rasn 0.28's APER codec departs from ITU-T X.691
where NGAP reaches it, so the generator replaces the derived codec there with
narrowly scoped `Encode` and `Decode` implementations written against rasn's
public codec traits:

- constrained `SEQUENCE OF` containers, whose length determinant and
  components rasn misaligns;
- size-constrained `UTF8String` values, whose size is not PER-visible, so
  their length is an unconstrained count of octets;
- fixed-size `BIT STRING` values longer than 16 bits, which are
  octet-aligned;
- `OCTET STRING` and `BIT STRING` components and alternatives with a
  two-octet length determinant, which take the types of the `sized` module.

All other values use rasn-derived codecs unchanged.

## Key types

| Type | Description |
| --- | --- |
| `NGAP_PDU` / `NGAPPDU` | Top-level initiating/successful/unsuccessful PDU choice |
| `InitiatingMessage` | Procedure code, criticality, and message open type |
| `AMFUENGAPID` | 40-bit AMF UE NGAP identifier represented as `u64` |
| `RANUENGAPID` | RAN UE NGAP identifier |
| `Cause` | Radio network, transport, NAS, protocol, or miscellaneous cause |
| `PLMNIdentity` | Three-octet TBCD PLMN |
| `TAI` | 5G tracking-area identity with a three-octet TAC |
| `NRCGI` | PLMN plus 36-bit NR cell identity |
| `SNSSAI` | Slice/service type with optional slice differentiator |
| `NASPDU` | Opaque 5GS NAS payload |

## Examples

```bash
cargo run -p oxirush-ngap --example build_pdu
cargo run -p oxirush-ngap --example decode_manually
cargo run -p oxirush-ngap --example extract_ies
```

- `build_pdu` covers Initial Context Setup request/response, UE Context Release,
  Handover Required/Acknowledge, NG Setup Failure, and standalone IE creation.
- `decode_manually` constructs and inspects an NG Setup Response through rasn's
  raw APER API.
- `extract_ies` extracts UE release, handover, and nested PDU-session/NAS data
  from decoded NGAP messages.

The three examples intentionally mirror the corresponding `oxirush-s1ap`
examples, substituting the standards-defined NGAP messages and IEs.

## References

- 3GPP TS 38.413: NG-RAN NG Application Protocol (NGAP)
- ITU-T X.691: ASN.1 Packed Encoding Rules

## Documentation

Full API reference: [**https://docs.rs/oxirush-ngap**](https://docs.rs/oxirush-ngap)

## Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Sign off your commits (`git commit -s`)
4. Open a Pull Request

### Developer Certificate of Origin (DCO)

By contributing to this project, you agree to the [Developer Certificate of Origin (DCO)](https://developercertificate.org/). This means that you have the right to submit your contributions and you agree to license them according to the project's license.

All commits should be signed-off with `git commit -s` to indicate your agreement to the DCO.

## License

Copyright 2025-2026 Valentin D'Emmanuele

Licensed under the Apache License, Version 2.0. See [LICENSE](https://github.com/linouxis9/oxirush-ngap/blob/master/LICENSE) for details.
