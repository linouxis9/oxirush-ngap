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
    )
    .unwrap();
    let mut view = inspect::message_view(&pdu).unwrap();
    assert_eq!(
        view.pointer("/message/protocolIEs/0/value"),
        Some(&serde_json::json!(1))
    );
    assert_eq!(
        view.pointer("/message/protocolIEs/3/value"),
        Some(&serde_json::json!(2))
    );
    assert_eq!(
        inspect::message_from_view(&view).unwrap().encode().unwrap(),
        pdu.encode().unwrap()
    );
    *view.pointer_mut("/message/protocolIEs/0/value").unwrap() = serde_json::json!(42);
    let edited = inspect::message_from_view(&view).unwrap();
    let view = inspect::message_view(&edited).unwrap();
    assert_eq!(
        view.pointer("/message/protocolIEs/0/value"),
        Some(&serde_json::json!(42))
    );
    assert_eq!(
        view.pointer("/message/protocolIEs/3/value"),
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
        let view = inspect::message_view(&pdu).unwrap();
        assert!(view["message"].is_object(), "{}", fields[0]);
        assert_eq!(
            inspect::message_from_view(&view).unwrap().encode().unwrap(),
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
    ).unwrap();
    let mut view = inspect::message_view(&pdu).unwrap();
    let path = "/message/protocolIEs/2/value/0/pDUSessionResourceSetupUnsuccessfulTransfer/decoded/cause/protocol";
    assert_eq!(
        view.pointer(path),
        Some(&serde_json::json!("semantic-error"))
    );
    assert_eq!(
        inspect::message_from_view(&view).unwrap().encode().unwrap(),
        pdu.encode().unwrap()
    );
    *view.pointer_mut(path).unwrap() = serde_json::json!("abstract-syntax-error-reject");
    let edited = inspect::message_from_view(&view).unwrap();
    let checked = inspect::message_view(&edited).unwrap();
    assert_eq!(
        checked.pointer(path),
        Some(&serde_json::json!("abstract-syntax-error-reject"))
    );
    assert_eq!(
        checked.pointer("/message/protocolIEs/0/value"),
        Some(&serde_json::json!(1))
    );
    view["typo"] = serde_json::json!(1);
    assert!(inspect::message_from_view(&view).is_err());
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
    let mut view = inspect::message_view(&pdu).unwrap();
    assert_eq!(
        inspect::message_from_view(&view).unwrap().encode().unwrap(),
        wire
    );
    let path = "/message/protocolIEs/0/value/0/sliceSupportList/0/s-NSSAI/sST";
    assert_eq!(view.pointer(path), Some(&serde_json::json!(1)));
    *view.pointer_mut(path).unwrap() = serde_json::json!(3);
    assert!(
        inspect::message_from_view(&view).is_err(),
        "a typed edit must not silently discard a future IE extension"
    );
}

#[test]
fn the_octets_of_an_ie_are_replaced_or_added_as_its_raw_value_without_value() {
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
    )
    .unwrap();
    let mut view = inspect::message_view(&pdu).unwrap();
    // RAN-UE-NGAP-ID 9, as its octets in place of the typed value.
    let ie = view.pointer_mut("/message/protocolIEs/1").unwrap();
    ie.as_object_mut().unwrap().remove("value");
    ie["_raw_value"] = serde_json::json!("0009");
    let ies = view.pointer_mut("/message/protocolIEs").unwrap();
    ies.as_array_mut()
        .unwrap()
        .push(serde_json::json!({"id": 60000, "criticality": "ignore", "_raw_value": "C0FFEE"}));
    let edited = inspect::message_from_view(&view).unwrap();
    let view = inspect::message_view(&edited).unwrap();
    assert_eq!(
        view.pointer("/message/protocolIEs/1/value"),
        Some(&serde_json::json!(9))
    );
    assert_eq!(
        view.pointer("/message/protocolIEs/2/value"),
        Some(&serde_json::json!("C0FFEE"))
    );
}

#[test]
fn a_new_ie_takes_a_typed_value_of_the_type_of_its_identifier() {
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
    )
    .unwrap();
    let with_cause = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )
    .unwrap();
    let added = |id: u16, value: serde_json::Value| {
        let mut view = inspect::message_view(&pdu).unwrap();
        let ies = view.pointer_mut("/message/protocolIEs").unwrap();
        ies.as_array_mut()
            .unwrap()
            .push(serde_json::json!({"id": id, "criticality": "ignore", "value": value}));
        inspect::message_from_view(&view)
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
    )
    .unwrap();
    let typed = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        IGNORE RRCEstablishmentCause(RRCEstablishmentCause::mo_Signalling),
        REJECT NAS_PDU(vec![0x7e, 0x00]),
    )
    .unwrap();
    let mut view = inspect::message_view(&pdu).unwrap();
    let ies = view.pointer_mut("/message/protocolIEs").unwrap();
    let ies = ies.as_array_mut().unwrap();
    // An ENUMERATED by its name and an OCTET STRING by its octets, not the
    // octets of their open types.
    ies.push(serde_json::json!({"id": 90, "criticality": "ignore", "value": "mo-Signalling"}));
    ies.push(serde_json::json!({"id": 38, "criticality": "reject", "value": "7E00"}));
    assert_eq!(
        inspect::message_from_view(&view).unwrap().encode().unwrap(),
        typed.encode().unwrap()
    );
    // Hexadecimal in lower case is the same octets.
    *view.pointer_mut("/message/protocolIEs/2/value").unwrap() = serde_json::json!("7e00");
    assert_eq!(
        inspect::message_from_view(&view).unwrap().encode().unwrap(),
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
    let mut view = inspect::message_view(&pdu).unwrap();
    let path = "/message/protocolIEs/0/value";
    assert_eq!(view.pointer(path), Some(&serde_json::json!("C0FFEE")));
    assert!(view["message"]["protocolIEs"][0]["_decode_error"].is_string());
    assert_eq!(
        inspect::message_from_view(&view).unwrap().encode().unwrap(),
        pdu.encode().unwrap()
    );
    *view.pointer_mut(path).unwrap() = serde_json::json!("DECAFBAD");
    let edited = inspect::message_from_view(&view).unwrap();
    assert_eq!(
        inspect::message_view(&edited).unwrap().pointer(path),
        Some(&serde_json::json!("DECAFBAD"))
    );
    // Its type is unknown, so a typed value has no encoding.
    *view.pointer_mut(path).unwrap() = serde_json::json!(5);
    assert!(inspect::message_from_view(&view).is_err());
}

#[test]
fn a_changed_raw_value_alone_sends_nothing_else() {
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
    )
    .unwrap();
    let mut view = inspect::message_view(&pdu).unwrap();
    *view
        .pointer_mut("/message/protocolIEs/1/_raw_value")
        .unwrap() = serde_json::json!("0009");
    assert_eq!(
        inspect::message_from_view(&view).unwrap().encode().unwrap(),
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
    let mut view = inspect::message_view(&pdu).unwrap();
    let path = "/message/protocolIEs/1/value/decoded/cause/radioNetwork";
    assert_eq!(view.pointer(path), Some(&serde_json::json!("unspecified")));
    assert_eq!(
        inspect::message_from_view(&view).unwrap().encode().unwrap(),
        wire
    );

    *view.pointer_mut(path).unwrap() = serde_json::json!("user-inactivity");
    let edited = inspect::message_from_view(&view).unwrap();
    let checked = inspect::message_view(&edited).unwrap();
    assert_eq!(
        checked.pointer(path),
        Some(&serde_json::json!("user-inactivity"))
    );
    // The other IEs keep the octets received.
    for index in [0, 2] {
        let raw = format!("/message/protocolIEs/{index}/_raw_value");
        assert_eq!(checked.pointer(&raw), view.pointer(&raw));
    }
}

#[test]
fn a_transfer_that_was_not_received_is_encoded_from_its_decoded_value() {
    let pdu = build_ngap!(SuccessfulOutcome, PDUSessionResourceSetup,
        REJECT, PDUSessionResourceSetupResponse,
        IGNORE AMF_UE_NGAP_ID(1u64),
        IGNORE RAN_UE_NGAP_ID(7u32),
    )
    .unwrap();
    let transfer = serde_json::json!({"dLQosFlowPerTNLInformation": {
        "uPTransportLayerInformation": {"gTPTunnel": {
            "transportLayerAddress": "10.0.0.1",
            "gTP-TEID": 9,
        }},
        "associatedQosFlowList": [{"qosFlowIdentifier": 1}],
    }});
    let written = |transfer: serde_json::Value| {
        let mut view = inspect::message_view(&pdu).unwrap();
        let ies = view.pointer_mut("/message/protocolIEs").unwrap();
        ies.as_array_mut().unwrap().push(serde_json::json!({
            "id": 75,
            "criticality": "ignore",
            "value": [{"pDUSessionID": 5, "pDUSessionResourceSetupResponseTransfer": transfer}],
        }));
        inspect::message_from_view(&view).and_then(|pdu| inspect::message_view(&pdu))
    };
    let sent = written(serde_json::json!({"decoded": transfer})).unwrap();
    let item = "/message/protocolIEs/2/value/0/pDUSessionResourceSetupResponseTransfer";
    assert_eq!(sent.pointer(&format!("{item}/decoded")), Some(&transfer));
    // The decoded value itself is the same transfer, and so are its octets.
    assert_eq!(written(transfer.clone()).unwrap(), sent);
    let octets = sent.pointer(&format!("{item}/_raw_transfer")).unwrap();
    let as_octets = serde_json::json!({"octets": octets});
    assert_eq!(written(as_octets).unwrap(), sent);
    assert_eq!(written(octets.clone()).unwrap(), sent);
    // A member beside the value, and a value that the transfer does not have.
    let error = written(serde_json::json!({"decoded": transfer, "other": 1})).unwrap_err();
    assert!(error.contains("its decoded value alone"), "{error}");
    let error = written(serde_json::json!({"octets": octets, "other": 1})).unwrap_err();
    assert!(error.contains("its decoded value alone"), "{error}");
    assert!(written(serde_json::json!({"decoded": {"no-such-member": 1}})).is_err());
    assert!(written(serde_json::json!({"no-such-member": 1})).is_err());

    // The value of an IE that contains a type is written the same way.
    let release = build_ngap!(
        InitiatingMessage,
        DistributionRelease,
        REJECT,
        DistributionReleaseRequest,
    )
    .unwrap();
    let received = NGAP_PDU::decode(&fixture("DistributionReleaseRequest")).unwrap();
    let received = inspect::message_view(&received).unwrap();
    // With its `decoded` member alone, then as the decoded value itself.
    for alone in [false, true] {
        let ies: Vec<_> = received["message"]["protocolIEs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|ie| {
                let mut ie = ie.clone();
                let members = ie.as_object_mut().unwrap();
                members.retain(|name, _| !name.starts_with('_'));
                let value = members.get_mut("value").unwrap();
                if let Some(decoded) = value.get("decoded").cloned() {
                    *value = match alone {
                        true => decoded,
                        false => serde_json::json!({"decoded": decoded}),
                    };
                }
                ie
            })
            .collect();
        let mut view = inspect::message_view(&release).unwrap();
        view["message"]["protocolIEs"] = ies.into();
        let sent = inspect::message_from_view(&view).unwrap().encode().unwrap();
        assert_eq!(sent, fixture("DistributionReleaseRequest"));
    }
}

#[test]
fn a_message_is_shown_and_written_as_its_name_and_its_ies_by_name() {
    // Every message of the fixtures is the same octets once it was shown and written.
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        let wire = hex::decode(fields[1]).unwrap();
        let shown = match inspect::message_tree(&NGAP_PDU::decode(&wire).unwrap()) {
            Ok(shown) => shown,
            // A private message has no IEs of the protocol to name.
            Err(reason) => {
                assert!(
                    reason.contains("PrivateMessage has no list of IEs"),
                    "{reason}"
                );
                continue;
            }
        };
        assert!(
            shown["message"].is_string() && shown["ies"].is_array(),
            "{}",
            fields[0]
        );
        let written = inspect::message_from_tree(&shown);
        assert_eq!(
            written.unwrap().encode().unwrap(),
            wire,
            "{}: {shown}",
            fields[0]
        );
    }
    // The name gives the procedure and the criticalities; an IE that the message does
    // not have says its own, and octets are sent as they are.
    let request = serde_json::json!({
        "message": "ue context release request",
        "ies": [
            {"amf-ue-ngap-id": 1},
            {"RAN-UE-NGAP-ID": 2},
            {"Cause": {"radioNetwork": "user-inactivity"}},
            {"65000": {"octets": "00"}, "criticality": "reject"},
        ],
    });
    let pdu = inspect::message_from_tree(&request).unwrap();
    assert_eq!(inspect::message_name(&pdu), Some("UEContextReleaseRequest"));
    let view = inspect::message_view(&pdu).unwrap();
    assert_eq!(view["criticality"], "ignore");
    let criticalities = view["message"]["protocolIEs"].as_array().unwrap().iter();
    let criticalities: Vec<_> = criticalities
        .map(|ie| ie["criticality"].as_str().unwrap())
        .collect();
    assert_eq!(criticalities, ["reject", "reject", "ignore", "reject"]);
    let shown = inspect::message_tree(&pdu).unwrap();
    assert_eq!(shown["message"], "UEContextReleaseRequest");
    assert_eq!(shown["ies"][0], serde_json::json!({"AMF-UE-NGAP-ID": 1}));
    assert_eq!(
        shown["ies"][3],
        serde_json::json!({"65000": {"octets": "00"}, "criticality": "reject"})
    );
    // What is not a message, not an IE, or an IE of another message without its
    // criticality, is refused with what is wrong.
    let refused = |tree: serde_json::Value| inspect::message_from_tree(&tree).unwrap_err();
    let named = |message: &str, ie: &str| serde_json::json!({"message": message, "ies": [{ie: 1}]});
    assert!(
        refused(named("UEContextReleaseRequst", "AMF-UE-NGAP-ID")).contains("is not a message")
    );
    assert!(refused(named("UEContextReleaseRequest", "AMF-UE-ID")).contains("is not an IE"));
    assert!(
        refused(named("UEContextReleaseRequest", "UEContextRequest"))
            .contains("says its \"criticality\"")
    );
    assert!(refused(serde_json::json!({"message": "Paging", "ies": [{}]})).contains("one IE"));
    assert!(refused(serde_json::json!({"message": "Paging", "ie": []})).contains("is no member"));
}

