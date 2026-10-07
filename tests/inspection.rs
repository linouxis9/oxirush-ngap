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
    assert_eq!(tree.pointer(path), Some(&serde_json::json!(1)));
    *tree.pointer_mut(path).unwrap() = serde_json::json!(3);
    assert!(
        inspect::encode_pdu(&tree).is_err(),
        "a typed edit must not silently discard a future IE extension"
    );
}

#[test]
fn the_octets_of_an_ie_are_replaced_or_added_as_its_raw_value_without_value() {
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
    );
    let mut tree = inspect::inspect_pdu(&pdu).unwrap();
    // RAN-UE-NGAP-ID 9, as its octets in place of the typed value.
    let ie = tree.pointer_mut("/message/protocolIEs/1").unwrap();
    ie.as_object_mut().unwrap().remove("value");
    ie["_raw_value"] = serde_json::json!("0009");
    let ies = tree.pointer_mut("/message/protocolIEs").unwrap();
    ies.as_array_mut()
        .unwrap()
        .push(serde_json::json!({"id": 60000, "criticality": "ignore", "_raw_value": "C0FFEE"}));
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
fn a_new_ie_takes_a_typed_value_of_the_type_of_its_identifier() {
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
    );
    let with_cause = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    );
    let added = |id: u16, value: serde_json::Value| {
        let mut tree = inspect::inspect_pdu(&pdu).unwrap();
        let ies = tree.pointer_mut("/message/protocolIEs").unwrap();
        ies.as_array_mut()
            .unwrap()
            .push(serde_json::json!({"id": id, "criticality": "ignore", "value": value}));
        inspect::encode_pdu(&tree)
    };
    let cause = serde_json::json!({"radioNetwork": "user-inactivity"});
    assert_eq!(
        added(15, cause.clone()).unwrap().encode().unwrap(),
        with_cause.encode().unwrap()
    );
    // A value that the type does not have, and an identifier without a type.
    assert!(added(15, serde_json::json!({"radioNetwork": "no-such-cause"})).is_err());
    assert!(added(60000, cause).unwrap_err().contains("_raw_value"));
}

