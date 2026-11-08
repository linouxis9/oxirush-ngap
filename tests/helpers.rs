//! The helpers that build the values that a program of NGAP writes most from their
//! parts, and what they refuse. TOY, the protocol of the tests of oxirush-aper, has the
//! tests of how an identity prints, which the helpers give.

use oxirush_ngap::ErrorKind;
use oxirush_ngap::helpers::*;
use oxirush_ngap::identity::Plmn;
use oxirush_ngap::ngap::*;

#[test]
fn a_plmn_identity_of_the_helpers_is_the_one_of_its_digits() {
    let identity = plmn("208", "93").unwrap();
    assert_eq!(
        identity,
        PLMNIdentity::from("208-93".parse::<Plmn>().unwrap())
    );
    assert_eq!(format!("{identity}"), "208-93");
    // An MNC of three digits, and one that starts with a zero (TS 38.413 clause 9.3.3.5)
    assert_eq!(plmn("310", "410").unwrap().0, [0x13, 0x40, 0x01]);
    assert_eq!(plmn("208", "093").unwrap().to_string(), "208-093");
    // In a tracking area, and the 5G-S-TMSI of a UE, printed whole
    let printed = format!("{:?}", tai(identity, [0, 0, 1]));
    assert!(printed.contains("PLMNIdentity(208-93)"), "{printed}");
    assert!(printed.contains("TAC(000001)"), "{printed}");
    let identity = FiveGSTMSI {
        amf_set_id: AMFSetID::new(1).unwrap(),
        amf_pointer: AMFPointer::new(0).unwrap(),
        five_g_tmsi: FiveGTMSI([0xde, 0xad, 0xbe, 0xef]),
        ie_extensions: None,
    };
    let printed = format!("{identity:?}");
    assert!(printed.contains("FiveGTMSI(deadbeef)"), "{printed}");
}

#[test]
fn an_argument_that_a_helper_does_not_take_is_an_error_and_no_panic() {
    let network = || plmn("208", "93").unwrap();

    // What panicked: an MCC of two digits, and an MNC of one.
    let panicked = [plmn("20", "93").unwrap_err(), plmn("208", "9").unwrap_err()];
    // What was cut short: an AMF Set ID over 10 bits and an AMF Pointer over 6
    // (TS 38.413 clauses 9.3.3.12 and 9.3.3.19), a gNB ID over the bits that it is given,
    // 22 to 32 (clause 9.3.1.6), a cell over the bits that follow them in an NR Cell
    // Identity of 36 (clause 9.3.1.7).
    let cut_short = [
        guami(network(), 1, 2000, 0).unwrap_err(),
        guami(network(), 1, 1, 70).unwrap_err(),
        global_gnb_id(network(), 0x1ff_ffff, 24).unwrap_err(),
        global_gnb_id(network(), 1, 33).unwrap_err(),
        nr_cgi(network(), 0x1ff_ffff, 24, 1).unwrap_err(),
        nr_cgi(network(), 1, 24, 0x1000).unwrap_err(),
    ];
    for error in panicked.iter().chain(&cut_short) {
        assert_eq!(error.kind(), ErrorKind::Argument, "{error}");
        assert!(std::error::Error::source(error).is_none());
    }
    assert_eq!(
        cut_short[0].to_string(),
        "the AMF Set ID 2000 does not fit in 10 bits"
    );

    // A TAC is three octets (clause 9.3.3.10), which the type of the argument says.
    let area = tai(network(), [0, 0, 1]);
    assert_eq!(area.tac.0[..], [0, 0, 1]);
}
