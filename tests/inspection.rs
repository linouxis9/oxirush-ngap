#![cfg(feature = "inspect")]
//! The trees of the PDUs of NGAP: those of the fixtures and the paths of each of their
//! values, the forms that are NGAP's, and its registry. TOY, the protocol of the tests of
//! oxirush-aper, has the tests of what `to_tree`, `from_tree` and the paths do with the
//! tree of any PDU.

use oxirush_ngap::tree;
use oxirush_ngap::tree::Value;
use oxirush_ngap::{Ngap, inspect, ngap::*};

/// The name that ASN.1 gives the message of a PDU.
fn name_of(pdu: &NGAPPDU<'_>) -> Option<&'static str> {
    Ngap::object_set(pdu.direction(), pdu.procedure_code()).map(|set| set.name)
}

/// The octets of the message `name` of the fixtures.
fn fixture(name: &str) -> Vec<u8> {
    let mut lines = include_str!("fixtures/messages.tsv").lines();
    let line = lines.find(|line| line.split('\t').next() == Some(name));
    hex::decode(line.unwrap().split('\t').nth(1).unwrap()).unwrap()
}

/// The messages of the fixtures: the name of each, and its octets.
fn fixtures() -> impl Iterator<Item = (&'static str, Vec<u8>)> {
    let lines = include_str!("fixtures/messages.tsv").lines();
    let lines = lines.filter(|line| !line.starts_with('#') && !line.is_empty());
    lines.map(|line| {
        let fields: Vec<_> = line.split('\t').collect();
        (fields[0], hex::decode(fields[1]).unwrap())
    })
}

/// The tree of the PDU that `tree` writes, which is written and decoded again.
fn sent(tree: &Value) -> Result<Value, oxirush_ngap::Error> {
    Ok(NGAPPDU::from_tree(tree)?.to_tree())
}

/// The tree of a UE CONTEXT RELEASE REQUEST of the IEs `ies`, written and decoded again.
fn released(ies: Value) -> Result<Value, oxirush_ngap::Error> {
    sent(&tree!({"message": "UEContextReleaseRequest", "ies": ies}))
}

/// The values at `path` in `tree`.
fn select(tree: &Value, path: &str) -> Vec<Value> {
    let selected = inspect::select(tree, path).unwrap();
    selected.into_iter().cloned().collect()
}

/// The value of the IE `name` of a tree, the first one of that name.
fn ie<'t>(tree: &'t mut Value, name: &str) -> &'t mut Value {
    let ies = tree["ies"].as_array_mut().unwrap();
    let entry = ies.iter_mut().find(|ie| ie.get(name).is_some()).unwrap();
    &mut entry[name]
}

#[test]
fn every_message_of_the_fixtures_is_its_octets_through_its_tree() {
    for (name, wire) in fixtures() {
        let pdu = NGAPPDU::decode(&wire).unwrap();
        let shown = pdu.to_tree();
        assert_eq!(shown["message"], tree!(name));
        // A private message has no IEs of the protocol to name: its octets.
        if name == "PrivateMessage" {
            assert!(shown["octets"].is_string() && shown.get("ies").is_none());
        } else {
            assert!(shown["ies"].is_array(), "{name}");
        }
        let written = NGAPPDU::from_tree(&shown).unwrap();
        assert_eq!(written.encode().unwrap(), wire, "{name}: {shown}");
        assert_eq!(written, pdu, "{name}");
    }
}

