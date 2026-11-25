/*
   OxiRush
   Copyright 2026 Valentin D'Emmanuele

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

//! The PDUs of a corpus as the NGAP library of free5GC reads them and as this crate reads
//! them, compared IE by IE.
//!
//! The crate's reading of each PDU is its type, its procedure code, its IEs in the order
//! of the PDU with the octets of each value, and what `check()` finds (of a PDU whose
//! message has no container of protocol IEs, its type and procedure code); the program of
//! `tests/free5gc/` writes what free5GC's library reads of the same octets, and
//! `tests/free5gc/compare.py` compares the two: whether each reads the PDU, its message,
//! and the value of each IE, by its id. A difference that `tests/free5gc/known.tsv` does not
//! explain fails the test. The library is that of free5GC v4.3.0's AMF, at the version of
//! `tests/free5gc/go.mod`, which `go` fetches.
//!
//! The test needs `go` and `python3`, and runs with `--ignored`:
//! `NGAP_FREE5GC_CORPUS=<rows> NGAP_FREE5GC_OUTPUT=<directory> cargo test --test free5gc
//! -- --ignored`, a PDU a row with its octets in the last column; without
//! `NGAP_FREE5GC_CORPUS`, the PDUs are those of `tests/wireshark/corpus.tsv`.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use oxirush_ngap::ngap::NGAPPDU;
use serde_json::{Value, json};

/// What the crate reads of `octets`, as one line of JSON.
fn reading(octets: &[u8]) -> Value {
    let mut entry = json!({"octets": hex::encode(octets)});
    let pdu = match NGAPPDU::decode(octets) {
        Ok(pdu) => pdu,
        Err(error) => {
            entry["error"] = json!(error.to_string());
            return entry;
        }
    };
    entry["pdu"] = json!(match pdu {
        NGAPPDU::initiatingMessage(_) => 0,
        NGAPPDU::successfulOutcome(_) => 1,
        NGAPPDU::unsuccessfulOutcome(_) => 2,
        // An extension of the CHOICE
        _ => -1,
    });
    entry["procedure"] = json!(pdu.procedure_code());
    entry["message"] = json!(format!("{:?}", pdu.kind()));
    match pdu.protocol_ies() {
        Ok(ies) => {
            let ies: Vec<Value> = ies
                .0
                .iter()
                .map(|ie| json!({"id": ie.id.0, "value": hex::encode(&*ie.value)}))
                .collect();
            entry["ies"] = json!(ies);
        }
        // A PRIVATE MESSAGE has a container of private IEs, which the
        // comparison does not read
        Err(error) => {
            entry["container"] = json!(error.to_string());
            return entry;
        }
    }
    match pdu.check() {
        Ok(deviations) => {
            let found: Vec<Value> = deviations
                .iter()
                .map(|deviation| json!({"kind": format!("{deviation:?}").split([' ', '{', '(']).next().unwrap_or(""), "text": deviation.to_string()}))
                .collect();
            entry["check"] = json!(found);
        }
        Err(error) => entry["error"] = json!(error.to_string()),
    }
    entry
}

/// The output of `command`, which must succeed.
fn output(command: &mut Command) -> Vec<u8> {
    let output = command
        .stdin(Stdio::null())
        .output()
        .unwrap_or_else(|error| panic!("{command:?}: {error}"));
    assert!(
        output.status.success(),
        "{command:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

#[test]
#[ignore = "needs go, which fetches free5GC's NGAP library, and python3"]
fn free5gc_reads_the_pdus_as_the_crate_does() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/free5gc");
    let corpus = std::env::var_os("NGAP_FREE5GC_CORPUS")
        .map_or_else(|| here.join("../wireshark/corpus.tsv"), PathBuf::from);
    let out = std::env::var_os("NGAP_FREE5GC_OUTPUT").map_or_else(
        || Path::new(env!("CARGO_TARGET_TMPDIR")).join("free5gc"),
        PathBuf::from,
    );
    std::fs::create_dir_all(&out).expect("the output directory");

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
    let readings = out.join("crate.jsonl");
    std::fs::write(&readings, lines).expect("the crate's readings");

    // The library at the version of go.mod, the ids of its fields, and what it reads
    let module: Value = serde_json::from_slice(&output(
        Command::new("go")
            .args(["mod", "download", "-json", "github.com/free5gc/ngap"])
            .current_dir(&here),
    ))
    .expect("the module as JSON");
    let ids = out.join("ids.tsv");
    output(
        Command::new("python3")
            .arg("-I")
            .arg(here.join("ids.py"))
            .arg(module["Dir"].as_str().expect("the directory of the module"))
            .arg(&ids),
    );
    let free5gc = output(
        Command::new("go")
            .args(["run", "."])
            .arg(&ids)
            .arg(&corpus)
            .current_dir(&here),
    );
    let theirs = out.join("free5gc.jsonl");
    std::fs::write(&theirs, free5gc).expect("free5GC's readings");

    let status = Command::new("python3")
        .arg("-I")
        .arg(here.join("compare.py"))
        .arg("--crate")
        .arg(&readings)
        .arg("--free5gc")
        .arg(&theirs)
        .arg("--known")
        .arg(here.join("known.tsv"))
        .arg("--output")
        .arg(&out)
        .stdin(Stdio::null())
        .status()
        .expect("python3 runs");
    assert!(
        status.success(),
        "free5GC and the crate differ: {}",
        out.display()
    );
}
