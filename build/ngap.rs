use std::fs;

use asn1_compiler::{
    Asn1Compiler,
    generator::{Codec, Derive, Visibility},
};

use anyhow::Result;

pub fn generate_ngap() -> Result<()> {
    let files: Vec<_> = fs::read_dir("ngap")
        .unwrap()
        .map(|f| f.unwrap().path())
        .collect();

    let mut compiler = Asn1Compiler::new(
        "src/ngap.rs",
        &Visibility::Public,
        vec![Codec::Aper],
        vec![
            Derive::Debug,
            Derive::PartialEq,
            Derive::Serialize,
            Derive::Deserialize,
        ],
    );
    compiler.compile_files(&files)?;

    Ok(())
}
