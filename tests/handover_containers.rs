#![cfg(feature = "inspect")]
//! The handover containers of S1AP in the messages of a handover from 5GS to EPS (TS
//! 38.413 clauses 9.3.1.20 and 9.3.1.21, TS 23.502 clause 4.11.1.2.1): the source gNB
//! sends the Source eNB to Target eNB Transparent Container of TS 36.413 in the HANDOVER
//! REQUIRED, and the AMF the Target eNB to Source eNB Transparent Container in the
//! HANDOVER COMMAND. Once the program registers the tables of oxirush-s1ap, a tree
//! writes each by the name of its type and the names of S1AP, and a view opens, reads
//! and edits it: S1AP encodes it as it encodes the container of its own HANDOVER
//! REQUEST.

use oxirush_ngap::inspect::Value;
use oxirush_ngap::{inspect, ngap::NGAPPDU, tree};
use oxirush_s1ap::s1ap::S1APPDU;

const SOURCE: &str = "SourceeNB-ToTargeteNB-TransparentContainer";
const TARGET: &str = "TargeteNB-ToSourceeNB-TransparentContainer";

/// The container of the source eNB: an RRC container, the target cell, and the cell
/// that the UE stayed in.
fn source() -> Value {
    let cell = tree!({"pLMNidentity": "208-93", "cell-ID": 0x1234567});
    tree!({
        "rRC-Container": "0102",
        "targetCell-ID": (cell.clone()),
        "uE-HistoryInformation": [{"e-UTRAN-Cell": {
            "global-Cell-ID": (cell),
            "cellType": {"cell-Size": "small"},
            "time-UE-StayedInCell": 30,
        }}],
    })
}

/// The octets that oxirush-s1ap writes for the container `name` with the value `value`,
/// in the IE of its own HANDOVER REQUEST or HANDOVER REQUEST ACKNOWLEDGE.
fn by_s1ap(message: &str, ie: &str, name: &str, value: Value) -> Value {
    let pdu = S1APPDU::from_tree(&tree!({"message": message, "ies": [
        {"MME-UE-S1AP-ID": 1},
        {(ie): {(name): (value)}},
    ]}))
    .unwrap();
    pdu.to_tree()["ies"][1][ie].clone()
}

/// The octets of the IE `ie` of a tree of NGAP.
fn octets(pdu: &NGAPPDU<'_>, ie: &str) -> Value {
    let tree = pdu.to_tree();
    let ies = tree["ies"].as_array().unwrap();
    let found = ies.iter().find(|entry| entry.get(ie).is_some());
    found.unwrap()[ie].clone()
}

#[test]
fn the_handover_required_to_eps_carries_the_container_of_the_source_enb() {
    oxirush_s1ap::inspect::register();
    let required = NGAPPDU::from_tree(&tree!({"message": "HandoverRequired", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"HandoverType": "fivegs-to-eps"},
        {"Cause": {"radioNetwork": "handover-desirable-for-radio-reason"}},
        {"SourceToTarget-TransparentContainer": {(SOURCE): (source())}},
    ]}))
    .unwrap();
    let container = "SourceToTarget-TransparentContainer";
    let expected = by_s1ap(
        "HandoverRequest",
        "Source-ToTarget-TransparentContainer",
        SOURCE,
        source(),
    );
    assert_eq!(octets(&required, container), expected);

    // Read: a path goes through the name of the type, into the members of S1AP
    let path = format!("/ngap/{container}/{SOURCE}");
    let size = format!("{path}/uE-HistoryInformation/0/e-UTRAN-Cell/cellType/cell-Size");
    assert_eq!(inspect::check_path("HandoverRequired", &size), Ok(()));
    let sizes = inspect::enumerated_at("HandoverRequired", &size).unwrap();
    assert_eq!(sizes, Some(&["verysmall", "small", "medium", "large"][..]));
    let mut view = required.to_view().unwrap();
    inspect::open(&mut view, &size).unwrap();
    assert_eq!(inspect::select(&view, &size).unwrap(), [&tree!("small")]);
    let plmn = format!("{path}/targetCell-ID/pLMNidentity");
    assert_eq!(inspect::select(&view, &plmn).unwrap(), [&tree!("208-93")]);
    for (path, value) in inspect::paths(&view) {
        assert_eq!(inspect::select(&view, &path).unwrap(), [&value], "{path}");
    }
    assert_eq!(NGAPPDU::from_view(&view).unwrap(), required);

    // Edited by the type of S1AP
    inspect::set(&mut view, &size, tree!("large")).unwrap();
    let edited = NGAPPDU::from_view(&view).unwrap();
    let mut large = source();
    large["uE-HistoryInformation"][0]["e-UTRAN-Cell"]["cellType"]["cell-Size"] = tree!("large");
    let expected = by_s1ap(
        "HandoverRequest",
        "Source-ToTarget-TransparentContainer",
        SOURCE,
        large,
    );
    assert_eq!(octets(&edited, container), expected);
    inspect::set(&mut view, &size, tree!("huge")).unwrap();
    assert!(NGAPPDU::from_view(&view).is_err());
}

#[test]
fn the_handover_command_to_eps_carries_the_container_of_the_target_enb() {
    oxirush_s1ap::inspect::register();
    let target = tree!({"rRC-Container": "0a0b0c"});
    let command = NGAPPDU::from_tree(&tree!({"message": "HandoverCommand", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"HandoverType": "fivegs-to-eps"},
        {"NASSecurityParametersFromNGRAN": "05"},
        {"TargetToSource-TransparentContainer": {(TARGET): (target.clone())}},
    ]}))
    .unwrap();
    let container = "TargetToSource-TransparentContainer";
    let expected = by_s1ap(
        "HandoverRequestAcknowledge",
        "Target-ToSource-TransparentContainer",
        TARGET,
        target,
    );
    assert_eq!(octets(&command, container), expected);
    assert_eq!(expected, tree!("00030a0b0c"));

    let rrc = format!("/ngap/{container}/{TARGET}/rRC-Container");
    let mut view = command.to_view().unwrap();
    inspect::open(&mut view, &rrc).unwrap();
    assert_eq!(inspect::select(&view, &rrc).unwrap(), [&tree!("0a0b0c")]);
    inspect::set(&mut view, &rrc, tree!("ff")).unwrap();
    let edited = NGAPPDU::from_view(&view).unwrap();
    assert_eq!(octets(&edited, container), tree!("0001ff"));
    // A container of NG-RAN is still written by the name of its type
    let own = "TargetNGRANNode-ToSourceNGRANNode-TransparentContainer";
    let written = NGAPPDU::from_tree(&tree!({"message": "HandoverCommand", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"HandoverType": "intra5gs"},
        {"TargetToSource-TransparentContainer": {(own): {"rRCContainer": "0a0b0c"}}},
    ]}));
    assert!(written.is_ok());
}
