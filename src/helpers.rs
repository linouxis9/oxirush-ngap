//! Convenience helpers for building common NGAP types.
//!
//! These helpers cover PLMN/TBCD encoding, NR identifiers, security bit
//! strings, and common AMF/gNB identities used by NGAP consumers.
//!
//! None of them panics. One that takes a number for a field of a fixed size
//! gives an [`Error`] of the kind `Argument` for a number that does not fit,
//! and cuts none short.

use rasn::types::{BitString, FixedBitString, FixedOctetString};

use crate::Error;
use crate::ngap::*;

/// `value`, which is `what`, when it fits in `bits` bits.
fn fits(what: &str, value: u64, bits: u32) -> Result<u64, Error> {
    match bits >= 64 || value >> bits == 0 {
        true => Ok(value),
        false => Err(Error::argument(format!(
            "{what} {value} does not fit in {bits} bits"
        ))),
    }
}

/// Convert an integer to a most-significant-bit-first bit string of `width` bits.
///
/// An error of the kind `Argument` for a width over the 64 bits of the value, and
/// for a value that does not fit in `width` bits.
pub fn int_to_bitvec(value: u64, width: usize) -> Result<BitString, Error> {
    let bits = u32::try_from(width).ok().filter(|bits| *bits <= 64);
    let bits = bits.ok_or_else(|| {
        Error::argument(format!(
            "a bit string of {width} bits does not come from a number of 64 bits"
        ))
    })?;
    let value = fits("the number", value, bits)?;
    Ok((0..bits).rev().map(|bit| (value >> bit) & 1 == 1).collect())
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

/// The octets of the PLMN identity of an MCC and an MNC: the three digits of `mcc`, and
/// the two or three of `mnc`, each a number from 0 to 9. `None` for an MNC of another
/// length, and for a digit over 9.
///
/// TS 38.413 clause 9.3.3.5 has two digits in each octet, "bits 4 to 1 of octet n encoding
/// digit 2n-1" and "bits 8 to 5 of octet n encoding digit 2n", of "3 digits from MCC
/// followed by either a filler digit plus 2 digits from MNC" or "3 digits from MNC",
/// where the filler is 1111. So octet 2 has the first digit of a three-digit MNC, and
/// octet 3 its second and its third: 310-410 is `13 40 01`. This is not the order of
/// TS 24.008, which a NAS message has, where octet 2 has the third digit of the MNC.
pub(crate) fn plmn_octets(mcc: [u8; 3], mnc: &[u8]) -> Option<[u8; 3]> {
    if mcc.iter().chain(mnc).any(|digit| *digit > 9) {
        return None;
    }
    // The six digits of the clause: a filler stands before an MNC of two digits.
    let [d1, d2, d3] = mcc;
    let [d4, d5, d6] = match *mnc {
        [first, second] => [0x0F, first, second],
        [first, second, third] => [first, second, third],
        _ => return None,
    };
    // Digit 2n-1 in bits 4 to 1 of octet n, and digit 2n in its bits 8 to 5.
    Some([d2 << 4 | d1, d4 << 4 | d3, d6 << 4 | d5])
}

/// The digits of the MCC and those of the MNC in the octets of a PLMN identity, in the
/// order of [`plmn_octets`]: the MNC has two after a filler, and three otherwise. `None`
/// for octets that are not a PLMN identity: a half octet is no digit, where it is not
/// the filler before an MNC of two digits.
pub(crate) fn plmn_digits(octets: [u8; 3]) -> Option<(String, String)> {
    let [first, second, third] = octets;
    let mcc = [first & 0x0F, first >> 4, second & 0x0F];
    let mnc = [second >> 4, third & 0x0F, third >> 4];
    let mnc = mnc.strip_prefix(&[0x0F]).unwrap_or(&mnc);
    let digits = |digits: &[u8]| {
        let digits = digits.iter();
        let digits = digits.map(|digit| char::from_digit((*digit).into(), 10));
        digits.collect::<Option<String>>()
    };
    Some((digits(&mcc)?, digits(mnc)?))
}

/// The digits that `text` is, when it is ASCII digits alone.
fn digits(text: &str) -> Option<Vec<u8>> {
    let digits = text.bytes();
    let digits = digits.map(|digit| digit.is_ascii_digit().then(|| digit - b'0'));
    digits.collect()
}

/// Encode MCC/MNC as the three octets of a `PLMNIdentity`, in the order of
/// TS 38.413 clause 9.3.3.5: `plmn("310", "410")` is `13 40 01`, and `plmn("208", "93")`
/// is `02 f8 39`.
///
/// An error of the kind `Argument` unless `mcc` is three decimal digits and `mnc` two
/// or three.
pub fn plmn(mcc: &str, mnc: &str) -> Result<PLMNIdentity, Error> {
    let refused = |what: &str, text: &str, has: &str| {
        Error::argument(format!(
            "{text:?} is not {what}: it has {has} digits, 0 to 9"
        ))
    };
    let mcc_digits = digits(mcc).and_then(|digits| <[u8; 3]>::try_from(digits).ok());
    let mcc_digits = mcc_digits.ok_or_else(|| refused("an MCC", mcc, "three"))?;
    let octets = digits(mnc).and_then(|mnc| plmn_octets(mcc_digits, &mnc));
    let octets = octets.ok_or_else(|| refused("an MNC", mnc, "two or three"))?;
    Ok(PLMNIdentity(FixedOctetString::new(octets)))
}

/// Decode a `PLMNIdentity` into MCC and MNC strings, in the order of
/// TS 38.413 clause 9.3.3.5.
///
/// `None` for octets that are not a PLMN identity: a half octet is no digit, where it
/// is not the filler before an MNC of two digits. `ab cd ef` is none, and so is
/// `02 f8 f9`.
pub fn plmn_from(identity: &PLMNIdentity) -> Option<(String, String)> {
    let octets = &identity.0;
    plmn_digits([octets[0], octets[1], octets[2]])
}

/// AMF Region ID field width.
pub const AMF_REGION_ID_BITS: usize = 8;
/// AMF Set ID field width.
pub const AMF_SET_ID_BITS: usize = 10;
/// AMF Pointer field width.
pub const AMF_POINTER_BITS: usize = 6;

/// Build a Globally Unique AMF Identifier.
///
/// An error of the kind `Argument` for an AMF Set ID that does not fit in 10 bits and
/// for an AMF Pointer that does not fit in 6 (TS 38.413 clauses 9.3.3.12 and 9.3.3.19).
pub fn guami(
    plmn_identity: PLMNIdentity,
    region_id: u8,
    set_id: u16,
    pointer: u8,
) -> Result<GUAMI, Error> {
    let set_id = fits("the AMF Set ID", set_id.into(), 10)?;
    let pointer = fits("the AMF Pointer", pointer.into(), 6)?;
    Ok(GUAMI::new(
        plmn_identity,
        AMFRegionID(fixed_bit_string::<8>(u64::from(region_id))),
        AMFSetID(fixed_bit_string::<10>(set_id)),
        AMFPointer(fixed_bit_string::<6>(pointer)),
        None,
    ))
}

/// Build a 5G Tracking Area Identity: a PLMN and the three octets of a TAC.
pub fn tai(plmn_identity: PLMNIdentity, tac: [u8; 3]) -> TAI {
    TAI::new(plmn_identity, TAC(FixedOctetString::new(tac)), None)
}

/// Build an NR Cell Global Identifier from a 24-bit gNB ID and 12-bit cell ID.
///
/// An error of the kind `Argument` for a gNB ID that does not fit in 24 bits and for a
/// cell that does not fit in the 12 bits that follow it in the 36 of an NR Cell
/// Identity (TS 38.413 clause 9.3.1.7).
pub fn nr_cgi(plmn_identity: PLMNIdentity, gnb_id: u32, cell_id: u16) -> Result<NRCGI, Error> {
    let gnb_id = fits("the gNB ID", gnb_id.into(), 24)?;
    let cell_id = fits("the cell of the gNB", cell_id.into(), 12)?;
    let nci = (gnb_id << 12) | cell_id;
    Ok(NRCGI::new(
        plmn_identity,
        NRCellIdentity(fixed_bit_string::<36>(nci)),
        None,
    ))
}

/// Build a `GlobalGNBID` for a 24-bit gNB identifier.
///
/// An error of the kind `Argument` for a gNB ID that does not fit in 24 bits.
pub fn global_gnb_id(plmn_identity: PLMNIdentity, gnb_id: u32) -> Result<GlobalGNBID, Error> {
    let gnb_id = fits("the gNB ID", gnb_id.into(), 24)?;
    Ok(GlobalGNBID::new(
        plmn_identity,
        GNBID::gNB_ID(int_to_bitvec(gnb_id, 24)?),
        None,
    ))
}

/// Build Single Network Slice Selection Assistance Information.
pub fn s_nssai(sst: u8, sd: Option<[u8; 3]>) -> SNSSAI {
    SNSSAI::new(
        SST(FixedOctetString::new([sst])),
        sd.map(|value| SD(FixedOctetString::new(value))),
        None,
    )
}

/// Build 5G UE security capabilities from the first octet of the NR encryption
/// bitmap and the first octet of the NR integrity bitmap.
///
/// Each is the first octet of an NGAP bitmap, whose leading bit is
/// 128-NEA1 or 128-NIA1 (TS 38.413 §9.3.1.86). The octets of the NAS UE
/// security capability IE lead with 5G-EA0 and 5G-IA0 (TS 24.501 §9.11.3.54),
/// so shift them left by one bit first. The E-UTRA bitmaps are all zeros:
/// EEA0 and EIA0 only.
pub fn ue_security_capabilities(encryption: u8, integrity: u8) -> UESecurityCapabilities {
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
    use crate::{ErrorKind, build_ngap};

    #[test]
    fn plmn_round_trips_for_two_digit_mnc() {
        let identity = plmn("208", "93").unwrap();
        assert_eq!(&identity.0[..], &[0x02, 0xF8, 0x39]);
        assert_eq!(plmn_from(&identity), Some(("208".into(), "93".into())));
    }

    #[test]
    fn plmn_round_trips_for_three_digit_mnc() {
        // The digits 9 9 9 0 7 0, two in each octet, the second over the first.
        let identity = plmn("999", "070").unwrap();
        assert_eq!(&identity.0[..], &[0x99, 0x09, 0x07]);
        assert_eq!(plmn_from(&identity), Some(("999".into(), "070".into())));
    }

    #[test]
    fn what_is_no_mcc_or_no_mnc_is_an_error_of_its_argument() {
        // Two digits and four for an MCC, one and four for an MNC, and what is no digit:
        // a letter, a sign, a digit of another script.
        for (mcc, mnc) in [
            ("20", "93"),
            ("2080", "93"),
            ("208", "9"),
            ("208", "9300"),
            ("20a", "93"),
            ("208", "+3"),
            ("208", "9٣"),
            ("", ""),
        ] {
            let error = plmn(mcc, mnc).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Argument, "{mcc}-{mnc}");
        }
        assert_eq!(
            plmn("20", "93").unwrap_err().to_string(),
            "\"20\" is not an MCC: it has three digits, 0 to 9"
        );
        assert_eq!(
            plmn("208", "9").unwrap_err().to_string(),
            "\"9\" is not an MNC: it has two or three digits, 0 to 9"
        );
        // The octets of the digits alone, for the notation of a PLMN identity.
        assert_eq!(plmn_octets([2, 0, 8], &[9, 3]), Some([0x02, 0xF8, 0x39]));
        assert_eq!(plmn_octets([2, 0, 8], &[9]), None);
        assert_eq!(plmn_octets([2, 0, 10], &[9, 3]), None);
    }

    #[test]
    fn octets_that_are_no_plmn_identity_have_no_mcc_and_no_mnc() {
        // No digit at all, a filler in the MNC, a filler in the MCC, a filler before
        // an MNC that has another one, and a digit over 9.
        for octets in [
            [0xab, 0xcd, 0xef],
            [0x02, 0xf8, 0xf9],
            [0x0f, 0xf8, 0x39],
            [0x02, 0xf8, 0x3f],
            [0x02, 0xa8, 0x39],
        ] {
            let identity = PLMNIdentity::from(octets);
            assert_eq!(plmn_from(&identity), None, "{octets:02x?}");
        }
    }

    #[test]
    fn common_identifiers_have_the_required_widths() {
        let plmn = plmn("208", "93").unwrap();
        // The 24-bit gNB ID and the 12-bit cell: a 36-bit NR cell identity,
        // octet-aligned after the PLMN (X.691 (02/2021) §16.10).
        let cgi = nr_cgi(plmn.clone(), 0xab_cdef, 0x123).unwrap();
        assert_eq!(
            cgi.n_rcell_identity.0[..36],
            int_to_bitvec(0xa_bcde_f123, 36).unwrap()[..]
        );
        assert_eq!(
            rasn::aper::encode(&cgi).unwrap(),
            [0x00, 0x02, 0xf8, 0x39, 0xab, 0xcd, 0xef, 0x12, 0x30]
        );
        let gnb = global_gnb_id(plmn, 0xab_cdef).unwrap();
        let GNBID::gNB_ID(bits) = gnb.g_nb_id else {
            panic!("expected gNB ID");
        };
        assert_eq!(bits, int_to_bitvec(0xab_cdef, 24).unwrap());
    }

    /// The number that the bits are, the first bit the most significant.
    fn number(bits: impl Iterator<Item = bool>) -> u64 {
        bits.fold(0, |number, bit| number << 1 | u64::from(bit))
    }

    #[test]
    fn a_number_that_does_not_fit_its_field_is_an_error_and_is_not_cut_short() {
        let network = || plmn("208", "93").unwrap();
        let argument = |error: Error| {
            assert_eq!(error.kind(), ErrorKind::Argument);
            error.to_string()
        };

        // An AMF Set ID of 10 bits and an AMF Pointer of 6: the last value of each,
        // and the one after it.
        let amf = guami(network(), 0xff, 1023, 63).unwrap();
        assert_eq!(number(amf.a_mfset_id.0[..10].iter().map(|bit| *bit)), 1023);
        assert_eq!(number(amf.a_mfpointer.0[..6].iter().map(|bit| *bit)), 63);
        assert_eq!(
            argument(guami(network(), 1, 1024, 0).unwrap_err()),
            "the AMF Set ID 1024 does not fit in 10 bits"
        );
        assert_eq!(
            argument(guami(network(), 1, 1, 64).unwrap_err()),
            "the AMF Pointer 64 does not fit in 6 bits"
        );

        // A gNB ID of 24 bits, alone and before the 12 bits of a cell.
        assert!(global_gnb_id(network(), 0xff_ffff).is_ok());
        assert_eq!(
            argument(global_gnb_id(network(), 0x100_0000).unwrap_err()),
            "the gNB ID 16777216 does not fit in 24 bits"
        );
        assert!(nr_cgi(network(), 0xff_ffff, 0xfff).is_ok());
        assert_eq!(
            argument(nr_cgi(network(), 0x100_0000, 1).unwrap_err()),
            "the gNB ID 16777216 does not fit in 24 bits"
        );
        assert_eq!(
            argument(nr_cgi(network(), 1, 0x1000).unwrap_err()),
            "the cell of the gNB 4096 does not fit in 12 bits"
        );
    }

    #[test]
    fn a_bit_string_has_the_bits_of_a_number_that_fits_its_width() {
        let bits = int_to_bitvec(0b101, 3).unwrap();
        assert_eq!(
            bits.iter().map(|bit| *bit).collect::<Vec<_>>(),
            [true, false, true]
        );
        assert_eq!(int_to_bitvec(0, 0).unwrap().len(), 0);
        assert_eq!(
            number(int_to_bitvec(u64::MAX, 64).unwrap().iter().map(|bit| *bit)),
            u64::MAX
        );
        // A number of more bits than the width, and a width of more bits than a number.
        let wide = int_to_bitvec(0b1000, 3).unwrap_err();
        assert_eq!(wide.kind(), ErrorKind::Argument);
        assert_eq!(wide.to_string(), "the number 8 does not fit in 3 bits");
        let long = int_to_bitvec(1, 65).unwrap_err();
        assert_eq!(long.kind(), ErrorKind::Argument);
        assert_eq!(
            long.to_string(),
            "a bit string of 65 bits does not come from a number of 64 bits"
        );
    }

    #[test]
    fn security_capability_octets_lead_with_the_first_algorithm() {
        // 128-NEA1 and 128-NEA2; 128-NIA2.
        let capabilities = ue_security_capabilities(0xC0, 0x40);
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
        )
        .expect("build PDU");
        let bytes = pdu.encode().expect("encode PDU");
        let decoded = NGAP_PDU::decode(&bytes).expect("decode PDU");
        assert_eq!(pdu, decoded);
        assert_eq!(decoded.procedure_name(), "UEContextReleaseRequest");
        assert!(decoded.is_initiating());
    }
}
