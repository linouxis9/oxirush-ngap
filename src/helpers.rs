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
//! use oxirush_ngap::{Bits, ngap::NgENBID};
//!
//! // A PLMN from its MCC and its MNC, and what is identified within it
//! let network = plmn("208", "93").unwrap();
//! let amf = guami(network, 1, 1, 0).unwrap(); // region, set, pointer
//! let tracking_area = tai(network, [0x00, 0x00, 0x01]); // TAC
//! let cell = nr_cgi(network, 0x123456, 24, 1).unwrap(); // gNB of 24 bits, cell
//! let gnb = global_gnb_id(network, 0x123456, 24).unwrap();
//! let ng_enb = NgENBID::macroNgENB_ID(Bits::new(0x12345, 20).unwrap());
//! let lte_cell = eutra_cgi(network, &ng_enb, 7).unwrap(); // ng-eNB, cell
//!
//! // A bit string from octets, and the NR algorithms of a UE: the first
//! // octet of its encryption bitmap, then of its integrity bitmap
//! let algorithm_mask = oxirush_ngap::BitString::from([0xe0]);
//! let capabilities = ue_security_capabilities(0xe0, 0xe0);
//!
//! // What is no MCC, and an AMF Set ID that does not fit in its 10 bits
//! println!("{}", plmn("20", "93").unwrap_err());
//! println!("{}", guami(network, 1, 1024, 0).unwrap_err());
//! # assert_eq!(printed, r##"`mcc` has 2 digits, and must have 3
//! # the AMF Set ID 1024 does not fit in 10 bits
//! # "##);
//! ```
//!
//! ```text
//! `mcc` has 2 digits, and must have 3
//! the AMF Set ID 1024 does not fit in 10 bits
//! ```
//!
//! A PLMN identity has its digits in the order of TS 38.413 clause 9.3.3.5:
//! `plmn("310", "410")` is the octets `13 40 01`, where a NAS message has
//! `13 00 14`. The `Plmn` of [`identity`](crate::identity) reads the MCC and the
//! MNC of a PLMN identity. A gNB ID has the length that it is given, 22 to 32 bits
//! (clause 9.3.1.6), and an ng-eNB ID that of its alternative (clause 9.3.1.8).

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

/// The PLMN identity of an MCC and an MNC, in the order of TS 38.413 clause 9.3.3.5:
/// `plmn("310", "410")` is `13 40 01`, and `plmn("208", "93")` is `02 f8 39`. The
/// `Plmn` of oxirush-utils converts to and from a `PLMNIdentity`, and reads its MCC and
/// its MNC.
///
/// An error of the kind `Argument` unless `mcc` is three decimal digits and `mnc` two
/// or three.
///
/// ```
/// use oxirush_ngap::helpers::plmn;
/// use oxirush_ngap::identity::Plmn;
///
/// let network = plmn("310", "410")?;
/// assert_eq!(network.0, [0x13, 0x40, 0x01]);
/// let read = Plmn::try_from(network).unwrap();
/// assert_eq!((read.mcc(), read.mnc()), ("310", "410"));
/// # Ok::<(), oxirush_ngap::Error>(())
/// ```
pub fn plmn(mcc: &str, mnc: &str) -> Result<PLMNIdentity, Error> {
    let plmn = Plmn::new(mcc, mnc).map_err(|error| Error::argument(error.to_string()))?;
    Ok(PLMNIdentity::from(plmn))
}

/// Build a Globally Unique AMF Identifier.
///
/// An error of the kind `Argument` for an AMF Set ID that does not fit in 10 bits and
/// for an AMF Pointer that does not fit in 6 (TS 38.413 clauses 9.3.3.12 and 9.3.3.19).
///
/// ```
/// use oxirush_ngap::helpers::{guami, plmn};
///
/// // AMF Region 1, AMF Set 1 and AMF Pointer 0, in PLMN 208-93
/// let amf = guami(plmn("208", "93")?, 1, 1, 0)?;
/// assert_eq!(amf.amf_region_id.0.value(), 1);
/// assert_eq!((amf.amf_set_id.0.value(), amf.amf_set_id.0.len()), (1, 10));
/// assert_eq!((amf.amf_pointer.0.value(), amf.amf_pointer.0.len()), (0, 6));
///
/// // An AMF Set ID of 11 bits
/// let error = guami(plmn("208", "93")?, 1, 1024, 0).unwrap_err();
/// assert_eq!(error.to_string(), "the AMF Set ID 1024 does not fit in 10 bits");
/// # Ok::<(), oxirush_ngap::Error>(())
/// ```
pub fn guami(
    plmn_identity: PLMNIdentity,
    region_id: u8,
    set_id: u16,
    pointer: u8,
) -> Result<GUAMI, Error> {
    let set_id = fits("the AMF Set ID", set_id.into(), 10)?;
    let pointer = fits("the AMF Pointer", pointer.into(), 6)?;
    Ok(GUAMI {
        plmn_identity,
        amf_region_id: AMFRegionID(Bits::from(region_id)),
        amf_set_id: AMFSetID(Bits::low(set_id, 10)),
        amf_pointer: AMFPointer(Bits::low(pointer, 6)),
        ie_extensions: None,
    })
}