/// The members of `value`, a tree or a part of one, that only repeat what its form says:
/// those of an IE or of a transfer in a view, and a `criticality` that the tables could
/// give.
/// `holder` is the name of the member that has `value`, or the list it is in.
fn redundant(value: &serde_json::Value, holder: &str, found: &mut Vec<String>) {
    use serde_json::Value;
    match value {
        Value::Object(members) => {
            for (name, member) in members {
                let wraps = match name.as_str() {
                    "decoded" | "id" | "extensionValue" => true,
                    // A bit string whose size varies has a `value` and a `length`.
                    "value" => !members.contains_key("length"),
                    // No table has the criticality of the IEs of a contained message.
                    "criticality" => holder != "protocolIEs",
                    name => name.starts_with('_'),
                };
                if wraps {
                    found.push(format!("{name} in {holder}"));
                }
                redundant(member, name, found);
            }
        }
        Value::Array(entries) => (entries.iter()).for_each(|entry| redundant(entry, holder, found)),
        _ => {}
    }
}

#[test]
fn a_message_is_shown_and_written_by_name_to_any_depth() {
    use serde_json::json;
    let shown = |wire: &[u8]| inspect::message_tree(&NGAP_PDU::decode(wire).unwrap());
    let written = |tree: &serde_json::Value| {
        inspect::message_from_tree(tree).map(|pdu| pdu.encode().unwrap())
    };
    // A transfer is its decoded value, and the IEs of the message that it contains go
    // by name. Their criticality is written: the tables have that of the IEs of the
    // messages of the procedures.
    let wire = fixture("PDUSessionResourceSetupRequest");
    let tunnel = json!({"gTPTunnel": {
        "transportLayerAddress": {"value": "00", "length": 1},
        "gTP-TEID": 577899919,
    }});
    let flow = json!({"qosFlowIdentifier": 0, "qosFlowLevelQosParameters": {
        "qosCharacteristics": {"nonDynamic5QI": {"fiveQI": 0}},
        "allocationAndRetentionPriority": {
            "priorityLevelARP": 1,
            "pre-emptionCapability": "shall-not-trigger-pre-emption",
            "pre-emptionVulnerability": "not-pre-emptable",
        },
    }});
    let request = json!({
        "message": "PDUSessionResourceSetupRequest",
        "ies": [
            {"AMF-UE-NGAP-ID": 0},
            {"RAN-UE-NGAP-ID": 0},
            {"PDUSessionResourceSetupListSUReq": [{
                "pDUSessionID": 0,
                "s-NSSAI": {"sST": 85},
                "pDUSessionResourceSetupRequestTransfer": {"protocolIEs": [
                    {"UL-NGU-UP-TNLInformation": tunnel, "criticality": "reject"},
                    {"PDUSessionType": "ipv4", "criticality": "reject"},
                    {"QosFlowSetupRequestList": [flow], "criticality": "reject"},
                ]},
            }]},
        ],
    });
    assert_eq!(shown(&wire).unwrap(), request);
    assert_eq!(written(&request).unwrap(), wire);
    // No message of the fixtures is shown with a member that only wraps a value, nor
    // with a criticality that the tables have.
    let mut found = Vec::new();
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        if let Ok(shown) = shown(&hex::decode(fields[1]).unwrap()) {
            redundant(&shown["ies"], fields[0], &mut found);
        }
    }
    assert_eq!(found, Vec::<String>::new());
    // The same message as a view has it, with `decoded` and an entry for each IE, and
    // with the two forms together.
    let transfer = "/ies/2/PDUSessionResourceSetupListSUReq/0\
        /pDUSessionResourceSetupRequestTransfer";
    let mut explicit = request.clone();
    *explicit.pointer_mut(transfer).unwrap() = json!({"decoded": {"protocolIEs": [
        {"id": 139, "criticality": "reject", "value": tunnel},
        {"id": "PDUSessionType", "criticality": "reject", "value": "ipv4"},
        {"QosFlowSetupRequestList": [flow], "criticality": "reject"},
    ]}});
    assert_eq!(written(&explicit).unwrap(), wire);
    // An IE of a contained message says its criticality, and one of an extension
    // container too: `message_tree` shows both.
    let mut unassigned = request.clone();
    let kind = unassigned.pointer_mut(&format!("{transfer}/protocolIEs/1"));
    kind.unwrap().as_object_mut().unwrap().remove("criticality");
    let error = written(&unassigned).unwrap_err();
    assert!(error.contains("Type says its \"criticality\""), "{error}");
    let mut extended = request.clone();
    let item = format!("{transfer}/protocolIEs/2/QosFlowSetupRequestList/0");
    let indicator = json!({"RedundantQosFlowIndicator": "true", "criticality": "ignore"});
    extended.pointer_mut(&item).unwrap()["iE-Extensions"] = json!([indicator]);
    let pdu = inspect::message_from_tree(&extended).unwrap();
    let view = inspect::message_view(&pdu).unwrap();
    let extension = "/ngap/PDUSessionResourceSetupListSUReq/0\
        /pDUSessionResourceSetupRequestTransfer/QosFlowSetupRequestList/0/iE-Extensions/0";
    let value = inspect::select(&view, &format!("{extension}/extensionValue")).unwrap();
    assert_eq!(value, [&json!("true")]);
    assert_eq!(inspect::message_tree(&pdu).unwrap(), extended);
    let indicator = json!({"RedundantQosFlowIndicator": "true"});
    extended.pointer_mut(&item).unwrap()["iE-Extensions"] = json!([indicator]);
    let error = written(&extended).unwrap_err();
    assert!(
        error.contains("Indicator says its \"criticality\""),
        "{error}"
    );
    // An IE that a value holds alone has the criticality of its object set.
    let identity = json!({"pLMNIdentity": "208-93", "tNGF-ID": {"tNGF-ID": "00000001"}});
    let setup = |node: serde_json::Value| {
        let node = json!({"GlobalRANNodeID": node});
        json!({"message": "NGSetupRequest", "ies": [node]})
    };
    let node = setup(json!({"choice-Extensions": {"GlobalTNGF-ID": identity}}));
    let pdu = inspect::message_from_tree(&node).unwrap();
    let view = inspect::message_view(&pdu).unwrap();
    let held = "/ngap/GlobalRANNodeID/choice-Extensions";
    let criticality = inspect::select(&view, &format!("{held}/criticality")).unwrap();
    assert_eq!(criticality, [&json!("reject")]);
    let network = inspect::select(&view, &format!("{held}/pLMNIdentity")).unwrap();
    assert_eq!(network, [&json!("208-93")]);
    assert_eq!(inspect::message_tree(&pdu).unwrap(), node);
    // One that its set does not have says its own, which is shown.
    let cause = json!({"Cause": {"misc": "unspecified"}});
    let error = written(&setup(json!({"choice-Extensions": cause}))).unwrap_err();
    assert!(error.contains("Cause says its \"criticality\""), "{error}");
    let cause = json!({"Cause": {"misc": "unspecified"}, "criticality": "ignore"});
    let node = setup(json!({"choice-Extensions": cause}));
    let pdu = inspect::message_from_tree(&node).unwrap();
    assert_eq!(inspect::message_tree(&pdu).unwrap(), node);
    // A transfer that does not decode has its octets, and so has an IE that contains a
    // type when its octets are not of that type.
    let mut undecoded = request.clone();
    *undecoded.pointer_mut(transfer).unwrap() = json!({"octets": "FF"});
    let pdu = inspect::message_from_tree(&undecoded).unwrap();
    assert_eq!(inspect::message_tree(&pdu).unwrap(), undecoded);
    let release = json!({
        "message": "DistributionReleaseRequest",
        "ies": [{"MBS-DistributionReleaseRequestTransfer": "FF"}],
    });
    let pdu = inspect::message_from_tree(&release).unwrap();
    let shown = inspect::message_tree(&pdu).unwrap();
    let octets = json!({"MBS-DistributionReleaseRequestTransfer": {"octets": "01FF"}});
    assert_eq!(shown["ies"][0], octets);
    assert_eq!(written(&shown).unwrap(), pdu.encode().unwrap());
    // What is in the place of an IE and is none is refused.
    let error = written(&setup(json!({"choice-Extensions": {"a": 1, "b": 2}}))).unwrap_err();
    assert!(error.contains("one IE"), "{error}");
}

#[test]
fn the_value_of_an_ie_and_of_a_transfer_may_be_left_out_of_a_path() {
    use serde_json::{Value, json};
    let name = "PDUSessionResourceSetupRequest";
    let view = inspect::message_view(&NGAP_PDU::decode(&fixture(name)).unwrap()).unwrap();
    let long = "/ngap/PDUSessionResourceSetupListSUReq/value/0\
        /pDUSessionResourceSetupRequestTransfer/decoded/QosFlowSetupRequestList/value/0";
    let transfer = "/ngap/PDUSessionResourceSetupListSUReq/0\
        /pDUSessionResourceSetupRequestTransfer";
    let list = format!("{transfer}/QosFlowSetupRequestList");
    let short = format!("{list}/0");
    // Each of the three is left out alone, and all of them together.
    let flow = inspect::select(&view, &format!("{long}/qosFlowIdentifier")).unwrap();
    assert_eq!(flow, [&json!(0)]);
    for path in [
        long.replacen("/value", "", 1),
        long.replace("/decoded", ""),
        long.replace("/QosFlowSetupRequestList/value", "/QosFlowSetupRequestList"),
        short.clone(),
        short.to_lowercase(),
    ] {
        let selected = inspect::select(&view, &format!("{path}/qosFlowIdentifier"));
        assert_eq!(selected.as_ref(), Ok(&flow), "{path}");
        let checked = inspect::check_path(name, &format!("{path}/qosFlowIdentifier"));
        assert_eq!(checked, Ok(()), "{path}");
    }
    // An IE of the contained message goes by its name, its position or its identifier.
    for ie in ["PDUSessionType", "1", "@id=134"] {
        let kind = inspect::select(&view, &format!("{transfer}/{ie}/value")).unwrap();
        assert_eq!(kind, [&json!("ipv4")], "{ie}");
    }
    // What an IE and a transfer have themselves comes first: `*` is each of their
    // members, and the entries of a value are after `value`.
    let select = |path: &str| inspect::select(&view, path).unwrap();
    let ie = "/ngap/PDUSessionResourceSetupListSUReq";
    assert_eq!(select(&format!("{ie}/criticality")), [&json!("reject")]);
    assert_eq!(select(&format!("{ie}/*")).len(), 3);
    assert_eq!(select(&format!("{ie}/value/*/pDUSessionID")), [&json!(0)]);
    assert!(select(&format!("{transfer}/octets"))[0].is_string());
    assert!(select(&format!("{transfer}/decoded"))[0].is_object());
    // The names of an ENUMERATED are found on the same path.
    let capability = "qosFlowLevelQosParameters/allocationAndRetentionPriority\
        /pre-emptionCapability";
    let names = inspect::enumerated_at(name, &format!("{short}/{capability}")).unwrap();
    assert!(names.is_some_and(|names| names.contains(&"may-trigger-pre-emption")));
    assert_eq!(
        inspect::enumerated_at(name, &format!("{long}/{capability}")),
        Ok(names)
    );
    // Edits take the same paths: a member is set, an entry is added and one taken out,
    // and the octets of the transfer are no longer those of its value.
    let mut edited = view.clone();
    let second = inspect::select(&view, &short).unwrap()[0].clone();
    inspect::set(&mut edited, &format!("{short}/qosFlowIdentifier"), json!(5)).unwrap();
    let error = inspect::select(&edited, &format!("{transfer}/octets")).unwrap_err();
    assert!(error.contains("the value was edited"), "{error}");
    inspect::insert(&mut edited, &format!("{list}/-"), second.clone()).unwrap();
    inspect::insert(&mut edited, &format!("{list}/0"), second).unwrap();
    inspect::set(&mut edited, &format!("{list}/0/e-RAB-ID"), json!(7)).unwrap();
    inspect::remove(&mut edited, &format!("{list}/2")).unwrap();
    inspect::set(
        &mut edited,
        &format!("{transfer}/PDUSessionType/value"),
        json!("ipv6"),
    )
    .unwrap();
    let after = inspect::message_view(&inspect::message_from_view(&edited).unwrap()).unwrap();
    let flows = inspect::select(&after, &format!("{list}/value/*/qosFlowIdentifier")).unwrap();
    assert_eq!(flows, [&json!(0), &json!(5)]);
    let bearers = inspect::select(&after, &format!("{list}/value/*/e-RAB-ID")).unwrap();
    assert_eq!(bearers, [&json!(7)]);
    let kind = inspect::select(&after, &format!("{transfer}/PDUSessionType/value")).unwrap();
    assert_eq!(kind, [&json!("ipv6")]);
    inspect::set(&mut edited, &format!("{list}/0/e-RAB-ID"), Value::Null).unwrap();
    let none = Vec::<&Value>::new();
    assert_eq!(
        inspect::select(&edited, &format!("{list}/0/e-RAB-ID")),
        Ok(none)
    );
    // A name that neither has is refused with the members of both, and the view is as
    // it was.
    let before = edited.clone();
    for (path, value, own) in [
        (
            format!("{list}/0/misspelled"),
            "is not a member of QosFlowSetupRequestItem, which has qosFlowIdentifier",
            None,
        ),
        (
            format!("{transfer}/misspelled"),
            "is not a member of PDUSessionResourceSetupRequestTransfer, which has protocolIEs",
            Some("; a transfer itself has decoded, octets"),
        ),
        (
            format!("{list}/misspelled"),
            "array index must be numeric",
            Some("; an IE itself has id, criticality, value, octets"),
        ),
        (
            "/ngap/RAN-UE-NGAP-ID/valeu".to_string(),
            "\"valeu\" is not a member of this value, which has none",
            Some("; an IE itself has id, criticality, value, octets"),
        ),
    ] {
        for error in [
            inspect::select(&edited, &path).map(|_| ()).unwrap_err(),
            inspect::set(&mut edited, &path, json!(1)).unwrap_err(),
            inspect::check_path(name, &path).unwrap_err(),
        ] {
            assert!(error.contains(value), "{path}: {error}");
            assert_eq!(
                own.is_some_and(|own| error.ends_with(own)),
                own.is_some(),
                "{error}"
            );
            assert_eq!(error.contains("itself"), own.is_some(), "{path}: {error}");
        }
    }
    assert_eq!(edited, before);
    // A member of a value that is named as one of its IE is that of the IE: the `value`
    // of a bit string whose size varies is reached after `value`.
    let address = json!({"value": "0A0000", "length": 24});
    let entry =
        json!({"id": "TraceCollectionEntityIPAddress", "criticality": "ignore", "value": address});
    let view = sent(&[entry]).unwrap();
    let ie = "/ngap/TraceCollectionEntityIPAddress";
    let select = |path: &str| inspect::select(&view, &format!("{ie}/{path}")).unwrap();
    assert_eq!(select("value"), [&address]);
    assert_eq!(select("value/value"), [&json!("0A0000")]);
    assert_eq!(select("length"), [&json!(24)]);
    assert_eq!(select("value/length"), [&json!(24)]);
}

