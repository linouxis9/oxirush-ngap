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

//! The value of an IE by its identifier, as the crate types it and as the rasn bindings of
//! `oracle/` type it: `compare`, which times the codecs on whole PDUs, decodes each IE of a
//! PDU with them. They are written from the list of the IEs of `src/registry.rs`, each
//! with its type, which is the same name in both. A transfer, which is octets that hold a
//! value of a type, is its octets. This file is the same in oxirush-ngap and oxirush-s1ap.

use std::fmt::Write;

fn main() {
    let registry = std::fs::read_to_string("../src/registry.rs").expect("src/registry.rs");
    println!("cargo::rerun-if-changed=../src/registry.rs");
    let ies = registry
        .split("ies! {")
        .nth(1)
        .expect("the list of the IEs");
    let ies = ies.split("\n}").next().unwrap();
    let (mut ours, mut theirs) = (String::new(), String::new());
    let (mut ours_read, mut theirs_read) = (String::new(), String::new());
    for line in ies.lines().map(str::trim).filter(|line| !line.is_empty()) {
        // `ID [CRITICALITY] "NAME" [@key] TYPE[, CONTAINED] [=> NAMES];`, or `ID "NAME"
        // where [...]` for an IE whose type is that of where it is, which has none here.
        let mut words = line.trim_end_matches(';').split_whitespace().peekable();
        let id: u16 = words.next().unwrap().parse().unwrap();
        let words = words.skip_while(|word| !word.starts_with('"')).skip(1);
        let mut words = words.skip_while(|word| *word == "@key");
        let ty = words.next().unwrap();
        if ty == "where" {
            continue;
        }
        let ty = ty.trim_end_matches([',', ';']);
        let (ours_type, theirs_type) = match ty {
            "oxirush_aper::Octets<'a>" => (
                "codec::aper::Octets<'_>".to_string(),
                "oracle::rasn::types::OctetString".to_string(),
            ),
            ty => {
                let path = ty.trim_end_matches("<'a>");
                let name = path.rsplit("::").next().unwrap();
                let module = path.split("::").nth(1).unwrap();
                let life = if ty.ends_with("<'a>") { "<'_>" } else { "" };
                (
                    format!("codec::{module}::{name}{life}"),
                    format!("oracle::bindings::{name}"),
                )
            }
        };
        writeln!(ours, "        {id} => ours_typed::<{ours_type}>(octets),").unwrap();
        writeln!(
            theirs,
            "        {id} => theirs_typed::<{theirs_type}>(octets),"
        )
        .unwrap();
        writeln!(
            ours_read,
            "        {id} => ours_read::<{ours_type}>(octets),"
        )
        .unwrap();
        writeln!(
            theirs_read,
            "        {id} => theirs_read::<{theirs_type}>(octets),"
        )
        .unwrap();
    }
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let file = format!(
        "/// The value of the IE `id` as the crate types it: `None` for an identifier that \
         the registry does not have.\n\
         pub fn ours(id: u16, octets: &[u8]) -> Option<Result<Box<dyn Ours + '_>, codec::Error>> {{\n\
         \x20   Some(match id {{\n{ours}        _ => return None,\n    }})\n}}\n\n\
         /// The value of the IE `id` as the rasn bindings of `oracle/` type it.\n\
         pub fn theirs(id: u16, octets: &[u8]) -> Option<Result<Box<dyn Theirs>, String>> {{\n\
         \x20   Some(match id {{\n{theirs}        _ => return None,\n    }})\n}}\n\n\
         /// Read the value of the IE `id` as the crate types it, and drop it.\n\
         pub fn ours_dropped(id: u16, octets: &[u8]) -> Option<Result<(), codec::Error>> {{\n\
         \x20   Some(match id {{\n{ours_read}        _ => return None,\n    }})\n}}\n\n\
         /// Read the value of the IE `id` as the rasn bindings type it, and drop it.\n\
         pub fn theirs_dropped(id: u16, octets: &[u8]) -> Option<Result<(), String>> {{\n\
         \x20   Some(match id {{\n{theirs_read}        _ => return None,\n    }})\n}}\n"
    );
    std::fs::write(out.join("ies.rs"), file).unwrap();
}
