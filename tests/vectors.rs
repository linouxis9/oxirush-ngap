#![cfg(feature = "inspect")]
//! The vectors of `tests/fixtures/vectors.tsv` through the JSON forms, which read each
//! IE, each transfer and each IE of a container with its type. pycrate encoded them:
//! `protocols/tests/fixtures/generate.py` of oxirush-aper says what they hold, and
//! `tests/aper_vectors.rs` has them as PDUs.

use std::collections::{BTreeMap, BTreeSet};

use oxirush_ngap::Ngap;
use oxirush_ngap::inspect;
use oxirush_ngap::ngap::NGAPPDU;
use oxirush_ngap::tree;
use oxirush_ngap::tree::Value;

const MESSAGES: &str = include_str!("fixtures/messages.tsv");
const VECTORS: &str = include_str!("fixtures/vectors.tsv");
const LATER: &str = include_str!("fixtures/later.tsv");
const NULL_PATHS: &str = include_str!("fixtures/null_paths.tsv");
const FUZZED: &str = include_str!("fixtures/fuzzed.tsv");

/// The IEs whose octets carry a type of TS 38.413 between two NG-RAN nodes, each with
/// the name of that type: the vectors have a value of it there.
const CARRIED: [(&str, &str); 3] = [
    (
        "SourceToTarget-TransparentContainer",
        "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer",
    ),
    (
        "TargetToSource-TransparentContainer",
        "TargetNGRANNode-ToSourceNGRANNode-TransparentContainer",
    ),
    (
        "TargettoSource-Failure-TransparentContainer",
        "TargetNGRANNode-ToSourceNGRANNode-FailureTransparentContainer",
    ),
];

/// The vectors of a file of the fixtures: the name of each, and the octets of its PDU.
fn vectors(file: &'static str) -> impl Iterator<Item = (&'static str, Vec<u8>)> {
    let lines = file.lines();
    let lines = lines.filter(|line| !line.starts_with('#') && !line.is_empty());
    lines.map(|line| {
        let mut fields = line.split('\t');
        let name = fields.next().unwrap();
        (name, hex::decode(fields.next().unwrap()).unwrap())
    })
}

/// The tree of a PDU, with the containers of a handover decoded as the types that they
/// carry.
fn opened(pdu: &NGAPPDU<'_>) -> Value {
    let mut tree = pdu.to_tree();
    for (ie, carried) in CARRIED {
        let ies = tree["ies"].as_array();
        if ies.is_some_and(|ies| ies.iter().any(|entry| entry.get(ie).is_some())) {
            inspect::open(&mut tree, &format!("/ngap/{ie}/{carried}")).unwrap();
        }
    }
    tree
}

/// The identifier of each IE that a tree holds, at any depth: in the message, in a list,
/// in a transfer and in an extension container, each an object of one member, the name
/// of the IE, beside its criticality. `unread` takes each value that is its octets, as
/// an IE or a transfer that was not read with its type is.
fn identifiers(value: &Value, found: &mut BTreeSet<u16>, unread: &mut Vec<String>) {
    let names: &BTreeMap<&str, u16> = &IDS;
    match value {
        Value::Object(members) if members.len() == 1 && members.contains_key("octets") => {
            unread.push(value.to_string());
        }
        Value::Object(members) => {
            let mut named = members.keys().filter(|name| *name != "criticality");
            if let (Some(name), None) = (named.next(), named.next()) {
                found.extend(names.get(name.as_str()));
            }
            for member in members.values() {
                identifiers(member, found, unread);
            }
        }
        Value::Array(entries) => {
            for entry in entries {
                identifiers(entry, found, unread);
            }
        }
        _ => {}
    }
}

/// The identifier of each IE by its name.
static IDS: std::sync::LazyLock<BTreeMap<&str, u16>> = std::sync::LazyLock::new(|| {
    Ngap::ie_names()
        .iter()
        .map(|&(id, name)| (name, id))
        .collect()
});

