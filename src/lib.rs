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

//! NGAP (NG Application Protocol) APER codec for 5G, per 3GPP TS 38.413.
//!
//! All types in the [`ngap`] module are **auto-generated at build time** from the
//! ASN.1 definitions in `ngap/` using [`asn1-compiler`](https://crates.io/crates/asn1-compiler).
//! Do not edit `ngap.rs` manually — modify the ASN.1 source files or the build script
//! in `build/` instead.
//!
//! # Usage
//!
//! ```ignore
//! use oxirush_ngap::ngap::*;
//! use asn1_codecs::{aper::AperCodec, PerCodecData};
//!
//! // Decode an NGAP PDU from APER bytes
//! let mut codec_data = PerCodecData::from_slice_aper(&bytes);
//! let pdu = NGAP_PDU::aper_decode(&mut codec_data).unwrap();
//!
//! // Encode back to APER
//! let mut output = PerCodecData::new_aper();
//! pdu.aper_encode(&mut output).unwrap();
//! ```

pub mod macros;
pub mod ngap;

/// Version of oxirush-ngap
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
