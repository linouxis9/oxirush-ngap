//! Convenience helpers for building common NGAP types.
//!
//! These helpers cover PLMN/TBCD encoding, NR identifiers, security bit
//! strings, and common AMF/gNB identities used by NGAP consumers.
//!
//! None of them panics. One that takes a number for a field of a fixed size
//! gives an [`Error`] of the kind `Argument` for a number that does not fit,
//! and cuts none short.

use oxirush_aper::{BitString, Bits};
use oxirush_utils::identity::Plmn;

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

/// The octets of the PLMN identity of an MCC and an MNC: the three digits of `mcc`, and
/// the two or three of `mnc`, each a number from 0 to 9. `None` for an MNC of another
/// length, and for a digit over 9.
///
/// The order is that of TS 38.413 clause 9.3.3.5, which `Plmn::to_ap_octets` of oxirush-utils
/// writes: octet 2 has the first digit of a three-digit MNC, and octet 3 its second
/// and its third, so that 310-410 is `13 40 01`. This is not the order of TS 24.008,
/// which a NAS message has, where octet 2 has the third digit of the MNC.
pub(crate) fn plmn_octets(mcc: [u8; 3], mnc: &[u8]) -> Option<[u8; 3]> {
    Some(Plmn::from_digits(mcc, mnc).ok()?.to_ap_octets())
}

/// The digits of the MCC and those of the MNC in the octets of a PLMN identity, in the
/// order of [`plmn_octets`]: the MNC has two after a filler, and three otherwise. `None`
/// for octets that are not a PLMN identity: a half octet is no digit, where it is not
/// the filler before an MNC of two digits.
pub(crate) fn plmn_digits(octets: [u8; 3]) -> Option<(String, String)> {
    let plmn = Plmn::from_ap_octets(octets).ok()?;
    Some((plmn.mcc().to_string(), plmn.mnc().to_string()))
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
    Ok(PLMNIdentity(octets))
}

/// Decode a `PLMNIdentity` into MCC and MNC strings, in the order of
/// TS 38.413 clause 9.3.3.5.
///
/// `None` for octets that are not a PLMN identity: a half octet is no digit, where it
/// is not the filler before an MNC of two digits. `ab cd ef` is none, and so is
/// `02 f8 f9`.
pub fn plmn_from(identity: &PLMNIdentity) -> Option<(String, String)> {
    plmn_digits(identity.0)
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
        AMFRegionID(Bits::from(region_id)),
        AMFSetID(Bits::low(set_id, AMF_SET_ID_BITS)),
        AMFPointer(Bits::low(pointer, AMF_POINTER_BITS)),
        None,
    ))
}

/// Build a 5G Tracking Area Identity: a PLMN and the three octets of a TAC.
pub fn tai(plmn_identity: PLMNIdentity, tac: [u8; 3]) -> TAI {
    TAI::new(plmn_identity, TAC(tac), None)
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
        NRCellIdentity(Bits::low(nci, 36)),
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
        GNBID::gNB_ID(Bits::low(gnb_id, 24)),
        None,
    ))
}

/// Build Single Network Slice Selection Assistance Information.
pub fn s_nssai(sst: u8, sd: Option<[u8; 3]>) -> SNSSAI {
    SNSSAI::new(SST([sst]), sd.map(SD), None)
}

/// Build 5G UE security capabilities from the first octet of the NR encryption
/// bitmap and the first octet of the NR integrity bitmap.
///
/// Each is the first octet of an NGAP bitmap, whose leading bit is
/// 128-NEA1 or 128-NIA1 (TS 38.413 §9.3.1.86). The octets of the NAS UE
/// security capability IE lead with 5G-EA0 and 5G-IA0 (TS 24.501 §9.11.3.54),
/// so shift them left by one bit first. The E-UTRA bitmaps are all zeros:
/// EEA0 and EIA0 only.
pub fn ue_security_capabilities(encryption: u8, integrity: u8) -> UESecurityCapabilities<'static> {
    // Sixteen bits, of which the first eight are the algorithms.
    let algorithms = |byte: u8| BitString::from([byte, 0]);
    UESecurityCapabilities::new(
        NRencryptionAlgorithms(algorithms(encryption)),
        NRintegrityProtectionAlgorithms(algorithms(integrity)),
        EUTRAencryptionAlgorithms(algorithms(0)),
        EUTRAintegrityProtectionAlgorithms(algorithms(0)),
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
        assert_eq!(identity.0, [0x02, 0xF8, 0x39]);
        assert_eq!(plmn_from(&identity), Some(("208".into(), "93".into())));
    }

    #[test]
    fn plmn_round_trips_for_three_digit_mnc() {
        // The digits 9 9 9 0 7 0, two in each octet, the second over the first.
        let identity = plmn("999", "070").unwrap();
        assert_eq!(identity.0, [0x99, 0x09, 0x07]);
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
        let cgi = nr_cgi(plmn, 0xab_cdef, 0x123).unwrap();
        assert_eq!(
            cgi.nr_cell_identity.0,
            Bits::new(0xa_bcde_f123, 36).unwrap()
        );
        assert_eq!(
            oxirush_aper::encode(&cgi).unwrap(),
            [0x00, 0x02, 0xf8, 0x39, 0xab, 0xcd, 0xef, 0x12, 0x30]
        );
        let gnb = global_gnb_id(plmn, 0xab_cdef).unwrap();
        let GNBID::gNB_ID(bits) = gnb.gnb_id else {
            panic!("expected gNB ID");
        };
        assert_eq!(bits, Bits::new(0xab_cdef, 24).unwrap());
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
        assert_eq!(
            (amf.amf_region_id.0.value(), amf.amf_region_id.0.len()),
            (0xff, 8)
        );
        assert_eq!(
            (amf.amf_set_id.0.value(), amf.amf_set_id.0.len()),
            (1023, 10)
        );
        assert_eq!(
            (amf.amf_pointer.0.value(), amf.amf_pointer.0.len()),
            (63, 6)
        );
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
    fn security_capability_octets_lead_with_the_first_algorithm() {
        // 128-NEA1 and 128-NEA2; 128-NIA2.
        let capabilities = ue_security_capabilities(0xC0, 0x40);
        let leading = |bits: &BitString| (0..3).map(|at| bits.get(at).unwrap()).collect::<Vec<_>>();
        assert_eq!(
            leading(&capabilities.nr_encryption_algorithms.0),
            [true, true, false]
        );
        assert_eq!(
            leading(&capabilities.nr_integrity_protection_algorithms.0),
            [false, true, false]
        );
        // Sixteen bits each, of which the last eight are not set.
        assert_eq!(capabilities.nr_encryption_algorithms.0.len(), 16);
        assert_eq!(
            capabilities.nr_encryption_algorithms.0.as_octets(),
            [0xC0, 0x00]
        );
        assert_eq!(
            capabilities.eutra_encryption_algorithms.0.as_octets(),
            [0, 0]
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
