/*
   OxiRush
   Copyright 2025-2026 Valentin D'Emmanuele

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! The PDUs of `tests/wireshark/corpus.tsv` as Wireshark reads them and as
//! this crate reads them, compared field by field.
//!
//! The crate's reading of each PDU is its tree and what `check()` finds,
//! written here as one line of JSON, with the container of a handover decoded
//! where the message says that it is an NG-RAN node's (see [`containers`]); `tests/wireshark/compare.py` has tshark
//! dissect the same octets as NGAP and compares the two along
//! `tests/wireshark/mapping.tsv`. A difference that
//! `tests/wireshark/known.tsv` does not explain fails the test. Without
//! tshark, or without Python 3, the test says that it is skipped and passes;
//! with a tshark other than the one that known.tsv was triaged with,
//! compare.py says so and passes.
//!
//! Other PDUs are compared with `NGAP_WIRESHARK_CORPUS=<rows>`, rows as those
//! of the corpus, and `NGAP_WIRESHARK_OUTPUT=<directory>` keeps the report:
//! `cargo test --features inspect --test wireshark`.

#![cfg(feature = "inspect")]

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use oxirush_ngap::Ngap;
use oxirush_ngap::inspect;
use oxirush_ngap::ngap::NGAPPDU;
use oxirush_utils::tree::{Map, Value as Tree};
use serde_json::{Value, json};

fn found(program: &str, argument: &str) -> bool {
    Command::new(program)
        .arg(argument)
        .output()
        .is_ok_and(|output| output.status.success())
}

/// The value of a view in the form of a tree: without what a view keeps of what was
/// received (its members that start with `_`), and an IE or an extension as its name
/// with its value.
fn as_tree(value: &Tree) -> Tree {
    match value {
        Tree::Object(members)
            if members.contains_key("id")
                && (members.contains_key("value") || members.contains_key("extensionValue")) =>
        {
            let id = members["id"].as_u64();
            let name = members
                .get("_ie_name")
                .and_then(Tree::as_str)
                .or_else(|| {
                    let names = Ngap::ie_names().iter();
                    names
                        .copied()
                        .find(|&(of, _)| Some(u64::from(of)) == id)
                        .map(|(_, name)| name)
                })
                .map_or_else(|| members["id"].to_string(), str::to_owned);
            let inner = members
                .get("value")
                .or_else(|| members.get("extensionValue"));
            let mut ie = Map::new();
            ie.insert(name, inner.map(as_tree).unwrap_or_default());
            if let Some(criticality) = members.get("criticality") {
                ie.insert("criticality".into(), criticality.clone());
            }
            Tree::Object(ie)
        }
        Tree::Object(members) => Tree::Object(
            members
                .iter()
                .filter(|(name, _)| !name.starts_with('_'))
                .map(|(name, member)| (name.clone(), as_tree(member)))
                .collect(),
        ),
        Tree::Array(entries) => Tree::Array(entries.iter().map(as_tree).collect()),
        other => other.clone(),
    }
}

/// The containers of a handover in the tree, decoded where the message says which
/// type their octets carry: TS 38.413 gives them by reference (§9.3.1.20, §9.3.1.21),
/// and they are an NG-RAN node's own when the HandoverType of the message is a
/// handover within the 5GS or to it from the EPS, and in a HANDOVER REQUEST
/// ACKNOWLEDGE, which the target NG-RAN node sends. This is the choice that Wireshark
/// makes; for another HandoverType, or none, the octets stay.
fn containers(pdu: &NGAPPDU<'_>, tree: &mut Tree) {
    let ies = tree
        .get("ies")
        .and_then(Tree::as_array)
        .cloned()
        .unwrap_or_default();
    let to_ngran = ies.iter().any(|ie| {
        matches!(
            ie.get("HandoverType").and_then(Tree::as_str),
            Some("intra5gs" | "eps-to-5gs")
        )
    });
    let acknowledge =
        tree.get("message").and_then(Tree::as_str) == Some("HandoverRequestAcknowledge");
    let Ok(mut view) = pdu.to_view() else { return };
    for (index, ie) in ies.iter().enumerate() {
        for (container, carried, decoded_when) in [
            (
                "SourceToTarget-TransparentContainer",
                "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer",
                to_ngran,
            ),
            (
                "TargetToSource-TransparentContainer",
                "TargetNGRANNode-ToSourceNGRANNode-TransparentContainer",
                to_ngran || acknowledge,
            ),
        ] {
            if !decoded_when || !ie.get(container).is_some_and(Tree::is_string) {
                continue;
            }
            let path = format!("/ngap/{container}/{carried}");
            if inspect::open(&mut view, &path).is_err() {
                continue;
            }
            // Octets that do not decode as the type keep their place in the tree
            if let Ok([opened]) = inspect::select(&view, &path).as_deref()
                && let Some(decoded) = opened.get("decoded")
            {
                let mut value = Map::new();
                value.insert(carried.into(), as_tree(decoded));
                if let Some(ie) = tree
                    .get_mut("ies")
                    .and_then(|ies| ies.as_array_mut()?.get_mut(index))
                    .and_then(Tree::as_object_mut)
                {
                    ie.insert(container.into(), Tree::Object(value));
                }
            }
        }
    }
}

/// What the crate reads of `octets`, as one line of JSON. The tree is
/// written as the crate prints it, whose objects keep the order of the PDU.
fn reading(octets: &[u8]) -> String {
    let head = json!({"octets": hex::encode(octets)}).to_string();
    let head = head.trim_end_matches('}');
    match NGAPPDU::decode(octets) {
        Ok(pdu) => {
            let check: Value = match pdu.check() {
                Ok(deviations) => deviations.iter().map(ToString::to_string).collect(),
                Err(error) => json!([format!("error: {error}")]),
            };
            let mut tree = pdu.to_tree();
            containers(&pdu, &mut tree);
            format!("{head},\"check\":{check},\"tree\":{tree}}}")
        }
        Err(error) => format!("{head},\"error\":{}}}", json!(error.to_string())),
    }
}

#[test]
fn wireshark_reads_the_pdus_as_the_crate_does() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/wireshark");
    if !found("tshark", "--version") {
        eprintln!("skipped: tshark is not installed, so the PDUs are not compared with Wireshark");
        return;
    }
    if !found("python3", "--version") {
        eprintln!("skipped: python3 is not installed, which compares the PDUs with Wireshark");
        return;
    }
    let corpus = std::env::var_os("NGAP_WIRESHARK_CORPUS")
        .map_or_else(|| here.join("corpus.tsv"), PathBuf::from);
    let output = std::env::var_os("NGAP_WIRESHARK_OUTPUT").map_or_else(
        || Path::new(env!("CARGO_TARGET_TMPDIR")).join("wireshark"),
        PathBuf::from,
    );
    std::fs::create_dir_all(&output).expect("the output directory");

    let rows = std::fs::read_to_string(&corpus).expect("the corpus");
    let mut lines = String::new();
    for row in rows
        .lines()
        .filter(|row| !row.is_empty() && !row.starts_with('#'))
    {
        let octets = row.split('\t').next_back().expect("a column");
        let octets = hex::decode(octets).unwrap_or_else(|error| panic!("{row}: {error}"));
        writeln!(lines, "{}", reading(&octets)).expect("a line");
    }
    let readings = output.join("crate.jsonl");
    std::fs::write(&readings, lines).expect("the crate's readings");

    let status = Command::new("python3")
        .arg("-I")
        .arg(here.join("compare.py"))
        .arg("--protocol")
        .arg("ngap")
        .arg("--corpus")
        .arg(&corpus)
        .arg("--crate")
        .arg(&readings)
        .arg("--mapping")
        .arg(here.join("mapping.tsv"))
        .arg("--known")
        .arg(here.join("known.tsv"))
        .arg("--output")
        .arg(&output)
        // A standard input of its own: tshark reads each capture from its
        // standard input, which a descriptor 0 left closed would take
        .stdin(Stdio::null())
        .status()
        .expect("python3 runs");
    assert!(
        status.success(),
        "Wireshark and the crate differ: {}",
        output.display()
    );
}
