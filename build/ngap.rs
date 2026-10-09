use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use rasn_compiler::OutputMode;
use rasn_compiler::prelude::{Compiler, RasnBackend, RasnConfig};
use regex::Regex;

pub fn generate_ngap() -> Result<()> {
    let mut files: Vec<PathBuf> = fs::read_dir("ngap")
        .context("read NGAP ASN.1 source directory")?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<std::io::Result<_>>()?;
    files.sort();

    let config = RasnConfig {
        // rasn-compiler's typed open types are experimental and currently emit
        // invalid bindings for TS 38.413. Stable opaque open types are wrapped
        // by the macros over the generated registry, retaining a typed public API.
        opaque_open_types: true,
        generate_from_impls: true,
        ..RasnConfig::default()
    };
    let output = Path::new("src/ngap.rs");

    let warnings = Compiler::<RasnBackend, _>::new_with_config(config)
        .add_asn_sources_by_path(files.iter())
        .set_output_mode(OutputMode::SingleFile(output.into()))
        .compile()
        .map_err(|error| anyhow!("compile NGAP ASN.1 definitions: {error}"))?;

    for warning in warnings {
        eprintln!("warning: {warning}");
    }

    post_process(output, &files)
}

fn post_process(path: &Path, asn_files: &[PathBuf]) -> Result<()> {
    let mut generated = fs::read_to_string(path).context("read generated NGAP bindings")?;
    generated.insert_str(0, "#![allow(clippy::large_enum_variant)]\n\n");

    // Plain bracketed specification references are otherwise parsed as rustdoc links.
    generated = generated.replace("[16]", "(reference 16)");

    generated = crate::containers::share(&generated, "ngap_containers")?;
    generated = crate::aper_fix::fix_constrained_sequences(&generated)?;
    generated = crate::aper_fix::fix_utf8_strings(&generated)?;
    generated = crate::aper_fix::fix_fixed_bit_strings(&generated)?;
    generated = crate::aper_fix::fix_long_inline_strings(&generated)?;
    generated = crate::aper_fix::fix_extensible_sequences(&generated)?;

    // The private IE container uses PrivateIE-ID, which rasn-compiler omits
    // from the import list of its module.
    generated = generated.replacen(
        "    use super::ngap_common_data_types::{Criticality, Presence, ProtocolIEID};",
        "    use super::ngap_common_data_types::{Criticality, Presence, PrivateIEID, ProtocolIEID};",
        1,
    );

    generated.push_str(&generate_support(&generated)?);
    let asn = asn_files
        .iter()
        .map(fs::read_to_string)
        .collect::<std::io::Result<Vec<_>>>()?
        .join("\n");
    // The procedures and the IEs, once for the macros and for the inspection.
    let registry = crate::registry::Registry::read("NGAP", &generated, &asn)?;
    fs::write("src/registry.rs", registry.list()?)?;
    fs::write(
        "src/inspect_registry.rs",
        crate::inspection::generate("NGAP", &generated, &asn, &registry)?,
    )?;
    fs::write(path, generated).context("write post-processed NGAP bindings")
}