#[test]
fn the_container_of_a_handover_is_written_and_selected_by_the_type_that_it_carries() {
    use serde_json::{Value, json};
    // Between NG-RAN nodes, the Source to Target Transparent Container of a HANDOVER
    // REQUIRED carries a Source NG-RAN Node to Target NG-RAN Node Transparent Container:
    // TS 38.413 clause 9.3.1.20 says so, and the ASN.1 has an OCTET STRING.
    let name = "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer";
    let container = json!({
        "rRCContainer": "0102",
        "targetCell-ID": {"nR-CGI": {"pLMNIdentity": "208-93", "nRCellIdentity": 4660}},
        "uEHistoryInformation": [{"lastVisitedCellInformation": {"eUTRANCell": "00"}}],
    });
    let required = |container: Value| {
        json!({"message": "HandoverRequired", "ies": [
            {"AMF-UE-NGAP-ID": 1},
            {"RAN-UE-NGAP-ID": 2},
            {"SourceToTarget-TransparentContainer": container},
        ]})
    };
    let pdu = inspect::message_from_tree(&required(json!({name: container}))).unwrap();
    // The octets are the APER encoding of the type, which the bindings read.
    let message: HandoverRequired = pdu.decode_value().unwrap();
    let octets = message.protocol_ies.0[2].value.as_bytes();
    let octets: SourceToTargetTransparentContainer =
        decode_open_type(&rasn::types::Any::new(octets.to_vec())).unwrap();
    let carried: SourceNGRANNodeToTargetNGRANNodeTransparentContainer =
        rasn::aper::decode(&octets.0).unwrap();
    assert_eq!(carried.r_rccontainer.0.as_ref(), [1, 2]);
    assert_eq!(rasn::aper::encode(&carried).unwrap(), octets.0.as_ref());
    // Nothing says what the octets carry: a view and a tree have them as they are.
    let mut view = inspect::message_view(&pdu).unwrap();
    let ie = "/ngap/SourceToTarget-TransparentContainer";
    let hexadecimal = json!(hex::encode_upper(&octets.0));
    let value = inspect::select(&view, &format!("{ie}/value")).unwrap();
    assert_eq!(value, [&hexadecimal]);
    let shown = inspect::message_tree(&pdu).unwrap();
    assert_eq!(shown, required(hexadecimal.clone()));
    // A path names the type. The types say whether it can select, and the octets are
    // opened before it does.
    let cell = format!("{ie}/{name}/targetCell-ID/nR-CGI/nRCellIdentity");
    assert_eq!(inspect::check_path("HandoverRequired", &cell), Ok(()));
    let error = inspect::select(&view, &cell).unwrap_err();
    assert!(error.contains("open decodes them"), "{error}");
    inspect::open(&mut view, &cell).unwrap();
    assert_eq!(inspect::select(&view, &cell).unwrap(), [&json!(4660)]);
    // What was written by the type is selected by the type, in any case of its name,
    // and its paths are listed.
    let decoded = inspect::select(&view, &format!("{ie}/{name}/decoded")).unwrap();
    assert_eq!(decoded, [&container]);
    let loose = format!("{ie}/value/{}/rrc_container", name.to_lowercase());
    assert_eq!(inspect::select(&view, &loose).unwrap(), [&json!("0102")]);
    let listed = format!("{ie}/value/{name}/decoded/rRCContainer");
    let paths = inspect::paths(&view);
    assert!(paths.iter().any(|(path, _)| *path == listed), "{paths:?}");
    for (path, value) in &paths {
        assert_eq!(inspect::select(&view, path).unwrap(), [value], "{path}");
    }
    // The octets that were received are kept, and are those of the transfer.
    let received = inspect::select(&view, &format!("{ie}/{name}/octets")).unwrap();
    assert_eq!(received, [&hexadecimal]);
    assert_eq!(inspect::message_from_view(&view).unwrap(), pdu);
    // An edit changes what the view has: octets are opened before it goes into them.
    let mut view = inspect::message_view(&pdu).unwrap();
    for path in [cell.clone(), cell.replace(name, &format!("value/{name}"))] {
        let error = inspect::set(&mut view, &path, json!(7)).unwrap_err();
        assert!(error.contains("open decodes them"), "{error}");
    }
    inspect::open(&mut view, &cell).unwrap();
    inspect::set(&mut view, &cell, json!(7)).unwrap();
    inspect::remove(&mut view, &format!("{ie}/{name}/uEHistoryInformation/0")).unwrap();
    let cells = json!({"lastVisitedCellInformation": {"uTRANCell": "01"}});
    inspect::insert(
        &mut view,
        &format!("{ie}/{name}/uEHistoryInformation/-"),
        cells,
    )
    .unwrap();
    let mut after = inspect::message_view(&inspect::message_from_view(&view).unwrap()).unwrap();
    assert!(inspect::select(&after, &format!("{ie}/value")).unwrap()[0].is_string());
    inspect::open(&mut after, &cell).unwrap();
    assert_eq!(inspect::select(&after, &cell).unwrap(), [&json!(7)]);
    let history = format!("{ie}/{name}/uEHistoryInformation/0/lastVisitedCellInformation");
    let visited = inspect::select(&after, &history).unwrap();
    assert_eq!(visited, [&json!({"uTRANCell": "01"})]);
    // The value of the IE is set by the type, and an IE is added by it: the container
    // of the other direction, and the one of a failure.
    let back = json!({"TargetNGRANNode-ToSourceNGRANNode-TransparentContainer": {
        "rRCContainer": "03",
    }});
    let failure = json!({"targetngrannode_tosourcengrannode_failuretransparentcontainer": {}});
    inspect::set(&mut after, &format!("{ie}/value"), json!({name: container})).unwrap();
    for (id, value) in [
        ("TargetToSource-TransparentContainer", &back),
        ("TargettoSource-Failure-TransparentContainer", &failure),
    ] {
        let entry = json!({"id": id, "criticality": "reject", "value": value});
        inspect::insert(&mut after, "/ngap/-", entry).unwrap();
    }
    let sent = inspect::message_view(&inspect::message_from_view(&after).unwrap()).unwrap();
    let octets = |ie: &str| inspect::select(&sent, &format!("/ngap/{ie}/value")).unwrap();
    assert_eq!(
        octets("SourceToTarget-TransparentContainer"),
        [&hexadecimal]
    );
    assert_eq!(
        octets("TargetToSource-TransparentContainer"),
        [&json!("000103")]
    );
    assert_eq!(
        octets("TargettoSource-Failure-TransparentContainer"),
        [&json!("00")]
    );
    // Octets that are not of the type say so where they are selected, and are sent
    // as they came.
    let other = inspect::message_from_tree(&required(json!("FF"))).unwrap();
    let mut view = inspect::message_view(&other).unwrap();
    inspect::open(&mut view, &cell).unwrap();
    let error = inspect::select(&view, &cell).unwrap_err();
    assert!(error.contains("the transfer did not decode"), "{error}");
    assert_eq!(inspect::message_from_view(&view).unwrap(), other);
    // A path that goes through no such name opens nothing.
    let before = view.clone();
    inspect::open(&mut view, "/ngap/RAN-UE-NGAP-ID/value").unwrap();
    assert_eq!(view, before);
    // The name is one of a type that the IE may carry.
    let written = inspect::message_from_tree(&required(back));
    let error = written.unwrap_err();
    assert!(
        error.contains(&format!("for a type that they carry: {name}")),
        "{error}"
    );
    for path in [
        format!("{ie}/NoSuchType/rRCContainer"),
        format!("{ie}/rRCContainer"),
    ] {
        let error = inspect::check_path("HandoverRequired", &path).unwrap_err();
        assert!(error.contains(&format!("which has {name}, at")), "{error}");
        assert_eq!(inspect::select(&view, &path), Err(error.clone()), "{path}");
        assert_eq!(inspect::open(&mut view, &path), Err(error), "{path}");
    }
}

/// Each value of `shown`, a tree or a part of one, with the path that joins the names
/// and the positions on the way to it. A list of plain values is one value.
fn joined(shown: &serde_json::Value, path: &str, found: &mut Vec<(String, serde_json::Value)>) {
    use serde_json::Value;
    match shown {
        Value::Object(members) if !members.is_empty() => {
            for (name, member) in members {
                joined(member, &format!("{path}/{name}"), found);
            }
        }
        Value::Array(entries) if entries.iter().any(|e| e.is_object() || e.is_array()) => {
            for (index, entry) in entries.iter().enumerate() {
                joined(entry, &format!("{path}/{index}"), found);
            }
        }
        value => found.push((path.to_string(), value.clone())),
    }
}

#[test]
fn the_names_that_a_message_shows_join_into_the_path_of_each_value() {
    use serde_json::json;
    let (mut values, mut whole, mut bits, mut empty) = (0, 0, 0, 0);
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        let pdu = NGAP_PDU::decode(&hex::decode(fields[1]).unwrap()).unwrap();
        // A private message has no IEs of the protocol to name.
        let Ok(shown) = inspect::message_tree(&pdu) else {
            continue;
        };
        let view = inspect::message_view(&pdu).unwrap();
        let ies = shown["ies"].as_array().unwrap();
        for (index, ie) in ies.iter().enumerate() {
            // An IE of the message by its position, then by its name, and by its name
            // alone, which selects it when the message has it once.
            let mut found = Vec::new();
            joined(ie, &format!("/ngap/{index}"), &mut found);
            for (path, value) in found {
                let at = format!("{} {path}", fields[0]);
                let selected = inspect::select(&view, &path).expect(&at);
                assert_eq!(inspect::check_path(fields[0], &path), Ok(()), "{at}");
                let named = path.replacen(&format!("/{index}/"), "/", 1);
                if !named.ends_with("/criticality") || named.matches('/').count() > 3 {
                    assert_eq!(inspect::select(&view, &named), Ok(selected.clone()), "{at}");
                }
                values += 1;
                // A path that ends with the name of an IE selects the IE, as the name
                // alone does in a list: a value that is the whole value of its IE is
                // after `value`.
                if selected[0].get("_raw_value").is_some() {
                    let of_the_ie = inspect::select(&view, &format!("{path}/value")).expect(&at);
                    assert_eq!(of_the_ie, [&value], "{at}");
                    whole += 1;
                    continue;
                }
                // A bit string whose size varies that is the value of an IE has a
                // member that is named as one of the IE: its `value` is after `value`.
                if path.ends_with("/value") && selected[0].get("length").is_some() {
                    let after = inspect::select(&view, &format!("{path}/value")).expect(&at);
                    assert_eq!(after, [&value], "{at}");
                    bits += 1;
                    continue;
                }
                // A transfer whose decoded value has no member is selected whole.
                if value == json!({}) && selected[0].get("_raw_transfer").is_some() {
                    assert_eq!(selected[0]["decoded"], value, "{at}");
                    empty += 1;
                    continue;
                }
                assert_eq!(selected, [&value], "{at}");
            }
        }
    }
    assert!(values > 500 && whole > 100, "{values} {whole}");
    assert!(bits > 0 && empty > 0, "{bits} {empty}");

    // An entry of the message that a transfer contains, by its position then its name.
    let name = "PDUSessionResourceSetupRequest";
    let mut view = inspect::message_view(&NGAP_PDU::decode(&fixture(name)).unwrap()).unwrap();
    let ies = "/ngap/PDUSessionResourceSetupListSUReq/0\
        /pDUSessionResourceSetupRequestTransfer/protocolIEs";
    let tunnel = format!("{ies}/0/UL-NGU-UP-TNLInformation/gTPTunnel/gTP-TEID");
    assert_eq!(
        inspect::select(&view, &tunnel).unwrap(),
        [&json!(577899919)]
    );
    // The name of another IE says which IE the entry is, where a view says it.
    let other = format!("{ies}/0/PDUSessionType/value");
    let error = inspect::select(&view, &other).unwrap_err();
    let which = "this IE is UL-NGU-UP-TNLInformation, not PDUSessionType: ";
    assert!(error.starts_with(which), "{error}");
    assert!(
        error.contains("is not a member of UPTransportLayerInformation"),
        "{error}"
    );
    assert_eq!(inspect::set(&mut view, &other, json!("ipv6")), Err(error));
    // Without a view, the entry at a position is the IE that the segment names, and
    // the path is read against its type.
    assert_eq!(inspect::check_path(name, &tunnel), Ok(()));
    assert_eq!(inspect::check_path(name, &other), Ok(()));
    let wrong = format!("{ies}/0/UL-NGU-UP-TNLInformation/gTPTunel");
    let error = inspect::check_path(name, &wrong).unwrap_err();
    assert!(
        error.contains("is not a member of UPTransportLayerInformation"),
        "{error}"
    );
    let kinds = inspect::enumerated_at(name, &other).unwrap();
    assert!(kinds.is_some_and(|kinds| kinds.contains(&"ipv4v6")));
    // An edit takes the same path, and the entry itself where the path ends with its
    // name.
    inspect::set(&mut view, &tunnel, json!(7)).unwrap();
    inspect::remove(&mut view, &format!("{ies}/1/PDUSessionType")).unwrap();
    let sent = inspect::message_view(&inspect::message_from_view(&view).unwrap()).unwrap();
    assert_eq!(inspect::select(&sent, &tunnel).unwrap(), [&json!(7)]);
    let names = inspect::select(&sent, &format!("{ies}/*/_ie_name")).unwrap();
    assert_eq!(
        names,
        [
            &json!("UL-NGU-UP-TNLInformation"),
            &json!("QosFlowSetupRequestList")
        ]
    );
    // A member of a value that is named as another IE is that member: the target of a
    // handover has a `targetRANNodeID`, and `Target-RAN-Node-ID` is an IE.
    let required = NGAP_PDU::decode(&fixture("HandoverRequired")).unwrap();
    let required = inspect::message_view(&required).unwrap();
    assert!(
        inspect::ie_names()
            .iter()
            .any(|(_, name)| *name == "Target-RAN-Node-ID")
    );
    let area = "/ngap/TargetID/targetRANNodeID/selectedTAI/tAC";
    assert_eq!(inspect::select(&required, area).unwrap(), [&json!(1779096)]);
    assert_eq!(inspect::check_path("HandoverRequired", area), Ok(()));
}

