//! Convenience helpers for building common NGAP types.
//!
//! These helpers cover PLMN/TBCD encoding, NR identifiers, security bit
//! strings, and common AMF/gNB identities used by NGAP consumers, and the
//! seconds of a Time to Wait ([`TimeToWait::seconds`]).
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

use crate::ngap::*;
use crate::{Error, ErrorKind};

/// `value`, which is `what`, when it fits in `bits` bits.
fn fits(what: &str, value: u64, bits: u32) -> Result<u64, Error> {
    match bits >= 64 || value >> bits == 0 {
        true => Ok(value),
        false => Err(Error::new(
            ErrorKind::Argument,
            format!("{what} {value} does not fit in {bits} bits"),
        )),
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
    let plmn =
        Plmn::new(mcc, mnc).map_err(|error| Error::new(ErrorKind::Argument, error.to_string()))?;
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
        false => Err(Error::new(
            ErrorKind::Argument,
            format!("a gNB ID has 22 to 32 bits, and {length} are given"),
        )),
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
            return Err(Error::new(
                ErrorKind::Argument,
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

impl TimeToWait {
    /// The time that the node is to wait at least before it starts the procedure again,
    /// in seconds (TS 38.413 clause 9.3.1.56): that of an NG SETUP FAILURE, a RAN
    /// CONFIGURATION UPDATE FAILURE or an AMF CONFIGURATION UPDATE FAILURE.
    ///
    /// ```
    /// use oxirush_ngap::ngap::TimeToWait;
    /// use std::time::Duration;
    ///
    /// let wait = Duration::from_secs(TimeToWait::v10s.seconds());
    /// assert_eq!(wait, Duration::from_secs(10));
    /// ```
    pub fn seconds(self) -> u64 {
        match self {
            Self::v1s => 1,
            Self::v2s => 2,
            Self::v5s => 5,
            Self::v10s => 10,
            Self::v20s => 20,
            Self::v60s => 60,
        }
    }
}
