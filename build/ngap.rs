use std::collections::HashMap;
use std::fmt::Write;
use std::fs;

use asn1_compiler::{
    Asn1Compiler,
    generator::{Codec, Derive, Visibility},
};

use anyhow::Result;
use regex::Regex;

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

    let content = fs::read_to_string("src/ngap.rs")?;
    let mut extra = String::new();

    // ── 1. Generate From<InnerType> impls for newtype structs ────────────────
    // Enables the build_ngap! macro to use `.into()` for automatic conversion:
    // raw values (e.g. u64) convert to newtypes, already-correct types use identity From.
    let re_newtype = Regex::new(r"(?m)^pub struct (\w+)\(pub (.+)\);$")?;
    writeln!(
        extra,
        "\n// Auto-generated From impls for newtype structs (used by build_ngap! macro)"
    )?;
    for cap in re_newtype.captures_iter(&content) {
        let name = &cap[1];
        let inner = &cap[2];
        // Skip multi-field tuple structs (contain commas outside of generics)
        if inner.contains(',') && !inner.contains('<') {
            continue;
        }
        writeln!(
            extra,
            "impl From<{inner}> for {name} {{ fn from(v: {inner}) -> Self {{ {name}(v) }} }}"
        )?;
    }

    // ── 2. Generate __ngap_ie_id! macro from #[asn(key = N)] attributes ─────
    // Parses ProtocolIEs_EntryValue enums to extract the IE ID for each variant.
    // This lets the build_ngap! macro auto-derive the IE ID from the variant suffix,
    // avoiding manual IE_ID constants and fragile paste! case conversion.
    let re_asn_key = Regex::new(r"#\[asn\(key = (\d+)")?;
    let re_variant = Regex::new(r"^\s+Id_(\w+)\(")?;
    let re_entry_value_enum = Regex::new(r"pub enum \w+ProtocolIEs_EntryValue")?;

    let mut ie_id_map: HashMap<String, u16> = HashMap::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut in_entry_value_enum = false;
    let mut pending_key: Option<u16> = None;

    for line in &lines {
        if re_entry_value_enum.is_match(line) {
            in_entry_value_enum = true;
            continue;
        }
        if in_entry_value_enum {
            // Detect end of enum
            if line.starts_with('}') {
                in_entry_value_enum = false;
                pending_key = None;
                continue;
            }
            if let Some(cap) = re_asn_key.captures(line) {
                pending_key = Some(cap[1].parse::<u16>()?);
            }
            if let Some(cap) = re_variant.captures(line) {
                if let Some(key) = pending_key {
                    let variant_suffix = cap[1].to_string();
                    // Deduplicate: same variant always has the same key
                    ie_id_map.entry(variant_suffix).or_insert(key);
                    pending_key = None;
                }
            }
        }
    }

    // Sort by variant name for stable output
    let mut entries: Vec<_> = ie_id_map.into_iter().collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    writeln!(
        extra,
        "\n// Auto-generated IE ID lookup macro (derived from #[asn(key = N)] attributes)"
    )?;
    writeln!(extra, "#[macro_export]")?;
    writeln!(extra, "#[doc(hidden)]")?;
    writeln!(extra, "macro_rules! __ngap_ie_id {{")?;
    for (variant_suffix, key) in &entries {
        writeln!(extra, "    ({variant_suffix}) => {{ {key}u16 }};")?;
    }
    writeln!(extra, "}}")?;

    // ── 3. Generate __ngap_proc_code! macro from {Direction}Value enums ──
    // Parses InitiatingMessageValue, SuccessfulOutcomeValue, UnsuccessfulOutcomeValue
    // to map procedure suffixes (e.g. InitialContextSetup) to procedure codes (u8).
    let re_direction_enum =
        Regex::new(r"pub enum (InitiatingMessage|SuccessfulOutcome|UnsuccessfulOutcome)Value")?;

    let mut proc_code_map: HashMap<String, u8> = HashMap::new();
    // Track which variants belong to each direction (for procedure_name/Display generation)
    let mut direction_variants: HashMap<String, Vec<String>> = HashMap::new();
    let mut in_direction_enum = false;
    let mut current_direction = String::new();
    pending_key = None;

    for line in &lines {
        if let Some(cap) = re_direction_enum.captures(line) {
            in_direction_enum = true;
            current_direction = cap[1].to_string();
            direction_variants
                .entry(current_direction.clone())
                .or_default();
            continue;
        }
        if in_direction_enum {
            if line.starts_with('}') {
                in_direction_enum = false;
                pending_key = None;
                continue;
            }
            if let Some(cap) = re_asn_key.captures(line) {
                pending_key = Some(cap[1].parse::<u16>()?);
            }
            if let Some(cap) = re_variant.captures(line) {
                if let Some(key) = pending_key {
                    let variant_suffix = cap[1].to_string();
                    proc_code_map
                        .entry(variant_suffix.clone())
                        .or_insert(key as u8);
                    direction_variants
                        .entry(current_direction.clone())
                        .or_default()
                        .push(variant_suffix);
                    pending_key = None;
                }
            }
        }
    }

    let mut proc_entries: Vec<_> = proc_code_map.into_iter().collect();
    proc_entries.sort_by(|a, b| a.0.cmp(&b.0));

    writeln!(
        extra,
        "\n// Auto-generated procedure code lookup macro (derived from {{Direction}}Value enums)"
    )?;
    writeln!(extra, "#[macro_export]")?;
    writeln!(extra, "#[doc(hidden)]")?;
    writeln!(extra, "macro_rules! __ngap_proc_code {{")?;
    for (variant_suffix, code) in &proc_entries {
        writeln!(extra, "    ({variant_suffix}) => {{ {code}u8 }};")?;
    }
    writeln!(extra, "}}")?;

    // ── 4. Generate impl NGAP_PDU: procedure_code, direction, procedure_name ──
    // Generates runtime inspection methods by matching on the inner Value enum variants.
    writeln!(
        extra,
        "\n// Auto-generated NGAP_PDU inspection methods and Display impl"
    )?;
    writeln!(extra, "impl NGAP_PDU {{")?;

    // procedure_code()
    writeln!(extra, "    /// Return the procedure code of this PDU.")?;
    writeln!(extra, "    pub fn procedure_code(&self) -> u8 {{")?;
    writeln!(extra, "        match self {{")?;
    writeln!(
        extra,
        "            NGAP_PDU::InitiatingMessage(m) => m.procedure_code.0,"
    )?;
    writeln!(
        extra,
        "            NGAP_PDU::SuccessfulOutcome(m) => m.procedure_code.0,"
    )?;
    writeln!(
        extra,
        "            NGAP_PDU::UnsuccessfulOutcome(m) => m.procedure_code.0,"
    )?;
    writeln!(extra, "        }}")?;
    writeln!(extra, "    }}")?;

    // direction()
    writeln!(
        extra,
        "    /// Return the PDU direction as a human-readable string."
    )?;
    writeln!(extra, "    pub fn direction(&self) -> &'static str {{")?;
    writeln!(extra, "        match self {{")?;
    writeln!(
        extra,
        "            NGAP_PDU::InitiatingMessage(_) => \"InitiatingMessage\","
    )?;
    writeln!(
        extra,
        "            NGAP_PDU::SuccessfulOutcome(_) => \"SuccessfulOutcome\","
    )?;
    writeln!(
        extra,
        "            NGAP_PDU::UnsuccessfulOutcome(_) => \"UnsuccessfulOutcome\","
    )?;
    writeln!(extra, "        }}")?;
    writeln!(extra, "    }}")?;

    // is_initiating / is_successful / is_unsuccessful
    writeln!(
        extra,
        "    /// Returns `true` if this is an initiating message."
    )?;
    writeln!(
        extra,
        "    pub fn is_initiating(&self) -> bool {{ matches!(self, NGAP_PDU::InitiatingMessage(_)) }}"
    )?;
    writeln!(
        extra,
        "    /// Returns `true` if this is a successful outcome."
    )?;
    writeln!(
        extra,
        "    pub fn is_successful(&self) -> bool {{ matches!(self, NGAP_PDU::SuccessfulOutcome(_)) }}"
    )?;
    writeln!(
        extra,
        "    /// Returns `true` if this is an unsuccessful outcome."
    )?;
    writeln!(
        extra,
        "    pub fn is_unsuccessful(&self) -> bool {{ matches!(self, NGAP_PDU::UnsuccessfulOutcome(_)) }}"
    )?;

    // procedure_name() — requires matching on all Value enum variants
    writeln!(
        extra,
        "    /// Return the procedure name as a human-readable string."
    )?;
    writeln!(extra, "    pub fn procedure_name(&self) -> &'static str {{")?;
    writeln!(extra, "        match self {{")?;

    for direction in &[
        "InitiatingMessage",
        "SuccessfulOutcome",
        "UnsuccessfulOutcome",
    ] {
        if let Some(variants) = direction_variants.get(*direction) {
            let mut sorted = variants.clone();
            sorted.sort();
            writeln!(
                extra,
                "            NGAP_PDU::{direction}(m) => match &m.value {{"
            )?;
            for v in &sorted {
                writeln!(
                    extra,
                    "                {direction}Value::Id_{v}(_) => \"{v}\","
                )?;
            }
            writeln!(extra, "            }},")?;
        }
    }

    writeln!(extra, "        }}")?;
    writeln!(extra, "    }}")?;
    writeln!(extra, "}}")?;

    // ── 5. Generate Display for NGAP_PDU ────────────────────────────────────
    // Format: "InitiatingMessage NGSetup (code=21)"
    writeln!(extra, "impl std::fmt::Display for NGAP_PDU {{")?;
    writeln!(
        extra,
        "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"
    )?;
    writeln!(
        extra,
        "        write!(f, \"{{}} {{}} (code={{}})\", self.direction(), self.procedure_name(), self.procedure_code())"
    )?;
    writeln!(extra, "    }}")?;
    writeln!(extra, "}}")?;

    // Write all extra code to ngap.rs
    let mut file = fs::OpenOptions::new().append(true).open("src/ngap.rs")?;
    use std::io::Write as IoWrite;
    file.write_all(extra.as_bytes())?;

    Ok(())
}