/// Build a 5G Tracking Area Identity: a PLMN and the three octets of a TAC.
///
/// ```
/// use oxirush_ngap::helpers::{plmn, tai};
///
/// // TAC 1 of PLMN 208-93
/// let area = tai(plmn("208", "93")?, [0x00, 0x00, 0x01]);
/// assert_eq!(area.plmn_identity.0, [0x02, 0xf8, 0x39]);
/// assert_eq!(area.tac.0, [0x00, 0x00, 0x01]);
/// # Ok::<(), oxirush_ngap::Error>(())
/// ```
pub fn tai(plmn_identity: PLMNIdentity, tac: [u8; 3]) -> TAI {
    TAI {
        plmn_identity,
        tac: TAC(tac),
        ie_extensions: None,
    }
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
///
/// ```
/// use oxirush_ngap::helpers::{nr_cgi, plmn};
///
/// // Cell 1 of the gNB 0x123456, whose ID has 24 bits: 12 bits are left for the cell
/// let cell = nr_cgi(plmn("208", "93")?, 0x123456, 24, 1)?;
/// assert_eq!(cell.nr_cell_identity.0.value(), 0x1234_5600_1);
///
/// // A cell of 13 bits beside a gNB ID of 24
/// let error = nr_cgi(plmn("208", "93")?, 0x123456, 24, 0x1000).unwrap_err();
/// assert_eq!(error.to_string(), "the cell of the gNB 4096 does not fit in 12 bits");
/// # Ok::<(), oxirush_ngap::Error>(())
/// ```
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
    Ok(NRCGI {
        plmn_identity,
        nr_cell_identity: NRCellIdentity(Bits::low(nci, 36)),
        ie_extensions: None,
    })
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
    Ok(GlobalGNBID {
        plmn_identity,
        gnb_id: GNBID::gNB_ID(Bits::low(gnb_id, length as usize)),
        ie_extensions: None,
    })
}

/// Build an E-UTRA Cell Global Identifier from an ng-eNB ID and the cell that the rest
/// of the 28 bits of the E-UTRA Cell Identity hold: the ng-eNB ID is its leftmost bits,
/// 20, 18 or 21 as its alternative has them (TS 38.413 clauses 9.3.1.8 and 9.3.1.9).
///
/// An error of the kind `Argument` for a cell that does not fit in the bits after the
/// ng-eNB ID, and for an ng-eNB ID of an extension.
///
/// ```
/// use oxirush_ngap::helpers::{eutra_cgi, plmn};
/// use oxirush_ngap::{Bits, ngap::NgENBID};
///
/// // Cell 7 of the macro ng-eNB 0x12345, of 20 bits: 8 bits are left for the cell
/// let ng_enb = NgENBID::macroNgENB_ID(Bits::new(0x12345, 20)?);
/// let cell = eutra_cgi(plmn("208", "93")?, &ng_enb, 7)?;
/// assert_eq!(cell.eutra_cell_identity.0.value(), 0x1234_507);
///
/// // A cell of 9 bits beside a macro ng-eNB ID
/// let error = eutra_cgi(plmn("208", "93")?, &ng_enb, 0x100).unwrap_err();
/// assert_eq!(error.to_string(), "the cell of the ng-eNB 256 does not fit in 8 bits");
/// # Ok::<(), oxirush_ngap::Error>(())
/// ```
pub fn eutra_cgi(
    plmn_identity: PLMNIdentity,
    ng_enb_id: &NgENBID,
    cell_id: u16,
) -> Result<EUTRACGI, Error> {
    let id = match ng_enb_id {
        NgENBID::macroNgENB_ID(id)
        | NgENBID::shortMacroNgENB_ID(id)
        | NgENBID::longMacroNgENB_ID(id) => id,
        NgENBID::choice_Extensions(_) => {
            return Err(Error::argument(
                "an ng-eNB ID of an extension has no bits of an E-UTRA Cell Identity",
            ));
        }
    };
    let bits = id.len() as u32;
    let cell_id = fits("the cell of the ng-eNB", cell_id.into(), 28 - bits)?;
    let eci = (id.value() << (28 - bits)) | cell_id;
    Ok(EUTRACGI {
        plmn_identity,
        eutra_cell_identity: EUTRACellIdentity(Bits::low(eci, 28)),
        ie_extensions: None,
    })
}

