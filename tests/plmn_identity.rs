//! The octets of a PLMN identity are those of TS 38.413 clause 9.3.3.5, and another
//! decoder reads them as the same network.
//!
//! The clause has two digits in each octet, "bits 4 to 1 of octet n encoding digit 2n-1"
//! and "bits 8 to 5 of octet n encoding digit 2n", of "3 digits from MCC followed by
//! either a filler digit plus 2 digits from MNC (in case of 2 digit MNC) or 3 digits
//! from MNC (in case of 3 digit MNC)", with 1111 as the filler. The octets of
//! [`IDENTITIES`] and of [`PAGING`] are written out from it by hand: none of them comes
//! from the code that they test.
//!
//! tshark 4.6.4 was given [`PAGING`] and printed [`TSHARK`] for it. The test that runs
//! tshark again is ignored, as tshark may not be installed:
//!
//! ```sh
//! cargo test --test plmn_identity -- --ignored
//! ```
//!
//! It needs `text2pcap` and `tshark` of Wireshark, and does what these commands do, with
//! `0000`, a space and the octets of the PDU in hexadecimal, a space between two, as the
//! one line of `pdu.txt`. 60 is the SCTP payload protocol identifier of NGAP.
//!
//! ```sh
//! text2pcap -q -S 38412,38412,60 pdu.txt pdu.pcap
//! tshark -r pdu.pcap -V | grep -E 'pLMNIdentity:|Mobile (Country|Network) Code'
//! ```

use oxirush_ngap::helpers::{plmn, plmn_from, tai};
use oxirush_ngap::ngap::*;
use oxirush_ngap::{Error, build_ngap_ie, extract_ngap_ies};

/// An MCC, an MNC and the octets of their PLMN identity, digit by digit from the clause.
const IDENTITIES: &[(&str, &str, [u8; 3])] = &[
    // The digits 2 0 8, a filler, 9 3: octet 1 has 0 over 2, octet 2 the filler over 8,
    // octet 3 has 3 over 9.
    ("208", "93", [0x02, 0xf8, 0x39]),
    // The digits 0 0 1, a filler, 0 1.
    ("001", "01", [0x00, 0xf1, 0x10]),
    // The digits 3 1 0 4 1 0: octet 2 has the first digit of the MNC over the last of
    // the MCC, and octet 3 the last digit of the MNC over its second.
    ("310", "410", [0x13, 0x40, 0x01]),
    // The digits 1 2 3 4 5 6, two in each octet, the second over the first.
    ("123", "456", [0x21, 0x43, 0x65]),
    // The digits 9 9 9 0 7 0.
    ("999", "070", [0x99, 0x09, 0x07]),
];

/// A PAGING for the tracking area 1 of 208-93 and the tracking area 2 of 310-410, whose
/// PLMN identities are the octets `02 f8 39` and `13 40 01`.
const PAGING: &str = "0018402000000200734007000840112233440067400e1002f83900000100134001000002";

/// What tshark 4.6.4 printed of the PLMN identities of [`PAGING`].
const TSHARK: &str = "\
pLMNIdentity: 02f839
Mobile Country Code (MCC): France (208)
Mobile Network Code (MNC): Thales communications & Security (93)
pLMNIdentity: 134001
Mobile Country Code (MCC): United States (310)
Mobile Network Code (MNC): AT&T Mobility (410)
";

/// What tshark 4.6.4 printed of the same message with the octets `13 00 14` in the place
/// of `13 40 01`: the order of TS 24.008, which a NAS message has and which this crate
/// wrote for 310-410, is another network to a decoder of NGAP.
const TSHARK_OF_THE_NAS_ORDER: &str = "\
pLMNIdentity: 02f839
Mobile Country Code (MCC): France (208)
Mobile Network Code (MNC): Thales communications & Security (93)
pLMNIdentity: 130014
Mobile Country Code (MCC): United States (310)
Mobile Network Code (MNC): Unknown (041)
";

/// The MCC and the MNC of each PLMN identity that tshark printed: the digits in the
/// parentheses that end its lines, two lines for each identity.
fn networks(printed: &str) -> Vec<(String, String)> {
    let codes = printed.lines().filter(|line| line.contains("Mobile "));
    let codes = codes.map(|line| {
        let code = line.rsplit('(').next().unwrap_or_default();
        code.trim_end().trim_end_matches(')').to_string()
    });
    let codes: Vec<String> = codes.collect();
    let pairs = codes.chunks(2);
    pairs
        .map(|pair| (pair[0].clone(), pair[1].clone()))
        .collect()
}