#[test]
fn a_new_ie_whose_value_is_a_string_is_typed_too() {
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
    );
    let typed = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        IGNORE RRCEstablishmentCause(RRCEstablishmentCause::mo_Signalling),
        REJECT NAS_PDU(vec![0x7e, 0x00]),
    );
    let mut tree = inspect::inspect_pdu(&pdu).unwrap();
    let ies = tree.pointer_mut("/message/protocolIEs").unwrap();
    let ies = ies.as_array_mut().unwrap();
    // An ENUMERATED by its name and an OCTET STRING by its octets, not the
    // octets of their open types.
    ies.push(serde_json::json!({"id": 90, "criticality": "ignore", "value": "mo-Signalling"}));
    ies.push(serde_json::json!({"id": 38, "criticality": "reject", "value": "7E00"}));
    assert_eq!(
        inspect::encode_pdu(&tree).unwrap().encode().unwrap(),
        typed.encode().unwrap()
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

/// The octets of the message `name` of the fixtures.
fn fixture(name: &str) -> Vec<u8> {
    let mut lines = include_str!("fixtures/messages.tsv").lines();
    let line = lines.find(|line| line.split('\t').next() == Some(name));
    hex::decode(line.unwrap().split('\t').nth(1).unwrap()).unwrap()
}

/// The entry of the IE `id` of a tree.
fn ie(tree: &mut serde_json::Value, id: u16) -> &mut serde_json::Value {
    let ies = tree["message"]["protocolIEs"].as_array_mut().unwrap();
    ies.iter_mut().find(|ie| ie["id"] == id).unwrap()
}

/// The tree of a message that has `entries` for IEs, encoded and decoded again.
fn sent(entries: &[serde_json::Value]) -> Result<serde_json::Value, String> {
    let pdu = build_ngap!(
        InitiatingMessage,
        UEContextReleaseRequest,
        IGNORE,
        UEContextReleaseRequest,
    );
    let mut tree = inspect::inspect_pdu(&pdu)?;
    *tree["message"]["protocolIEs"].as_array_mut().unwrap() = entries.to_vec();
    inspect::inspect_pdu(&inspect::encode_pdu(&tree)?)
}

#[test]
fn well_known_values_are_shown_and_taken_as_they_are_usually_written() {
    use serde_json::json;
    // NG SETUP REQUEST: GlobalRANNodeID is IE 27 and SupportedTAList IE 102.
    let wire = fixture("NGSetupRequest");
    let mut tree = inspect::inspect_pdu(&NGAP_PDU::decode(&wire).unwrap()).unwrap();
    let area = &ie(&mut tree, 102)["value"][0];
    let slice = "/broadcastPLMNList/0/tAISliceSupportList/0/s-NSSAI/sST";
    assert_eq!(area["tAC"], json!(0x60B72F));
    assert_eq!(
        area["broadcastPLMNList"][0]["pLMNIdentity"],
        json!("208-93")
    );
    assert_eq!(area.pointer(slice), Some(&json!(0x59)));
    // What is not edited keeps its octets.
    assert_eq!(inspect::encode_pdu(&tree).unwrap().encode().unwrap(), wire);

    // Edited as they are shown, a number in hexadecimal too.
    let area = &mut ie(&mut tree, 102)["value"][0];
    area["tAC"] = json!(7);
    area["broadcastPLMNList"][0]["pLMNIdentity"] = json!("001-01");
    *area.pointer_mut(slice).unwrap() = json!("0x2A");
    let edited = inspect::encode_pdu(&tree).unwrap();
    let mut shown = inspect::inspect_pdu(&edited).unwrap();
    let area = &ie(&mut shown, 102)["value"][0];
    assert_eq!(area["tAC"], json!(7));
    assert_eq!(
        area["broadcastPLMNList"][0]["pLMNIdentity"],
        json!("001-01")
    );
    assert_eq!(area.pointer(slice), Some(&json!(42)));
    assert_eq!(
        ie(&mut shown, 27)["_raw_value"],
        ie(&mut tree, 27)["_raw_value"]
    );

    // As JER writes them, which is what a tree had before.
    let area = &mut ie(&mut tree, 102)["value"][0];
    area["tAC"] = json!("000007");
    area["broadcastPLMNList"][0]["pLMNIdentity"] = json!("00F110");
    *area.pointer_mut(slice).unwrap() = json!("2A");
    assert_eq!(
        inspect::encode_pdu(&tree).unwrap().encode().unwrap(),
        edited.encode().unwrap()
    );

    // Neither form, and a number that the type has no room for.
    for (member, refused) in [
        ("tAC", json!("7")),
        ("tAC", json!(1 << 24)),
        ("tAC", json!("0x1000000")),
        ("pLMNIdentity", json!("208-9")),
        ("pLMNIdentity", json!("20-893")),
    ] {
        let mut tree = inspect::inspect_pdu(&NGAP_PDU::decode(&wire).unwrap()).unwrap();
        let area = &mut ie(&mut tree, 102)["value"][0];
        match member {
            "tAC" => area["tAC"] = refused.clone(),
            _ => area["broadcastPLMNList"][0]["pLMNIdentity"] = refused.clone(),
        }
        assert!(inspect::encode_pdu(&tree).is_err(), "{member} {refused}");
    }
}

#[test]
fn the_parts_of_a_guami_are_numbers_of_their_bits() {
    use serde_json::json;
    // INITIAL CONTEXT SETUP REQUEST: GUAMI is IE 28.
    let wire = fixture("InitialContextSetupRequest");
    let mut tree = inspect::inspect_pdu(&NGAP_PDU::decode(&wire).unwrap()).unwrap();
    let guami = &mut ie(&mut tree, 28)["value"];
    assert_eq!(guami["aMFSetID"], json!(0));
    guami["pLMNIdentity"] = json!("310-410");
    guami["aMFRegionID"] = json!(0xCA);
    guami["aMFSetID"] = json!(0x3FF);
    guami["aMFPointer"] = json!(1);
    let edited = inspect::encode_pdu(&tree).unwrap();
    let mut shown = inspect::inspect_pdu(&edited).unwrap();
    // The same GUAMI as the crate's helper builds from these numbers.
    let network = oxirush_ngap::helpers::plmn("310", "410");
    let expected = oxirush_ngap::helpers::guami(network, 0xCA, 0x3FF, 1);
    let expected = encode_open_type(&expected).unwrap();
    assert_eq!(
        ie(&mut shown, 28)["_raw_value"],
        json!(hex::encode_upper(expected.as_bytes()))
    );
    assert_eq!(ie(&mut shown, 28)["value"]["aMFSetID"], json!(0x3FF));
    // The AMF Set ID has ten bits.
    ie(&mut tree, 28)["value"]["aMFSetID"] = json!(0x400);
    assert!(inspect::encode_pdu(&tree).is_err());
}

#[test]
fn a_tunnel_endpoint_is_an_ip_address_and_a_number() {
    use oxirush_ngap::helpers::bytes_to_bitvec;
    use serde_json::json;
    let added = |id: u16, value: serde_json::Value| {
        sent(&[json!({"id": id, "criticality": "ignore", "value": value})])
    };
    // UL-NGU-UP-TNLInformation is IE 139, a UPTransportLayerInformation.
    let tunnel = json!({"gTPTunnel": {"transportLayerAddress": "10.0.0.1", "gTP-TEID": 7}});
    let mut shown = added(139, tunnel.clone()).unwrap();
    let expected = UPTransportLayerInformation::gTPTunnel(GTPTunnel::new(
        TransportLayerAddress(bytes_to_bitvec(&[10, 0, 0, 1])),
        GTPTEID::from([0, 0, 0, 7]),
        None,
    ));
    let expected = encode_open_type(&expected).unwrap();
    assert_eq!(ie(&mut shown, 139)["value"], tunnel);
    assert_eq!(
        ie(&mut shown, 139)["_raw_value"],
        json!(hex::encode_upper(expected.as_bytes()))
    );
    // TraceCollectionEntityIPAddress is IE 109, a TransportLayerAddress itself.
    for (address, octets) in [
        ("10.0.0.1", "0A000001"),
        ("2001:db8::1", "20010DB8000000000000000000000001"),
        (
            "10.0.0.1,2001:db8::1",
            "0A00000120010DB8000000000000000000000001",
        ),
    ] {
        let mut shown = added(109, json!(address)).unwrap();
        assert_eq!(ie(&mut shown, 109)["value"], json!(address));
        // As JER writes it, which is what a tree had before.
        let jer = json!({"value": octets, "length": octets.len() * 4});
        let mut same = added(109, jer).unwrap();
        assert_eq!(
            ie(&mut same, 109)["_raw_value"],
            ie(&mut shown, 109)["_raw_value"]
        );
    }
    // An address of another length is neither: it stays as JER has it.
    let other = json!({"value": "0A0000", "length": 24});
    let mut shown = added(109, other.clone()).unwrap();
    assert_eq!(ie(&mut shown, 109)["value"], other);
    for refused in ["10.0.0", "2001:db8::1,10.0.0.1", "10.0.0.1,10.0.0.2"] {
        assert!(added(109, json!(refused)).is_err(), "{refused}");
    }
}

#[test]
fn an_enumerated_value_is_named_whatever_its_case_and_separators() {
    use serde_json::json;
    let causes = |cause: &str, establishment: &str| {
        sent(&[
            json!({"id": 15, "criticality": "ignore", "value": {"radioNetwork": cause}}),
            json!({"id": 90, "criticality": "ignore", "value": establishment}),
        ])
    };
    let mut exact = causes("user-inactivity", "mo-Signalling").unwrap();
    let mut loose = causes("User_Inactivity", "MO SIGNALLING").unwrap();
    for id in [15, 90] {
        assert_eq!(ie(&mut loose, id)["value"], ie(&mut exact, id)["value"]);
        assert_eq!(
            ie(&mut loose, id)["_raw_value"],
            ie(&mut exact, id)["_raw_value"]
        );
    }
    // A name of another type, and no name at all.
    assert!(causes("mo-Signalling", "mo-Signalling").is_err());
    assert!(causes("user-inactivit", "mo-Signalling").is_err());
    // UserPlaneErrorIndicator (IE 429) and the UserPlaneFailureType of IE 435 spell one
    // name in two ways: each takes it in its own.
    let tunnel = json!({"gTPTunnel": {"transportLayerAddress": "10.0.0.1", "gTP-TEID": 1}});
    let mut shown = sent(&[
        json!({"id": 429, "criticality": "ignore", "value": "GTP-U-ERROR-INDICATION-RECEIVED"}),
        json!({"id": 435, "criticality": "ignore", "value": {
            "userPlaneFailureType": "GTP-U-ERROR-INDICATION-RECEIVED",
            "uL-NGU-UP-TNLInformation": tunnel, "dL-NGU-UP-TNLInformation": tunnel}}),
    ])
    .unwrap();
    assert_eq!(
        ie(&mut shown, 429)["value"],
        json!("gTP-U-error-indication-received")
    );
    assert_eq!(
        ie(&mut shown, 435)["value"]["userPlaneFailureType"],
        json!("gtp-u-error-indication-received")
    );
}
