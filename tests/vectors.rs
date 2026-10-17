#![cfg(feature = "inspect")]
//! The vectors of `tests/fixtures/vectors.tsv` through the JSON forms, which read each
//! IE, each transfer and each IE of a container with its type. pycrate encoded them:
//! `tests/fixtures/generate.py` says what they hold, and `tests/aper_vectors.rs` has
//! them as PDUs.

use std::collections::{BTreeMap, BTreeSet};

use oxirush_ngap::inspect;
use oxirush_ngap::ngap::NGAP_PDU;
use oxirush_ngap::tree::Value;

const MESSAGES: &str = include_str!("fixtures/messages.tsv");
const VECTORS: &str = include_str!("fixtures/vectors.tsv");
const LATER: &str = include_str!("fixtures/later.tsv");
const NULL_PATHS: &str = include_str!("fixtures/null_paths.tsv");

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

/// The view of a PDU, with the containers of a handover decoded as the types that they
/// carry.
fn opened(pdu: &NGAP_PDU) -> Value {
    let mut view = pdu.to_view().unwrap();
    for (ie, carried) in CARRIED {
        let ies = view["message"]["protocolIEs"].as_array();
        if ies.is_some_and(|ies| ies.iter().any(|entry| entry["_ie_name"] == ie)) {
            inspect::open(&mut view, &format!("/ngap/{ie}/{carried}")).unwrap();
        }
    }
    view
}

/// `value` without what a view keeps of what was received: the members whose name
/// starts with `_`, at any depth.
fn written(value: &Value) -> Value {
    match value {
        Value::Object(members) => {
            let kept = members.iter().filter(|(name, _)| !name.starts_with('_'));
            Value::Object(
                kept.map(|(name, value)| (name.clone(), written(value)))
                    .collect(),
            )
        }
        Value::Array(entries) => Value::Array(entries.iter().map(written).collect()),
        other => other.clone(),
    }
}