/// The octets of the PDU of a tree, or why it has none.
fn octets(tree: &Value) -> Result<String, String> {
    let pdu = NGAPPDU::from_tree(tree).map_err(|error| error.to_string())?;
    let octets = pdu.encode().map_err(|error| error.to_string())?;
    Ok(hex::encode(octets))
}

#[test]
fn each_vector_is_read_with_its_types_and_written_to_the_same_octets() {
    let mut count = 0;
    for (name, wire) in vectors(VECTORS) {
        count += 1;
        let pdu = NGAPPDU::decode(&wire).unwrap();
        let tree = opened(&pdu);
        let wire = Ok(hex::encode(&wire));

        // Every IE and every transfer is read with its type, and the tree is written
        // from its values alone: each IE, each transfer and each container from its type.
        if !name.starts_with("PrivateMessage/") {
            let (mut found, mut unread) = (BTreeSet::new(), Vec::new());
            identifiers(&tree, &mut found, &mut unread);
            assert_eq!(unread, Vec::<String>::new(), "{name}");
        }
        assert_eq!(octets(&tree), wire, "{name}: its tree");
    }
    assert!(count > 400, "{count} vectors");
}

/// How much of the protocol the fixtures hold. The registry names 489 IEs, those of the
/// extension containers among them, and 144 messages.
#[test]
fn the_vectors_hold_each_message_and_each_ie_of_the_registry() {
    let held = |file| {
        let (mut found, mut unread) = (BTreeSet::new(), Vec::new());
        for (_, wire) in vectors(file) {
            let tree = opened(&NGAPPDU::decode(&wire).unwrap());
            identifiers(&tree["ies"], &mut found, &mut unread);
        }
        found
    };
    let names: BTreeMap<u16, &str> = Ngap::ie_names().iter().copied().collect();
    let (minimal, content) = (held(MESSAGES), held(VECTORS));
    assert_eq!(names.len(), 489);

    // The messages of `messages.tsv` have what is mandatory: 111 IEs of the 489.
    assert!(minimal.iter().all(|id| names.contains_key(id)));
    assert_eq!(minimal.len(), 111);

    // The vectors of `vectors.tsv` have every IE of the registry.
    assert!(content.iter().all(|id| names.contains_key(id)));
    let missing: Vec<_> = names.keys().filter(|id| !content.contains(id)).collect();
    let missing: Vec<_> = missing.into_iter().map(|id| names[id]).collect();
    assert_eq!(missing, Vec::<&str>::new());
    assert_eq!(content.len(), 489);

    // Each of the 144 messages is in both files.
    let messages: BTreeSet<_> = Ngap::object_sets().iter().map(|set| set.name).collect();
    let of = |file| -> BTreeSet<&str> {
        let names = vectors(file).map(|(name, _)| name.split('/').next().unwrap());
        names.collect()
    };
    assert_eq!(messages.len(), 144);
    assert_eq!(of(MESSAGES), messages);
    assert_eq!(of(VECTORS), messages);
}

