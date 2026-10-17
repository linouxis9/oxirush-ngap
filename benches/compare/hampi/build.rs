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

//! The bindings of hampi's asn1-compiler of the six modules of the specification, which
//! `build/extract_asn1.py` writes: `ASN1` is their directory, and `PROTOCOL` `ngap` or
//! `s1ap`.

use asn1_compiler::{
    generator::{Codec, Derive, Visibility},
    Asn1Compiler,
};

fn main() {
    let directory = std::env::var("ASN1").expect("ASN1: the directory of the modules");
    let protocol = std::env::var("PROTOCOL").expect("PROTOCOL: ngap or s1ap");
    println!("cargo::rerun-if-env-changed=ASN1");
    println!("cargo::rerun-if-env-changed=PROTOCOL");
    // `cfg(s1ap)` names the PDU type of S1AP in `main.rs`.
    println!("cargo::rustc-check-cfg=cfg(s1ap)");
    if protocol == "s1ap" {
        println!("cargo::rustc-cfg=s1ap");
    }
    let prefix = protocol.to_uppercase();
    let files: Vec<String> = [
        "CommonDataTypes",
        "Constants",
        "Containers",
        "IEs",
        "PDU-Contents",
        "PDU-Descriptions",
    ]
    .iter()
    .map(|module| format!("{directory}/{prefix}-{module}.asn"))
    .collect();
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs");
    let mut compiler = Asn1Compiler::new(
        out.to_str().unwrap(),
        &Visibility::Public,
        vec![Codec::Aper],
        vec![Derive::Debug, Derive::Clone],
    );
    compiler
        .compile_files(&files)
        .expect("hampi compiles the modules");
}
