//! Convenience helpers for building common NGAP types.
//!
//! These helpers cover PLMN/TBCD encoding, NR identifiers, security bit
//! strings, and common AMF/gNB identities used by NGAP consumers.

use rasn::types::{BitString, FixedBitString, FixedOctetString};

use crate::ngap::*;

/// Convert an integer to a most-significant-bit-first bit string of `width` bits.
///
/// # Panics
///
/// Panics when `width` exceeds 64.
pub fn int_to_bitvec(value: u64, width: usize) -> BitString {
    assert!(width <= 64, "width must be <= 64");
    (0..width)
        .rev()
        .map(|bit| (value >> bit) & 1 == 1)
        .collect()
}

/// Convert bytes to a most-significant-bit-first bit string.
pub fn bytes_to_bitvec(bytes: &[u8]) -> BitString {
    bytes
        .iter()
        .flat_map(|byte| (0..8u8).rev().map(move |bit| (byte >> bit) & 1 == 1))
        .collect()
}

fn fixed_bit_string<const N: usize>(value: u64) -> FixedBitString<N> {
    let mut bits = FixedBitString::<N>::ZERO;
    for index in 0..N {
        bits.set(index, (value >> (N - index - 1)) & 1 == 1);
    }
    bits
}

/// Encode MCC/MNC as a 3-octet TBCD `PLMNIdentity`.
///
/// MCC must contain three decimal digits; MNC must contain two or three.
pub fn plmn(mcc: &str, mnc: &str) -> PLMNIdentity {
    assert!(
        mcc.len() == 3 && mcc.bytes().all(|byte| byte.is_ascii_digit()),
        "MCC must be 3 ASCII digits"
    );
    assert!(
        (mnc.len() == 2 || mnc.len() == 3) && mnc.bytes().all(|byte| byte.is_ascii_digit()),
        "MNC must be 2 or 3 ASCII digits"
    );

    let mcc_digits: Vec<u8> = mcc.bytes().map(|byte| byte - b'0').collect();
    let mnc_digits: Vec<u8> = mnc.bytes().map(|byte| byte - b'0').collect();
    let mut bytes = [0u8; 3];
    bytes[0] = (mcc_digits[1] << 4) | mcc_digits[0];
    if mnc_digits.len() == 2 {
        bytes[1] = 0xF0 | mcc_digits[2];
        bytes[2] = (mnc_digits[1] << 4) | mnc_digits[0];
    } else {
        bytes[1] = (mnc_digits[2] << 4) | mcc_digits[2];
        bytes[2] = (mnc_digits[1] << 4) | mnc_digits[0];
    }
    PLMNIdentity(FixedOctetString::new(bytes))
}

/// Decode a `PLMNIdentity` into MCC and MNC strings.
pub fn plmn_from(identity: &PLMNIdentity) -> (String, String) {
    let bytes = &identity.0;
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
    (mcc, mnc)
}

/// AMF Region ID field width.
pub const AMF_REGION_ID_BITS: usize = 8;
/// AMF Set ID field width.
pub const AMF_SET_ID_BITS: usize = 10;
/// AMF Pointer field width.
pub const AMF_POINTER_BITS: usize = 6;

/// Build a Globally Unique AMF Identifier.
pub fn guami(plmn_identity: PLMNIdentity, region_id: u8, set_id: u16, pointer: u8) -> GUAMI {
    GUAMI::new(
        plmn_identity,
        AMFRegionID(fixed_bit_string::<8>(u64::from(region_id))),
        AMFSetID(fixed_bit_string::<10>(u64::from(set_id))),
        AMFPointer(fixed_bit_string::<6>(u64::from(pointer))),
        None,
    )
}

/// Build a 5G Tracking Area Identity.
///
/// # Panics
///
/// Panics unless `tac` contains exactly three octets.
pub fn tai(plmn_identity: PLMNIdentity, tac: &[u8]) -> TAI {
    let tac: [u8; 3] = tac
        .try_into()
        .expect("5G TAC must contain exactly 3 octets");
    TAI::new(plmn_identity, TAC(FixedOctetString::new(tac)), None)
}

/// Build an NR Cell Global Identifier from a 24-bit gNB ID and 12-bit cell ID.
pub fn nr_cgi(plmn_identity: PLMNIdentity, gnb_id: u32, cell_id: u16) -> NRCGI {
    let nci = (u64::from(gnb_id & 0x00FF_FFFF) << 12) | u64::from(cell_id & 0x0FFF);
    NRCGI::new(
        plmn_identity,
        NRCellIdentity(fixed_bit_string::<36>(nci)),
        None,
    )
}