/// `later.tsv` has messages of a later release: pycrate encoded them with modules in
/// which a few types have additions after their extension marker, 64 of them for one
/// type, and encoded the same messages without the additions with the modules as they
/// are. The additions are skipped where they are, in a list and in a container too
/// (X.691 (02/2021) 19.7-19.9): the PDU keeps the octets of its IEs, and its tree has an
/// IE that holds an addition as its octets, which it gives back, and every other IE as
/// the message without the additions has it.
#[test]
fn the_additions_of_a_later_release_are_kept_as_pycrate_writes_them() {
    let mut count = 0;
    for line in LATER.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<_> = line.split('\t').collect();
        let (name, later, known) = (fields[0], fields[1], fields[4]);
        assert!(later.len() > known.len(), "{name}");
        count += 1;

        let pdu = NGAPPDU::decode(&hex::decode(later).unwrap())
            .unwrap()
            .into_owned();
        assert_eq!(hex::encode(pdu.encode().unwrap()), later, "{name}");
        let tree = opened(&pdu);
        assert_eq!(octets(&tree), Ok(later.to_string()), "{name}");
        let without = opened(&NGAPPDU::decode(&hex::decode(known).unwrap()).unwrap());
        let (Some(Value::Array(ies)), Some(Value::Array(known))) =
            (tree.get("ies"), without.get("ies"))
        else {
            panic!("{name}: a message of IEs");
        };
        assert_eq!(ies.len(), known.len(), "{name}");
        let mut kept = 0;
        for (ie, known) in ies.iter().zip(known) {
            let (mut found, mut unread) = (BTreeSet::new(), Vec::new());
            identifiers(ie, &mut found, &mut unread);
            match unread.is_empty() {
                true => assert_eq!(ie, known, "{name}"),
                false => kept += 1,
            }
        }
        assert!(kept > 0, "{name}");
    }
    assert_eq!(count, 9);
}

/// The first finding of the review of the move of the paths to oxirush-utils: an
/// alternative of the type NULL, as `{"noReporting": null}`, was read as absent, so that
/// `paths()` did not list it and `select()` selected nothing. `null_paths.tsv` has the 44
/// paths of the vectors that it found: each is listed and selects `null`, and `null` set
/// there is that alternative again. The review of the release candidate found 43 more in
/// the same vectors under a procedure code that the specification does not have, which
/// `select()` did not take.
#[test]
fn each_alternative_of_the_type_null_is_listed_and_selected() {
    let wires: BTreeMap<_, _> = vectors(VECTORS).collect();
    let mut count = 0;
    for line in NULL_PATHS.lines().filter(|line| !line.starts_with('#')) {
        let mut fields = line.split('\t');
        let (name, path) = (fields.next().unwrap(), fields.next().unwrap());
        let mut octets = wires[name].clone();
        if let Some(code) = fields.next() {
            octets[1] = code.parse().unwrap();
        }
        let pdu = NGAPPDU::decode(&octets).unwrap();
        let mut tree = pdu.to_tree();
        assert_eq!(
            inspect::select(&tree, path),
            Ok(vec![&Value::Null]),
            "{name} {path}"
        );
        let paths = inspect::paths(&tree);
        assert!(
            paths.contains(&(path.to_string(), Value::Null)),
            "{name} {path}"
        );
        inspect::set(&mut tree, path, Value::Null).unwrap();
        assert_eq!(NGAPPDU::from_tree(&tree).unwrap(), pdu, "{name} {path}");
        count += 1;
    }
    assert_eq!(count, 44 + 43);
}

/// The NG SETUP REQUEST whose values pycrate writes in the JSON Encoding Rules of ITU-T
/// X.697: the procedure code, the direction and the criticality of its PDU, and the
/// protocol IEs of its message.
fn setup_request(ies: Value) -> Value {
    tree!({
        "procedure_code": 21,
        "direction": "InitiatingMessage",
        "criticality": "reject",
        "message": {"protocolIEs": ies},
    })
}