/// The octets of the message `name` of the fixtures.
fn fixture(name: &str) -> Vec<u8> {
    let mut lines = include_str!("fixtures/messages.tsv").lines();
    let line = lines.find(|line| line.split('\t').next() == Some(name));
    hex::decode(line.unwrap().split('\t').nth(1).unwrap()).unwrap()
}

/// The entry of the IE `id` of a view.
fn ie(view: &mut serde_json::Value, id: u16) -> &mut serde_json::Value {
    let ies = view["message"]["protocolIEs"].as_array_mut().unwrap();
    ies.iter_mut().find(|ie| ie["id"] == id).unwrap()
}

/// The view of a message that has `entries` for IEs, encoded and decoded again.
fn sent(entries: &[serde_json::Value]) -> Result<serde_json::Value, String> {
    let pdu = build_ngap!(
        InitiatingMessage,
        UEContextReleaseRequest,
        IGNORE,
        UEContextReleaseRequest,
    )
    .unwrap();
    let mut view = inspect::message_view(&pdu)?;
    *view["message"]["protocolIEs"].as_array_mut().unwrap() = entries.to_vec();
    inspect::message_view(&inspect::message_from_view(&view)?)
}

#[test]
fn well_known_values_are_shown_and_taken_as_they_are_usually_written() {
    use serde_json::json;
    // NG SETUP REQUEST: GlobalRANNodeID is IE 27 and SupportedTAList IE 102.
    let wire = fixture("NGSetupRequest");
    let mut view = inspect::message_view(&NGAP_PDU::decode(&wire).unwrap()).unwrap();
    let area = &ie(&mut view, 102)["value"][0];
    let slice = "/broadcastPLMNList/0/tAISliceSupportList/0/s-NSSAI/sST";
    assert_eq!(area["tAC"], json!(0x60B72F));
    assert_eq!(
        area["broadcastPLMNList"][0]["pLMNIdentity"],
        json!("208-93")
    );
    assert_eq!(area.pointer(slice), Some(&json!(0x59)));
    // What is not edited keeps its octets.
    assert_eq!(
        inspect::message_from_view(&view).unwrap().encode().unwrap(),
        wire
    );

    // Edited as they are shown, a number in hexadecimal too.
    let area = &mut ie(&mut view, 102)["value"][0];
    area["tAC"] = json!(7);
    area["broadcastPLMNList"][0]["pLMNIdentity"] = json!("001-01");
    *area.pointer_mut(slice).unwrap() = json!("0x2A");
    let edited = inspect::message_from_view(&view).unwrap();
    let mut shown = inspect::message_view(&edited).unwrap();
    let area = &ie(&mut shown, 102)["value"][0];
    assert_eq!(area["tAC"], json!(7));
    assert_eq!(
        area["broadcastPLMNList"][0]["pLMNIdentity"],
        json!("001-01")
    );
    assert_eq!(area.pointer(slice), Some(&json!(42)));
    assert_eq!(
        ie(&mut shown, 27)["_raw_value"],
        ie(&mut view, 27)["_raw_value"]
    );

    // As JER writes them, which is what a view had before.
    let area = &mut ie(&mut view, 102)["value"][0];
    area["tAC"] = json!("000007");
    area["broadcastPLMNList"][0]["pLMNIdentity"] = json!("00F110");
    *area.pointer_mut(slice).unwrap() = json!("2A");
    assert_eq!(
        inspect::message_from_view(&view).unwrap().encode().unwrap(),
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
        let mut view = inspect::message_view(&NGAP_PDU::decode(&wire).unwrap()).unwrap();
        let area = &mut ie(&mut view, 102)["value"][0];
        match member {
            "tAC" => area["tAC"] = refused.clone(),
            _ => area["broadcastPLMNList"][0]["pLMNIdentity"] = refused.clone(),
        }
        assert!(
            inspect::message_from_view(&view).is_err(),
            "{member} {refused}"
        );
    }
}

#[test]
fn the_parts_of_a_guami_are_numbers_of_their_bits() {
    use serde_json::json;
    // INITIAL CONTEXT SETUP REQUEST: GUAMI is IE 28.
    let wire = fixture("InitialContextSetupRequest");
    let mut view = inspect::message_view(&NGAP_PDU::decode(&wire).unwrap()).unwrap();
    let guami = &mut ie(&mut view, 28)["value"];
    assert_eq!(guami["aMFSetID"], json!(0));
    guami["pLMNIdentity"] = json!("310-410");
    guami["aMFRegionID"] = json!(0xCA);
    guami["aMFSetID"] = json!(0x3FF);
    guami["aMFPointer"] = json!(1);
    let edited = inspect::message_from_view(&view).unwrap();
    let mut shown = inspect::message_view(&edited).unwrap();
    // The same GUAMI as the crate's helper builds from these numbers.
    let network = oxirush_ngap::helpers::plmn("310", "410").unwrap();
    let expected = oxirush_ngap::helpers::guami(network, 0xCA, 0x3FF, 1).unwrap();
    let expected = encode_open_type(&expected).unwrap();
    assert_eq!(
        ie(&mut shown, 28)["_raw_value"],
        json!(hex::encode_upper(expected.as_bytes()))
    );
    assert_eq!(ie(&mut shown, 28)["value"]["aMFSetID"], json!(0x3FF));
    // The AMF Set ID has ten bits.
    ie(&mut view, 28)["value"]["aMFSetID"] = json!(0x400);
    assert!(inspect::message_from_view(&view).is_err());
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
        // As JER writes it, which is what a view had before.
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

/// A UE CONTEXT RELEASE REQUEST: two UE NGAP IDs and a cause.
fn release_request() -> NGAP_PDU {
    build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(Cause::radioNetwork(CauseRadioNetwork::user_inactivity)),
    )
    .unwrap()
}

#[test]
fn the_ies_of_a_message_have_paths_by_their_names() {
    use serde_json::json;
    let view = inspect::message_view(&release_request()).unwrap();
    let paths = inspect::paths(&view);
    let printed: Vec<_> = (paths.iter())
        .map(|(path, value)| format!("{path} = {value}"))
        .collect();
    assert_eq!(
        printed,
        [
            "/criticality = \"ignore\"",
            "/direction = \"InitiatingMessage\"",
            "/ngap/AMF-UE-NGAP-ID/criticality = \"reject\"",
            "/ngap/AMF-UE-NGAP-ID/id = 10",
            "/ngap/AMF-UE-NGAP-ID/value = 1",
            "/ngap/RAN-UE-NGAP-ID/criticality = \"reject\"",
            "/ngap/RAN-UE-NGAP-ID/id = 85",
            "/ngap/RAN-UE-NGAP-ID/value = 7",
            "/ngap/Cause/criticality = \"ignore\"",
            "/ngap/Cause/id = 15",
            "/ngap/Cause/value/radioNetwork = \"user-inactivity\"",
            "/procedure_code = 42",
        ]
    );
    // A name is taken in any case, and an IE also by its identifier or its position.
    for path in [
        "/ngap/RAN-UE-NGAP-ID/value",
        "/ngap/ran_ue_ngap_id/value",
        "/ngap/@id=85/value",
        "/ngap/1/value",
    ] {
        assert_eq!(inspect::select(&view, path).unwrap(), [&json!(7)], "{path}");
    }
    // The octets that an IE was received as.
    let octets = inspect::select(&view, "/ngap/Cause/octets").unwrap();
    assert_eq!(octets, [&json!("0500")]);
    assert_eq!(inspect::select(&view, "/ngap/*/id").unwrap().len(), 3);
}

#[test]
fn every_path_of_every_canonical_procedure_selects_its_value() {
    let mut shorter = 0;
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        let pdu = NGAP_PDU::decode(&hex::decode(fields[1]).unwrap()).unwrap();
        let view = inspect::message_view(&pdu).unwrap();
        for (path, value) in inspect::paths(&view) {
            // The list that holds the IEs of a message is never named.
            assert!(!path.contains("protocolIEs"), "{} {path}", fields[0]);
            let selected = inspect::select(&view, &path);
            assert_eq!(selected, Ok(vec![&value]), "{} {path}", fields[0]);
            // The path that leaves out what it may selects the same value.
            let short = short(&view, &path);
            let selected = inspect::select(&view, &short);
            assert_eq!(selected, Ok(vec![&value]), "{} {short}", fields[0]);
            shorter += usize::from(short != path);
        }
    }
    assert!(shorter > 300, "{shorter}");
}

/// `path` of `view` without the `value` of each IE and the `decoded` of each transfer on
/// its way, which a path may leave out. One that the path ends with stays, and so does
/// one before a member of the value that is named as a member of the IE or of the
/// transfer.
fn short(view: &serde_json::Value, path: &str) -> String {
    let segments: Vec<&str> = path.split('/').skip(1).collect();
    let mut short = String::new();
    for (at, segment) in segments.iter().enumerate() {
        // What has the segment, by the path so far.
        let holder = match at {
            0 => None,
            at => inspect::select(view, &format!("/{}", segments[..at].join("/")))
                .unwrap()
                .pop(),
        };
        let has = |member: &str| holder.is_some_and(|holder| holder.get(member).is_some());
        let own: &[&str] = match *segment {
            "value" if has("_raw_value") => &["id", "criticality", "value", "octets"],
            "extensionValue" if has("_raw_value") => {
                &["id", "criticality", "extensionValue", "octets"]
            }
            "decoded" if has("_raw_transfer") => &["decoded", "octets"],
            _ => &[],
        };
        let left_out = !own.is_empty()
            && (segments.get(at + 1))
                .is_some_and(|next| !own.contains(next) && !next.starts_with('_'));
        if !left_out {
            short.push('/');
            short.push_str(segment);
        }
    }
    short
}

#[test]
fn an_ie_that_is_there_twice_goes_by_its_position() {
    use serde_json::json;
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        REJECT AMF_UE_NGAP_ID(2u64),
    )
    .unwrap();
    let mut view = inspect::message_view(&pdu).unwrap();
    let paths = inspect::paths(&view);
    let values: Vec<_> = (paths.iter())
        .filter(|(path, _)| path.ends_with("/value"))
        .map(|(path, value)| (path.as_str(), value))
        .collect();
    assert_eq!(
        values,
        [
            ("/ngap/0/value", &json!(1)),
            ("/ngap/RAN-UE-NGAP-ID/value", &json!(7)),
            ("/ngap/2/value", &json!(2)),
        ]
    );
    // The name selects each of them, in the order of the message.
    let both = inspect::select(&view, "/ngap/AMF-UE-NGAP-ID/value").unwrap();
    assert_eq!(both, [&json!(1), &json!(2)]);
    inspect::set(&mut view, "/ngap/2/value", json!(3)).unwrap();
    let edited = inspect::message_from_view(&view).unwrap();
    let view = inspect::message_view(&edited).unwrap();
    let both = inspect::select(&view, "/ngap/AMF-UE-NGAP-ID/value").unwrap();
    assert_eq!(both, [&json!(1), &json!(3)]);
}

#[test]
fn a_value_set_at_a_path_changes_the_octets_of_its_ie_alone() {
    use serde_json::json;
    let mut view = inspect::message_view(&release_request()).unwrap();
    let octets = |view: &serde_json::Value| -> Vec<serde_json::Value> {
        let octets = inspect::select(view, "/ngap/*/octets").unwrap();
        octets.into_iter().cloned().collect()
    };
    let before = octets(&view);
    inspect::set(&mut view, "/ngap/RAN-UE-NGAP-ID/value", json!(9)).unwrap();
    let cause = json!("Release Due To NGRAN Generated Reason");
    inspect::set(&mut view, "/ngap/Cause/value/radioNetwork", cause).unwrap();
    let view = inspect::message_view(&inspect::message_from_view(&view).unwrap()).unwrap();
    let after = octets(&view);
    assert_eq!(after[0], before[0]);
    assert_eq!(after[1], json!("0009"));
    assert_ne!(after[2], before[2]);
    let cause = inspect::select(&view, "/ngap/Cause/value/radioNetwork").unwrap();
    assert_eq!(cause, [&json!("release-due-to-ngran-generated-reason")]);
}

#[test]
fn ies_are_added_taken_out_and_given_their_octets_at_a_path() {
    use serde_json::json;
    let mut view = inspect::message_view(&release_request()).unwrap();
    // An IE by the name of its identifier, with its typed value, at the end.
    let name = json!({"id": "RANNodeName", "criticality": "ignore", "value": "gnb-1"});
    inspect::insert(&mut view, "/ngap/-", name).unwrap();
    // Given octets as an IE, before the cause.
    let unknown = json!({"id": 60000, "criticality": "ignore", "octets": "C0FFEE"});
    inspect::insert(&mut view, "/ngap/Cause", unknown).unwrap();
    inspect::remove(&mut view, "/ngap/AMF-UE-NGAP-ID").unwrap();
    inspect::set(&mut view, "/ngap/RAN-UE-NGAP-ID/octets", json!("400102")).unwrap();
    let view = inspect::message_view(&inspect::message_from_view(&view).unwrap()).unwrap();
    let ids: Vec<_> = (inspect::select(&view, "/ngap/*/id").unwrap().into_iter())
        .map(|id| id.as_u64().unwrap())
        .collect();
    assert_eq!(ids, [85, 60000, 15, 82]);
    let select = |path: &str| inspect::select(&view, path).unwrap();
    assert_eq!(select("/ngap/RAN-UE-NGAP-ID/value"), [&json!(258)]);
    assert_eq!(select("/ngap/RANNodeName/value"), [&json!("gnb-1")]);
    assert_eq!(select("/ngap/@id=60000/octets"), [&json!("C0FFEE")]);
    // An IE that the message does not have is nothing, not an error.
    assert!(select("/ngap/AMF-UE-NGAP-ID/value").is_empty());
}

