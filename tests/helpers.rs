//! The helpers that build the values that a program of NGAP writes most from their
//! parts, and what they refuse. TOY, the protocol of the tests of oxirush-aper, has the
//! tests of how an identity prints, which the helpers give.

use oxirush_ngap::ErrorKind;
use oxirush_ngap::aper::{BitString, Bits};
use oxirush_ngap::helpers::*;
use oxirush_ngap::identity::Plmn;
use oxirush_ngap::ngap::*;
use oxirush_ngap::{Error, build_ngap};

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

/// TS 38.413 clause 9.3.1.56: ENUMERATED (1s, 2s, 5s, 10s, 20s, 60s, ...).
#[test]
fn a_time_to_wait_is_its_seconds() {
    let seconds = [
        TimeToWait::v1s,
        TimeToWait::v2s,
        TimeToWait::v5s,
        TimeToWait::v10s,
        TimeToWait::v20s,
        TimeToWait::v60s,
    ]
    .map(TimeToWait::seconds);
    assert_eq!(seconds, [1, 2, 5, 10, 20, 60]);
}

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
    assert_eq!(
        decoded.kind(),
        oxirush_ngap::NgapPduKind::UEContextReleaseRequest
    );
    assert_eq!(decoded.direction(), oxirush_ngap::Direction::Initiating);
}
