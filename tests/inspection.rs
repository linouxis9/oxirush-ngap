#![cfg(feature = "inspect")]

use oxirush_ngap::{build_ngap, inspect, ngap::*};

#[test]
fn inspect_repeated_ies_and_edit_one_without_reordering() {
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
        REJECT AMF_UE_NGAP_ID(2u64),
    );
    let mut tree = inspect::inspect_pdu(&pdu).unwrap();
    assert_eq!(
        tree.pointer("/message/protocolIEs/0/value"),
        Some(&serde_json::json!(1))
    );
    assert_eq!(
        tree.pointer("/message/protocolIEs/3/value"),
        Some(&serde_json::json!(2))
    );
    assert_eq!(
        inspect::encode_pdu(&tree).unwrap().encode().unwrap(),
        pdu.encode().unwrap()
    );
    *tree.pointer_mut("/message/protocolIEs/0/value").unwrap() = serde_json::json!(42);
    let edited = inspect::encode_pdu(&tree).unwrap();
    let tree = inspect::inspect_pdu(&edited).unwrap();
    assert_eq!(
        tree.pointer("/message/protocolIEs/0/value"),
        Some(&serde_json::json!(42))
    );
    assert_eq!(
        tree.pointer("/message/protocolIEs/3/value"),
        Some(&serde_json::json!(2))
    );
}

#[test]
fn inspect_every_canonical_procedure_and_round_trip() {
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        let wire = hex::decode(fields[1]).unwrap();
        let pdu = NGAP_PDU::decode(&wire).unwrap();
        let tree = inspect::inspect_pdu(&pdu).unwrap();
        assert!(tree["message"].is_object(), "{}", fields[0]);
        assert_eq!(
            inspect::encode_pdu(&tree).unwrap().encode().unwrap(),
            wire,
            "{}",
            fields[0]
        );
    }
}

#[test]
fn contained_transfer_edits_are_typed_and_do_not_reorder_outer_ies() {
    let transfer = PDUSessionResourceSetupUnsuccessfulTransfer::new(
        Cause::protocol(CauseProtocol::semantic_error),
        None,
        None,
    );
    let raw = encode_open_type(&transfer).unwrap().as_bytes().to_vec();
    let pdu = build_ngap!(SuccessfulOutcome, PDUSessionResourceSetup,
        IGNORE, PDUSessionResourceSetupResponse,
        IGNORE AMF_UE_NGAP_ID(1u64),
        IGNORE RAN_UE_NGAP_ID(7u32),
        IGNORE PDUSessionResourceFailedToSetupListSURes(PDUSessionResourceFailedToSetupListSURes(vec![PDUSessionResourceFailedToSetupItemSURes::new(PDUSessionID(7), raw.into(), None)])),
    );
    let mut tree = inspect::inspect_pdu(&pdu).unwrap();
    let path = "/message/protocolIEs/2/value/0/pDUSessionResourceSetupUnsuccessfulTransfer/decoded/cause/protocol";
    assert_eq!(
        tree.pointer(path),
        Some(&serde_json::json!("semantic-error"))
    );
    assert_eq!(
        inspect::encode_pdu(&tree).unwrap().encode().unwrap(),
        pdu.encode().unwrap()
    );
    *tree.pointer_mut(path).unwrap() = serde_json::json!("abstract-syntax-error-reject");
    let edited = inspect::encode_pdu(&tree).unwrap();
    let checked = inspect::inspect_pdu(&edited).unwrap();
    assert_eq!(
        checked.pointer(path),
        Some(&serde_json::json!("abstract-syntax-error-reject"))
    );
    assert_eq!(
        checked.pointer("/message/protocolIEs/0/value"),
        Some(&serde_json::json!(1))
    );
    tree["typo"] = serde_json::json!(1);
    assert!(inspect::encode_pdu(&tree).is_err());
}

#[test]
fn editing_a_known_ie_cannot_drop_unknown_sequence_additions() {
    // The first SliceSupportItem has a future SEQUENCE addition. Its bytes
    // come from the independent unknown_extensions fixture, inside PLMNSupportList.
    let raw = hex::decode("0002f839000180080801000010").unwrap();
    let list: PLMNSupportList = decode_open_type(&raw.clone().into()).unwrap();
    assert_eq!(list.0[0].slice_support_list.0[1].s_nssai.s_st.0[0], 2);
    let response = NGSetupResponse::new(ProtocolIEContainer(vec![ProtocolIEField::new(
        ProtocolIEID(80),
        Criticality::reject,
        raw.into(),
    )]));
    let pdu = NGAP_PDU::successfulOutcome(SuccessfulOutcome::new(
        ProcedureCode(21),
        Criticality::reject,
        encode_open_type(&response).unwrap(),
    ));
    let wire = pdu.encode().unwrap();
    let mut tree = inspect::inspect_pdu(&pdu).unwrap();
    assert_eq!(inspect::encode_pdu(&tree).unwrap().encode().unwrap(), wire);
    let path = "/message/protocolIEs/0/value/0/sliceSupportList/0/s-NSSAI/sST";
    assert_eq!(tree.pointer(path), Some(&serde_json::json!("01")));
    *tree.pointer_mut(path).unwrap() = serde_json::json!("03");
    assert!(
        inspect::encode_pdu(&tree).is_err(),
        "a typed edit must not silently discard a future IE extension"
    );
}