#[test]
fn the_ies_of_a_contained_message_go_by_name_in_its_place() {
    use serde_json::json;
    let pdu = NGAP_PDU::decode(&fixture("PDUSessionResourceSetupRequest")).unwrap();
    let mut view = inspect::message_view(&pdu).unwrap();
    let transfer = "/ngap/PDUSessionResourceSetupListSUReq/value/0\
        /pDUSessionResourceSetupRequestTransfer/decoded";
    let paths = inspect::paths(&view);
    let contained: Vec<_> = (paths.iter())
        .filter_map(|(path, _)| path.strip_prefix(transfer))
        .collect();
    assert!(!contained.is_empty());
    // The list that holds them is never named.
    assert!(paths.iter().all(|(path, _)| !path.contains("protocolIEs")));
    let ie = contained[0].split('/').nth(1).unwrap();
    assert!(
        inspect::ie_names().iter().any(|(_, name)| *name == ie),
        "{ie}"
    );
    // An IE of the contained message is selected as one of the PDU is, and the path
    // that names the list is taken too.
    let ids = |view: &serde_json::Value, path: &str| -> Vec<serde_json::Value> {
        let ids = inspect::select(view, &format!("{transfer}{path}")).unwrap();
        ids.into_iter().cloned().collect()
    };
    let all = ids(&view, "/*/id");
    assert_eq!(all, ids(&view, "/protocolIEs/*/id"));
    for path in [
        format!("/{ie}/id"),
        format!("/{}/id", ie.to_lowercase()),
        "/0/id".into(),
        format!("/@id={}/id", all[0]),
        format!("/protocolIEs/{ie}/id"),
    ] {
        assert_eq!(ids(&view, &path), [all[0].clone()], "{path}");
    }
    // Edits take the same paths: the criticality of one IE, then the IE itself.
    let criticality = format!("{transfer}/{ie}/criticality");
    inspect::set(&mut view, &criticality, json!("ignore")).unwrap();
    let edited = inspect::select(&view, &criticality).unwrap();
    assert_eq!(edited, [&json!("ignore")]);
    inspect::remove(&mut view, &format!("{transfer}/{ie}")).unwrap();
    assert_eq!(ids(&view, "/*/id").len(), all.len() - 1);
    let unknown = json!({"id": 60000, "criticality": "ignore", "octets": "00"});
    inspect::insert(&mut view, &format!("{transfer}/-"), unknown).unwrap();
    assert_eq!(ids(&view, "/*/id").last(), Some(&json!(60000)));
    let error = inspect::select(&view, &format!("{transfer}/misspelled")).unwrap_err();
    let reason = "is not a member of PDUSessionResourceSetupRequestTransfer";
    assert!(error.contains(reason), "{error}");
}

#[test]
fn a_path_that_names_nothing_is_refused_and_changes_nothing() {
    use serde_json::json;
    let mut view = inspect::message_view(&release_request()).unwrap();
    let before = view.clone();
    for (path, reason) in [
        ("/ngap/NoSuchIE/value", "\"NoSuchIE\" is not an IE of NGAP"),
        (
            "/ngap/Cause/value/misspelled",
            "\"misspelled\" is not a member of Cause, which has radioNetwork, transport, nas, \
             protocol, misc, choice-Extensions",
        ),
        (
            "/ngap/Cause/value/radioNetwork/deeper",
            "\"deeper\" is not a member of this value, which has none",
        ),
        (
            "/misspelled",
            "\"misspelled\" is not a member of a view, which has procedure_code, direction, \
             criticality, message",
        ),
        ("/ngap/+0/id", "\"+0\" is not an IE of NGAP"),
        ("/ngap/00/id", "\"00\" is not an IE of NGAP"),
        ("ngap/Cause", "must start with /"),
        ("/ngap/@id=70000", "IE id must be u16"),
    ] {
        let error = inspect::select(&view, path).unwrap_err();
        assert!(error.contains(reason), "{path}: {error}");
    }
    for (path, reason) in [
        ("/ngap/NoSuchIE/value", "is not an IE of NGAP"),
        ("/ngap/GUAMI/value", "selected no field"),
        ("/ngap/7/value", "selected no field"),
    ] {
        let error = inspect::set(&mut view, path, json!(1)).unwrap_err();
        assert!(error.contains(reason), "{path}: {error}");
    }
    assert!(inspect::remove(&mut view, "/ngap/GUAMI").is_err());
    let both = json!({"id": "Cause", "criticality": "ignore", "value": 1, "octets": "00"});
    assert!(inspect::insert(&mut view, "/ngap/-", both).is_err());
    let unnamed = json!({"id": "NoSuchIE", "criticality": "ignore", "octets": "00"});
    assert!(inspect::insert(&mut view, "/ngap/-", unnamed).is_err());
    assert_eq!(view, before);
}

#[test]
fn a_message_goes_by_the_name_that_asn1_gives_it() {
    use serde_json::json;
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        let pdu = NGAP_PDU::decode(&hex::decode(fields[1]).unwrap()).unwrap();
        assert_eq!(inspect::message_name(&pdu), Some(fields[0]));
    }
    // A name finds its message again, however it is written, and no other does.
    for (direction, code, name) in inspect::message_names() {
        let found = Some((direction, code));
        assert_eq!(inspect::message_named(name), found, "{name}");
        assert_eq!(inspect::message_named(&name.to_lowercase()), found);
        assert_eq!(inspect::message_named(&name.replace('-', "")), found);
    }
    assert_eq!(inspect::message_named("NoSuchMessage"), None);
    // The view of a message that is written from its name.
    let (direction, procedure_code) = inspect::message_named("ue context release request").unwrap();
    let mut view = json!({
        "direction": direction,
        "procedure_code": procedure_code,
        "criticality": "ignore",
        "message": {"protocolIEs": []},
    });
    inspect::insert(
        &mut view,
        "/ngap/-",
        json!({"id": "RAN-UE-NGAP-ID", "criticality": "reject", "value": 7}),
    )
    .unwrap();
    let pdu = inspect::message_from_view(&view).unwrap();
    assert_eq!(inspect::message_name(&pdu), Some("UEContextReleaseRequest"));
    assert!(pdu.is_initiating());
}

#[test]
fn an_edit_changes_what_is_sent_or_is_refused() {
    use serde_json::{Value, json};
    let mut view = inspect::message_view(&release_request()).unwrap();
    let before = view.clone();
    // The value of an IE is not taken out: its octets would go out as they came.
    for edit in [
        inspect::remove(&mut view, "/ngap/Cause/value"),
        inspect::set(&mut view, "/ngap/Cause/value", Value::Null),
    ] {
        let error = edit.unwrap_err();
        assert!(error.contains("value is not taken out"), "{error}");
    }
    assert_eq!(view, before);
    // `*` is each member of a value, without what the view keeps of what came.
    let members = inspect::select(&view, "/ngap/Cause/*").unwrap();
    assert_eq!(members.len(), 3, "{members:?}");
    // The octets of an IE whose value was edited are no longer its octets.
    inspect::set(&mut view, "/ngap/RAN-UE-NGAP-ID/value", json!(9)).unwrap();
    let error = inspect::select(&view, "/ngap/RAN-UE-NGAP-ID/octets").unwrap_err();
    assert!(error.contains("the value was edited"), "{error}");
    let other = inspect::select(&view, "/ngap/AMF-UE-NGAP-ID/octets").unwrap();
    assert_eq!(other.len(), 1);
    assert!(
        inspect::paths(&view)
            .iter()
            .all(|(path, _)| !path.contains("_edited"))
    );
    let sent = inspect::message_view(&inspect::message_from_view(&view).unwrap()).unwrap();
    let octets = inspect::select(&sent, "/ngap/RAN-UE-NGAP-ID/octets").unwrap();
    assert_eq!(octets, [&json!("0009")]);
    // Octets that are set are those of the IE again.
    inspect::set(&mut view, "/ngap/RAN-UE-NGAP-ID/octets", json!("0007")).unwrap();
    let octets = inspect::select(&view, "/ngap/RAN-UE-NGAP-ID/octets").unwrap();
    assert_eq!(octets, [&json!("0007")]);
    assert_eq!(
        inspect::message_from_view(&view).unwrap(),
        release_request()
    );
    // The name of an ENUMERATED value of the PDU itself is taken as any other.
    inspect::set(&mut view, "/criticality", json!("Reject")).unwrap();
    let pdu = inspect::message_from_view(&view).unwrap();
    assert_eq!(
        inspect::message_view(&pdu).unwrap()["criticality"],
        "reject"
    );
}

#[test]
fn null_takes_an_optional_member_out_and_a_transfer_has_its_octets() {
    use serde_json::{Value, json};
    let pdu = NGAP_PDU::decode(&fixture("PDUSessionResourceSetupRequest")).unwrap();
    let view = inspect::message_view(&pdu).unwrap();
    let item = "/ngap/PDUSessionResourceSetupListSUReq/value/0";
    let transfer = format!("{item}/pDUSessionResourceSetupRequestTransfer");
    let sent = |view: &Value| inspect::message_view(&inspect::message_from_view(view)?);
    // An optional member that is there is taken out, and one that is not is not
    // selected.
    let sd = format!("{item}/s-NSSAI/sD");
    let mut edited = view.clone();
    inspect::set(&mut edited, &sd, json!("010203")).unwrap();
    let mut edited = sent(&edited).unwrap();
    assert_eq!(inspect::select(&edited, &sd).unwrap(), [&json!("010203")]);
    inspect::set(&mut edited, &sd, Value::Null).unwrap();
    let none = Vec::<&Value>::new();
    assert_eq!(inspect::select(&sent(&edited).unwrap(), &sd).unwrap(), none);
    let error = inspect::set(&mut edited, &sd, Value::Null).unwrap_err();
    assert!(error.contains("selected no field"), "{error}");
    // The octets of a transfer are selected and set as those of an IE are.
    let octets = format!("{transfer}/octets");
    let received = inspect::select(&view, &octets).unwrap()[0].clone();
    assert!(received.as_str().is_some_and(|octets| octets.len() > 8));
    let mut edited = view.clone();
    inspect::set(&mut edited, &octets, json!("00")).unwrap();
    let after = sent(&edited).unwrap();
    assert_eq!(inspect::select(&after, &octets).unwrap(), [&json!("00")]);
    // Its decoded value is not taken out, and what is under it is no longer its octets.
    let mut edited = view.clone();
    let error = inspect::remove(&mut edited, &format!("{transfer}/decoded")).unwrap_err();
    assert!(error.contains("decoded is not taken out"), "{error}");
    let kind = format!("{transfer}/decoded/PDUSessionType/value");
    inspect::set(&mut edited, &kind, json!("ipv6")).unwrap();
    let error = inspect::select(&edited, &octets).unwrap_err();
    assert!(error.contains("the value was edited"), "{error}");
    let after = sent(&edited).unwrap();
    assert_eq!(inspect::select(&after, &kind).unwrap(), [&json!("ipv6")]);
    assert_ne!(inspect::select(&after, &octets).unwrap(), [&received]);
    // A member that a transfer does not have is refused, not left out.
    let mut edited = view.clone();
    let raw = "/message/protocolIEs/2/value/0/pDUSessionResourceSetupRequestTransfer";
    edited.pointer_mut(raw).unwrap()["octets"] = json!("00");
    let error = inspect::message_from_view(&edited).unwrap_err();
    assert!(error.contains("has no member \"octets\""), "{error}");
}

#[test]
fn what_is_nested_deeper_than_the_inspection_goes_keeps_its_octets() {
    // A cause in the extension of a cause, forty times.
    let mut cause = Cause::misc(CauseMisc::unspecified);
    for _ in 0..40 {
        let value = encode_open_type(&cause).unwrap();
        cause = Cause::choice_Extensions(ProtocolIEField::new(
            ProtocolIEID(15),
            Criticality::ignore,
            value,
        ));
    }
    let pdu = build_ngap!(InitiatingMessage, UEContextReleaseRequest,
        IGNORE, UEContextReleaseRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(7u32),
        IGNORE Cause(cause),
    )
    .unwrap();
    let pdu = NGAP_PDU::decode(&pdu.encode().unwrap()).unwrap();
    let view = inspect::message_view(&pdu).unwrap();
    let paths = inspect::paths(&view);
    let (deepest, why) = (paths.iter())
        .find(|(path, _)| path.ends_with("/_decode_error"))
        .expect("an IE that was not decoded");
    assert!(why.as_str().unwrap().contains("nesting exceeds"), "{why}");
    assert!(
        deepest.matches("choice-Extensions").count() > 20,
        "{deepest}"
    );
    // The other IEs are read, and the PDU is sent as it came.
    let ran = inspect::select(&view, "/ngap/RAN-UE-NGAP-ID/value").unwrap();
    assert_eq!(ran, [&serde_json::json!(7)]);
    assert_eq!(inspect::message_from_view(&view).unwrap(), pdu);
}

#[test]
fn a_message_has_the_ies_of_its_object_set() {
    let id = |name: &str| {
        let mut names = inspect::ie_names().iter();
        names.find(|(_, known)| *known == name).unwrap().0
    };
    let ies = inspect::message_ies("InitialContextSetupRequest").unwrap();
    assert_eq!(ies[0], (id("AMF-UE-NGAP-ID"), true));
    assert!(ies.contains(&(id("PDUSessionResourceSetupListCxtReq"), false)));
    // The presence of an IE that is there on a condition is not mandatory.
    assert!(ies.contains(&(id("UEAggregateMaximumBitRate"), false)));
    let listed = |name: &str| ies.iter().any(|(ie, _)| *ie == id(name));
    assert!(!listed("PDUSessionResourceSetupListSUReq"));
    // A name is taken as that of a message is, and a message without IEs has none.
    assert_eq!(
        inspect::message_ies("initial-context-setup-request"),
        Some(ies)
    );
    assert_eq!(inspect::message_ies("PrivateMessage"), Some(&[][..]));
    assert_eq!(inspect::message_ies("NoSuchMessage"), None);
    // Every message has its IEs in the order of its set, each of them known.
    for (.., name) in inspect::message_names() {
        let ies = inspect::message_ies(name).unwrap();
        assert!(name == "PrivateMessage" || !ies.is_empty(), "{name}");
        for (ie, _) in ies {
            assert!(
                inspect::ie_names().iter().any(|(known, _)| known == ie),
                "{name} {ie}"
            );
        }
    }
}

#[test]
fn a_message_has_the_criticality_that_asn1_assigns() {
    let id = |name: &str| {
        let mut names = inspect::ie_names().iter();
        names.find(|(_, known)| *known == name).unwrap().0
    };
    let request = inspect::message_ie_criticalities("InitialContextSetupRequest").unwrap();
    assert_eq!(request[0], (10, "reject"));
    assert_eq!(request[0].0, id("AMF-UE-NGAP-ID"));
    let assistance = id("CoreNetworkAssistanceInformationForInactive");
    assert!(request.contains(&(assistance, "ignore")));
    // An IE has the criticality that the set of each message gives it.
    let response = inspect::message_ie_criticalities("InitialContextSetupResponse").unwrap();
    assert_eq!(response[0], (id("AMF-UE-NGAP-ID"), "ignore"));
    // A name is taken as that of a message is, and a message without IEs has none.
    assert_eq!(
        inspect::message_ie_criticalities("initial-context-setup-request"),
        Some(request)
    );
    assert_eq!(
        inspect::message_ie_criticalities("PrivateMessage"),
        Some(&[][..])
    );
    assert_eq!(inspect::message_ie_criticalities("NoSuchMessage"), None);
    // The procedure has its own, which is that of each of its messages.
    let procedure = inspect::message_criticality;
    assert_eq!(procedure("InitialContextSetupRequest"), Some("reject"));
    assert_eq!(procedure("initial-context-setup-response"), Some("reject"));
    assert_eq!(procedure("Paging"), Some("ignore"));
    assert_eq!(procedure("ErrorIndication"), Some("ignore"));
    assert_eq!(procedure("NoSuchMessage"), None);
    // Every message has the IEs of its set in their order, each with a criticality.
    let known = ["reject", "ignore", "notify"];
    for (.., name) in inspect::message_names() {
        let ies = inspect::message_ies(name).unwrap().iter().map(|(ie, _)| ie);
        let assigned = inspect::message_ie_criticalities(name).unwrap();
        assert!(ies.eq(assigned.iter().map(|(ie, _)| ie)), "{name}");
        for (ie, criticality) in assigned {
            assert!(known.contains(criticality), "{name} {ie}");
        }
        assert!(known.contains(&procedure(name).unwrap()), "{name}");
    }
}