/// The identifier of each IE that a view holds, at any depth: in the message, in a list,
/// in a transfer and in an extension container. `unread` takes why an IE or a transfer
/// was not read with its type.
fn identifiers(value: &Value, found: &mut BTreeSet<u16>, unread: &mut Vec<String>) {
    match value {
        Value::Object(members) => {
            if let (Some(id), true) = (members.get("id"), members.contains_key("criticality")) {
                found.extend(id.as_u64().map(|id| id as u16));
            }
            unread.extend(members.get("_decode_error").map(Value::to_string));
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

/// The octets of the PDU of a view, or why it has none.
fn octets(view: &Value) -> Result<String, String> {
    let pdu = NGAP_PDU::from_view(view).map_err(|error| error.to_string())?;
    let octets = pdu.encode().map_err(|error| error.to_string())?;
    Ok(hex::encode(octets))
}

#[test]
fn each_vector_is_read_with_its_types_and_written_to_the_same_octets() {
    let mut count = 0;
    for (name, wire) in vectors(VECTORS) {
        count += 1;
        let pdu = NGAP_PDU::decode(&wire).unwrap();
        let view = opened(&pdu);
        let wire = Ok(hex::encode(wire));

        // Every IE and every transfer is read with its type.
        let (mut found, mut unread) = (BTreeSet::new(), Vec::new());
        identifiers(&view, &mut found, &mut unread);
        assert_eq!(unread, Vec::<String>::new(), "{name}");

        // The view keeps the octets received, and without them it is written from its
        // values alone: each IE, each transfer and each container from its type.
        assert_eq!(octets(&view), wire, "{name}: its view");
        assert_eq!(octets(&written(&view)), wire, "{name}: its values");
        assert_eq!(pdu.with_view(&view).unwrap(), pdu, "{name}");

        // And so is its tree, for a message that has IEs to name.
        if !name.starts_with("PrivateMessage/") {
            let tree = pdu.to_tree();
            let again = NGAP_PDU::from_tree(&tree).unwrap().encode().unwrap();
            assert_eq!(Ok(hex::encode(again)), wire, "{name}: its tree");
        }
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
            let view = opened(&NGAP_PDU::decode(&wire).unwrap());
            identifiers(&view["message"], &mut found, &mut unread);
        }
        found
    };
    let names: BTreeMap<u16, &str> = inspect::ie_names().iter().copied().collect();
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
    let messages: BTreeSet<_> = inspect::message_names().map(|(.., name)| name).collect();
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
/// are. The additions are skipped where they are, in a list and in a container too, and
/// what is read is the message without them (X.691 (02/2021) 19.7-19.9).
#[test]
fn the_additions_of_a_later_release_are_skipped_as_pycrate_writes_them() {
    let mut count = 0;
    for line in LATER.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<_> = line.split('\t').collect();
        let (name, later, known) = (fields[0], fields[1], fields[4]);
        assert!(later.len() > known.len(), "{name}");
        count += 1;

        // The PDU keeps the octets of its IEs, the additions among them.
        let pdu = NGAP_PDU::decode(&hex::decode(later).unwrap()).unwrap();
        assert_eq!(hex::encode(pdu.encode().unwrap()), later, "{name}");

        // Each IE is read with its type, as the same value as without the additions.
        let view = opened(&pdu);
        let (mut found, mut unread) = (BTreeSet::new(), Vec::new());
        identifiers(&view, &mut found, &mut unread);
        assert_eq!(unread, Vec::<String>::new(), "{name}");
        let without = opened(&NGAP_PDU::decode(&hex::decode(known).unwrap()).unwrap());
        assert_eq!(written(&view), written(&without), "{name}");

        // And its values are written as the message without them.
        assert_eq!(octets(&written(&view)), Ok(known.to_string()), "{name}");
    }
    assert_eq!(count, 9);
}

/// The first finding of the review of the move of the paths to oxirush-utils: an
/// alternative of the type NULL, as `{"noReporting": null}`, was read as absent, so that
/// `paths()` did not list it and `select()` selected nothing. `null_paths.tsv` has the 44
/// paths of the vectors that it found: each is listed and selects `null`, and `null` set
/// there is that alternative again.
#[test]
fn each_alternative_of_the_type_null_is_listed_and_selected() {
    let wires: BTreeMap<_, _> = vectors(VECTORS).collect();
    let mut count = 0;
    for line in NULL_PATHS.lines().filter(|line| !line.starts_with('#')) {
        let (name, path) = line.split_once('\t').unwrap();
        let pdu = NGAP_PDU::decode(&wires[name]).unwrap();
        let mut view = pdu.to_view().unwrap();
        assert_eq!(
            inspect::select(&view, path),
            Ok(vec![&Value::Null]),
            "{name} {path}"
        );
        let paths = inspect::paths(&view);
        assert!(
            paths.contains(&(path.to_string(), Value::Null)),
            "{name} {path}"
        );
        inspect::set(&mut view, path, Value::Null).unwrap();
        assert_eq!(pdu.with_view(&view).unwrap(), pdu, "{name} {path}");
        count += 1;
    }
    assert_eq!(count, 44);
}

/// The values of the vectors as the reference has them: `tests/fixtures/generate.py
/// --values FILE` writes each as a view, in the JSON encoding of ASN.1, and
/// `REFERENCE_VALUES` is that file. A PDU that is written from the view of a vector is
/// the octets of the vector: the crate gives each name, each number and each string of
/// the reference the bits that the reference gave it.
#[test]
#[ignore = "needs the values that tests/fixtures/generate.py --values writes"]
fn a_pdu_written_from_the_values_of_the_reference_is_the_octets_of_the_reference() {
    let path = std::env::var("REFERENCE_VALUES").expect("REFERENCE_VALUES is a file");
    let values = std::fs::read_to_string(path).unwrap();
    let wires: BTreeMap<_, _> = vectors(VECTORS).collect();
    let mut failures = Vec::new();
    let mut count = 0;
    for line in values.lines() {
        let (name, view) = line.split_once('\t').unwrap();
        let view: Value = serde_json::from_str(view).unwrap();
        count += 1;
        match octets(&view) {
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
