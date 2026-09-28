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
//! All protocol types in [`ngap`] are generated from the official 3GPP ASN.1
//! modules using [`rasn-compiler`](https://crates.io/crates/rasn-compiler) and
//! checked into the crate as Rust source.
//! Encoding and decoding use [`rasn`](https://crates.io/crates/rasn)'s Aligned
//! Packed Encoding Rules implementation.
//!
//! ```ignore
//! use oxirush_ngap::{build_ngap, ngap::*};
//!
//! let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
//!     IGNORE, UEContextReleaseRequest,
//!     REJECT AMF_UE_NGAP_ID(1u64),
//!     REJECT RAN_UE_NGAP_ID(7u32),
//!     IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
//! );
//! let bytes = pdu.encode().unwrap();
//! let decoded = NGAP_PDU::decode(&bytes).unwrap();
//! assert_eq!(decoded.procedure_name(), "UEContextReleaseRequest");
//! ```

pub mod helpers;
pub mod macros;
pub mod ngap;
pub mod sized;

#[doc(hidden)]
pub use paste as __paste;
#[doc(hidden)]
pub use rasn as __rasn;

pub use ngap::NgapPduKind;

/// Version of oxirush-ngap.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