#[test]
fn the_messages_of_the_fixtures_have_the_criticality_that_asn1_assigns() {
    let mut ies = 0;
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let fields: Vec<_> = line.split('\t').collect();
        let pdu = NGAP_PDU::decode(&hex::decode(fields[1]).unwrap()).unwrap();
        let view = inspect::message_view(&pdu).unwrap();
        let procedure = inspect::message_criticality(fields[0]);
        assert_eq!(view["criticality"].as_str(), procedure, "{}", fields[0]);
        let assigned = inspect::message_ie_criticalities(fields[0]).unwrap();
        // A private message has no IEs of an object set.
        let entries = view["message"]["protocolIEs"].as_array();
        for ie in entries.into_iter().flatten() {
            let id = ie["id"].as_u64().unwrap();
            let mut assigned = assigned.iter().filter(|(known, _)| u64::from(*known) == id);
            let assigned = assigned.next().map(|(_, criticality)| *criticality);
            assert_eq!(ie["criticality"].as_str(), assigned, "{}", fields[0]);
            ies += 1;
        }
    }
    assert!(ies > 300, "{ies}");
}

#[test]
fn an_ie_that_a_value_holds_alone_has_the_criticality_of_its_object_sets() {
    let id = |name: &str| {
        let mut names = inspect::ie_names().iter();
        names.find(|(_, known)| *known == name).unwrap().0
    };
    // The alternatives that extend the CHOICE of a node and that of a location.
    let item = |name: &str| inspect::item_criticality(id(name));
    assert_eq!(inspect::item_criticality(240), Some("reject"));
    assert_eq!(item("GlobalTNGF-ID"), Some("reject"));
    assert_eq!(item("UserLocationInformationTNGF"), Some("ignore"));
    // The IEs of a message and those of a transfer are in no single container.
    assert_eq!(item("AMF-UE-NGAP-ID"), None);
    assert_eq!(item("PDUSessionType"), None);
    assert_eq!(inspect::item_criticality(u16::MAX), None);
}

#[test]
fn the_identifiers_of_the_registry_are_the_constants_of_the_bindings() {
    use std::collections::BTreeMap;
    let letters = |name: &str| -> String {
        let letters = name.chars().filter(char::is_ascii_alphanumeric);
        letters.flat_map(char::to_lowercase).collect()
    };
    // `pub const ID_<NAME>: <Type> = <Type>(<number>);`, as the compiler writes them.
    let mut constants: BTreeMap<(String, String), u64> = BTreeMap::new();
    for constant in include_str!("../src/ngap.rs")
        .split("pub const ID_")
        .skip(1)
    {
        let constant: String = constant
            .split(';')
            .next()
            .unwrap()
            .split_whitespace()
            .collect();
        let (name, value) = constant.split_once(':').unwrap();
        let (kind, number) = value.split_once('=').unwrap();
        let number = number.trim_start_matches(kind).trim_matches(['(', ')']);
        constants.insert((kind.into(), letters(name)), number.parse().unwrap());
    }
    for (id, name) in inspect::ie_names() {
        let constant = constants.get(&("ProtocolIEID".into(), letters(name)));
        assert_eq!(constant, Some(&u64::from(*id)), "{name}");
    }
    // The procedures of the list: `<code> <Name> <criticality> {`.
    let mut procedures = 0;
    for line in include_str!("../src/registry.rs").lines() {
        let words: Vec<_> = line.split_whitespace().take(4).collect();
        let [code, name, _, "{"] = words[..] else {
            continue;
        };
        let constant = constants.get(&("ProcedureCode".into(), letters(name)));
        assert_eq!(constant, Some(&code.parse().unwrap()), "{name}");
        procedures += 1;
    }
    assert!(procedures > 50, "{procedures}");
}

#[test]
fn a_kind_that_the_specification_does_not_have_says_the_direction_of_its_pdu() {
    let pdu = NGAP_PDU::unsuccessfulOutcome(UnsuccessfulOutcome::new(
        ProcedureCode(46),
        Criticality::ignore,
        rasn::types::Any::new(vec![0]),
    ));
    let oxirush_ngap::NgapPduKind::Other {
        direction,
        procedure_code,
    } = pdu.kind()
    else {
        panic!("{:?}", pdu.kind());
    };
    assert_eq!((direction, procedure_code), (pdu.direction(), 46));
}

#[test]
fn a_member_that_is_absent_selects_nothing_and_one_that_its_type_has_not_is_an_error() {
    use serde_json::{Value, json};
    let view = inspect::message_view(&release_request()).unwrap();
    let none = Vec::<&Value>::new();
    let select = |path: &str| inspect::select(&view, path);
    // The alternative of a CHOICE that is there, and another one.
    let cause = json!("user-inactivity");
    assert_eq!(select("/ngap/Cause/value/radioNetwork").unwrap(), [&cause]);
    assert_eq!(select("/ngap/Cause/value/nas").unwrap(), none);
    // What follows an IE or a member that is not there is still a path of its type.
    assert_eq!(select("/ngap/GUAMI/value/aMFRegionID").unwrap(), none);
    let error = select("/ngap/GUAMI/value/misspelled").unwrap_err();
    let members = "is not a member of GUAMI, which has pLMNIdentity, aMFRegionID, aMFSetID, \
                   aMFPointer, iE-Extensions";
    assert!(error.contains(members), "{error}");
    let error = select("/ngap/Cause/value/nas/deeper").unwrap_err();
    assert!(error.contains("which has none"), "{error}");
    // After `*`, a member is an error when no value can have it.
    assert_eq!(select("/ngap/*/value/radioNetwork").unwrap(), [&cause]);
    assert_eq!(select("/ngap/*/value/*").unwrap(), [&cause]);
    let error = select("/ngap/*/value/misspelled").unwrap_err();
    assert!(error.contains("is not a member of"), "{error}");
    // A list of values has no IEs to name.
    let pdu = NGAP_PDU::decode(&fixture("PDUSessionResourceSetupRequest")).unwrap();
    let setup = inspect::message_view(&pdu).unwrap();
    let item = "/ngap/PDUSessionResourceSetupListSUReq/value";
    assert_eq!(
        inspect::select(&setup, &format!("{item}/0/s-NSSAI/sD")).unwrap(),
        none
    );
    let error = inspect::select(&setup, &format!("{item}/Cause/value")).unwrap_err();
    assert!(error.contains("go by position"), "{error}");
    let error = inspect::select(&setup, &format!("{item}/0/s-NSSAI/sd-typo")).unwrap_err();
    assert!(
        error.contains("is not a member of SNSSAI, which has sST, sD, iE-Extensions"),
        "{error}"
    );
}

#[test]
fn an_edit_names_a_member_that_the_type_has_and_keeps_what_it_always_has() {
    use serde_json::{Value, json};
    let mut view = inspect::message_view(&release_request()).unwrap();
    let before = view.clone();
    let error = inspect::set(&mut view, "/ngap/Cause/value/misspelled", json!(1)).unwrap_err();
    assert!(error.contains("is not a member of Cause"), "{error}");
    let error = inspect::remove(&mut view, "/ngap/Cause/value/radioNetwork").unwrap_err();
    assert!(error.contains("a CHOICE has one alternative"), "{error}");
    let error = inspect::remove(&mut view, "/ngap/Cause/criticality").unwrap_err();
    assert!(error.contains("its type always has it"), "{error}");
    assert_eq!(view, before);
    // An alternative takes the place of the one that is there.
    inspect::set(&mut view, "/ngap/Cause/value/nas", json!("normal-release")).unwrap();
    let sent = inspect::message_view(&inspect::message_from_view(&view).unwrap()).unwrap();
    let cause = inspect::select(&sent, "/ngap/Cause/value").unwrap();
    assert_eq!(cause, [&json!({"nas": "normal-release"})]);
    // A member that the type always has stays, and an optional one is added by its name.
    let pdu = NGAP_PDU::decode(&fixture("PDUSessionResourceSetupRequest")).unwrap();
    let mut setup = inspect::message_view(&pdu).unwrap();
    let slice = "/ngap/PDUSessionResourceSetupListSUReq/value/0/s-NSSAI";
    let error = inspect::remove(&mut setup, &format!("{slice}/sST")).unwrap_err();
    assert!(error.contains("its type always has it"), "{error}");
    let error = inspect::set(&mut setup, &format!("{slice}/sST"), Value::Null).unwrap_err();
    assert!(error.contains("its type always has it"), "{error}");
    inspect::set(&mut setup, &format!("{slice}/sd"), json!("010203")).unwrap();
    let sent = inspect::message_view(&inspect::message_from_view(&setup).unwrap()).unwrap();
    let sd = inspect::select(&sent, &format!("{slice}/sD")).unwrap();
    assert_eq!(sd, [&json!("010203")]);
}

#[test]
fn a_name_is_its_letters_and_its_digits() {
    use serde_json::json;
    let view = inspect::message_view(&release_request()).unwrap();
    for path in [
        "/ngap/RAN-UE-NGAP-ID/value",
        "/NGAP/ran_ue_ngap_id/Value",
        "/Ngap/ranuengapid/value",
        "/ngap/RAN UE NGAP ID/value",
        "/message/protocol-ies/1/value",
        "/Message/ProtocolIEs/1/VALUE",
    ] {
        assert_eq!(inspect::select(&view, path).unwrap(), [&json!(7)], "{path}");
    }
    for path in [
        "/ngap/cause/value/radionetwork",
        "/ngap/Cause/value/Radio-Network",
    ] {
        let cause = inspect::select(&view, path).unwrap();
        assert_eq!(cause, [&json!("user-inactivity")], "{path}");
    }
    assert_eq!(
        inspect::select(&view, "/Procedure-Code").unwrap(),
        inspect::select(&view, "/procedure_code").unwrap()
    );
    // A member that a view keeps of what was received is named as it is.
    assert!(inspect::select(&view, "/ngap/Cause/_raw_value").is_ok());
    assert!(inspect::select(&view, "/ngap/Cause/rawvalue").is_err());
    // An edit writes the member by the name that ASN.1 gives it.
    let mut edited = view.clone();
    inspect::set(&mut edited, "/ngap/cause/value/NAS", json!("deregister")).unwrap();
    let cause = inspect::select(&edited, "/ngap/Cause/value").unwrap();
    assert_eq!(cause, [&json!({"nas": "deregister"})]);
    assert!(inspect::message_from_view(&edited).is_ok());
    assert_eq!(
        inspect::message_named("ue context-release_REQUEST"),
        Some(("InitiatingMessage", 42))
    );
}

