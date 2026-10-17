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

//! A codec for NGAP, the NG Application Protocol of 3GPP TS 38.413, in aligned PER.
//!
//! ```
//! use oxirush_ngap::{build_ngap, ngap::*};
//!
//! let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
//!     IGNORE, UEContextReleaseRequest,
//!     REJECT AMF_UE_NGAP_ID(1u64),
//!     REJECT RAN_UE_NGAP_ID(7u32),
//!     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
//! )?;
//! let bytes = pdu.encode()?;
//! let decoded = NGAP_PDU::decode(&bytes)?;
//! assert_eq!(decoded.procedure_name(), "UEContextReleaseRequest");
//! # Ok::<(), oxirush_ngap::Error>(())
//! ```
//!
//! The types of [`ngap`] are generated from the ASN.1 modules of the specification and
//! checked in: `rasn-compiler` resolves the modules, and the generator of the crate
//! writes each type with its codec on [`oxirush-aper`](https://github.com/linouxis9/oxirush-aper).
//! The README has the examples, task by task.
//!
//! Every function and macro that can fail returns an [`Error`]: its [`ErrorKind`] is
//! what a caller matches on, and its `Display` says what is wrong.
//!
//! The codec builds, encodes and decodes a message whatever its IEs are. `check()` of
//! a PDU reports how it deviates from what the ASN.1 assigns its message, each as a
//! [`Deviation`]: a criticality of the PDU or of an IE that is not the assigned one, an
//! IE that the message cannot have, has twice or has out of order, a mandatory IE that
//! is missing, an IE whose value does not decode or is of a later version of its type,
//! an encoding in another form than the canonical one, and a procedure that the
//! specification does not have. `criticality_diagnostics()` of the PDU gathers them
//! into the Criticality Diagnostics IE of the answer.
//!
//! # Values that borrow, and values that are kept
//!
//! A value that is decoded borrows the octets that it holds from what it was decoded
//! from: the NAS-PDU of a message is a slice of the PDU, and nothing is copied. A type
//! that holds octets has a lifetime for that, as `InitialUEMessage<'a>` and
//! `NASPDU<'a>`. A type that holds none, as `Cause`, `TAI`, `NRCGI` or `GUAMI`, has no
//! lifetime. The extension container of a value owns its octets, as a peer seldom sends
//! one, so that it gives the value no lifetime.
//!
//! | To | Write |
//! | --- | --- |
//! | Read a message where it was received | `let pdu = NGAP_PDU::decode(&octets)?;` then `pdu.decode_value()?`: the message borrows from `pdu`, which is a variable of its own |
//! | Keep a value after the PDU is gone | `value.into_owned()`, which gives the same type with the lifetime `'static`, as `NASPDU<'static>`: this is the type to write in a struct |
//! | Build a value | From data that it owns: `NASPDU::from(vec)` is a `NASPDU<'static>` |
//! | Read a PDU in the buffer that it was received in | [`NGAP_PDU::decode_borrowed`](ngap::NGAPPDU::decode_borrowed), which copies nothing: the PDU borrows from the buffer |
//!
//! [`NGAP_PDU`](ngap::NGAP_PDU) is the PDU that owns its octets, `NGAPPDU<'static>`,
//! and has no lifetime to write.
//!
//! ```
//! use oxirush_ngap::{Error, build_ngap, extract_ngap_ies, ngap::*};
//!
//! /// What a node keeps of a UE after its first message.
//! struct Ue {
//!     ran_id: u32,
//!     // The context owns these octets: `'static`, and no lifetime on `Ue`
//!     nas: NASPDU<'static>,
//! }
//!
//! fn admit(pdu: &NGAP_PDU) -> Result<Ue, Error> {
//!     // `message` borrows from `pdu`, and `nas` from `message`
//!     let message: InitialUEMessage = pdu.decode_value()?;
//!     extract_ngap_ies!(message, InitialUEMessage,
//!         req ran_id: u32 = RAN_UE_NGAP_ID(id),
//!         req nas: NASPDU = NAS_PDU(nas) => nas,
//!     );
//!     // Kept beyond the PDU: the octets are copied here, once
//!     Ok(Ue { ran_id, nas: nas.into_owned() })
//! }
//!
//! let pdu = build_ngap!(InitiatingMessage, InitialUEMessage, IGNORE, InitialUEMessage,
//!     REJECT RAN_UE_NGAP_ID(7u32),
//!     REJECT NAS_PDU(vec![0x7e, 0x00, 0x41]),
//! )?;
//! let ue = admit(&pdu)?;
//! drop(pdu);
//! assert_eq!((ue.ran_id, &ue.nas.0[..]), (7, &[0x7e, 0x00, 0x41][..]));
//! # Ok::<(), Error>(())
//! ```
//!
//! A message may be held across an `.await`. The future of this handler owns the
//! buffer, the PDU borrows from the buffer, the message from the PDU and the NAS-PDU
//! from the message. The future is `Send`, as every value of the bindings is.
//!
//! ```
//! use oxirush_ngap::{Error, build_ngap, extract_ngap_ies, ngap::*};
//!
//! async fn deliver(nas: &[u8]) {
//!     assert_eq!(nas, [0x7e, 0x00, 0x5d]);
//! }
//!
//! async fn handle(received: Vec<u8>) -> Result<(), Error> {
//!     let pdu = NGAP_PDU::decode_borrowed(&received)?;
//!     let message: UplinkNASTransport = pdu.decode_value()?;
//!     extract_ngap_ies!(message, UplinkNASTransport,
//!         req nas: NASPDU = NAS_PDU(nas) => nas,
//!     );
//!     // `nas` is a slice of `received`, here and after the await
//!     deliver(&nas.0).await;
//!     assert!(nas.0.is_borrowed());
//!     Ok(())
//! }
//!
//! let transport = build_ngap!(InitiatingMessage, UplinkNASTransport, IGNORE, UplinkNASTransport,
//!     REJECT AMF_UE_NGAP_ID(1u64),
//!     REJECT RAN_UE_NGAP_ID(7u32),
//!     REJECT NAS_PDU(vec![0x7e, 0x00, 0x5d]),
//! )?;
//! let handler = handle(transport.encode()?);
//!
//! // A runtime may move the handler to another thread
//! fn is_send(_: &impl Send) {}
//! is_send(&handler);
//! # let mut handler = std::pin::pin!(handler);
//! # let mut context = std::task::Context::from_waker(std::task::Waker::noop());
//! # assert!(handler.as_mut().poll(&mut context).is_ready());
//! # Ok::<(), Error>(())
//! ```
//!
//! # The names of the fields
//!
//! A field has the words of the identifier that ASN.1 gives its member, in lower case
//! with `_` between them, as a reader of the specification says them: `pLMNIdentity` is
//! `plmn_identity`, `tAC` is `tac`, `iE-Extensions` is `ie_extensions`, `nRCellIdentity`
//! is `nr_cell_identity`, `gNB-ID` is `gnb_id`, `aMFRegionID` is `amf_region_id`.
//!
//! 1. A hyphen ends a word, and so does a lower-case letter or a digit before a
//!    capital: `procedureCode` is `procedure_code`.
//! 2. The first letter of an identifier belongs to the capitals after it: the
//!    identifier starts in lower case whatever its first word is.
//! 3. Capitals in a row are an abbreviation, and the last of them starts a word when a
//!    lower-case letter follows. When the capitals without it are no abbreviation of
//!    the list and the capitals with it are one, they end the abbreviation:
//!    `pLMNidentity` is `plmn_identity` too. An `s` after them is their plural:
//!    `protocolIEs` is `protocol_ies`.
//! 4. Capitals that are several abbreviations of the list are these:
//!    `uERLFReportContainer` is `ue_rlf_report_container`.
//! 5. An abbreviation that the specifications write with a lower-case letter is one
//!    word wherever it is: `a2X`, `AIoT`, `IoT`, `QoS`, `QoE`, `ProSe`, `eNB`, `gNB`,
//!    `IPsec`, `IPv4`, `IPv6`, `NRPPa`, `LPPa`, `n3IWF`, `eDRX`, `eCGI`.
//!    `globalgNB-ID` is `global_gnb_id`.
//! 6. `fiveG` and `fiveGS` are `five_g` and `five_gs`, and `5GS` is one word.
//!
//! The list of the abbreviations is `ABBREVIATIONS` of `generator/src/names.rs` in
//! oxirush-aper, with the identifiers that the specifications write otherwise than
//! their others, as `parentTImeSource`. `build/fields.tsv` of the repository has each
//! field of each type with the name that rasn-compiler gives it, which the fields had
//! before this rule and have in 0.4.2 for every field of a type that 0.4.2 has but two,
//! the `targetRANNodeID-RIM` of `RIMInformationTransfer` and the `targetRANNodeID-SON`
//! of `SONConfigurationTransfer`. The JSON forms keep the identifiers of ASN.1.
//!
//! A `SEQUENCE` is a struct that a literal builds, and its `new` takes the members in
//! their order. An extensible `CHOICE` or `ENUMERATED` is `#[non_exhaustive]`: a later
//! version of the specification adds alternatives and values to it, which a `match`
//! leaves room for with a `_` arm. A member that a later version adds to a `SEQUENCE`
//! changes its struct and its `new`, and comes with a breaking release of the crate.
//!
//! # The types of the values
//!
//! | ASN.1 | Rust |
//! | --- | --- |
//! | `INTEGER` with a constraint | The native number that holds its range: `RANUENGAPID(pub u32)` for `(0..4294967295)` |
//! | `INTEGER` whose constraint has an extension marker | `u64`: `BitRate(pub u64)` for `(0..4000000000000, ...)` |
//! | `OCTET STRING (SIZE (n))` | `[u8; n]`: `PLMNIdentity(pub [u8; 3])` |
//! | Another `OCTET STRING`, and an open type | [`Octets`]: `NASPDU<'a>(pub Octets<'a>)` |
//! | `BIT STRING` of 64 bits at most, without an extension marker | [`Bits`], a number with a length, which is `Copy`. A type of one size is made of the number: `AMFSetID::new(1)?` |
//! | Another `BIT STRING` | [`BitString`], which borrows its octets when it is longer than 64 bits |
//! | `PrintableString`, `VisibleString`, `UTF8String` | `Cow<'a, str>`: `AMFName<'a>` |
//! | `ENUMERATED`, `CHOICE`, `SEQUENCE`, `SEQUENCE OF` | An enum, an enum, a struct, and a `Vec` in a struct |
//!
//! A bit string is a [`Bits`] when its type says that 64 bits hold it. A string that
//! may be longer, and one whose size has an extension marker, which a later release
//! may send longer, is a [`BitString`]:
//!
//! | [`Bits`] | [`BitString`] |
//! | --- | --- |
//! | `AMFRegionID` (8 bits), `AMFSetID` (10), `AMFPointer` (6) | `TransportLayerAddress` (1 to 160, ...) |
//! | `GNBID::gNB_ID` (22 to 32), `NRCellIdentity` (36), `EUTRACellIdentity` (28) | `SecurityKey` (256) |
//! | `MaskedIMEISV` (64), `CAGID` (32), `NID` (44) | `NRencryptionAlgorithms`, `NRintegrityProtectionAlgorithms`, `EUTRAencryptionAlgorithms` and `EUTRAintegrityProtectionAlgorithms` (16, ...) |
//! | `MessageIdentifier`, `SerialNumber`, `CRNTI` (16), `InterfacesToTrace` (8) | `RATRestrictionInformation` (8, ...), `HashedUEIdentityIndexValue` (13, ...), `UpdateFeedback` (8, ...) |
//!
//! The bindings hold an `INTEGER` in 64 bits. A number that an extension marker lets
//! a later release send beyond that does not decode, with an error of the kind
//! `Decode` that says `the number does not fit in 64 bits`. The PDU still decodes:
//! its IEs are octets until one is read, and the JSON forms of the `inspect` feature
//! show an IE that does not decode as the octets received, and write them back.

