//! Convenience helpers for building common NGAP types.
//!
//! These helpers cover PLMN/TBCD encoding, NR identifiers, security bit
//! strings, and common AMF/gNB identities used by NGAP consumers.
//!
//! None of them panics. One that takes a number for a field of a fixed size
//! gives an [`Error`] of the kind `Argument` for a number that does not fit,
//! and cuts none short.
//!
//! ```
//! # use std::fmt::Write as _;
//! # let mut printed = String::new();
//! # macro_rules! println { ($($argument:tt)*) => { writeln!(printed, $($argument)*).unwrap() } }
//! use oxirush_ngap::helpers::*;
//!
//! // A PLMN from its MCC and its MNC, and what is identified within it
//! let network = plmn("208", "93").unwrap();
//! let amf = guami(network, 1, 1, 0).unwrap(); // region, set, pointer
//! let tracking_area = tai(network, [0x00, 0x00, 0x01]); // TAC
//! let cell = nr_cgi(network, 0x123456, 24, 1).unwrap(); // gNB of 24 bits, cell
//! let gnb = global_gnb_id(network, 0x123456, 24).unwrap();
//! let ng_enb = global_ng_enb_id(network, NgEnbKind::Macro, 0x12345).unwrap();
//! let lte_cell = eutra_cgi(network, NgEnbKind::Macro, 0x12345, 7).unwrap(); // ng-eNB, cell
//!
//! // A bit string from octets, and the NR algorithms of a UE: the first
//! // octet of its encryption bitmap, then of its integrity bitmap
//! let algorithm_mask = oxirush_ngap::BitString::from([0xe0]);
//! let capabilities = ue_security_capabilities(0xe0, 0xe0);
//!
//! // What is no MCC, and an AMF Set ID that does not fit in its 10 bits
//! println!("{}", plmn("20", "93").unwrap_err());
//! println!("{}", guami(network, 1, 1024, 0).unwrap_err());
//! # assert_eq!(printed, r##""20" is not an MCC: it has three digits, 0 to 9
//! # the AMF Set ID 1024 does not fit in 10 bits
//! # "##);
//! ```
//!
//! ```text
//! "20" is not an MCC: it has three digits, 0 to 9
//! the AMF Set ID 1024 does not fit in 10 bits
//! ```
//!
//! A PLMN identity has its digits in the order of TS 38.413 clause 9.3.3.5:
//! `plmn("310", "410")` is the octets `13 40 01`, where a NAS message has
//! `13 00 14`. [`plmn_from`] reads the MCC and the MNC of a PLMN identity, and a
//! gNB ID has the length that it is given, 22 to 32 bits (clause 9.3.1.6), and
//! an ng-eNB ID that of its kind (clause 9.3.1.8).

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
///
/// ```
/// use oxirush_ngap::helpers::{plmn, plmn_from};
///
/// let network = plmn("310", "410")?;
/// assert_eq!(network.0, [0x13, 0x40, 0x01]);
/// assert_eq!(plmn_from(&network), Some(("310".into(), "410".into())));
/// # Ok::<(), oxirush_ngap::Error>(())
/// ```
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
///
/// ```
/// use oxirush_ngap::helpers::plmn_from;
/// use oxirush_ngap::ngap::PLMNIdentity;
///
/// assert_eq!(plmn_from(&PLMNIdentity([0x02, 0xf8, 0x39])), Some(("208".into(), "93".into())));
/// assert_eq!(plmn_from(&PLMNIdentity([0xab, 0xcd, 0xef])), None);
/// ```
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

/// The length of a gNB ID: 22 to 32 bits (TS 38.413 clause 9.3.1.6).
fn of_a_gnb_id(length: u32) -> Result<u32, Error> {
    match (22..=32).contains(&length) {
        true => Ok(length),
        false => Err(Error::argument(format!(
            "a gNB ID has 22 to 32 bits, and {length} are given"
        ))),
    }
}

/// Build an NR Cell Global Identifier from a gNB ID of `gnb_length` bits, 22 to 32, and
/// the cell that the rest of the 36 bits of the NR Cell Identity hold: the gNB ID is
/// its leftmost bits (TS 38.413 clauses 9.3.1.6 and 9.3.1.7), and a gNB ID of 24 bits
/// leaves 12 for the cell.
///
/// An error of the kind `Argument` for a length outside 22 to 32, for a gNB ID that does
/// not fit in it, and for a cell that does not fit in the bits after it.
pub fn nr_cgi(
    plmn_identity: PLMNIdentity,
    gnb_id: u32,
    gnb_length: u32,
    cell_id: u16,
) -> Result<NRCGI, Error> {
    let length = of_a_gnb_id(gnb_length)?;
    let gnb_id = fits("the gNB ID", gnb_id.into(), length)?;
    let cell_id = fits("the cell of the gNB", cell_id.into(), 36 - length)?;
    let nci = (gnb_id << (36 - length)) | cell_id;
    Ok(NRCGI::new(
        plmn_identity,
        NRCellIdentity(Bits::low(nci, 36)),
        None,
    ))
}

