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
//! The crate's reading of each PDU is its tree and what `check()` finds, written here
//! as one line of JSON, with the container of a handover decoded where the message says
//! that it is an NG-RAN node's (see [`containers`]); `compare.py` of
//! `protocols/tests/wireshark/` in the checkout of oxirush-aper beside the repository
//! has tshark dissect the same octets as NGAP and compares the two along
//! `tests/wireshark/mapping.tsv`. A difference that `tests/wireshark/known.tsv` does
//! not explain fails the test. Without tshark, without Python 3 or without that
//! checkout, the test says that it is skipped and passes; with a tshark other than the
//! one that known.tsv was triaged with, compare.py says so and passes.
//!
//! Other PDUs are compared with `NGAP_WIRESHARK_CORPUS=<rows>`, rows as those
//! of the corpus, and `NGAP_WIRESHARK_OUTPUT=<directory>` keeps the report:
//! `cargo test --features inspect --test wireshark`.

#![cfg(feature = "inspect")]

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use oxirush_ngap::inspect;
use oxirush_ngap::ngap::NGAPPDU;
use oxirush_utils::tree::Value as Tree;
use serde_json::{Value, json};

fn found(program: &str, argument: &str) -> bool {
    Command::new(program)
        .arg(argument)
        .output()
        .is_ok_and(|output| output.status.success())
}

/// The containers of a handover in the tree, decoded where the message says which
/// type their octets carry: TS 38.413 gives them by reference (§9.3.1.20, §9.3.1.21),
/// and they are an NG-RAN node's own when the HandoverType of the message is a
/// handover within the 5GS or to it from the EPS, and in a HANDOVER REQUEST
/// ACKNOWLEDGE, which the target NG-RAN node sends. This is the choice that Wireshark
/// makes; for another HandoverType, or none, the octets stay.
fn containers(tree: &mut Tree) {
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
            let mut opened = tree.clone();
            if inspect::open(&mut opened, &format!("/ngap/{index}/{carried}")).is_err() {
                continue;
            }
            // Octets that do not decode as the type keep their place in the tree
            let value = &opened["ies"][index][container][carried];
            if value.get("octets").is_none() {
                *tree = opened;
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
            containers(&mut tree);
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
    let compare = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../oxirush-aper/protocols/tests/wireshark/compare.py");
    if !compare.exists() {
        eprintln!(
            "skipped: the checkout of oxirush-aper beside the repository, which compares the PDUs with Wireshark, is not there"
        );
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
        .arg(&compare)
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
