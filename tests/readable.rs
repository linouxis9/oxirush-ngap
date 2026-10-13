//! How an identity prints. It is printed whole, and as it is read: a PLMN identity as
//! its MCC and its MNC, and a fixed number of octets, as a TMSI or a TAC, in
//! hexadecimal, where `Debug` derived for a type gave a list of numbers.

use oxirush_ngap::helpers::{plmn, tai};
use oxirush_ngap::identity::Plmn;
use oxirush_ngap::ngap::*;

#[test]
fn a_plmn_identity_prints_as_its_mcc_and_its_mnc() {
    let identity = plmn("208", "93").unwrap();
    assert_eq!(
        format!("{identity} {identity:?}"),
        "208-93 PLMNIdentity(208-93)"
    );
    // An MNC of three digits, and one that starts with a zero
    assert_eq!(plmn("310", "410").unwrap().to_string(), "310-410");
    assert_eq!(plmn("208", "093").unwrap().to_string(), "208-093");

    // In a value that holds one
    let printed = format!("{:?}", tai(identity, [0, 0, 1]));
    assert!(printed.contains("PLMNIdentity(208-93)"), "{printed}");
    assert!(printed.contains("TAC(000001)"), "{printed}");

    // Octets that are no PLMN identity print as the octets that they are
    let none = PLMNIdentity([0xab, 0xcd, 0xef]);
    assert_eq!(format!("{none} {none:?}"), "abcdef PLMNIdentity(abcdef)");
}

/// TS 38.413 clause 9.3.3.5: octet 2 has the first digit of an MNC of three digits, and
/// octet 3 its second and its third.
#[test]
fn a_plmn_identity_converts_to_and_from_a_plmn() {
    let network: Plmn = "310-410".parse().unwrap();
    let identity = PLMNIdentity::from(network);
    assert_eq!(identity.0, [0x13, 0x40, 0x01]);
    assert_eq!(identity, plmn("310", "410").unwrap());
    assert_eq!(Plmn::try_from(identity), Ok(network));
    assert_eq!((network.mcc(), network.mnc()), ("310", "410"));

    // A filler in the MNC: no PLMN, and the error says the member
    let error = Plmn::try_from(PLMNIdentity([0x02, 0xf8, 0xf9])).unwrap_err();
    assert_eq!(error.member, "mnc");
}

#[test]
fn a_fixed_number_of_octets_prints_in_hexadecimal() {
    let tmsi = FiveGTMSI([0xde, 0xad, 0xbe, 0xef]);
    assert_eq!(format!("{tmsi} {tmsi:?}"), "deadbeef FiveGTMSI(deadbeef)");
    let tac = TAC([0x00, 0x00, 0x01]);
    assert_eq!(format!("{tac} {tac:?}"), "000001 TAC(000001)");
    assert_eq!(format!("{:?}", GTPTEID([0, 0, 0, 1])), "GTPTEID(00000001)");

    // A 5G-S-TMSI is printed whole, with its TMSI as it is read
    let identity = FiveGSTMSI::new(
        AMFSetID::new(1).unwrap(),
        AMFPointer::new(0).unwrap(),
        tmsi,
        None,
    );
    let printed = format!("{identity:?}");
    assert!(printed.contains("FiveGTMSI(deadbeef)"), "{printed}");
}