/// Build a `GlobalGNBID` for a gNB ID of `gnb_length` bits, 22 to 32 (TS 38.413 clause
/// 9.3.1.6).
///
/// An error of the kind `Argument` for a length outside 22 to 32, and for a gNB ID that
/// does not fit in it.
pub fn global_gnb_id(
    plmn_identity: PLMNIdentity,
    gnb_id: u32,
    gnb_length: u32,
) -> Result<GlobalGNBID, Error> {
    let length = of_a_gnb_id(gnb_length)?;
    let gnb_id = fits("the gNB ID", gnb_id.into(), length)?;
    Ok(GlobalGNBID::new(
        plmn_identity,
        GNBID::gNB_ID(Bits::low(gnb_id, length as usize)),
        None,
    ))
}

/// The kinds of ng-eNB ID of TS 38.413 clause 9.3.1.8, each of the bits that it has: the
/// leftmost bits of the 28 of the E-UTRA Cell Identity of each cell that the ng-eNB
/// serves.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NgEnbKind {
    /// A macro ng-eNB ID, of 20 bits, which leaves 8 bits for the cell.
    Macro,
    /// A short macro ng-eNB ID, of 18 bits, which leaves 10 bits for the cell.
    ShortMacro,
    /// A long macro ng-eNB ID, of 21 bits, which leaves 7 bits for the cell.
    LongMacro,
}

impl NgEnbKind {
    /// The bits of an ng-eNB ID of this kind.
    pub const fn bits(self) -> u32 {
        match self {
            Self::Macro => 20,
            Self::ShortMacro => 18,
            Self::LongMacro => 21,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Macro => "the macro ng-eNB ID",
            Self::ShortMacro => "the short macro ng-eNB ID",
            Self::LongMacro => "the long macro ng-eNB ID",
        }
    }
}

/// Build a `GlobalNgENBID` for an ng-eNB ID of a kind (TS 38.413 clause 9.3.1.8).
///
/// An error of the kind `Argument` for an ng-eNB ID that does not fit in the bits of its
/// kind.
pub fn global_ng_enb_id(
    plmn_identity: PLMNIdentity,
    kind: NgEnbKind,
    ng_enb_id: u32,
) -> Result<GlobalNgENBID, Error> {
    let bits = kind.bits();
    let id = Bits::low(fits(kind.name(), ng_enb_id.into(), bits)?, bits as usize);
    let id = match kind {
        NgEnbKind::Macro => NgENBID::macroNgENB_ID(id),
        NgEnbKind::ShortMacro => NgENBID::shortMacroNgENB_ID(id),
        NgEnbKind::LongMacro => NgENBID::longMacroNgENB_ID(id),
    };
    Ok(GlobalNgENBID::new(plmn_identity, id, None))
}