/// The review of the release candidate is right that a string of decimal digits is never
/// read as octets in a tree, where it is a number or a PLMN identity; in the JSON Encoding
/// Rules it is octets, and the values of the reference are written in them:
/// `__tree_of_jer` reads them as a tree has them, and `from_tree` refuses them as they are.
#[test]
fn the_json_encoding_rules_are_read_as_their_octets() {
    let jer = setup_request(tree!([
        {"id": 27, "criticality": "reject", "value": {"globalGNB-ID": {
            "pLMNIdentity": "02F839",
            "gNB-ID": {"gNB-ID": {"value": "000004", "length": 22}},
        }}},
        {"id": 102, "criticality": "reject", "value": [{
            "tAC": "000001",
            "broadcastPLMNList": [{
                "pLMNIdentity": "02F839",
                "tAISliceSupportList": [{"s-NSSAI": {"sST": "01"}}],
            }],
        }]},
        {"id": 21, "criticality": "ignore", "value": "v128"},
    ]));
    let readable = tree!({"message": "NGSetupRequest", "ies": [
        {"GlobalRANNodeID": {"globalGNB-ID": {
            "pLMNIdentity": "208-93",
            "gNB-ID": {"gNB-ID": {"value": "000004", "length": 22}},
        }}},
        {"SupportedTAList": [{
            "tAC": 1,
            "broadcastPLMNList": [{
                "pLMNIdentity": "208-93",
                "tAISliceSupportList": [{"s-NSSAI": {"sST": 1}}],
            }],
        }]},
        {"DefaultPagingDRX": "v128"},
    ]});
    let read = inspect::__tree_of_jer(&jer).unwrap();
    assert_eq!(read["ies"][1]["SupportedTAList"][0]["tAC"], tree!(1));
    let read = NGAPPDU::from_tree(&read).unwrap();
    assert_eq!(read, NGAPPDU::from_tree(&readable).unwrap());
    let mut as_jer = readable.clone();
    as_jer["ies"][1]["SupportedTAList"][0]["tAC"] = tree!("000001");
    let error = NGAPPDU::from_tree(&as_jer).unwrap_err().to_string();
    assert!(
        error.contains(r#""000001" is a number in quotes: write 1 without quotes"#),
        "{error}"
    );
}

/// The values of the vectors as the reference has them: `generate.py --values FILE` of
/// oxirush-aper writes each in the JSON encoding of ASN.1, and `REFERENCE_VALUES` is that
/// file. A PDU that is written from the tree of the values of a vector is
/// the octets of the vector: the crate gives each name, each number and each string of
/// the reference the bits that the reference gave it.
#[test]
#[ignore = "needs the values that generate.py --values of oxirush-aper writes"]
fn a_pdu_written_from_the_values_of_the_reference_is_the_octets_of_the_reference() {
    let path = std::env::var("REFERENCE_VALUES").expect("REFERENCE_VALUES is a file");
    let values = std::fs::read_to_string(path).unwrap();
    let wires: BTreeMap<_, _> = vectors(VECTORS).collect();
    let mut failures = Vec::new();
    let mut count = 0;
    for line in values.lines() {
        let (name, values) = line.split_once('\t').unwrap();
        let values: Value = serde_json::from_str(values).unwrap();
        count += 1;
        // The values are written in the JSON Encoding Rules, which a tree reads so.
        let tree = inspect::__tree_of_jer(&values).unwrap();
        match octets(&tree) {
            Ok(written) if written == hex::encode(&wires[name]) => {}
            Ok(_) => failures.push(format!("{name}: other octets")),
            Err(error) => failures.push(format!("{name}: {error}")),
        }
    }
    assert_eq!(count, wires.len());
    assert!(
        failures.is_empty(),
        "{} of {count}:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// The PDUs that a fuzzer found a fault with hold to the properties of the fuzz target
/// `json`: the review of the release candidate found a PDU of a procedure that the
/// specification does not have, whose paths listed a value that `select()` did not take.
/// Each gives its tree back, and each path that its tree lists selects its value.
#[test]
fn the_pdus_that_a_fuzzer_found_a_fault_with_hold_to_the_properties_of_its_target() {
    let mut count = 0;
    for (name, octets) in vectors(FUZZED) {
        let pdu = NGAPPDU::decode(&octets).unwrap();
        let tree = pdu.to_tree();
        assert_eq!(NGAPPDU::from_tree(&tree).as_ref(), Ok(&pdu), "{name}");
        for (path, value) in inspect::paths(&tree) {
            assert_eq!(
                inspect::select(&tree, &path),
                Ok(vec![&value]),
                "{name} {path}"
            );
        }
        count += 1;
    }
    assert_eq!(count, 1);
}