#[test]
fn a_path_is_checked_against_a_message_without_a_view() {
    // Every path of every message is one that its message can have.
    for line in include_str!("fixtures/messages.tsv")
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
    {
        let wire = hex::decode(line.split('\t').nth(1).unwrap()).unwrap();
        let pdu = NGAP_PDU::decode(&wire).unwrap();
        let name = inspect::message_name(&pdu).unwrap();
        let view = inspect::message_view(&pdu).unwrap();
        for (path, _) in inspect::paths(&view) {
            let checked = inspect::check_path(name, &path);
            assert!(checked.is_ok(), "{name} {path}: {checked:?}");
            // And so is the path that leaves out what it may.
            let short = short(&view, &path);
            let checked = inspect::check_path(name, &short);
            assert!(checked.is_ok(), "{name} {short}: {checked:?}");
        }
    }
    let transfer = "/ngap/PDUSessionResourceSetupListSUReq/value/0\
        /pDUSessionResourceSetupRequestTransfer";
    let short_transfer = "/ngap/PDUSessionResourceSetupListSUReq/0\
        /pDUSessionResourceSetupRequestTransfer";
    for (message, path) in [
        ("UEContextReleaseRequest", "/ngap/Cause/value/nas"),
        (
            "ue-context-release-request",
            "/ngap/cause/value/radio_network",
        ),
        ("UEContextReleaseRequest", "/ngap/Cause/octets"),
        ("UEContextReleaseRequest", "/ngap/*/value/radioNetwork"),
        ("UEContextReleaseRequest", "/ngap/0/criticality"),
        ("UEContextReleaseRequest", "/ngap/-"),
        ("UEContextReleaseRequest", "/ngap"),
        ("UEContextReleaseRequest", "/procedure_code"),
        ("UEContextReleaseRequest", "/message/protocolIEs/0/value"),
        // An identifier that has no type may be anything.
        ("UEContextReleaseRequest", "/ngap/@id=60000/value/anything"),
        (
            "InitialContextSetupRequest",
            "/ngap/GUAMI/value/aMFRegionID",
        ),
        (
            "InitialContextSetupRequest",
            "/ngap/AllowedNSSAI/value/*/s-NSSAI/sD",
        ),
        (
            "PDUSessionResourceSetupRequest",
            &format!("{transfer}/decoded/PDUSessionType/value"),
        ),
        (
            "PDUSessionResourceSetupRequest",
            &format!("{transfer}/decoded/QosFlowSetupRequestList/value/0/qosFlowIdentifier"),
        ),
        (
            "PDUSessionResourceSetupRequest",
            &format!("{transfer}/octets"),
        ),
        // The value of an IE and the decoded value of a transfer may be left out.
        ("UEContextReleaseRequest", "/ngap/Cause/nas"),
        ("UEContextReleaseRequest", "/ngap/*/radioNetwork"),
        (
            "InitialContextSetupRequest",
            "/ngap/AllowedNSSAI/0/s-NSSAI/sD",
        ),
        (
            "PDUSessionResourceSetupRequest",
            &format!("{short_transfer}/QosFlowSetupRequestList/0/qosFlowIdentifier"),
        ),
        (
            "PDUSessionResourceSetupRequest",
            &format!("{short_transfer}/@id=136/-"),
        ),
        // A segment under an IE whose type is not known may be a member of its value.
        ("UEContextReleaseRequest", "/ngap/@id=60000/anything"),
    ] {
        let checked = inspect::check_path(message, path);
        assert!(checked.is_ok(), "{message} {path}: {checked:?}");
    }
    for (message, path, reason) in [
        ("NoSuchMessage", "/ngap/Cause", "is not a message of NGAP"),
        (
            "InitialContextSetupRequest",
            "/ngap/PDUSessionResourceSetupListSUReq",
            "is not an IE of InitialContextSetupRequest, which has AMF-UE-NGAP-ID, RAN-UE-NGAP-ID",
        ),
        (
            "UEContextReleaseRequest",
            "/ngap/Cause/value/misspelled",
            "is not a member of Cause",
        ),
        (
            "UEContextReleaseRequest",
            "/ngap/*/value/misspelled",
            "is not a member of",
        ),
        (
            "UEContextReleaseRequest",
            "/ngap/RAN-UE-NGAP-ID/value/deeper",
            "which has none",
        ),
        (
            "UEContextReleaseRequest",
            "/ngap/RAN-UE-NGAP-ID/value/*/deeper",
            "selects nothing in a message UEContextReleaseRequest",
        ),
        // A name that neither an IE nor its value has: the members of both.
        (
            "UEContextReleaseRequest",
            "/ngap/Cause/misspelled",
            "is not a member of Cause, which has radioNetwork, transport, nas, protocol, misc, \
             choice-Extensions, at /ngap/Cause/misspelled; an IE itself has id, criticality, \
             value, octets",
        ),
        (
            "UEContextReleaseRequest",
            "/ngap/RAN-UE-NGAP-ID/valeu",
            "is not a member of this value, which has none, at /ngap/RAN-UE-NGAP-ID/valeu; an \
             IE itself has id, criticality, value, octets",
        ),
        (
            "UEContextReleaseRequest",
            "/ngap/Cause/radioNetwork/deeper",
            "which has none",
        ),
        (
            "UEContextReleaseRequest",
            "/ngap/NoSuchIE",
            "is not an IE of NGAP",
        ),
        (
            "PDUSessionResourceSetupRequest",
            &format!("{transfer}/decoded/misspelled"),
            "is not a member of PDUSessionResourceSetupRequestTransfer",
        ),
        (
            "PDUSessionResourceSetupRequest",
            &format!("{transfer}/misspelled"),
            "is not a member of PDUSessionResourceSetupRequestTransfer, which has protocolIEs",
        ),
        (
            "PDUSessionResourceSetupRequest",
            &format!("{transfer}/misspelled"),
            "; a transfer itself has decoded, octets",
        ),
        (
            "PDUSessionResourceSetupRequest",
            &format!("{short_transfer}/QosFlowSetupRequestList/0/misspelled"),
            "is not a member of QosFlowSetupRequestItem",
        ),
        (
            "PDUSessionResourceSetupRequest",
            "/ngap/PDUSessionResourceSetupListSUReq/Cause",
            "go by position",
        ),
        (
            "PDUSessionResourceSetupRequest",
            "/ngap/PDUSessionResourceSetupListSUReq/value/Cause",
            "go by position",
        ),
    ] {
        let error = inspect::check_path(message, path).unwrap_err();
        assert!(error.contains(reason), "{message} {path}: {error}");
    }
}

#[test]
fn an_ie_is_written_by_its_name_and_its_octets_wherever_a_view_has_ies() {
    use serde_json::json;
    let pdu = NGAP_PDU::decode(&fixture("PDUSessionResourceSetupRequest")).unwrap();
    let view = inspect::message_view(&pdu).unwrap();
    let transfer = "/ngap/PDUSessionResourceSetupListSUReq/value/0\
        /pDUSessionResourceSetupRequestTransfer/decoded";
    let raw = "/message/protocolIEs/2/value/0/pDUSessionResourceSetupRequestTransfer/decoded";
    let sent = |view: &serde_json::Value| inspect::message_view(&inspect::message_from_view(view)?);
    let kind = json!({"id": "PDUSessionType", "criticality": "reject", "value": "ipv6"});
    let unknown = json!({"id": 60000, "criticality": "ignore", "octets": "00"});
    // On a path that names the list, as on one that has the root.
    let mut edited = view.clone();
    inspect::remove(&mut edited, &format!("{transfer}/PDUSessionType")).unwrap();
    inspect::insert(&mut edited, &format!("{raw}/protocolIEs/-"), kind.clone()).unwrap();
    inspect::insert(
        &mut edited,
        &format!("{raw}/protocolIEs/-"),
        unknown.clone(),
    )
    .unwrap();
    let after = sent(&edited).unwrap();
    let written = format!("{transfer}/PDUSessionType/value");
    assert_eq!(inspect::select(&after, &written).unwrap(), [&json!("ipv6")]);
    let octets = format!("{transfer}/@id=60000/octets");
    assert_eq!(inspect::select(&after, &octets).unwrap(), [&json!("00")]);
    // In a value that is set whole, at any depth, and in a view written by hand.
    let mut edited = view.clone();
    let ies = json!({"protocolIEs": [kind, unknown]});
    inspect::set(&mut edited, transfer, ies).unwrap();
    let after = sent(&edited).unwrap();
    assert_eq!(inspect::select(&after, &written).unwrap(), [&json!("ipv6")]);
    assert_eq!(inspect::select(&after, &octets).unwrap(), [&json!("00")]);
    let mut edited = view.clone();
    let ies = edited["message"]["protocolIEs"].as_array_mut().unwrap();
    ies[0]["id"] = json!("amf-ue-ngap-id");
    ies.push(json!({"id": "RANNodeName", "criticality": "ignore", "octets": "0461"}));
    let after = sent(&edited).unwrap();
    assert_eq!(after["message"]["protocolIEs"][0]["id"], json!(10));
    let name = inspect::select(&after, "/ngap/RANNodeName/octets").unwrap();
    assert_eq!(name, [&json!("0461")]);
    // The octets of an IE that was added are set at its path too.
    let mut edited = view.clone();
    let name = json!({"id": "RANNodeName", "criticality": "ignore", "value": "a"});
    inspect::insert(&mut edited, "/ngap/-", name).unwrap();
    inspect::set(&mut edited, "/ngap/RANNodeName/octets", json!("0461")).unwrap();
    let after = sent(&edited).unwrap();
    let name = inspect::select(&after, "/ngap/RANNodeName/octets").unwrap();
    assert_eq!(name, [&json!("0461")]);
    // A name that no IE has is refused, and so are a value and octets together.
    let mut edited = view.clone();
    edited["message"]["protocolIEs"][0]["id"] = json!("NoSuchIE");
    let error = inspect::message_from_view(&edited).unwrap_err();
    assert!(error.contains("is not an IE of NGAP"), "{error}");
    let mut edited = view.clone();
    edited["message"]["protocolIEs"][0]["octets"] = json!("00");
    let error = inspect::message_from_view(&edited).unwrap_err();
    assert!(
        error.contains("its value or its octets, not both"),
        "{error}"
    );
}

#[test]
fn the_octets_of_an_ie_without_a_value_have_a_path() {
    use serde_json::json;
    let mut view = inspect::message_view(&release_request()).unwrap();
    // The octets repeat a value that is there.
    let listed = |view: &serde_json::Value, path: &str| {
        let paths = inspect::paths(view);
        paths.iter().any(|(listed, _)| listed == path)
    };
    assert!(!listed(&view, "/ngap/Cause/octets"));
    inspect::set(&mut view, "/ngap/Cause/octets", json!("0540")).unwrap();
    assert!(listed(&view, "/ngap/Cause/octets"));
    assert!(!listed(&view, "/ngap/Cause/value"));
    for (path, value) in inspect::paths(&view) {
        assert_eq!(inspect::select(&view, &path).unwrap(), [&value], "{path}");
    }
}

#[test]
fn a_private_message_has_no_ies_under_the_root() {
    let pdu = NGAP_PDU::decode(&fixture("PrivateMessage")).unwrap();
    let view = inspect::message_view(&pdu).unwrap();
    let error = inspect::select(&view, "/ngap/Cause").unwrap_err();
    let reason = "a message PrivateMessage has no IEs under /ngap: it has privateIEs";
    assert!(error.contains(reason), "{error}");
    assert!(!error.contains("protocolIEs"), "{error}");
    let error = inspect::check_path("PrivateMessage", "/ngap/Cause").unwrap_err();
    assert!(error.contains(reason), "{error}");
    assert!(inspect::select(&view, "/message/privateIEs/0/id/local").is_ok());
    assert!(inspect::check_path("PrivateMessage", "/message/privateIEs/0/id/global").is_ok());
}

#[test]
fn the_identifier_that_the_octets_were_decoded_as_is_a_number() {
    use serde_json::json;
    let mut view = inspect::message_view(&release_request()).unwrap();
    inspect::set(&mut view, "/ngap/Cause/value/nas", json!("normal-release")).unwrap();
    ie(&mut view, 15)["_original_id"] = json!("fifteen");
    let error = inspect::message_from_view(&view).unwrap_err();
    assert!(error.contains("_original_id is the identifier"), "{error}");
}

/// A request to modify a UE context: its aggregate maximum bit rate for the downlink,
/// and UE security capabilities whose NR encryption algorithms are this many bits.
fn modification(rate: u64, bits: usize) -> NGAP_PDU {
    let mask =
        |length: usize| -> rasn::types::BitString { (0..length).map(|bit| bit < 2).collect() };
    let rates = UEAggregateMaximumBitRate::new(BitRate(rate.into()), BitRate(1u64.into()), None);
    let capabilities = UESecurityCapabilities::new(
        NRencryptionAlgorithms(mask(bits)),
        NRintegrityProtectionAlgorithms(mask(16)),
        EUTRAencryptionAlgorithms(mask(16)),
        EUTRAintegrityProtectionAlgorithms(mask(16)),
        None,
    );
    build_ngap!(InitiatingMessage, UEContextModification,
        REJECT, UEContextModificationRequest,
        REJECT AMF_UE_NGAP_ID(1u64),
        REJECT RAN_UE_NGAP_ID(1u32),
        IGNORE UEAggregateMaximumBitRate(rates),
        REJECT UESecurityCapabilities(capabilities),
    )
    .unwrap()
}

/// rasn's JER writes an INTEGER as an `i64`, where ITU-T X.697 (02/2021) §21 has a number
/// of any size. An IE with an integer of 2^63 or more has no typed value: it stays its
/// octets, as the README says.
#[test]
fn an_ie_with_an_integer_of_two_to_the_63_stays_octets() {
    use serde_json::json;
    let rates = "/ngap/UEAggregateMaximumBitRate";
    let downlink = format!("{rates}/uEAggregateMaximumBitRateDL");

    // The largest integer that is typed is read, edited and encoded.
    let largest = (1u64 << 63) - 1;
    let mut view = inspect::message_view(&modification(largest, 16)).unwrap();
    assert_eq!(
        inspect::select(&view, &downlink).unwrap(),
        [&json!(largest)]
    );
    inspect::set(&mut view, &downlink, json!(largest - 1)).unwrap();
    let edited = inspect::message_from_view(&view).unwrap();
    assert_eq!(edited, modification(largest - 1, 16));

    // One more, and the IE is its octets beside why it has no value. The rest of the PDU
    // is read.
    let pdu = modification(1 << 63, 16);
    let wire = pdu.encode().unwrap();
    let mut view = inspect::message_view(&NGAP_PDU::decode(&wire).unwrap()).unwrap();
    let entry = view["message"]["protocolIEs"][2].clone();
    assert!(entry["_decode_error"].is_string(), "{entry}");
    assert_eq!(entry["value"], entry["_raw_value"]);
    let octets = inspect::select(&view, &format!("{rates}/octets")).unwrap()[0].clone();
    assert_eq!(octets, entry["_raw_value"]);
    assert!(inspect::select(&view, &downlink).is_err());
    let other = "/ngap/RAN-UE-NGAP-ID/value";
    assert_eq!(inspect::select(&view, other).unwrap(), [&json!(1)]);

    // A tree has the octets too, and both forms send them as they were received, also
    // beside an edit of another IE.
    let tree = inspect::message_tree(&pdu).unwrap();
    assert_eq!(
        tree["ies"][2],
        json!({"UEAggregateMaximumBitRate": {"octets": octets}})
    );
    assert_eq!(
        inspect::message_from_tree(&tree).unwrap().encode().unwrap(),
        wire
    );
    assert_eq!(
        inspect::message_from_view(&view).unwrap().encode().unwrap(),
        wire
    );
    inspect::set(&mut view, other, json!(2)).unwrap();
    let edited = inspect::message_view(&inspect::message_from_view(&view).unwrap()).unwrap();
    assert_eq!(inspect::select(&edited, other).unwrap(), [&json!(2)]);
    assert_eq!(
        inspect::select(&edited, &format!("{rates}/octets")).unwrap(),
        [&octets]
    );

    // Such an integer is not written as a typed value either.
    let written = json!({"message": "UEContextModificationRequest", "ies": [
        {"AMF-UE-NGAP-ID": 1},
        {"RAN-UE-NGAP-ID": 1},
        {"UEAggregateMaximumBitRate": {
            "uEAggregateMaximumBitRateDL": 1u64 << 63,
            "uEAggregateMaximumBitRateUL": 1,
        }},
    ]});
    assert!(inspect::message_from_tree(&written).is_err());
}