/// Build an E-UTRA Cell Global Identifier from an ng-eNB ID of a kind and the cell that
/// the rest of the 28 bits of the E-UTRA Cell Identity hold: the ng-eNB ID is its
/// leftmost bits (TS 38.413 clauses 9.3.1.8 and 9.3.1.9).
///
/// An error of the kind `Argument` for an ng-eNB ID that does not fit in the bits of its
/// kind, and for a cell that does not fit in the bits after it.
pub fn eutra_cgi(
    plmn_identity: PLMNIdentity,
    kind: NgEnbKind,
    ng_enb_id: u32,
    cell_id: u16,
) -> Result<EUTRACGI, Error> {
    let bits = kind.bits();
    let ng_enb_id = fits(kind.name(), ng_enb_id.into(), bits)?;
    let cell_id = fits("the cell of the ng-eNB", cell_id.into(), 28 - bits)?;
    let eci = (ng_enb_id << (28 - bits)) | cell_id;
    Ok(EUTRACGI::new(
        plmn_identity,
        EUTRACellIdentity(Bits::low(eci, 28)),
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

    /// Each kind of ng-eNB ID of TS 38.413 clause 9.3.1.8 has its bits, the leftmost of
    /// the E-UTRA Cell Identity, and the cell the rest of the 28.
    #[test]
    fn each_kind_of_ng_enb_id_has_its_bits_and_the_cell_the_rest() {
        let network = plmn("208", "93").unwrap();
        for (kind, bits) in [
            (NgEnbKind::Macro, 20),
            (NgEnbKind::ShortMacro, 18),
            (NgEnbKind::LongMacro, 21),
        ] {
            assert_eq!(kind.bits(), bits);
            let most = (1u32 << bits) - 1;
            let global = global_ng_enb_id(network, kind, most).unwrap();
            let id = match global.ng_enb_id {
                NgENBID::macroNgENB_ID(id) => (NgEnbKind::Macro, id),
                NgENBID::shortMacroNgENB_ID(id) => (NgEnbKind::ShortMacro, id),
                NgENBID::longMacroNgENB_ID(id) => (NgEnbKind::LongMacro, id),
                NgENBID::choice_Extensions(_) => unreachable!(),
            };
            assert_eq!(
                (id.0, id.1.len(), id.1.value()),
                (kind, bits as usize, u64::from(most))
            );
            assert!(global_ng_enb_id(network, kind, most + 1).is_err());
            // The ng-eNB ID, then the cell.
            let cell = (1u16 << (28 - bits)) - 1;
            let cgi = eutra_cgi(network, kind, 1, cell).unwrap();
            assert_eq!(
                cgi.eutra_cell_identity.0.value(),
                (1 << (28 - bits)) | u64::from(cell)
            );
            assert!(eutra_cgi(network, kind, 1, cell + 1).is_err());
            assert!(eutra_cgi(network, kind, most + 1, 0).is_err());
        }
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
        let cgi = nr_cgi(plmn, 0xab_cdef, 24, 0x123).unwrap();
        assert_eq!(
            cgi.nr_cell_identity.0,
            Bits::new(0xa_bcde_f123, 36).unwrap()
        );
        assert_eq!(
            oxirush_aper::encode(&cgi).unwrap(),
            [0x00, 0x02, 0xf8, 0x39, 0xab, 0xcd, 0xef, 0x12, 0x30]
        );
        let gnb = global_gnb_id(plmn, 0xab_cdef, 24).unwrap();
        let GNBID::gNB_ID(bits) = gnb.gnb_id else {
            panic!("expected gNB ID");
        };
        assert_eq!(bits, Bits::new(0xab_cdef, 24).unwrap());
        // The gNB ID is the leftmost bits whatever its length: 22 leave 14 for the cell,
        // 32 leave 4.
        let short = nr_cgi(plmn, 0x3f_ffff, 22, 0x2abc).unwrap();
        assert_eq!(
            short.nr_cell_identity.0,
            Bits::new(0x3f_ffff << 14 | 0x2abc, 36).unwrap()
        );
        let long = nr_cgi(plmn, 0xffff_ffff, 32, 0xa).unwrap();
        assert_eq!(
            long.nr_cell_identity.0,
            Bits::new(0xf_ffff_fffa, 36).unwrap()
        );
        let GNBID::gNB_ID(bits) = global_gnb_id(plmn, 1, 22).unwrap().gnb_id else {
            panic!("expected gNB ID");
        };
        assert_eq!((bits.value(), bits.len()), (1, 22));
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
        assert!(global_gnb_id(network(), 0xff_ffff, 24).is_ok());
        assert_eq!(
            argument(global_gnb_id(network(), 0x100_0000, 24).unwrap_err()),
            "the gNB ID 16777216 does not fit in 24 bits"
        );
        assert!(nr_cgi(network(), 0xff_ffff, 24, 0xfff).is_ok());
        assert_eq!(
            argument(nr_cgi(network(), 0x100_0000, 24, 1).unwrap_err()),
            "the gNB ID 16777216 does not fit in 24 bits"
        );
        assert_eq!(
            argument(nr_cgi(network(), 1, 24, 0x1000).unwrap_err()),
            "the cell of the gNB 4096 does not fit in 12 bits"
        );
        assert_eq!(
            argument(nr_cgi(network(), 1, 32, 0x10).unwrap_err()),
            "the cell of the gNB 16 does not fit in 4 bits"
        );
        // A length that a gNB ID does not have.
        for length in [0, 21, 33] {
            assert_eq!(
                argument(global_gnb_id(network(), 1, length).unwrap_err()),
                format!("a gNB ID has 22 to 32 bits, and {length} are given")
            );
            assert!(nr_cgi(network(), 1, length, 1).is_err());
        }
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
