//! Convenience helpers for building NGAP types.
//!
//! These utilities eliminate repetitive bitvec construction and NGAP type
//! assembly that every NGAP consumer needs.
//!
//! # Bitvec helpers
//!
//! NGAP uses `BitVec<u8, Msb0>` for many fields (AMF identity components,
//! security keys, gNB-ID, NR cell identity). These helpers convert common
//! Rust types to the required bitvec format:
//!
//! ```ignore
//! use oxirush_ngap::helpers::{int_to_bitvec, bytes_to_bitvec};
//!
//! let region_id = int_to_bitvec(1, 8);    // AMFRegionID: 8 bits
//! let set_id = int_to_bitvec(1, 10);      // AMFSetID: 10 bits
//! let pointer = int_to_bitvec(0, 6);      // AMFPointer: 6 bits
//! let kgnb = bytes_to_bitvec(&[0u8; 32]); // SecurityKey: 256 bits
//! ```
//!
//! # Type builders
//!
//! High-level constructors for commonly used NGAP IEs:
//!
//! ```ignore
//! use oxirush_ngap::helpers::*;
//!
//! let plmn = plmn("208", "93");
//! let g = guami(plmn.clone(), 1, 1, 0);
//! let t = tai(plmn.clone(), &[0x00, 0x00, 0x01]);
//! let cgi = nr_cgi(plmn.clone(), 0x000001, 1);
//! let gnb = global_gnb_id(plmn.clone(), 0x000001);
//! let nssai = s_nssai(1, Some([0x00, 0x00, 0x01]));
//! ```

use bitvec::prelude::*;

use crate::ngap::*;

// ── Bitvec helpers ──────────────────────────────────────────────────────────

/// Convert an integer to a MSB-first bitvec of the given width.
///
/// Extracts `width` bits from `value`, most-significant bit first.
/// Used for AMF identity fields, gNB-ID, cell identity, etc.
///
/// # Panics
///
/// Panics if `width > 64`.
pub fn int_to_bitvec(value: u64, width: usize) -> BitVec<u8, Msb0> {
    assert!(width <= 64, "width must be <= 64");
    (0..width).rev().map(|i| (value >> i) & 1 == 1).collect()
}

/// Convert a byte slice to a MSB-first bitvec (8 bits per byte).
///
/// Used for security keys (K_gNB = 32 bytes → 256 bits), NH keys, etc.
pub fn bytes_to_bitvec(bytes: &[u8]) -> BitVec<u8, Msb0> {
    bytes
        .iter()
        .flat_map(|byte| (0..8u8).rev().map(move |i| (byte >> i) & 1 == 1))
        .collect()
}

// ── PLMN encoding ───────────────────────────────────────────────────────────

/// Encode MCC/MNC as a `PLMNIdentity` (3 octets, TBCD per TS 24.501 §9.11.3.4).
///
/// MCC must be 3 decimal digits, MNC must be 2 or 3 decimal digits.
///
/// ```ignore
/// use oxirush_ngap::helpers::plmn;
/// let id = plmn("208", "93"); // MCC=208, MNC=93 → [0x02, 0xF8, 0x39]
/// ```
///
/// # Panics
///
/// Panics if MCC is not 3 digits or MNC is not 2-3 digits.
pub fn plmn(mcc: &str, mnc: &str) -> PLMNIdentity {
    assert!(
        mcc.len() == 3 && mcc.bytes().all(|b| b.is_ascii_digit()),
        "MCC must be 3 ASCII digits"
    );
    assert!(
        (mnc.len() == 2 || mnc.len() == 3) && mnc.bytes().all(|b| b.is_ascii_digit()),
        "MNC must be 2 or 3 ASCII digits"
    );
    let mcc_d: Vec<u8> = mcc.bytes().map(|b| b - b'0').collect();
    let mnc_d: Vec<u8> = mnc.bytes().map(|b| b - b'0').collect();

    let mut bytes = [0u8; 3];
    bytes[0] = (mcc_d[1] << 4) | mcc_d[0];
    if mnc_d.len() == 2 {
        bytes[1] = 0xF0 | mcc_d[2];
        bytes[2] = (mnc_d[1] << 4) | mnc_d[0];
    } else {
        bytes[1] = (mnc_d[2] << 4) | mcc_d[2];
        bytes[2] = (mnc_d[1] << 4) | mnc_d[0];
    }
    PLMNIdentity(bytes.to_vec())
}

/// Decode a `PLMNIdentity` back to (MCC, MNC) strings.
///
/// Returns `None` if the identity is shorter than 3 bytes.
pub fn plmn_from(id: &PLMNIdentity) -> Option<(String, String)> {
    let bytes = &id.0;
    if bytes.len() < 3 {
        return None;
    }
    let mcc0 = bytes[0] & 0x0F;
    let mcc1 = (bytes[0] >> 4) & 0x0F;
    let mcc2 = bytes[1] & 0x0F;
    let mnc_hi = (bytes[1] >> 4) & 0x0F;
    let mnc0 = bytes[2] & 0x0F;
    let mnc1 = (bytes[2] >> 4) & 0x0F;

    let mcc = format!("{mcc0}{mcc1}{mcc2}");
    let mnc = if mnc_hi == 0xF {
        format!("{mnc0}{mnc1}")
    } else {
        format!("{mnc0}{mnc1}{mnc_hi}")
    };
    Some((mcc, mnc))
}