/// The tracking areas that [`PAGING`] is for, built with the helpers.
fn tracking_areas() -> TAIListForPaging<'static> {
    let areas = [("208", "93", [0, 0, 1]), ("310", "410", [0, 0, 2])];
    let items = areas
        .map(|(mcc, mnc, tac)| TAIListForPagingItem::new(tai(plmn(mcc, mnc).unwrap(), tac), None));
    TAIListForPaging(items.to_vec())
}

#[test]
fn the_octets_of_a_plmn_identity_are_those_of_the_clause() {
    for (mcc, mnc, octets) in IDENTITIES {
        let identity = plmn(mcc, mnc).unwrap();
        assert_eq!(&identity.0[..], octets, "{mcc}-{mnc}");
        let read = plmn_from(&PLMNIdentity::from(*octets));
        let digits = (mcc.to_string(), mnc.to_string());
        assert_eq!(read, Some(digits), "{octets:02x?}");
    }
}

#[test]
fn the_identities_of_a_paging_are_those_that_tshark_reads() -> Result<(), Error> {
    // What tshark printed names the two networks, in the order of the message.
    let read = networks(TSHARK);
    assert_eq!(
        read,
        [("208".into(), "93".into()), ("310".into(), "410".into())]
    );

    // The helpers read the same networks in the same octets.
    let pdu = NGAP_PDU::decode(&hex::decode(PAGING).unwrap()).expect("decode PDU");
    let paging: Paging = pdu.decode_value().expect("decode message");
    extract_ngap_ies!(&paging, Paging,
        req areas: TAIListForPaging = TAIListForPaging(areas) => areas,
    );
    let identities = areas
        .0
        .iter()
        .map(|area| plmn_from(&area.t_ai.p_lmnidentity));
    assert_eq!(identities.collect::<Option<Vec<_>>>(), Some(read));

    // And write the octets of the message for them.
    assert_eq!(areas, tracking_areas());
    let written = build_ngap_ie!(Paging, IGNORE TAIListForPaging(tracking_areas())).unwrap();
    assert_eq!(written, paging.protocol_ies.0[1]);
    assert_eq!(hex::encode(pdu.encode().unwrap()), PAGING);
    Ok(())
}

#[test]
fn the_order_of_a_nas_message_is_another_network() {
    // The octets that TS 24.008 has for 310-410 are those of 310-041 here, as tshark
    // reads them.
    let read = networks(TSHARK_OF_THE_NAS_ORDER);
    assert_eq!(
        read,
        [("208".into(), "93".into()), ("310".into(), "041".into())]
    );
    let identity = PLMNIdentity::from([0x13, 0x00, 0x14]);
    assert_eq!(plmn_from(&identity), Some(read[1].clone()));
    assert_eq!(&plmn("310", "041").unwrap().0[..], [0x13, 0x00, 0x14]);
}

/// Octets that are no PLMN identity have no MCC and no MNC: a half octet that is no
/// digit, where it is not the filler before an MNC of two digits.
#[test]
fn octets_that_are_no_plmn_identity_are_refused() {
    for octets in [
        // No digit at all.
        [0xab, 0xcd, 0xef],
        // A filler in the MNC, in the MCC, and before an MNC that has another one.
        [0x02, 0xf8, 0xf9],
        [0x0f, 0xf8, 0x39],
        [0x02, 0xf8, 0x3f],
        // A half octet over 9.
        [0x02, 0xa8, 0x39],
    ] {
        let identity = PLMNIdentity::from(octets);
        assert_eq!(plmn_from(&identity), None, "{octets:02x?}");
    }
}