#[test]
fn the_octets_of_an_ie_are_replaced_or_added_as_its_value_without_raw_value() {
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
    );
    let mut tree = inspect::inspect_pdu(&pdu).unwrap();
    // RAN-UE-NGAP-ID 9, as its octets in place of the typed value.
    let ie = tree.pointer_mut("/message/protocolIEs/1").unwrap();
    ie.as_object_mut().unwrap().remove("_raw_value");
    ie["value"] = serde_json::json!("0009");
    let ies = tree.pointer_mut("/message/protocolIEs").unwrap();
    ies.as_array_mut()
        .unwrap()
        .push(serde_json::json!({"id": 60000, "criticality": "ignore", "value": "C0FFEE"}));
    let edited = inspect::encode_pdu(&tree).unwrap();
    let tree = inspect::inspect_pdu(&edited).unwrap();
    assert_eq!(
        tree.pointer("/message/protocolIEs/1/value"),
        Some(&serde_json::json!(9))
    );
    assert_eq!(
        tree.pointer("/message/protocolIEs/2/value"),
        Some(&serde_json::json!("C0FFEE"))
    );
}

#[test]
fn an_ie_that_does_not_decode_is_edited_as_its_octets() {
    let request = UEContextReleaseRequest::new(ProtocolIEContainer(vec![ProtocolIEField::new(
        ProtocolIEID(60000),
        Criticality::ignore,
        vec![0xC0, 0xFF, 0xEE].into(),
    )]));
    let pdu = NGAP_PDU::initiatingMessage(InitiatingMessage::new(
        ProcedureCode(42),
        Criticality::ignore,
        encode_open_type(&request).unwrap(),
    ));
    let mut tree = inspect::inspect_pdu(&pdu).unwrap();
    let path = "/message/protocolIEs/0/value";
    assert_eq!(tree.pointer(path), Some(&serde_json::json!("C0FFEE")));
    assert!(tree["message"]["protocolIEs"][0]["_decode_error"].is_string());
    assert_eq!(
        inspect::encode_pdu(&tree).unwrap().encode().unwrap(),
        pdu.encode().unwrap()
    );
    *tree.pointer_mut(path).unwrap() = serde_json::json!("DECAFBAD");
    let edited = inspect::encode_pdu(&tree).unwrap();
    assert_eq!(
        inspect::inspect_pdu(&edited).unwrap().pointer(path),
        Some(&serde_json::json!("DECAFBAD"))
    );
    // Its type is unknown, so a typed value has no encoding.
    *tree.pointer_mut(path).unwrap() = serde_json::json!(5);
    assert!(inspect::encode_pdu(&tree).is_err());
}

#[test]
fn a_changed_raw_value_alone_sends_nothing_else() {
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
    );
    let mut tree = inspect::inspect_pdu(&pdu).unwrap();
    *tree
        .pointer_mut("/message/protocolIEs/1/_raw_value")
        .unwrap() = serde_json::json!("0009");
    assert_eq!(
        inspect::encode_pdu(&tree).unwrap().encode().unwrap(),
        pdu.encode().unwrap()
    );
}

#[test]
fn an_ie_that_contains_a_type_is_decoded_and_edited_like_a_transfer() {
    // DISTRIBUTION RELEASE REQUEST of the fixtures: its second IE is
    // id-MBS-DistributionReleaseRequestTransfer, an OCTET STRING (CONTAINING ...).
    let wire =
        hex::decode("00460022000003012b000700328d3ee97789012c000a090079f487fab2100000000f40020000")
            .unwrap();
    let pdu = NGAP_PDU::decode(&wire).unwrap();
    let mut tree = inspect::inspect_pdu(&pdu).unwrap();
    let path = "/message/protocolIEs/1/value/decoded/cause/radioNetwork";
    assert_eq!(tree.pointer(path), Some(&serde_json::json!("unspecified")));
    assert_eq!(inspect::encode_pdu(&tree).unwrap().encode().unwrap(), wire);

    *tree.pointer_mut(path).unwrap() = serde_json::json!("user-inactivity");
    let edited = inspect::encode_pdu(&tree).unwrap();
    let checked = inspect::inspect_pdu(&edited).unwrap();
    assert_eq!(
        checked.pointer(path),
        Some(&serde_json::json!("user-inactivity"))
    );
    // The other IEs keep the octets received.
    for index in [0, 2] {
        let raw = format!("/message/protocolIEs/{index}/_raw_value");
        assert_eq!(checked.pointer(&raw), tree.pointer(&raw));
    }
}