// ── NGAP type builders ──────────────────────────────────────────────────────

/// AMF Region ID field width (TS 38.413 §9.3.3.12).
pub const AMF_REGION_ID_BITS: usize = 8;
/// AMF Set ID field width.
pub const AMF_SET_ID_BITS: usize = 10;
/// AMF Pointer field width.
pub const AMF_POINTER_BITS: usize = 6;

/// Build a `GUAMI` (Globally Unique AMF Identifier).
///
/// ```ignore
/// use oxirush_ngap::helpers::*;
/// let g = guami(plmn("208", "93"), 1, 1, 0);
/// ```
pub fn guami(plmn_identity: PLMNIdentity, region_id: u8, set_id: u16, pointer: u8) -> GUAMI {
    GUAMI {
        plmn_identity,
        amf_region_id: AMFRegionID(int_to_bitvec(region_id as u64, AMF_REGION_ID_BITS)),
        amf_set_id: AMFSetID(int_to_bitvec(set_id as u64, AMF_SET_ID_BITS)),
        amf_pointer: AMFPointer(int_to_bitvec(pointer as u64, AMF_POINTER_BITS)),
        ie_extensions: None,
    }
}

/// Build a `TAI` (Tracking Area Identity).
///
/// `tac` should be 3 bytes (24-bit Tracking Area Code).
///
/// ```ignore
/// use oxirush_ngap::helpers::*;
/// let t = tai(plmn("208", "93"), &[0x00, 0x00, 0x01]);
/// ```
pub fn tai(plmn_identity: PLMNIdentity, tac: &[u8]) -> TAI {
    TAI {
        plmn_identity,
        tac: TAC(tac.to_vec()),
        ie_extensions: None,
    }
}

/// Build an `NR_CGI` (NR Cell Global Identity).
///
/// The NR Cell Identity is 36 bits: `gnb_id` (24 bits) | `cell_id` (12 bits).
///
/// ```ignore
/// use oxirush_ngap::helpers::*;
/// let cgi = nr_cgi(plmn("208", "93"), 0x000001, 1);
/// ```
pub fn nr_cgi(plmn_identity: PLMNIdentity, gnb_id: u32, cell_id: u16) -> NR_CGI {
    let nr_cell_value = ((gnb_id & 0x00FF_FFFF) as u64) << 12 | (cell_id & 0x0FFF) as u64;
    NR_CGI {
        plmn_identity,
        nr_cell_identity: NRCellIdentity(int_to_bitvec(nr_cell_value, 36)),
        ie_extensions: None,
    }
}

/// Build a `GlobalGNB_ID` from a PLMN and 24-bit gNB identifier.
///
/// ```ignore
/// use oxirush_ngap::helpers::*;
/// let gnb = global_gnb_id(plmn("208", "93"), 0x000001);
/// ```
pub fn global_gnb_id(plmn_identity: PLMNIdentity, gnb_id: u32) -> GlobalGNB_ID {
    let id_bits = int_to_bitvec((gnb_id & 0x00FF_FFFF) as u64, 24);
    GlobalGNB_ID {
        plmn_identity,
        gnb_id: GNB_ID::GNB_ID(GNB_ID_gNB_ID(id_bits)),
        ie_extensions: None,
    }
}

/// Build an `S_NSSAI` (Single Network Slice Selection Assistance Information).
///
/// ```ignore
/// use oxirush_ngap::helpers::*;
/// let nssai = s_nssai(1, Some([0x00, 0x00, 0x01])); // SST=1, SD=000001
/// let nssai = s_nssai(1, None);                       // SST=1, no SD
/// ```
pub fn s_nssai(sst: u8, sd: Option<[u8; 3]>) -> S_NSSAI {
    S_NSSAI {
        sst: SST(vec![sst]),
        sd: sd.map(|d| SD(d.to_vec())),
        ie_extensions: None,
    }
}

/// Build `UESecurityCapabilities` from 2-byte NAS capability
/// (byte 0 = encryption algorithms, byte 1 = integrity algorithms).
///
/// Each byte is expanded to a 16-bit bitvec (only high 8 bits used for NR;
/// EUTRA fields are zeroed).
pub fn ue_security_capabilities(ue_sec_cap: &[u8]) -> UESecurityCapabilities {
    let enc_byte = ue_sec_cap.first().copied().unwrap_or(0);
    let int_byte = ue_sec_cap.get(1).copied().unwrap_or(0);

    let byte_to_bv16 = |b: u8| -> BitVec<u8, Msb0> {
        (0..16u8)
            .map(|i| {
                if i < 8 {
                    (b >> (7 - i)) & 1 == 1
                } else {
                    false
                }
            })
            .collect()
    };

    let zero16: BitVec<u8, Msb0> = (0..16u8).map(|_| false).collect();
    UESecurityCapabilities {
        n_rencryption_algorithms: NRencryptionAlgorithms(byte_to_bv16(enc_byte)),
        n_rintegrity_protection_algorithms: NRintegrityProtectionAlgorithms(byte_to_bv16(int_byte)),
        eutr_aencryption_algorithms: EUTRAencryptionAlgorithms(zero16.clone()),
        eutr_aintegrity_protection_algorithms: EUTRAintegrityProtectionAlgorithms(zero16),
        ie_extensions: None,
    }
}