/// With the `inspect` feature such octets are shown as they are, in hexadecimal, and
/// are the same octets again: they are not shown as an MCC and an MNC that they are not.
#[cfg(feature = "inspect")]
#[test]
fn octets_that_are_no_plmn_identity_are_shown_as_octets() {
    use oxirush_ngap::inspect;
    use serde_json::json;

    for octets in ["abcdef", "02f8f9", "0ff839", "02f83f", "02a839"] {
        let wire = hex::decode(PAGING.replacen("134001", octets, 1)).unwrap();
        let pdu = NGAP_PDU::decode(&wire).unwrap();
        let view = pdu.to_view().unwrap();
        let shown = inspect::select(&view, "/ngap/TAIListForPaging/1/tAI/pLMNIdentity").unwrap();
        assert_eq!(shown, [&json!(octets.to_uppercase())]);
        let tree = pdu.to_tree().unwrap();
        assert_eq!(
            tree["ies"][1]["TAIListForPaging"][1]["tAI"]["pLMNIdentity"],
            octets.to_uppercase()
        );
        assert_eq!(NGAP_PDU::from_view(&view).unwrap().encode().unwrap(), wire);
        assert_eq!(NGAP_PDU::from_tree(&tree).unwrap().encode().unwrap(), wire);
    }
}

/// With the `inspect` feature a PLMN identity is written and shown as its MCC and its
/// MNC, `"310-410"`, in a view and in a tree: the same octets.
#[cfg(feature = "inspect")]
#[test]
fn the_notation_of_a_plmn_identity_has_the_octets_of_the_clause() {
    use oxirush_ngap::inspect;
    use serde_json::json;

    // Shown: the tree and the view of the PAGING name the networks that tshark reads.
    let wire = hex::decode(PAGING).unwrap();
    let pdu = NGAP_PDU::decode(&wire).unwrap();
    let tree = pdu.to_tree().unwrap();
    let areas = &tree["ies"][1]["TAIListForPaging"];
    assert_eq!(areas[0]["tAI"]["pLMNIdentity"], "208-93");
    assert_eq!(areas[1]["tAI"]["pLMNIdentity"], "310-410");
    let mut view = pdu.to_view().unwrap();
    let identities = "/ngap/TAIListForPaging/value/*/tAI/pLMNIdentity";
    let shown = inspect::select(&view, identities).unwrap();
    assert_eq!(shown, [&json!("208-93"), &json!("310-410")]);

    // Written: the tree is the octets of the message, and so is each identity of the
    // vectors, set through the view.
    assert_eq!(NGAP_PDU::from_tree(&tree).unwrap().encode().unwrap(), wire);
    for (mcc, mnc, octets) in IDENTITIES {
        let first = "/ngap/TAIListForPaging/0/tAI/pLMNIdentity";
        inspect::set(&mut view, first, json!(format!("{mcc}-{mnc}"))).unwrap();
        let edited = NGAP_PDU::from_view(&view).unwrap();
        let paging: Paging = edited.decode_value().unwrap();
        let areas: TAIListForPaging = decode_open_type(&paging.protocol_ies.0[1].value).unwrap();
        assert_eq!(&areas.0[0].t_ai.p_lmnidentity.0[..], octets, "{mcc}-{mnc}");
        let shown = edited.to_view().unwrap();
        let shown = inspect::select(&shown, first).unwrap();
        assert_eq!(shown, [&json!(format!("{mcc}-{mnc}"))]);
    }
}

/// Run tshark on [`PAGING`], and on the same message with the octets of the order of a
/// NAS message: it prints the networks that the tests above have from its recorded
/// output.
#[test]
#[ignore = "needs text2pcap and tshark of Wireshark"]
fn tshark_reads_the_identities_of_the_paging() {
    let tshark = |pdu: &str| -> String {
        let directory = std::env::temp_dir().join(format!("plmn-identity-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let (text, capture) = (directory.join("pdu.txt"), directory.join("pdu.pcap"));
        let octets: Vec<&str> = (0..pdu.len() / 2)
            .map(|at| &pdu[2 * at..2 * at + 2])
            .collect();
        std::fs::write(&text, format!("0000 {}\n", octets.join(" "))).unwrap();
        let converted = std::process::Command::new("text2pcap")
            .args(["-q", "-S", "38412,38412,60"])
            .args([&text, &capture])
            .status();
        assert!(converted.expect("text2pcap runs").success());
        let read = std::process::Command::new("tshark")
            .arg("-r")
            .arg(&capture)
            .arg("-V")
            .output();
        let read = read.expect("tshark runs");
        std::fs::remove_dir_all(&directory).unwrap();
        String::from_utf8(read.stdout).unwrap()
    };
    assert_eq!(networks(&tshark(PAGING)), networks(TSHARK));
    let nas_order = PAGING.replace("134001", "130014");
    assert_eq!(
        networks(&tshark(&nas_order)),
        networks(TSHARK_OF_THE_NAS_ORDER)
    );
}
