use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use rasn_compiler::OutputMode;
use rasn_compiler::prelude::{Compiler, RasnBackend, RasnConfig};
use regex::{Captures, Regex};

pub fn generate_ngap() -> Result<()> {
    let mut files: Vec<PathBuf> = fs::read_dir("ngap")
        .context("read NGAP ASN.1 source directory")?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<std::io::Result<_>>()?;
    files.sort();

    let config = RasnConfig {
        // rasn-compiler's typed open types are experimental and currently emit
        // invalid bindings for TS 38.413. Stable opaque open types are wrapped
        // by the generated macros below, retaining a typed public API.
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
        println!("cargo:warning={warning}");
    }

    post_process(output, &files)
}

fn post_process(path: &Path, asn_files: &[PathBuf]) -> Result<()> {
    let mut generated = fs::read_to_string(path).context("read generated NGAP bindings")?;
    generated.insert_str(0, "#![allow(clippy::large_enum_variant)]\n\n");

    // Plain bracketed specification references are otherwise parsed as rustdoc links.
    generated = generated.replace("[16]", "(reference 16)");

    // rasn-compiler already resolves every parameterized container invocation to
    // a concrete anonymous type. These now-unused imports refer to parameterized
    // definitions that intentionally have no standalone Rust representation.
    let container_imports = Regex::new(r"(?ms)^    use super::ngap_containers::\{.*?^    \};\n")?;
    generated = container_imports.replace_all(&generated, "").into_owned();
    generated = crate::aper_fix::fix_constrained_sequences(&generated)?;
    generated = crate::aper_fix::fix_utf8_strings(&generated)?;
    generated = crate::aper_fix::fix_fixed_bit_strings(&generated)?;
    generated = crate::aper_fix::fix_long_inline_strings(&generated)?;
    generated = crate::aper_fix::fix_extensible_sequences(&generated)?;

    // The concrete private/extension containers below use these common types,
    // but rasn-compiler omits both from the generated module import list.
    generated = generated.replacen(
        "    use super::ngap_common_data_types::{Criticality, Presence, ProtocolIEID};",
        "    use super::ngap_common_data_types::{Criticality, Presence, PrivateIEID, ProtocolIEID};",
        1,
    );
    // The resolved extension containers of the IEs module need the common
    // types their IMPORTS name only through the parameterized containers.
    let ies_module = "pub mod ngap_ies {\n    extern crate alloc;\n";
    if !generated.contains("    use super::ngap_common_data_types::{Presence, ProtocolExtensionID};") {
        generated = generated.replacen(
            ies_module,
            &format!(
                "{ies_module}    use super::ngap_common_data_types::{{Presence, ProtocolExtensionID}};\n"
            ),
            1,
        );
    }

    // Some resolved ProtocolIE containers use primitive/anonymous field types
    // while equivalent containers use the named common types. Normalize only
    // message ProtocolIE entries; their APER representations are identical.
    let protocol_ie_block = Regex::new(
        r"(?ms)(    pub struct Anonymous[A-Za-z0-9_]+ProtocolIEs \{.*?^    \}\n    impl Anonymous[A-Za-z0-9_]+ProtocolIEs \{.*?^    \}\n)",
    )?;
    let anonymous_criticality = Regex::new(r"Anonymous[A-Za-z0-9_]+ProtocolIEsCriticality")?;
    generated = protocol_ie_block
        .replace_all(&generated, |captures: &Captures<'_>| {
            let block = captures[1]
                .replace("pub id: u16", "pub id: ProtocolIEID")
                .replace("id: u16,", "id: ProtocolIEID,");
            anonymous_criticality
                .replace_all(&block, "Criticality")
                .into_owned()
        })
        .into_owned();

    let support = generate_support(&generated, asn_files)?;
    generated.push_str(&support);
    fs::write(path, generated).context("write post-processed NGAP bindings")
}