// ── NGAP PDU encode/decode ──────────────────────────────────────────────────

impl NGAP_PDU {
    /// Encode this PDU to APER wire bytes.
    ///
    /// ```ignore
    /// use oxirush_ngap::{build_ngap, ngap::*};
    ///
    /// let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
    ///     REJECT, UEContextReleaseRequest,
    ///     REJECT AMF_UE_NGAP_ID(1u64),
    ///     REJECT RAN_UE_NGAP_ID(0u32),
    ///     IGNORE Cause(Cause::RadioNetwork(CauseRadioNetwork(CauseRadioNetwork::USER_INACTIVITY))),
    /// );
    /// let bytes = pdu.encode().expect("APER encode failed");
    /// ```
    pub fn encode(&self) -> Result<Vec<u8>, asn1_codecs::PerCodecError> {
        use asn1_codecs::PerCodecData;
        use asn1_codecs::aper::AperCodec;
        let mut codec = PerCodecData::new_aper();
        self.aper_encode(&mut codec)?;
        Ok(codec.into_bytes())
    }

    /// Decode an NGAP PDU from APER wire bytes.
    ///
    /// ```ignore
    /// use oxirush_ngap::ngap::NGAP_PDU;
    ///
    /// let pdu = NGAP_PDU::decode(&aper_bytes).expect("APER decode failed");
    /// ```
    pub fn decode(bytes: &[u8]) -> Result<Self, asn1_codecs::PerCodecError> {
        use asn1_codecs::PerCodecData;
        use asn1_codecs::aper::AperCodec;
        let mut codec = PerCodecData::from_slice_aper(bytes);
        Self::aper_decode(&mut codec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plmn_2digit_mnc() {
        let id = plmn("208", "93");
        assert_eq!(id.0, vec![0x02, 0xF8, 0x39]);
        let (mcc, mnc) = plmn_from(&id).unwrap();
        assert_eq!(mcc, "208");
        assert_eq!(mnc, "93");
    }

    #[test]
    fn plmn_3digit_mnc() {
        let id = plmn("999", "070");
        let (mcc, mnc) = plmn_from(&id).unwrap();
        assert_eq!(mcc, "999");
        assert_eq!(mnc, "070");
    }

    #[test]
    fn int_to_bitvec_width() {
        let bv = int_to_bitvec(1, 8);
        assert_eq!(bv.len(), 8);
        assert!(bv[7]); // LSB
        assert!(!bv[0]); // MSB
    }

    #[test]
    fn guami_construction() {
        let g = guami(plmn("208", "93"), 1, 1, 0);
        assert_eq!(g.plmn_identity.0, vec![0x02, 0xF8, 0x39]);
        assert_eq!(g.amf_region_id.0.len(), AMF_REGION_ID_BITS);
        assert_eq!(g.amf_set_id.0.len(), AMF_SET_ID_BITS);
        assert_eq!(g.amf_pointer.0.len(), AMF_POINTER_BITS);
    }

    #[test]
    fn nr_cgi_36bits() {
        let cgi = nr_cgi(plmn("208", "93"), 0x000001, 1);
        assert_eq!(cgi.nr_cell_identity.0.len(), 36);
    }

    #[test]
    fn s_nssai_with_sd() {
        let n = s_nssai(1, Some([0x00, 0x00, 0x01]));
        assert_eq!(n.sst.0, vec![1]);
        assert_eq!(n.sd.unwrap().0, vec![0x00, 0x00, 0x01]);
    }

    #[test]
    fn s_nssai_without_sd() {
        let n = s_nssai(1, None);
        assert_eq!(n.sst.0, vec![1]);
        assert!(n.sd.is_none());
    }

    #[test]
    fn encode_decode_roundtrip() {
        use crate::{build_ngap, ngap::*};
        let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
            REJECT, UEContextReleaseRequest,
            REJECT AMF_UE_NGAP_ID(1u64),
            REJECT RAN_UE_NGAP_ID(0u32),
            IGNORE Cause(Cause::RadioNetwork(CauseRadioNetwork(CauseRadioNetwork::USER_INACTIVITY))),
        );
        let bytes = pdu.encode().expect("encode");
        let decoded = NGAP_PDU::decode(&bytes).expect("decode");
        assert_eq!(pdu, decoded);
    }
}