/// rasn's JER writes a BIT STRING of one extensible size as its octets alone, where
/// ITU-T X.697 (02/2021) §7.2.1 and §24.3 give it its length. A string of another size
/// than its type gives would be shown as a string that it is not, 20 bits as 24: its IE
/// has no typed value and stays its octets, as the README says.
#[test]
fn an_ie_with_a_string_of_bits_of_another_size_stays_octets() {
    use serde_json::json;
    let capabilities = "/ngap/UESecurityCapabilities";
    let member = |of: &str| format!("{capabilities}/{of}");
    let algorithms = member("nRencryptionAlgorithms");
    let other = "/ngap/RAN-UE-NGAP-ID/value";

    // The size that the type gives is typed: hexadecimal, read and edited.
    let mut view = inspect::message_view(&modification(1, 16)).unwrap();
    assert_eq!(
        inspect::select(&view, &algorithms).unwrap(),
        [&json!("C000")]
    );
    inspect::set(&mut view, &algorithms, json!("8000")).unwrap();
    let edited = inspect::message_view(&inspect::message_from_view(&view).unwrap()).unwrap();
    assert_eq!(
        inspect::select(&edited, &algorithms).unwrap(),
        [&json!("8000")]
    );

    // 20 bits and 24 bits are the same three octets to JER. Each is the octets of its IE
    // beside why it has no value, and the rest of the PDU is read.
    let mut received = Vec::new();
    for bits in [20, 24] {
        let pdu = modification(1, bits);
        let wire = pdu.encode().unwrap();
        let view = inspect::message_view(&NGAP_PDU::decode(&wire).unwrap()).unwrap();
        let entry = view["message"]["protocolIEs"][3].clone();
        let error = entry["_decode_error"].as_str().unwrap_or_default();
        assert!(
            error.starts_with("JER does not represent this value"),
            "{entry}"
        );
        assert_eq!(entry["value"], entry["_raw_value"]);
        assert!(inspect::select(&view, &algorithms).is_err());
        assert_eq!(inspect::select(&view, other).unwrap(), [&json!(1)]);

        // A tree has the octets too, and both forms send them as they were received.
        let octets = json!({"UESecurityCapabilities": {"octets": entry["_raw_value"]}});
        let tree = inspect::message_tree(&pdu).unwrap();
        assert_eq!(tree["ies"][3], octets);
        assert_eq!(
            inspect::message_from_tree(&tree).unwrap().encode().unwrap(),
            wire
        );
        assert_eq!(
            inspect::message_from_view(&view).unwrap().encode().unwrap(),
            wire
        );
        received.push(entry["_raw_value"].clone());
    }
    assert_ne!(received[0], received[1]);

    // The octets of the IE are set in its place.
    let mut view = inspect::message_view(&modification(1, 24)).unwrap();
    let usual = inspect::message_view(&modification(1, 16)).unwrap();
    let usual = inspect::select(&usual, &member("octets")).unwrap()[0].clone();
    inspect::set(&mut view, &member("octets"), usual).unwrap();
    assert_eq!(
        inspect::message_from_view(&view).unwrap(),
        modification(1, 16)
    );

    // Such a string is not written as a typed value either.
    let written = json!({"message": "UEContextModificationRequest", "ies": [
        {"AMF-UE-NGAP-ID": 1},
        {"RAN-UE-NGAP-ID": 1},
        {"UESecurityCapabilities": {
            "nRencryptionAlgorithms": "C00000",
            "nRintegrityProtectionAlgorithms": "C000",
            "eUTRAencryptionAlgorithms": "C000",
            "eUTRAintegrityProtectionAlgorithms": "C000",
        }},
    ]});
    assert!(inspect::message_from_tree(&written).is_err());
}

/// What a PDU that is written from `tree` has for the IE of the extension container at
/// `extension`, a path, whose type is the one that the type which the container extends
/// gives it. In a view the value at `at` under the IE is `value`, and the octets of the
/// IE are `octets`, which the bindings give a value of its type. The PDU is the same
/// octets through its view and through its tree. Set to `other` through the view, the IE
/// has `other_octets`, and the edited PDU is its tree again.
fn an_extension_has_its_type(
    tree: serde_json::Value,
    extension: &str,
    at: &str,
    (value, octets): (serde_json::Value, rasn::types::Any),
    (other, other_octets): (serde_json::Value, rasn::types::Any),
) {
    let at = format!("{extension}/{at}");
    let received = |view: &serde_json::Value| -> String {
        let octets = inspect::select(view, &format!("{extension}/octets")).unwrap();
        octets[0].as_str().unwrap().to_lowercase()
    };

    // Written from its tree, and typed in the view of what was received.
    let pdu = inspect::message_from_tree(&tree).unwrap();
    let wire = pdu.encode().unwrap();
    let mut view = inspect::message_view(&NGAP_PDU::decode(&wire).unwrap()).unwrap();
    assert_eq!(inspect::select(&view, &at).unwrap(), [&value], "{at}");
    assert_eq!(received(&view), hex::encode(octets.as_bytes()), "{at}");
    let entries = inspect::select(&view, extension).unwrap();
    assert!(entries[0]["_decode_error"].is_null(), "{}", entries[0]);

    // The same octets back, through the view and through the tree.
    assert_eq!(
        inspect::message_from_view(&view).unwrap().encode().unwrap(),
        wire
    );
    let shown = inspect::message_tree(&pdu).unwrap();
    assert_eq!(shown, tree);
    assert_eq!(
        inspect::message_from_tree(&shown)
            .unwrap()
            .encode()
            .unwrap(),
        wire
    );

    // Edited through the view, and the edited PDU through its tree.
    inspect::set(&mut view, &at, other.clone()).unwrap();
    let edited = inspect::message_from_view(&view).unwrap();
    let after = inspect::message_view(&edited).unwrap();
    assert_eq!(inspect::select(&after, &at).unwrap(), [&other], "{at}");
    assert_eq!(
        received(&after),
        hex::encode(other_octets.as_bytes()),
        "{at}"
    );
    let shown = inspect::message_tree(&edited).unwrap();
    let written = inspect::message_from_tree(&shown).unwrap();
    assert_eq!(written.encode().unwrap(), edited.encode().unwrap());
}

/// id-CurrentQoSParaSetIndex is an AlternativeQoSParaSetIndex where it extends an
/// associated QoS flow and an AlternativeQoSParaSetNotifyIndex where it extends a notified
/// one: an IE of an extension container has the type that the type which the container
/// extends gives it, and the two types give the same number other octets.
#[test]
fn the_qos_parameter_set_index_has_the_type_of_what_it_extends() {
    use serde_json::json;
    let index = |index: u8| encode_open_type(&AlternativeQoSParaSetIndex(index.into())).unwrap();
    let notified =
        |index: u8| encode_open_type(&AlternativeQoSParaSetNotifyIndex(index.into())).unwrap();
    assert_eq!(
        (index(3).as_bytes(), notified(3).as_bytes()),
        (&[0x20][..], &[0x18][..])
    );

    // Of a QoS flow that a notification is about.
    let notify = json!({"message": "PDUSessionResourceNotify", "ies": [
        {"AMF-UE-NGAP-ID": 1},
        {"RAN-UE-NGAP-ID": 2},
        {"PDUSessionResourceNotifyList": [{
            "pDUSessionID": 5,
            "pDUSessionResourceNotifyTransfer": {"qosFlowNotifyList": [{
                "qosFlowIdentifier": 1,
                "notificationCause": "fulfilled",
                "iE-Extensions": [{"CurrentQoSParaSetIndex": 3, "criticality": "ignore"}],
            }]},
        }]},
    ]});
    let notify_transfer = "/ngap/PDUSessionResourceNotifyList/0/pDUSessionResourceNotifyTransfer";
    let notified_flow = format!("{notify_transfer}/qosFlowNotifyList/0/iE-Extensions");
    an_extension_has_its_type(
        notify,
        &format!("{notified_flow}/0"),
        "extensionValue",
        (json!(3), notified(3)),
        (json!(0), notified(0)),
    );

    // Of a QoS flow that is associated with a tunnel.
    let setup = json!({"message": "PDUSessionResourceSetupResponse", "ies": [
        {"AMF-UE-NGAP-ID": 1},
        {"RAN-UE-NGAP-ID": 2},
        {"PDUSessionResourceSetupListSURes": [{
            "pDUSessionID": 5,
            "pDUSessionResourceSetupResponseTransfer": {"dLQosFlowPerTNLInformation": {
                "uPTransportLayerInformation": {"gTPTunnel": {
                    "transportLayerAddress": "10.0.0.1",
                    "gTP-TEID": 1,
                }},
                "associatedQosFlowList": [{
                    "qosFlowIdentifier": 1,
                    "iE-Extensions": [{"CurrentQoSParaSetIndex": 3, "criticality": "ignore"}],
                }],
            }},
        }]},
    ]});
    let setup_transfer =
        "/ngap/PDUSessionResourceSetupListSURes/0/pDUSessionResourceSetupResponseTransfer";
    let associated_flow = format!(
        "{setup_transfer}/dLQosFlowPerTNLInformation/associatedQosFlowList/0/iE-Extensions"
    );
    an_extension_has_its_type(
        setup,
        &format!("{associated_flow}/CurrentQoSParaSetIndex"),
        "extensionValue",
        (json!(3), index(3)),
        (json!(8), index(8)),
    );

    // A path is read against the type without a view, by the name of the IE, by its
    // identifier and after its position, and a value that the type does not have is
    // refused.
    for (message, extensions) in [
        ("PDUSessionResourceNotify", &notified_flow),
        ("PDUSessionResourceSetupResponse", &associated_flow),
    ] {
        for ie in [
            "CurrentQoSParaSetIndex",
            "@id=221",
            "0/CurrentQoSParaSetIndex",
        ] {
            let value = format!("{extensions}/{ie}/extensionValue");
            assert_eq!(inspect::check_path(message, &value), Ok(()), "{value}");
            assert_eq!(inspect::enumerated_at(message, &value), Ok(None), "{value}");
            let member = format!("{value}/member");
            let refused = inspect::check_path(message, &member).unwrap_err();
            assert!(refused.contains("which has none"), "{refused}");
        }
    }

    // Where it extends nothing that gives it a type, it has none: it is sent as octets.
    let alone = |value: serde_json::Value| {
        json!({"message": "PDUSessionResourceNotify", "ies": [
            {"CurrentQoSParaSetIndex": value, "criticality": "ignore"},
        ]})
    };
    let refused = inspect::message_from_tree(&alone(json!(3))).unwrap_err();
    assert!(
        refused.contains("NGAP IE 221 has a type where it extends"),
        "{refused}"
    );
    assert!(
        refused.contains("QosFlowNotifyItem") && refused.contains("none here"),
        "{refused}"
    );
    let pdu = inspect::message_from_tree(&alone(json!({"octets": "18"}))).unwrap();
    let view = inspect::message_view(&pdu).unwrap();
    let error = view["message"]["protocolIEs"][0]["_decode_error"].as_str();
    assert!(
        error.is_some_and(|error| error.contains("none here")),
        "{view}"
    );
    assert_eq!(
        inspect::message_tree(&pdu).unwrap(),
        alone(json!({"octets": "18"}))
    );
}

/// id-QosFlowAdditionalInfoList is a QosFlowAdditionalInfoListRelCom where it extends the
/// transfer of a release command and a QosFlowAdditionalInfoListRelRes where it extends
/// that of a release response: the items of the first have a downlink container, and
/// those of the second an uplink one.
#[test]
fn the_additional_information_of_qos_flows_has_the_type_of_what_it_extends() {
    use serde_json::json;
    let flow = || QosFlowIdentifier(1.into());
    let container = |octets: &[u8]| Some(TLContainer(octets.to_vec().into()));
    let command = |octets: &[u8]| {
        let item = QosFlowAdditionalInfoItemRelCom::new(flow(), container(octets), None);
        encode_open_type(&QosFlowAdditionalInfoListRelCom(vec![item])).unwrap()
    };
    let response = |octets: &[u8]| {
        let item = QosFlowAdditionalInfoItemRelRes::new(flow(), container(octets), None);
        encode_open_type(&QosFlowAdditionalInfoListRelRes(vec![item])).unwrap()
    };

    // Of the transfer of a release command.
    let flows = |container: &str| json!([{"qosFlowIdentifier": 1, container: "0102"}]);
    let released = |container: &str| {
        json!({"message": "PDUSessionResourceReleaseCommand", "ies": [
            {"AMF-UE-NGAP-ID": 1},
            {"RAN-UE-NGAP-ID": 2},
            {"PDUSessionResourceToReleaseListRelCmd": [{
                "pDUSessionID": 5,
                "pDUSessionResourceReleaseCommandTransfer": {
                    "cause": {"nas": "normal-release"},
                    "iE-Extensions": [
                        {"QosFlowAdditionalInfoList": flows(container), "criticality": "ignore"},
                    ],
                },
            }]},
        ]})
    };
    let command_transfer = "/ngap/PDUSessionResourceToReleaseListRelCmd/0\
        /pDUSessionResourceReleaseCommandTransfer/iE-Extensions";
    an_extension_has_its_type(
        released("downlinkTLContainer"),
        &format!("{command_transfer}/QosFlowAdditionalInfoList"),
        "0/downlinkTLContainer",
        (json!("0102"), command(&[1, 2])),
        (json!("0A0B0C"), command(&[10, 11, 12])),
    );

    // Of the transfer of a release response.
    let responded = |container: &str| {
        json!({"message": "PDUSessionResourceReleaseResponse", "ies": [
            {"AMF-UE-NGAP-ID": 1},
            {"RAN-UE-NGAP-ID": 2},
            {"PDUSessionResourceReleasedListRelRes": [{
                "pDUSessionID": 5,
                "pDUSessionResourceReleaseResponseTransfer": {"iE-Extensions": [
                    {"QosFlowAdditionalInfoList": flows(container), "criticality": "ignore"},
                ]},
            }]},
        ]})
    };
    let response_transfer = "/ngap/PDUSessionResourceReleasedListRelRes/0\
        /pDUSessionResourceReleaseResponseTransfer/iE-Extensions";
    an_extension_has_its_type(
        responded("uplinkTLContainer"),
        &format!("{response_transfer}/0"),
        "0/uplinkTLContainer",
        (json!("0102"), response(&[1, 2])),
        (json!("0A0B0C"), response(&[10, 11, 12])),
    );

    // Each has the members of its type, written and at a path, and not those of the other.
    for wrong in [
        released("uplinkTLContainer"),
        responded("downlinkTLContainer"),
    ] {
        let refused = inspect::message_from_tree(&wrong).unwrap_err();
        assert!(
            refused.contains("a member that its ASN.1 type does not have"),
            "{refused}"
        );
    }
    for (message, extensions, has, has_not) in [
        (
            "PDUSessionResourceReleaseCommand",
            command_transfer,
            "downlinkTLContainer",
            "uplinkTLContainer",
        ),
        (
            "PDUSessionResourceReleaseResponse",
            response_transfer,
            "uplinkTLContainer",
            "downlinkTLContainer",
        ),
    ] {
        for ie in [
            "QosFlowAdditionalInfoList",
            "@id=397",
            "0/QosFlowAdditionalInfoList",
        ] {
            let item = format!("{extensions}/{ie}/0");
            assert_eq!(
                inspect::check_path(message, &format!("{item}/{has}")),
                Ok(()),
                "{item}"
            );
            let refused = inspect::check_path(message, &format!("{item}/{has_not}")).unwrap_err();
            assert!(
                refused.contains("is not a member of QosFlowAdditionalInfoItemRel"),
                "{refused}"
            );
            assert!(refused.contains(has), "{refused}");
        }
    }
}
