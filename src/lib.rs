/*
   OxiRush
   Copyright 2025 Valentin D'Emmanuele

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! # oxirush-ngap
//!
//! A complete NGAP (NG Application Protocol) codec for 5G, auto-generated from 3GPP ASN.1 definitions (TS 38.413) using Aligned PER (APER) encoding.
//!
//! Part of the [OxiRush](https://github.com/linouxis9/oxirush) project — a 5G Core Network testing framework.
//!
//! ## Features
//!
//! - **Full TS 38.413 coverage** — every NGAP procedure, IE, and PDU type
//! - **Auto-generated from ASN.1** — the codec is rebuilt at `cargo build` from the canonical 3GPP ASN.1 files, tracking spec updates automatically
//! - **APER encoding/decoding** — standards-compliant Aligned PER via [`asn1-codecs`](https://crates.io/crates/asn1-codecs)
//! - **Serde support** — all generated types derive `Serialize`/`Deserialize`
//! - **Round-trip fidelity** — encode then decode produces identical structures
//! - **Builder macros** — `build_ngap!` and `build_ngap_ie!` eliminate the deeply nested ProtocolIEs boilerplate
//! - **Extraction macro** — `extract_ngap_ies!` pulls typed fields from decoded messages with `req`/`opt` semantics
//! - **Auto-derived IE IDs and procedure codes** — no manual constants needed, derived at build time from ASN.1 `#[asn(key = N)]` attributes
//! - **Encode/decode convenience** — `pdu.encode()` and `NGAP_PDU::decode(&bytes)` wrap the raw APER codec
//! - **PDU inspection** — `pdu.procedure_name()`, `pdu.direction()`, `pdu.procedure_code()`, `pdu.is_initiating()`
//! - **Display impl** — `format!("{pdu}")` → `"InitiatingMessage NGSetup (code=21)"`
//! - **Type builders** — `plmn()`, `guami()`, `tai()`, `nr_cgi()`, `global_gnb_id()`, `s_nssai()` for common NGAP IEs
//! - **Bitvec helpers** — `int_to_bitvec()`, `bytes_to_bitvec()` for AMF identity, security keys, cell IDs
//!
//! ## Quick start
//!
//! ```toml
//! [dependencies]
//! oxirush-ngap = "0.3"
//! ```
//!
//! ### Encode an NGAP PDU (with macros)
//!
//! The auto-generated types are deeply nested. The `build_ngap!` macro provides a concise DSL:
//!
//! ```ignore
//! use oxirush_ngap::{build_ngap, ngap::*};
//! use oxirush_ngap::helpers::*;
//!
//! // Simple: UEContextReleaseRequest with a cause
//! let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
//!     REJECT, UEContextReleaseRequest,
//!     REJECT AMF_UE_NGAP_ID(1u64),
//!     REJECT RAN_UE_NGAP_ID(0u32),
//!     IGNORE Cause(Cause::RadioNetwork(CauseRadioNetwork(CauseRadioNetwork::USER_INACTIVITY))),
//! );
//!
//! // Complex: InitialContextSetupRequest with GUAMI, NSSAI, security key helpers
//! let pdu = build_ngap!(InitiatingMessage, InitialContextSetup,
//!     REJECT, InitialContextSetupRequest,
//!     REJECT AMF_UE_NGAP_ID(1u64),
//!     REJECT RAN_UE_NGAP_ID(0u32),
//!     REJECT UEAggregateMaximumBitRate(UEAggregateMaximumBitRate {
//!         ue_aggregate_maximum_bit_rate_dl: BitRate(1_000_000_000),
//!         ue_aggregate_maximum_bit_rate_ul: BitRate(1_000_000_000),
//!         ie_extensions: None,
//!     }),
//!     REJECT GUAMI(guami(plmn("208", "93"), 1, 1, 0)),
//!     REJECT AllowedNSSAI(vec![AllowedNSSAI_Item {
//!         s_nssai: s_nssai(1, Some([0x00, 0x00, 0x01])),
//!         ie_extensions: None,
//!     }]),
//!     REJECT SecurityKey(bytes_to_bitvec(&[0u8; 32])),
//!     IGNORE NAS_PDU(vec![0x7e, 0x00, 0x42]),
//! );
//!
//! // Encode to APER wire format
//! let wire_bytes = pdu.encode().unwrap();
//!
//! // Inspect the PDU
//! println!("{pdu}");                        // "InitiatingMessage InitialContextSetup (code=14)"
//! assert_eq!(pdu.procedure_name(), "InitialContextSetup");
//! assert!(pdu.is_initiating());
//! ```
//!
//! `build_ngap!` arguments: `(Direction, Procedure, Criticality, MessageType, IEs...)`
//!
//! Each IE: `Criticality IeName(value)` — IE IDs and procedure codes are auto-derived at build time.
//! Raw values (e.g. `u64`) auto-convert to newtypes via `.into()`.
//!
//! Build individual IEs when you need conditional logic:
//!
//! ```ignore
//! use oxirush_ngap::{build_ngap_ie, ngap::*};
//!
//! let cause_ie = build_ngap_ie!(UEContextReleaseRequest, IGNORE
//!     Cause(Cause::RadioNetwork(CauseRadioNetwork(CauseRadioNetwork::USER_INACTIVITY)))
//! );
//! ```
//!
//! ### Decode and extract IEs (with macros)
//!
//! The `extract_ngap_ies!` macro pulls typed fields from a decoded message. Required fields that are missing cause the enclosing function to return `Err(MissingIeError)`. Optional fields stay as `Option<T>`.
//!
//! ```ignore
//! use oxirush_ngap::{extract_ngap_ies, ngap::*, macros::MissingIeError};
//!
//! // Simple: extract UE IDs and an optional cause
//! fn handle_release(req: UEContextReleaseRequest) -> Result<(), MissingIeError> {
//!     extract_ngap_ies!(req, UEContextReleaseRequest,
//!         req amf_id: u64     = AMF_UE_NGAP_ID(id),           // required, default .0
//!         req ran_id: u32     = RAN_UE_NGAP_ID(id),           // required, default .0
//!         opt cause:  String  = Cause(c) => format!("{c:?}"), // optional + custom expr
//!     );
//!     // amf_id: u64, ran_id: u32, cause: Option<String>
//!     println!("AMF={amf_id} RAN={ran_id} cause={cause:?}");
//!     Ok(())
//! }
//!
//! // Complex: extract many IEs with pattern matching and type conversions
//! fn handle_handover(req: HandoverRequired) -> Result<(), MissingIeError> {
//!     extract_ngap_ies!(req, HandoverRequired,
//!         req amf_id: u64 = AMF_UE_NGAP_ID(id),
//!         req ran_id: u32 = RAN_UE_NGAP_ID(id),
//!         opt cause_rn: u8 = Cause(c) =>
//!             if let Cause::RadioNetwork(rn) = c { rn.0 } else { 0 },
//!         opt ho_type: u8 = HandoverType(ht),           // default .0
//!         opt container: Vec<u8> =
//!             SourceToTarget_TransparentContainer(c) => c.0.clone(),
//!     );
//!     println!("HO UE {} type {:?} cause {:?}", amf_id, ho_type, cause_rn);
//!     Ok(())
//! }
//! ```
//!
//! `extract_ngap_ies!` arguments: `(msg_var, MessageType, fields...)`
//!
//! Each field: `req|opt name: Type = IeName(binding)` with optional `=> custom_expr`
//!
//! When `=> expr` is omitted, defaults to `binding.0` (newtype unwrap).
//!
//! ### Encode / Decode
//!
//! ```ignore
//! use oxirush_ngap::ngap::NGAP_PDU;
//!
//! // Encode NGAP_PDU to APER bytes
//! let bytes = pdu.encode().unwrap();
//!
//! // Decode APER bytes to NGAP_PDU
//! let decoded = NGAP_PDU::decode(&bytes).unwrap();
//! ```
//!
//! ### Type builders (helpers module)
//!
//! ```ignore
//! use oxirush_ngap::helpers::*;
//!
//! let p = plmn("208", "93");                          // PLMNIdentity (3-byte TBCD)
//! let g = guami(plmn("208", "93"), 1, 1, 0);          // GUAMI (PLMN + AMF identity)
//! let t = tai(plmn("208", "93"), &[0x00, 0x00, 0x01]); // TAI (PLMN + TAC)
//! let cgi = nr_cgi(plmn("208", "93"), 0x000001, 1);    // NR-CGI (36-bit cell ID)
//! let gnb = global_gnb_id(plmn("208", "93"), 0x000001); // GlobalGNB-ID (24-bit gNB-ID)
//! let nssai = s_nssai(1, Some([0x00, 0x00, 0x01]));    // S-NSSAI (SST + optional SD)
//! let sec = ue_security_capabilities(&[0xE0, 0xE0]);   // UESecurityCapabilities
//!
//! // Bitvec conversion for NGAP bitstring fields
//! let key = bytes_to_bitvec(&[0u8; 32]);               // SecurityKey (256 bits)
//! let region = int_to_bitvec(1, 8);                     // AMFRegionID (8 bits)
//! ```
//!
//! ### Without macros
//!
//! The auto-generated types are fully usable without macros — you can construct `NGAP_PDU` values directly and pattern-match on decoded ones. The macros simply eliminate the repetitive `ProtocolIE_ID(...)`, `Criticality(...)`, and `{Msg}ProtocolIEs_EntryValue::Id_...` boilerplate. See `examples/decode_manually.rs` for a complete encode -> decode -> inspect example without macros.
//!
//! ## How code generation works
//!
//! The build script (`build/main.rs`) runs at `cargo build` time:
//!
//! 1. **ASN.1 compilation** — reads the 3GPP ASN.1 source files from `ngap/` (`NGAP-PDU-Descriptions.asn`, `NGAP-PDU-Contents.asn`, `NGAP-IEs.asn`, `NGAP-CommonDataTypes.asn`, `NGAP-Constants.asn`, `NGAP-Containers.asn`) and compiles them to Rust using [`asn1-compiler`](https://crates.io/crates/asn1-compiler)
//! 2. **Post-processing** — the build script then parses the generated code to emit:
//!    - `From<InnerType>` impls for all single-field newtypes (enables `.into()` auto-conversion in `build_ngap!`)
//!    - `__ngap_ie_id!` macro — maps IE variant names to numeric IDs by parsing `#[asn(key = N)]` attributes on `ProtocolIEs_EntryValue` enums (207 arms)
//!    - `__ngap_proc_code!` macro — maps procedure names to codes by parsing `{Direction}Value` enums (66 arms)
//!    - `impl NGAP_PDU` — `procedure_code()`, `direction()`, `procedure_name()`, `is_initiating()` / `is_successful()` / `is_unsuccessful()`
//!    - `impl Display for NGAP_PDU` — human-readable PDU formatting
//! 3. **Output** — `src/ngap.rs`, the complete APER codec with helper macros and impls (~21K lines)
//!
//! **Do not edit `src/ngap.rs` manually.** Modify the ASN.1 files in `ngap/` or the build script in `build/` instead.
//!
//! ## Key types
//!
//! | Type | Description |
//! |------|-------------|
//! | `NGAP_PDU` | Top-level enum: `InitiatingMessage`, `SuccessfulOutcome`, `UnsuccessfulOutcome` |
//! | `InitiatingMessage` | Procedure code + criticality + value (e.g., `NGSetupRequest`, `InitialUEMessage`) |
//! | `SuccessfulOutcome` | Response to initiating message (e.g., `NGSetupResponse`) |
//! | `UnsuccessfulOutcome` | Failure response (e.g., `NGSetupFailure`, `HandoverPreparationFailure`) |
//! | `AMF_UE_NGAP_ID` / `RAN_UE_NGAP_ID` | UE context identifiers (u64 / u32 newtypes) |
//! | `Cause` | Enum: `RadioNetwork`, `Transport`, `NAS`, `Protocol`, `Misc` sub-causes |
//! | `PLMNIdentity` | 3-byte TBCD-encoded PLMN (MCC + MNC) |
//! | `S_NSSAI` | Network slice: SST (1 byte) + optional SD (3 bytes) |
//! | `TAI` | Tracking Area Identity: PLMN + TAC |
//! | `NAS_PDU` | Opaque NAS payload (decode with [oxirush-nas](https://github.com/linouxis9/oxirush-nas)) |
//! | `GNB_ID` | gNodeB identifier (22-32 bits) |
//! | `Criticality` | IE criticality: `REJECT`, `IGNORE`, or `NOTIFY` |
//! | `MissingIeError` | Error returned by `extract_ngap_ies!` when a required IE is absent |
//!
//! ## Macro reference
//!
//! | Macro | Purpose |
//! |-------|---------|
//! | `build_ngap!(Dir, Proc, Crit, Msg, IEs...)` | Build a complete `NGAP_PDU` |
//! | `build_ngap_ie!(Msg, Crit IE(val))` | Build a single `ProtocolIEs_Entry` |
//! | `extract_ngap_ies!(var, Msg, fields...)` | Extract typed fields from a decoded message |
//!
//! `build_ngap!` and `build_ngap_ie!` auto-derive IE IDs, procedure codes, and `Id_` variant names. Raw values auto-convert to newtypes via `.into()`. `extract_ngap_ies!` accesses `.protocol_i_es.0` internally — pass the message variable directly.
//!
//! ## Examples
//!
//! ```bash
//! cargo run --example build_pdu         # Build NGAP PDUs using build_ngap! and build_ngap_ie!
//! cargo run --example extract_ies       # Extract IEs from a decoded PDU using extract_ngap_ies!
//! cargo run --example decode_manually   # Encode/decode without macros (raw types)
//! ```
//!
//! ## 3GPP references
//!
//! - **TS 38.413** — NGAP specification (procedures, messages, IEs)
//! - **ITU-T X.691** — ASN.1 Packed Encoding Rules (PER)

pub mod helpers;
pub mod macros;
pub mod ngap;

/// Version of oxirush-ngap
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
