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
//! written here as one line of JSON; `tests/wireshark/compare.py` has tshark
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

use oxirush_ngap::ngap::NGAP_PDU;
use serde_json::{Value, json};

fn found(program: &str, argument: &str) -> bool {
    Command::new(program)
        .arg(argument)
        .output()
        .is_ok_and(|output| output.status.success())
}

/// What the crate reads of `octets`, as one line of JSON. The tree is
/// written as the crate prints it, whose objects keep the order of the PDU.
fn reading(octets: &[u8]) -> String {
    let head = json!({"octets": hex::encode(octets)}).to_string();
    let head = head.trim_end_matches('}');
    match NGAP_PDU::decode(octets) {
        Ok(pdu) => {
            let check: Value = match pdu.check() {
                Ok(deviations) => deviations.iter().map(ToString::to_string).collect(),
                Err(error) => json!([format!("error: {error}")]),
            };
            format!("{head},\"check\":{check},\"tree\":{}}}", pdu.to_tree())
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