/// Build Single Network Slice Selection Assistance Information: a Slice/Service Type,
/// and a Slice Differentiator when it has one (TS 38.413 clause 9.3.1.24).
///
/// ```
/// use oxirush_ngap::helpers::s_nssai;
///
/// // The slice of SST 1, of the Slice Differentiator 0000ff, and the one without
/// let slice = s_nssai(1, Some([0x00, 0x00, 0xff]));
/// assert_eq!((slice.sst.0, slice.sd.map(|sd| sd.0)), ([1], Some([0x00, 0x00, 0xff])));
/// assert_eq!(s_nssai(1, None).sd, None);
/// ```
pub fn s_nssai(sst: u8, sd: Option<[u8; 3]>) -> SNSSAI {
    SNSSAI {
        sst: SST([sst]),
        sd: sd.map(SD),
        ie_extensions: None,
    }
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
    UESecurityCapabilities {
        nr_encryption_algorithms: NRencryptionAlgorithms(algorithms(encryption)),
        nr_integrity_protection_algorithms: NRintegrityProtectionAlgorithms(algorithms(integrity)),
        eutra_encryption_algorithms: EUTRAencryptionAlgorithms(algorithms(0)),
        eutra_integrity_protection_algorithms: EUTRAintegrityProtectionAlgorithms(algorithms(0)),
        ie_extensions: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ErrorKind, build_ngap};

    #[test]
    fn a_plmn_identity_has_the_order_of_the_application_protocols() {
        let read = |identity: PLMNIdentity| {
            let plmn = Plmn::try_from(identity).unwrap();
            (plmn.mcc().to_string(), plmn.mnc().to_string())
        };
        let identity = plmn("208", "93").unwrap();
        assert_eq!(identity.0, [0x02, 0xF8, 0x39]);
        assert_eq!(read(identity), ("208".into(), "93".into()));
        // The digits 9 9 9 0 7 0, two in each octet, the second over the first.
        let identity = plmn("999", "070").unwrap();
        assert_eq!(identity.0, [0x99, 0x09, 0x07]);
        assert_eq!(read(identity), ("999".into(), "070".into()));
        // What is no MCC or no MNC is an error of the argument.
        for (mcc, mnc) in [("20", "93"), ("208", "9300"), ("20a", "93"), ("", "")] {
            let error = plmn(mcc, mnc).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Argument, "{mcc}-{mnc}");
        }
    }

    /// Each alternative of an ng-eNB ID of TS 38.413 clause 9.3.1.8 has its bits, the
    /// leftmost of the E-UTRA Cell Identity, and the cell the rest of the 28.
    #[test]
    fn each_ng_enb_id_has_its_bits_and_the_cell_the_rest() {
        let network = plmn("208", "93").unwrap();
        for (ng_enb, bits) in [
            (NgENBID::macroNgENB_ID as fn(Bits) -> NgENBID, 20),
            (NgENBID::shortMacroNgENB_ID, 18),
            (NgENBID::longMacroNgENB_ID, 21),
        ] {
            let id = ng_enb(Bits::new(1, bits).unwrap());
            let cell = (1u16 << (28 - bits)) - 1;
            let cgi = eutra_cgi(network, &id, cell).unwrap();
            assert_eq!(
                cgi.eutra_cell_identity.0.value(),
                (1 << (28 - bits)) | u64::from(cell)
            );
            assert_eq!(
                eutra_cgi(network, &id, cell + 1).unwrap_err().to_string(),
                format!(
                    "the cell of the ng-eNB {} does not fit in {} bits",
                    cell + 1,
                    28 - bits
                )
            );
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
        let pdu = build_ngap!(IGNORE UEContextReleaseRequest,
            REJECT AMF_UE_NGAP_ID(1u64),
            REJECT RAN_UE_NGAP_ID(7u32),
            IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
        )
        .expect("build PDU");
        let bytes = pdu.encode().expect("encode PDU");
        let decoded = NGAPPDU::decode(&bytes).expect("decode PDU");
        assert_eq!(pdu, decoded);
        assert_eq!(decoded.kind(), crate::NgapPduKind::UEContextReleaseRequest);
        assert_eq!(decoded.direction(), crate::Direction::Initiating);
    }
}
