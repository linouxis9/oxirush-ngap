# oxirush-ngap

[![Crates.io](https://img.shields.io/crates/v/oxirush-ngap.svg)](https://crates.io/crates/oxirush-ngap)
[![Documentation](https://docs.rs/oxirush-ngap/badge.svg)](https://docs.rs/oxirush-ngap)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

A complete NGAP (NG Application Protocol) codec for 5G, auto-generated from 3GPP ASN.1 definitions (TS 38.413) using Aligned PER (APER) encoding.

Part of the [OxiRush](https://github.com/linouxis9/oxirush) project — a 5G Core Network testing framework.

## Features

- **Full TS 38.413 coverage** — every NGAP procedure, IE, and PDU type
- **Auto-generated from ASN.1** — the codec is rebuilt at `cargo build` from the canonical 3GPP ASN.1 files, tracking spec updates automatically
- **APER encoding/decoding** — standards-compliant Aligned PER via [`asn1-codecs`](https://crates.io/crates/asn1-codecs)
- **Serde support** — all generated types derive `Serialize`/`Deserialize`
- **Round-trip fidelity** — encode then decode produces identical structures
- **Builder macros** — `build_ngap!` and `build_ngap_ie!` eliminate the deeply nested ProtocolIEs boilerplate
- **Extraction macro** — `extract_ngap_ies!` pulls typed fields from decoded messages with `req`/`opt` semantics
- **IE ID / procedure code constants** — all TS 38.413 §9.1-9.2 constants in `macros` module

## Quick start

```toml
[dependencies]
oxirush-ngap = "0.1"
```

### Encode an NGAP PDU (with macros)

The auto-generated types are deeply nested. The `build_ngap!` macro provides a concise DSL:

```rust
use oxirush_ngap::{build_ngap, build_ngap_ie, ngap::*, macros::*};
use asn1_codecs::{aper::AperCodec, PerCodecData};

// Build a complete NGAP PDU in one expression
let pdu = build_ngap!(InitiatingMessage, Id_UEContextReleaseRequest,
    PROC_UE_CONTEXT_RELEASE_REQUEST, REJECT, UEContextReleaseRequest,
    REJECT IE_AMF_UE_NGAP_ID => Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID(1)),
    REJECT IE_RAN_UE_NGAP_ID => Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID(0)),
    IGNORE IE_CAUSE => Id_Cause(
        Cause::RadioNetwork(CauseRadioNetwork(CauseRadioNetwork::USER_INACTIVITY))
    ),
);

// Encode to APER wire format
let mut output = PerCodecData::new_aper();
pdu.aper_encode(&mut output).unwrap();
let wire_bytes = output.into_bytes();
```

`build_ngap!` arguments: `(Direction, OuterVariant, ProcedureCode, Criticality, MessageType, IEs...)`

Each IE: `Criticality IE_ID_CONSTANT => VariantName(value)`

Build individual IEs when you need conditional logic:

```rust
use oxirush_ngap::{build_ngap_ie, ngap::*, macros::*};

let cause_ie = build_ngap_ie!(UEContextReleaseRequest, IGNORE IE_CAUSE =>
    Id_Cause(Cause::RadioNetwork(CauseRadioNetwork(CauseRadioNetwork::USER_INACTIVITY)))
);
```

### Decode and extract IEs (with macros)

The `extract_ngap_ies!` macro pulls typed fields from a decoded message. Required fields that are missing cause the enclosing function to return `Err(MissingIeError)`. Optional fields stay as `Option<T>`.

```rust
use oxirush_ngap::{extract_ngap_ies, ngap::*, macros::MissingIeError};
use asn1_codecs::{aper::AperCodec, PerCodecData};

fn handle(aper_bytes: &[u8]) -> Result<(), MissingIeError> {
    let mut codec = PerCodecData::from_slice_aper(aper_bytes);
    let pdu = NGAP_PDU::aper_decode(&mut codec).unwrap();

    if let NGAP_PDU::InitiatingMessage(msg) = pdu {
        if let InitiatingMessageValue::Id_UEContextReleaseRequest(req) = msg.value {
            extract_ngap_ies!(&req.protocol_i_es.0, UEContextReleaseRequestProtocolIEs_EntryValue,
                req amf_id: u64     = Id_AMF_UE_NGAP_ID(id),           // required
                req ran_id: u32     = Id_RAN_UE_NGAP_ID(id),           // required
                opt cause:  String  = Id_Cause(c) => format!("{c:?}"), // optional + custom expr
            );
            // amf_id: u64, ran_id: u32, cause: Option<String>
            println!("AMF={amf_id} RAN={ran_id} cause={cause:?}");
        }
    }
    Ok(())
}
```

`extract_ngap_ies!` arguments: `(&ies_slice, EntryValueEnum, fields...)`

Each field: `req|opt name: Type = Variant(binding)` with optional `=> custom_expr`

When `=> expr` is omitted, defaults to `binding.0` (newtype unwrap).

### Decode an NGAP PDU (without macros)

```rust
use oxirush_ngap::ngap::*;
use asn1_codecs::{aper::AperCodec, PerCodecData};

let aper_bytes: &[u8] = &[/* ... APER-encoded NGAP PDU ... */];
let mut codec_data = PerCodecData::from_slice_aper(aper_bytes);
let pdu = NGAP_PDU::aper_decode(&mut codec_data).unwrap();

match pdu {
    NGAP_PDU::InitiatingMessage(msg) => {
        println!("Initiating: procedure_code={}", msg.procedure_code.0);
    }
    NGAP_PDU::SuccessfulOutcome(msg) => {
        println!("Success: procedure_code={}", msg.procedure_code.0);
    }
    NGAP_PDU::UnsuccessfulOutcome(msg) => {
        println!("Failure: procedure_code={}", msg.procedure_code.0);
    }
}
```

### Encode an NGAP PDU (without macros)

```rust
use oxirush_ngap::ngap::*;
use asn1_codecs::{aper::AperCodec, PerCodecData};

let pdu = NGAP_PDU::SuccessfulOutcome(SuccessfulOutcome {
    procedure_code: ProcedureCode(14),
    criticality: Criticality(Criticality::REJECT),
    value: SuccessfulOutcomeValue::Id_InitialContextSetup(
        InitialContextSetupResponse {
            protocol_i_es: InitialContextSetupResponseProtocolIEs(vec![
                InitialContextSetupResponseProtocolIEs_Entry {
                    id: ProtocolIE_ID(10),
                    criticality: Criticality(Criticality::IGNORE),
                    value: InitialContextSetupResponseProtocolIEs_EntryValue
                        ::Id_AMF_UE_NGAP_ID(AMF_UE_NGAP_ID(1)),
                },
                InitialContextSetupResponseProtocolIEs_Entry {
                    id: ProtocolIE_ID(85),
                    criticality: Criticality(Criticality::IGNORE),
                    value: InitialContextSetupResponseProtocolIEs_EntryValue
                        ::Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID(0)),
                },
            ]),
        },
    ),
});

let mut output = PerCodecData::new_aper();
pdu.aper_encode(&mut output).unwrap();
let wire_bytes = output.into_bytes();
```

## How code generation works

The build script (`build/main.rs`) runs at `cargo build` time:

1. Reads the 3GPP ASN.1 source files from `ngap/`:
   - `NGAP-PDU-Descriptions.asn` — top-level PDU definitions
   - `NGAP-PDU-Contents.asn` — procedure message contents
   - `NGAP-IEs.asn` — information element definitions
   - `NGAP-CommonDataTypes.asn` — shared types
   - `NGAP-Constants.asn` — protocol constants
   - `NGAP-Containers.asn` — generic container types
2. Compiles ASN.1 to Rust using [`asn1-compiler`](https://crates.io/crates/asn1-compiler)
3. Outputs `src/ngap.rs` — the complete APER codec (~21K lines)

**Do not edit `src/ngap.rs` manually.** Modify the ASN.1 files in `ngap/` or the build script in `build/` instead.

## Key types

| Type | Description |
|------|-------------|
| `NGAP_PDU` | Top-level enum: `InitiatingMessage`, `SuccessfulOutcome`, `UnsuccessfulOutcome` |
| `InitiatingMessage` | Procedure code + criticality + value (e.g., `NGSetupRequest`, `InitialUEMessage`) |
| `SuccessfulOutcome` | Response to initiating message (e.g., `NGSetupResponse`) |
| `PLMNIdentity` | 3-byte TBCD-encoded PLMN (MCC + MNC) |
| `NAS_PDU` | Opaque NAS payload (decode with `oxirush-nas`) |
| `GNB_ID` | gNodeB identifier (22-32 bits) |
| `AMF_UE_NGAP_ID` / `RAN_UE_NGAP_ID` | UE context identifiers |
| `MissingIeError` | Error returned by `extract_ngap_ies!` when a required IE is absent |

## Examples

```bash
cargo run --example decode_ngsetup   # Decode an NGSetupResponse from APER bytes
cargo run --example build_pdu        # Build NGAP PDUs using build_ngap! and build_ngap_ie!
cargo run --example extract_ies      # Extract IEs from a decoded PDU using extract_ngap_ies!
```

## 3GPP references

- **TS 38.413** — NGAP specification (procedures, messages, IEs)
- **ITU-T X.691** — ASN.1 Packed Encoding Rules (PER)

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

All commits must be signed off (`git commit -s`) per the Developer Certificate of Origin.

### Developer Certificate of Origin (DCO)

By contributing to this project, you agree to the [Developer Certificate of Origin (DCO)](https://developercertificate.org/). This means that you have the right to submit your contributions and you agree to license them according to the project's license.

All commits should be signed-off with `git commit -s` to indicate your agreement to the DCO.

## License

Copyright 2026 Valentin D'Emmanuele

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