/// The members of `value`, a tree or a part of one, that only repeat what its form says:
/// those of an IE or of a transfer of another form, and a `criticality` that the tables
/// could give. `holder` is the name of the member that has `value`, or the list it is in.
fn redundant(value: &Value, holder: &str, found: &mut Vec<String>) {
    match value {
        Value::Object(members) => {
            for (name, member) in members {
                let wraps = match name.as_str() {
                    "decoded" | "id" | "extensionValue" => true,
                    // A bit string whose size varies has a `value` and a `length`.
                    "value" => !members.contains_key("length"),
                    // The tables have the criticality that an object set assigns to the
                    // IEs of a message and of a transfer, and to those of a single container.
                    "criticality" => true,
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
fn the_messages_of_the_fixtures_are_shown_and_written_by_name_to_any_depth() {
    let shown = |wire: &[u8]| NGAPPDU::decode(wire).unwrap().to_tree();
    let written = |tree: &Value| NGAPPDU::from_tree(tree).map(|pdu| pdu.encode().unwrap());
    // A transfer is its decoded value, and the IEs of the message that it contains go
    // by name, with the criticality that the object set of the transfer assigns them.
    let wire = fixture("PDUSessionResourceSetupRequest");
    let tunnel = tree!({"gTPTunnel": {
        "transportLayerAddress": {"value": "00", "length": 1},
        "gTP-TEID": 577899919,
    }});
    let flow = tree!({"qosFlowIdentifier": 0, "qosFlowLevelQosParameters": {
        "qosCharacteristics": {"nonDynamic5QI": {"fiveQI": 0}},
        "allocationAndRetentionPriority": {
            "priorityLevelARP": 1,
            "pre-emptionCapability": "shall-not-trigger-pre-emption",
            "pre-emptionVulnerability": "not-pre-emptable",
        },
    }});
    let request = tree!({
        "message": "PDUSessionResourceSetupRequest",
        "ies": [
            {"AMF-UE-NGAP-ID": 0},
            {"RAN-UE-NGAP-ID": 0},
            {"PDUSessionResourceSetupListSUReq": [{
                "pDUSessionID": 0,
                "s-NSSAI": {"sST": 85},
                "pDUSessionResourceSetupRequestTransfer": {"protocolIEs": [
                    {"UL-NGU-UP-TNLInformation": &tunnel},
                    {"PDUSessionType": "ipv4"},
                    {"QosFlowSetupRequestList": [&flow]},
                ]},
            }]},
        ],
    });
    assert_eq!(shown(&wire), request);
    assert_eq!(written(&request).unwrap(), wire);
    // No message of the fixtures is shown with a member that only wraps a value, nor
    // with a criticality that the tables have.
    let mut found = Vec::new();
    for (name, wire) in fixtures() {
        let shown = shown(&wire);
        if shown.get("ies").is_some() {
            redundant(&shown["ies"], name, &mut found);
        }
    }
    assert_eq!(found, Vec::<String>::new());
}

/// Each value of `shown`, a tree or a part of one, with the JSON pointer of the tree that
/// joins the names and the positions on the way to it. A list of plain values is one
/// value.
fn joined(shown: &Value, path: &str, found: &mut Vec<(String, Value)>) {
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
fn the_names_and_the_positions_of_a_tree_are_the_path_of_each_value() {
    let mut values = 0;
    for (name, wire) in fixtures() {
        let tree = NGAPPDU::decode(&wire).unwrap().to_tree();
        // A private message has no IEs of the protocol to name.
        let Some(Value::Array(ies)) = tree.get("ies") else {
            continue;
        };
        for (index, ie) in ies.iter().enumerate() {
            // An IE of the message by its position, then by its name, and by its name
            // alone, which selects it when the message has it once.
            let mut found = Vec::new();
            joined(ie, &format!("/ngap/{index}"), &mut found);
            for (path, value) in found {
                let at = format!("{name} {path}");
                assert_eq!(select(&tree, &path), std::slice::from_ref(&value), "{at}");
                assert_eq!(inspect::check_path(name, &path), Ok(()), "{at}");
                let named = path.replacen(&format!("/{index}/"), "/", 1);
                let selected = inspect::select(&tree, &named).expect(&at);
                assert!(selected.contains(&&value), "{at}");
                values += 1;
            }
        }
    }
    assert!(values > 500, "{values}");

    // A member of a value that is named as another IE is that member: the target of a
    // handover has a `targetRANNodeID`, and `Target-RAN-Node-ID` is an IE.
    let required = NGAPPDU::decode(&fixture("HandoverRequired"))
        .unwrap()
        .to_tree();
    assert!(
        Ngap::ie_names()
            .iter()
            .any(|(_, name)| *name == "Target-RAN-Node-ID")
    );
    let area = "/ngap/TargetID/targetRANNodeID/selectedTAI/tAC";
    assert_eq!(select(&required, area), [tree!(1779096)]);
    assert_eq!(inspect::check_path("HandoverRequired", area), Ok(()));
}

#[test]
fn the_parts_of_a_guami_are_numbers_of_their_bits() {
    let wire = fixture("InitialContextSetupRequest");
    let mut tree = NGAPPDU::decode(&wire).unwrap().to_tree();
    let guami = ie(&mut tree, "GUAMI");
    assert_eq!(guami["aMFSetID"], tree!(0));
    guami["pLMNIdentity"] = tree!("310-410");
    guami["aMFRegionID"] = tree!(0xCA);
    guami["aMFSetID"] = tree!(0x3FF);
    guami["aMFPointer"] = tree!(1);
    let edited = NGAPPDU::from_tree(&tree).unwrap();
    // The same GUAMI as the crate's helper builds from these numbers.
    let network = oxirush_ngap::helpers::plmn("310", "410").unwrap();
    let expected = oxirush_ngap::helpers::guami(network, 0xCA, 0x3FF, 1).unwrap();
    let expected = oxirush_ngap::encode(&expected).unwrap();
    let ies = edited.protocol_ies().unwrap();
    let held = ies.0.iter().find(|ie| ie.id.0 == 28).unwrap();
    assert_eq!(held.value.to_vec(), expected);
    assert_eq!(ie(&mut edited.to_tree(), "GUAMI")["aMFSetID"], tree!(0x3FF));
    // The AMF Set ID has ten bits.
    ie(&mut tree, "GUAMI")["aMFSetID"] = tree!(0x400);
    assert!(NGAPPDU::from_tree(&tree).is_err());
}

#[test]
fn two_enumerated_types_that_spell_one_name_in_two_ways_each_take_it_in_its_own() {
    // UserPlaneErrorIndicator and the UserPlaneFailureType of UserPlaneFailureIndication
    // spell one name in two ways: each takes it in its own.
    let tunnel = tree!({"gTPTunnel": {"transportLayerAddress": "10.0.0.1", "gTP-TEID": 1}});
    let mut shown = released(tree!([
        {"UserPlaneErrorIndicator": "GTP-U-ERROR-INDICATION-RECEIVED", "criticality": "ignore"},
        {"UserPlaneFailureIndication": {
            "userPlaneFailureType": "GTP-U-ERROR-INDICATION-RECEIVED",
            "uL-NGU-UP-TNLInformation": &tunnel, "dL-NGU-UP-TNLInformation": &tunnel},
         "criticality": "ignore"},
    ]))
    .unwrap();
    assert_eq!(
        *ie(&mut shown, "UserPlaneErrorIndicator"),
        tree!("gTP-U-error-indication-received")
    );
    assert_eq!(
        ie(&mut shown, "UserPlaneFailureIndication")["userPlaneFailureType"],
        tree!("gtp-u-error-indication-received")
    );
}

#[test]
fn every_path_of_every_message_selects_its_value_and_is_one_that_its_message_can_have() {
    for (name, wire) in fixtures() {
        let tree = NGAPPDU::decode(&wire).unwrap().to_tree();
        for (path, value) in inspect::paths(&tree) {
            // The list that holds the IEs of a message is never named.
            assert!(!path.contains("protocolIEs"), "{name} {path}");
            assert_eq!(select(&tree, &path), [value], "{name} {path}");
            let checked = inspect::check_path(name, &path);
            assert!(checked.is_ok(), "{name} {path}: {checked:?}");
        }
    }
}

#[test]
fn the_messages_of_the_fixtures_go_by_the_names_that_asn1_gives_them() {
    for (name, wire) in fixtures() {
        let pdu = NGAPPDU::decode(&wire).unwrap();
        assert_eq!(name_of(&pdu), Some(name));
    }
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
    for (id, name) in Ngap::ie_names() {
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

/// The handover container of S1AP that the IE may carry is written by the name of
/// its type once the program registers the tables of oxirush-s1ap: before, the name is
/// refused with the call that registers them. No test of this file registers them.
#[test]
fn a_container_of_the_other_protocol_says_the_call_that_registers_its_tables() {
    let tree = oxirush_ngap::tree!({"message": "HandoverRequired", "ies": [
        {"AMF-UE-NGAP-ID": 42},
        {"RAN-UE-NGAP-ID": 7},
        {"SourceToTarget-TransparentContainer": {"SourceeNB-ToTargeteNB-TransparentContainer": {}}},
    ]});
    let error = NGAPPDU::from_tree(&tree).unwrap_err().to_string();
    let call = "SourceeNB-ToTargeteNB-TransparentContainer is a type of S1AP: call oxirush_s1ap::inspect::register()";
    assert!(error.contains(call), "{error}");
    let path =
        "/ngap/SourceToTarget-TransparentContainer/SourceeNB-ToTargeteNB-TransparentContainer/rRC";
    let error = inspect::check_path("HandoverRequired", path)
        .unwrap_err()
        .to_string();
    assert!(error.contains(call), "{error}");
}

#[test]
fn the_messages_of_the_fixtures_have_the_criticality_that_asn1_assigns() {
    let mut ies = 0;
    for (name, wire) in fixtures() {
        let pdu = NGAPPDU::decode(&wire).unwrap();
        let set = Ngap::object_set_named(name).unwrap();
        assert_eq!(pdu.criticality(), set.criticality, "{name}");
        // Their trees leave out what the object sets assign.
        let tree = pdu.to_tree();
        assert!(tree.get("criticality").is_none(), "{name}");
        let assigned = set.ies.unwrap_or_default();
        // A private message has no IEs of an object set.
        let Ok(listed) = pdu.protocol_ies() else {
            continue;
        };
        for ie in &listed.0 {
            let mut assigned = assigned.iter().filter(|(known, ..)| *known == ie.id.0);
            let assigned = assigned.next().map(|(_, criticality, _)| *criticality);
            assert_eq!(Some(ie.criticality), assigned, "{name}");
            ies += 1;
        }
        let shown = tree["ies"].as_array().unwrap();
        assert!(
            shown.iter().all(|ie| ie.get("criticality").is_none()),
            "{name}"
        );
    }
    assert!(ies > 300, "{ies}");
}

/// What a PDU that is written from `tree` has for the IE of the extension container at
/// `extension`, a path to its value, whose type is the one that the type which the
/// container extends gives it: the value at `at` under it is `value`, and the octets of
/// the IE are `octets`, which the bindings give a value of its type. The PDU is the same
/// octets through its tree. Set to `other`, the IE has `other_octets`.
fn an_extension_has_its_type(
    tree: Value,
    extension: &str,
    at: &str,
    (value, octets): (Value, Vec<u8>),
    (other, other_octets): (Value, Vec<u8>),
    octets_at: impl Fn(&NGAPPDU<'_>) -> Vec<u8>,
) {
    let at = match at {
        "" => extension.to_string(),
        at => format!("{extension}/{at}"),
    };
    // Written from its tree, and typed in the tree of what was received.
    let pdu = NGAPPDU::from_tree(&tree).unwrap();
    assert_eq!(octets_at(&pdu), octets, "{at}");
    let wire = pdu.encode().unwrap();
    let mut shown = NGAPPDU::decode(&wire).unwrap().to_tree();
    assert_eq!(select(&shown, &at), [value], "{at}");
    assert_eq!(shown, tree);
    assert_eq!(NGAPPDU::from_tree(&shown).unwrap().encode().unwrap(), wire);
    // Edited.
    inspect::set(&mut shown, &at, other.clone()).unwrap();
    let edited = NGAPPDU::from_tree(&shown).unwrap();
    assert_eq!(select(&edited.to_tree(), &at), [other], "{at}");
    assert_eq!(octets_at(&edited), other_octets, "{at}");
}

/// id-QosFlowAdditionalInfoList is a QosFlowAdditionalInfoListRelCom where it extends the
/// transfer of a release command and a QosFlowAdditionalInfoListRelRes where it extends
/// that of a release response: the items of the first have a downlink container, and
/// those of the second an uplink one.
#[test]
fn the_additional_information_of_qos_flows_has_the_type_of_what_it_extends() {
    let flow = || QosFlowIdentifier(1);
    let container = |octets: &[u8]| Some(TLContainer(octets.to_vec().into()));
    let command = |octets: &[u8]| {
        let item = QosFlowAdditionalInfoItemRelCom {
            qos_flow_identifier: flow(),
            downlink_tl_container: container(octets),
            ie_extensions: None,
        };
        oxirush_ngap::encode(&QosFlowAdditionalInfoListRelCom(vec![item])).unwrap()
    };
    let response = |octets: &[u8]| {
        let item = QosFlowAdditionalInfoItemRelRes {
            qos_flow_identifier: flow(),
            uplink_tl_container: container(octets),
            ie_extensions: None,
        };
        oxirush_ngap::encode(&QosFlowAdditionalInfoListRelRes(vec![item])).unwrap()
    };
    // Of the transfer of a release command.
    let flows = |container: &str| tree!([{"qosFlowIdentifier": 1, container: "0102"}]);
    let released = |container: &str| {
        tree!({"message": "PDUSessionResourceReleaseCommand", "ies": [
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
    let of_command = |pdu: &NGAPPDU<'_>| {
        let ies = pdu.protocol_ies().unwrap();
        let list: PDUSessionResourceToReleaseListRelCmd =
            oxirush_ngap::decode(&ies.0[2].value).unwrap();
        let transfer: PDUSessionResourceReleaseCommandTransfer =
            oxirush_ngap::decode(&list.0[0].pdu_session_resource_release_command_transfer).unwrap();
        transfer.ie_extensions.unwrap().0[0]
            .extension_value
            .to_vec()
    };
    an_extension_has_its_type(
        released("downlinkTLContainer"),
        &format!("{command_transfer}/QosFlowAdditionalInfoList"),
        "0/downlinkTLContainer",
        (tree!("0102"), command(&[1, 2])),
        (tree!("0a0b0c"), command(&[10, 11, 12])),
        of_command,
    );
    // Of the transfer of a release response.
    let responded = |container: &str| {
        tree!({"message": "PDUSessionResourceReleaseResponse", "ies": [
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
    let of_response = |pdu: &NGAPPDU<'_>| {
        let ies = pdu.protocol_ies().unwrap();
        let list: PDUSessionResourceReleasedListRelRes =
            oxirush_ngap::decode(&ies.0[2].value).unwrap();
        let transfer: PDUSessionResourceReleaseResponseTransfer =
            oxirush_ngap::decode(&list.0[0].pdu_session_resource_release_response_transfer)
                .unwrap();
        transfer.ie_extensions.unwrap().0[0]
            .extension_value
            .to_vec()
    };
    an_extension_has_its_type(
        responded("uplinkTLContainer"),
        &format!("{response_transfer}/0"),
        "0/uplinkTLContainer",
        (tree!("0102"), response(&[1, 2])),
        (tree!("0a0b0c"), response(&[10, 11, 12])),
        of_response,
    );
    // Each has the members of its type, written and at a path, and not those of the other.
    for wrong in [
        released("uplinkTLContainer"),
        responded("downlinkTLContainer"),
    ] {
        let refused = NGAPPDU::from_tree(&wrong).unwrap_err().to_string();
        assert!(
            refused.contains("is not a member that its type has"),
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
            let refused = inspect::check_path(message, &format!("{item}/{has_not}"))
                .unwrap_err()
                .to_string();
            assert!(
                refused.contains("is not a member of QosFlowAdditionalInfoItemRel"),
                "{refused}"
            );
            assert!(refused.contains(has), "{refused}");
        }
    }
}

#[test]
fn every_pdu_is_its_octets_through_its_tree_whatever_its_ies_hold() {
    // Each PDU of the fixtures, and each with the last bit of the value of one of its IEs
    // changed, which is a padding bit of many.
    let mut octets = 0;
    let mut same = |wire: &[u8]| {
        let Ok(pdu) = NGAPPDU::decode(wire) else {
            return;
        };
        let tree = pdu.to_tree();
        assert_eq!(NGAPPDU::from_tree(&tree).as_ref(), Ok(&pdu), "{tree}");
        octets += tree.to_string().matches("\"octets\"").count();
    };
    for file in [
        include_str!("fixtures/messages.tsv"),
        include_str!("fixtures/vectors.tsv"),
        include_str!("fixtures/captured.tsv"),
        include_str!("fixtures/later.tsv"),
        include_str!("fixtures/fuzzed.tsv"),
    ] {
        let lines = file
            .lines()
            .filter(|line| !line.starts_with('#') && !line.is_empty());
        for line in lines {
            let wire = hex::decode(line.split('\t').nth(1).unwrap()).unwrap();
            same(&wire);
            let Ok(pdu) = NGAPPDU::decode(&wire) else {
                continue;
            };
            let Ok(ies) = pdu.protocol_ies() else {
                continue;
            };
            let ies = ies.into_owned();
            for at in 0..ies.0.len() {
                let mut changed = ies.clone();
                let mut value = changed.0[at].value.to_vec();
                let Some(last) = value.last_mut() else {
                    continue;
                };
                *last ^= 1;
                changed.0[at].value = oxirush_ngap::Octets::owned(value);
                let mut pdu = pdu.clone();
                pdu.set_protocol_ies(changed).unwrap();
                same(&pdu.encode().unwrap());
            }
        }
    }
    // Some of them have IEs that the tree has as their octets.
    assert!(octets > 100, "{octets}");
}