/// Build a `GlobalGNBID` for a 24-bit gNB identifier.
pub fn global_gnb_id(plmn_identity: PLMNIdentity, gnb_id: u32) -> GlobalGNBID {
    GlobalGNBID::new(
        plmn_identity,
        GNBID::gNB_ID(int_to_bitvec(u64::from(gnb_id & 0x00FF_FFFF), 24)),
        None,
    )
}

/// Build Single Network Slice Selection Assistance Information.
pub fn s_nssai(sst: u8, sd: Option<[u8; 3]>) -> SNSSAI {
    SNSSAI::new(
        SST(FixedOctetString::new([sst])),
        sd.map(|value| SD(FixedOctetString::new(value))),
        None,
    )
}

/// Build 5G UE security capabilities from encryption and integrity octets.
///
/// Each octet is the first octet of an NGAP bitmap, whose leading bit is
/// 128-NEA1 or 128-NIA1 (TS 38.413 §9.3.1.86). The octets of the NAS UE
/// security capability IE lead with 5G-EA0 and 5G-IA0 (TS 24.501 §9.11.3.54),
/// so shift them left by one bit first. The E-UTRA bitmaps are all zeros:
/// EEA0 and EIA0 only.
pub fn ue_security_capabilities(capabilities: &[u8]) -> UESecurityCapabilities {
    let encryption = capabilities.first().copied().unwrap_or(0);
    let integrity = capabilities.get(1).copied().unwrap_or(0);
    let algorithms = |byte: u8| {
        (0..16u8)
            .map(|index| index < 8 && (byte >> (7 - index)) & 1 == 1)
            .collect()
    };
    let zeroes = || (0..16).map(|_| false).collect();

    UESecurityCapabilities::new(
        NRencryptionAlgorithms(algorithms(encryption)),
        NRintegrityProtectionAlgorithms(algorithms(integrity)),
        EUTRAencryptionAlgorithms(zeroes()),
        EUTRAintegrityProtectionAlgorithms(zeroes()),
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_ngap;

    #[test]
    fn plmn_round_trips_for_two_digit_mnc() {
        let identity = plmn("208", "93");
        assert_eq!(&identity.0[..], &[0x02, 0xF8, 0x39]);
        assert_eq!(plmn_from(&identity), ("208".into(), "93".into()));
    }

    #[test]
    fn plmn_round_trips_for_three_digit_mnc() {
        let identity = plmn("999", "070");
        assert_eq!(plmn_from(&identity), ("999".into(), "070".into()));
    }

    #[test]
    fn common_identifiers_have_the_required_widths() {
        let plmn = plmn("208", "93");
        let cgi = nr_cgi(plmn.clone(), 1, 1);
        assert_eq!(cgi.n_rcell_identity.0[..36].len(), 36);
        let gnb = global_gnb_id(plmn, 1);
        let GNBID::gNB_ID(bits) = gnb.g_nb_id else {
            panic!("expected gNB ID");
        };
        assert_eq!(bits.len(), 24);
    }

    #[test]
    fn security_capability_octets_lead_with_the_first_algorithm() {
        // 128-NEA1 and 128-NEA2; 128-NIA2.
        let capabilities = ue_security_capabilities(&[0xC0, 0x40]);
        let leading = |bits: &BitString| bits.iter().take(3).map(|bit| *bit).collect::<Vec<_>>();
        assert_eq!(
            leading(&capabilities.n_rencryption_algorithms.0),
            [true, true, false]
        );
        assert_eq!(
            leading(&capabilities.n_rintegrity_protection_algorithms.0),
            [false, true, false]
        );
    }

    #[test]
    fn pdu_encode_decode_round_trip() {
        let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
            IGNORE, UEContextReleaseRequest,
            REJECT AMF_UE_NGAP_ID(1u64),
            REJECT RAN_UE_NGAP_ID(7u32),
            IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
        );
        let bytes = pdu.encode().expect("encode PDU");
        let decoded = NGAP_PDU::decode(&bytes).expect("decode PDU");
        assert_eq!(pdu, decoded);
        assert_eq!(decoded.procedure_name(), "UEContextReleaseRequest");
        assert!(decoded.is_initiating());
    }
}