#![forbid(unsafe_code)]

extern crate alloc;

mod check;
mod error;
pub mod helpers;
#[cfg(oxirush_ngap_hooks)]
#[doc(hidden)]
pub mod hooks;
#[cfg(feature = "inspect")]
pub mod inspect;
#[cfg(feature = "inspect")]
mod inspect_paths;
#[cfg(feature = "inspect")]
mod inspect_registry;
pub mod macros;
mod message;
// The bindings are formatted where they are generated, by the rustfmt of the toolchain
// that the generator runs with: a later one breaks some of their long lines otherwise.
#[rustfmt::skip]
pub mod ngap;
mod pdu;
#[doc(hidden)]
pub mod registry;

/// The aligned PER runtime that the bindings are made of, at the version that this
/// crate is built with. The values of the bindings are its types, such as
/// [`Octets`], [`Bits`] and [`BitString`],
/// and the error of the crate is its error: a crate that names them takes them from
/// here, and needs no dependency on `oxirush-aper` of its own.
pub use oxirush_aper as aper;
/// The identities of 3GPP networks as values, of the `oxirush-utils` that this crate
/// is built with: a `PLMNIdentity` converts to and from its `Plmn`, which has the MCC and
/// the MNC.
pub use oxirush_utils::identity;
/// The trees of `oxirush-utils`, of the version that this crate is built with: a PDU and
/// each value of the bindings as JSON whose members keep the order of the specification,
/// [`tree::Value`], and `tree!`, which writes one in the code as `serde_json::json!` does.
/// A crate that reads or writes them takes them from here.
#[cfg(feature = "inspect")]
pub use oxirush_utils::tree;

pub use check::{Deviation, Ngap};
pub use message::Message;
pub use ngap::NgapPduKind;
pub use oxirush_aper::{
    AsnType, BitString, Bits, Decode, Encode, Error, ErrorKind, Fault, Faults, Octets,
};

/// The examples of the README, which the documentation tests compile and run.
#[cfg(all(doctest, feature = "serde"))]
#[doc = include_str!("../README.md")]
struct Readme;

/// Version of oxirush-ngap.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