fn generate_support(generated: &str, asn_files: &[PathBuf]) -> Result<String> {
    let mut asn = String::new();
    for file in asn_files {
        asn.push_str(&fs::read_to_string(file)?);
        asn.push('\n');
    }

    let generated_type = Regex::new(r"(?m)^    pub (?:struct|enum|type) ([A-Za-z][A-Za-z0-9_]*)")?;
    let mut rust_types = BTreeMap::new();
    for captures in generated_type.captures_iter(generated) {
        let name = captures[1].to_string();
        if !name.starts_with("Anonymous") {
            rust_types.entry(canonical(&name)).or_insert(name);
        }
    }

    let newtype = Regex::new(r"(?m)^    pub struct ([A-Za-z][A-Za-z0-9_]*)\(pub ([^;\n]+)\);$")?;
    let mut newtypes = BTreeMap::new();
    for captures in newtype.captures_iter(generated) {
        let name = captures[1].to_string();
        let inner = captures[2].to_string();
        if !inner.contains(", pub ") {
            newtypes.entry(name).or_insert(inner);
        }
    }

    let ie_constant =
        Regex::new(r"(?m)^\s*(id-[A-Za-z][A-Za-z0-9-]*)\s+ProtocolIE-ID\s+::=\s+(\d+)")?;
    let ie_constants: BTreeMap<String, u16> = ie_constant
        .captures_iter(&asn)
        .map(|captures| Ok((captures[1].to_string(), captures[2].parse()?)))
        .collect::<Result<_>>()?;

    let ie_object = Regex::new(
        r"(?s)\{\s*ID\s+(id-[A-Za-z][A-Za-z0-9-]*)\s+CRITICALITY\s+[A-Za-z-]+\s+TYPE\s+(OCTET\s+STRING(?:\s*\(CONTAINING\s+[A-Za-z][A-Za-z0-9-]*\s*\))?|[A-Za-z][A-Za-z0-9-]*)\s+PRESENCE",
    )?;
    let mut ies: BTreeMap<String, (u16, String)> = BTreeMap::new();
    let mut type_aliases: BTreeMap<String, BTreeSet<u16>> = BTreeMap::new();
    for captures in ie_object.captures_iter(&asn) {
        let id_name = &captures[1];
        let asn_type = &captures[2];
        let Some(id) = ie_constants.get(id_name).copied() else {
            continue;
        };
        // Paths below `$crate`. An inline OCTET STRING, such as the type of
        // id-NGAP-Message or the OCTET STRING (CONTAINING
        // MBSSessionSetupOrModRequestTransfer) of
        // id-MBSSessionSetupRequestTransfer, has no generated type.
        let (rust_type, path) = if asn_type.starts_with("OCTET") {
            (None, "__rasn::types::OctetString".to_string())
        } else {
            let Some(rust_type) = rust_types.get(&canonical(asn_type)).cloned() else {
                continue;
            };
            let path = format!("ngap::{rust_type}");
            (Some(rust_type), path)
        };

        // The IE-name spelling used by TS 38.413 names its own IE. The rasn
        // type name is a concise alias, added below, unless an IE has that
        // name: id-OldAMF is an AMFName, but `AMFName` names id-AMFName.
        ies.entry(macro_ident(id_name.trim_start_matches("id-")))
            .or_insert((id, path));
        if let Some(rust_type) = rust_type {
            type_aliases.entry(rust_type).or_default().insert(id);
        }
    }
    // A type name is an alias only for the one IE of that type: the type of
    // id-SONConfigurationTransferDL and -UL could otherwise address either.
    // Nor is it one when an IE outside the macros' reach has that name:
    // id-SelectedNID is the one NID, but `NID` would read as id-NID, an
    // extension IE.
    let ie_names: BTreeSet<String> = ie_constants
        .keys()
        .map(|id_name| macro_ident(id_name.trim_start_matches("id-")))
        .collect();
    for (rust_type, ids) in type_aliases {
        if let [id] = ids.into_iter().collect::<Vec<_>>()[..]
            && !ie_names.contains(&rust_type)
        {
            ies.entry(rust_type.clone())
                .or_insert((id, format!("ngap::{rust_type}")));
        }
    }

    let procedure_constant =
        Regex::new(r"(?m)^\s*(id-[A-Za-z][A-Za-z0-9-]*)\s+ProcedureCode\s+::=\s+(\d+)")?;
    let mut procedures = BTreeMap::new();
    let mut procedure_ids = BTreeMap::new();
    for captures in procedure_constant.captures_iter(&asn) {
        let id_name = captures[1].to_string();
        let code: u8 = captures[2].parse()?;
        let name = macro_ident(id_name.trim_start_matches("id-"));
        procedures.insert(name.clone(), code);
        procedure_ids.insert(id_name, name);
    }

    let procedure_block =
        Regex::new(r"(?ms)^[A-Za-z][A-Za-z0-9-]*\s+NGAP-ELEMENTARY-PROCEDURE\s+::=\s*\{(.*?)^\}")?;
    let procedure_ref = Regex::new(r"PROCEDURE CODE\s+(id-[A-Za-z][A-Za-z0-9-]*)")?;
    let mut directions: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
    for captures in procedure_block.captures_iter(&asn) {
        let body = &captures[1];
        let Some(id) = procedure_ref
            .captures(body)
            .map(|capture| capture[1].to_string())
        else {
            continue;
        };
        let Some(name) = procedure_ids.get(&id).cloned() else {
            continue;
        };
        for (label, direction) in [
            ("INITIATING MESSAGE", "Initiating"),
            ("SUCCESSFUL OUTCOME", "Successful"),
            ("UNSUCCESSFUL OUTCOME", "Unsuccessful"),
        ] {
            if body.contains(label) {
                directions
                    .entry(direction)
                    .or_default()
                    .insert(name.clone());
            }
        }
    }

    let mut out = String::new();
    writeln!(
        out,
        "\n// Auto-generated NGAP compatibility surface and ASN.1-derived lookups."
    )?;
    writeln!(out, "pub use ngap_common_data_types::*;")?;
    writeln!(out, "pub use ngap_constants::*;")?;
    writeln!(out, "pub use ngap_ies::*;")?;
    writeln!(out, "pub use ngap_pdu_contents::*;")?;
    writeln!(out, "pub use ngap_pdu_descriptions::*;")?;
    writeln!(out, "#[allow(non_camel_case_types)]")?;
    writeln!(out, "pub type NGAP_PDU = NGAPPDU;")?;
    if rust_types.values().any(|name| name == "AMFUENGAPID") {
        writeln!(out, "#[allow(non_camel_case_types)]")?;
        writeln!(out, "pub type AMF_UE_NGAP_ID = AMFUENGAPID;")?;
    }
    if rust_types.values().any(|name| name == "RANUENGAPID") {
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

    writeln!(out, "#[macro_export]")?;
    writeln!(out, "#[doc(hidden)]")?;
    writeln!(out, "macro_rules! __ngap_ie_id {{")?;
    for (name, (id, _)) in &ies {
        writeln!(out, "    ({name}) => {{ {id}u16 }};")?;
    }
    writeln!(out, "}}")?;

    writeln!(out, "#[macro_export]")?;
    writeln!(out, "#[doc(hidden)]")?;
    writeln!(out, "macro_rules! __ngap_encode_ie {{")?;
    for (name, (_, path)) in &ies {
        writeln!(
            out,
            "    ({name}, $value:expr) => {{ {{ let value: $crate::{path} = ($value).into(); $crate::ngap::encode_open_type(&value) }} }};"
        )?;
    }
    writeln!(out, "}}")?;

    writeln!(out, "#[macro_export]")?;
    writeln!(out, "#[doc(hidden)]")?;
    writeln!(out, "macro_rules! __ngap_decode_ie {{")?;
    for (name, (_, path)) in &ies {
        writeln!(
            out,
            "    ({name}, $value:expr) => {{ $crate::ngap::decode_open_type::<$crate::{path}>($value) }};"
        )?;
    }
    writeln!(out, "}}")?;

    writeln!(out, "#[macro_export]")?;
    writeln!(out, "#[doc(hidden)]")?;
    writeln!(out, "macro_rules! __ngap_proc_code {{")?;
    for (name, code) in &procedures {
        writeln!(out, "    ({name}) => {{ {code}u8 }};")?;
    }
    writeln!(out, "}}")?;

    writeln!(out, "#[allow(non_camel_case_types)]")?;
    writeln!(out, "#[derive(Clone, Debug, PartialEq, Eq)]")?;
    writeln!(out, "#[non_exhaustive]")?;
    writeln!(out, "pub enum NgapPduKind {{")?;
    for direction in ["Initiating", "Successful", "Unsuccessful"] {
        if let Some(names) = directions.get(direction) {
            for name in names {
                writeln!(out, "    {direction}_{name},")?;
            }
        }
    }
    writeln!(
        out,
        "    Other {{ direction: &'static str, procedure_code: u8 }},"
    )?;
    writeln!(out, "}}")?;

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
    writeln!(out, "    /// Return the ASN.1 procedure name.")?;
    writeln!(out, "    pub fn procedure_name(&self) -> &'static str {{")?;
    writeln!(out, "        match self.procedure_code() {{")?;
    for (name, code) in &procedures {
        writeln!(out, "            {code} => \"{name}\",")?;
    }
    writeln!(out, "            _ => \"Unknown\",")?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(
        out,
        "    /// Return the canonical direction/procedure kind."
    )?;
    writeln!(out, "    pub fn kind(&self) -> NgapPduKind {{")?;
    writeln!(out, "        let code = self.procedure_code();")?;
    writeln!(out, "        match self {{")?;
    for (variant, direction) in [
        ("initiatingMessage", "Initiating"),
        ("successfulOutcome", "Successful"),
        ("unsuccessfulOutcome", "Unsuccessful"),
    ] {
        writeln!(out, "            NGAPPDU::{variant}(_) => match code {{")?;
        if let Some(names) = directions.get(direction) {
            for name in names {
                if let Some(code) = procedures.get(name) {
                    writeln!(
                        out,
                        "                {code} => NgapPduKind::{direction}_{name},"
                    )?;
                }
            }
        }
        writeln!(
            out,
            "                _ => NgapPduKind::Other {{ direction: \"{direction}\", procedure_code: code }},"
        )?;
        writeln!(out, "            }},")?;
    }
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    writeln!(out, "impl NgapPduKind {{")?;
    writeln!(out, "    /// Return this kind's NGAP procedure code.")?;
    writeln!(out, "    pub fn procedure_code(&self) -> u8 {{")?;
    writeln!(out, "        match self {{")?;
    for direction in ["Initiating", "Successful", "Unsuccessful"] {
        if let Some(names) = directions.get(direction) {
            for name in names {
                if let Some(code) = procedures.get(name) {
                    writeln!(
                        out,
                        "            NgapPduKind::{direction}_{name} => {code},"
                    )?;
                }
            }
        }
    }
    writeln!(
        out,
        "            NgapPduKind::Other {{ procedure_code, .. }} => *procedure_code,"
    )?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
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

fn canonical(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn macro_ident(value: &str) -> String {
    value.replace('-', "_")
}