fn generate_support(generated: &str) -> Result<String> {
    let generated_type = Regex::new(r"(?m)^    pub (?:struct|enum|type) ([A-Za-z][A-Za-z0-9_]*)")?;
    let rust_types: BTreeSet<&str> = generated_type
        .captures_iter(generated)
        .map(|captures| captures.get(1).expect("a name").as_str())
        .collect();

    let newtype = Regex::new(r"(?m)^    pub struct ([A-Za-z][A-Za-z0-9_]*)\(pub ([^;\n]+)\);$")?;
    let mut newtypes = BTreeMap::new();
    for captures in newtype.captures_iter(generated) {
        let name = captures[1].to_string();
        let inner = captures[2].to_string();
        if !inner.contains(", pub ") {
            newtypes.entry(name).or_insert(inner);
        }
    }

    let mut out = String::new();
    writeln!(
        out,
        "\n// Auto-generated NGAP compatibility surface."
    )?;
    writeln!(out, "pub use ngap_common_data_types::*;")?;
    writeln!(out, "pub use ngap_constants::*;")?;
    writeln!(out, "pub use ngap_containers::*;")?;
    writeln!(out, "pub use ngap_ies::*;")?;
    writeln!(out, "pub use ngap_pdu_contents::*;")?;
    writeln!(out, "pub use ngap_pdu_descriptions::*;")?;
    writeln!(out, "#[allow(non_camel_case_types)]")?;
    writeln!(out, "pub type NGAP_PDU = NGAPPDU;")?;
    if rust_types.contains("AMFUENGAPID") {
        writeln!(out, "#[allow(non_camel_case_types)]")?;
        writeln!(out, "pub type AMF_UE_NGAP_ID = AMFUENGAPID;")?;
    }
    if rust_types.contains("RANUENGAPID") {
        writeln!(out, "#[allow(non_camel_case_types)]")?;
        writeln!(out, "pub type RAN_UE_NGAP_ID = RANUENGAPID;")?;
    }

    writeln!(
        out,
        "use rasn::prelude::{{BitString, FixedBitString, FixedOctetString, Integer, OctetString, PrintableString, SequenceOf, Utf8String, VisibleString}};"
    )?;
    writeln!(
        out,
        "\n// Auto-generated newtype conversions used by the builder macros."
    )?;
    for (name, inner) in &newtypes {
        writeln!(
            out,
            "impl From<{inner}> for {name} {{ fn from(value: {inner}) -> Self {{ Self(value) }} }}"
        )?;
        if inner == "OctetString" {
            writeln!(
                out,
                "impl From<Vec<u8>> for {name} {{ fn from(value: Vec<u8>) -> Self {{ Self(value.into()) }} }}"
            )?;
        } else if let Some(size) = inner
            .strip_prefix("FixedOctetString<")
            .and_then(|value| value.strip_suffix('>'))
        {
            writeln!(
                out,
                "impl From<[u8; {size}]> for {name} {{ fn from(value: [u8; {size}]) -> Self {{ Self(value.into()) }} }}"
            )?;
        }
    }

    // The kinds, the names of the procedures and their codes are those of the registry.
    writeln!(out, "pub use crate::registry::NgapPduKind;")?;

    writeln!(out, "impl NGAPPDU {{")?;
    writeln!(out, "    /// Encode this PDU using Aligned PER.")?;
    writeln!(
        out,
        "    pub fn encode(&self) -> Result<Vec<u8>, rasn::error::EncodeError> {{"
    )?;
    writeln!(out, "        rasn::aper::encode(self)")?;
    writeln!(out, "    }}")?;
    writeln!(out, "    /// Decode an NGAP PDU from Aligned PER bytes.")?;
    writeln!(
        out,
        "    pub fn decode(bytes: &[u8]) -> Result<Self, rasn::error::DecodeError> {{"
    )?;
    writeln!(out, "        decode_complete(bytes)")?;
    writeln!(out, "    }}")?;
    writeln!(
        out,
        "    /// Decode the typed message held by this PDU's open type."
    )?;
    writeln!(
        out,
        "    pub fn decode_value<T: rasn::Decode>(&self) -> Result<T, rasn::error::DecodeError> {{"
    )?;
    writeln!(out, "        let value = match self {{")?;
    writeln!(
        out,
        "            NGAPPDU::initiatingMessage(message) => &message.value,"
    )?;
    writeln!(
        out,
        "            NGAPPDU::successfulOutcome(message) => &message.value,"
    )?;
    writeln!(
        out,
        "            NGAPPDU::unsuccessfulOutcome(message) => &message.value,"
    )?;
    writeln!(out, "        }};")?;
    writeln!(out, "        decode_open_type(value)")?;
    writeln!(out, "    }}")?;
    writeln!(out, "    /// Return the procedure code of this PDU.")?;
    writeln!(out, "    pub fn procedure_code(&self) -> u8 {{")?;
    writeln!(out, "        match self {{")?;
    writeln!(
        out,
        "            NGAPPDU::initiatingMessage(message) => message.procedure_code.0,"
    )?;
    writeln!(
        out,
        "            NGAPPDU::successfulOutcome(message) => message.procedure_code.0,"
    )?;
    writeln!(
        out,
        "            NGAPPDU::unsuccessfulOutcome(message) => message.procedure_code.0,"
    )?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(
        out,
        "    /// Return the PDU direction as a human-readable string."
    )?;
    writeln!(out, "    pub fn direction(&self) -> &'static str {{")?;
    writeln!(out, "        match self {{")?;
    writeln!(
        out,
        "            NGAPPDU::initiatingMessage(_) => \"InitiatingMessage\","
    )?;
    writeln!(
        out,
        "            NGAPPDU::successfulOutcome(_) => \"SuccessfulOutcome\","
    )?;
    writeln!(
        out,
        "            NGAPPDU::unsuccessfulOutcome(_) => \"UnsuccessfulOutcome\","
    )?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "    /// Returns `true` for an initiating message.")?;
    writeln!(
        out,
        "    pub fn is_initiating(&self) -> bool {{ matches!(self, NGAPPDU::initiatingMessage(_)) }}"
    )?;
    writeln!(out, "    /// Returns `true` for a successful outcome.")?;
    writeln!(
        out,
        "    pub fn is_successful(&self) -> bool {{ matches!(self, NGAPPDU::successfulOutcome(_)) }}"
    )?;
    writeln!(out, "    /// Returns `true` for an unsuccessful outcome.")?;
    writeln!(
        out,
        "    pub fn is_unsuccessful(&self) -> bool {{ matches!(self, NGAPPDU::unsuccessfulOutcome(_)) }}"
    )?;
    writeln!(out, "}}")?;

    writeln!(out, "impl std::fmt::Display for NGAPPDU {{")?;
    writeln!(
        out,
        "    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"
    )?;
    writeln!(
        out,
        "        write!(formatter, \"{{}} {{}} (code={{}})\", self.direction(), self.procedure_name(), self.procedure_code())"
    )?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    writeln!(
        out,
        "/// Encode a typed ASN.1 value for an NGAP open type using APER."
    )?;
    writeln!(out, "///")?;
    writeln!(
        out,
        "/// An open type holds a complete encoding, in which an empty encoding"
    )?;
    writeln!(
        out,
        "/// becomes one zero octet (X.691 (07/2002) §10.1.4, §10.2.1)."
    )?;
    writeln!(
        out,
        "pub fn encode_open_type<T: rasn::Encode>(value: &T) -> Result<rasn::types::Any, rasn::error::EncodeError> {{"
    )?;
    writeln!(out, "    let mut bytes = rasn::aper::encode(value)?;")?;
    writeln!(out, "    if bytes.is_empty() {{")?;
    writeln!(out, "        bytes.push(0);")?;
    writeln!(out, "    }}")?;
    writeln!(out, "    Ok(rasn::types::Any::new(bytes))")?;
    writeln!(out, "}}")?;
    writeln!(
        out,
        "/// Decode a typed ASN.1 value from an NGAP open type using APER."
    )?;
    writeln!(
        out,
        "pub fn decode_open_type<T: rasn::Decode>(value: &rasn::types::Any) -> Result<T, rasn::error::DecodeError> {{"
    )?;
    writeln!(out, "    decode_complete(value.as_bytes())")?;
    writeln!(out, "}}")?;
    writeln!(out, "fn decode_complete<T: rasn::Decode>(bytes: &[u8]) -> Result<T, rasn::error::DecodeError> {{")?;
    writeln!(out, "    if bytes.is_empty() {{")?;
    writeln!(out, "        return Err(<rasn::error::DecodeError as rasn::de::Error>::custom(")?;
    writeln!(out, "            \"APER value must contain a complete encoding\", rasn::Codec::Aper,")?;
    writeln!(out, "        ));")?;
    writeln!(out, "    }}")?;
    writeln!(out, "    let (decoded, remainder) = rasn::aper::decode_with_remainder(bytes)?;")?;
    writeln!(out, "    // A zero-bit field-list has exactly one zero octet as its complete encoding.")?;
    writeln!(out, "    let zero_bit_encoding = bytes == [0] && remainder == bytes;")?;
    writeln!(out, "    if !remainder.is_empty() && !zero_bit_encoding {{")?;
    writeln!(out, "        return Err(<rasn::error::DecodeError as rasn::de::Error>::custom(")?;
    writeln!(out, "            \"APER complete encoding has trailing whole octets\", rasn::Codec::Aper,")?;
    writeln!(out, "        ));")?;
    writeln!(out, "    }}")?;
    writeln!(out, "    Ok(decoded)")?;
    writeln!(out, "}}")?;

    Ok(out)
}
