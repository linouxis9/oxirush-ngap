use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use rasn_compiler::OutputMode;
use rasn_compiler::prelude::{Compiler, RasnBackend, RasnConfig};
use regex::Regex;

/// The IEs whose octets ASN.1 leaves open, each with the type of this module that TS
/// 38.413 gives them for a handover between NG-RAN nodes: clauses 9.3.1.20, 9.3.1.21
/// and 9.3.1.186. For another target system the octets are those of its own
/// specification.
const CARRIED: &[(&str, &str)] = &[
    (
        "SourceToTarget-TransparentContainer",
        "SourceNGRANNode-ToTargetNGRANNode-TransparentContainer",
    ),
    (
        "TargetToSource-TransparentContainer",
        "TargetNGRANNode-ToSourceNGRANNode-TransparentContainer",
    ),
    (
        "TargettoSource-Failure-TransparentContainer",
        "TargetNGRANNode-ToSourceNGRANNode-FailureTransparentContainer",
    ),
];

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
        crate::inspection::generate("NGAP", &generated, &asn, &registry, CARRIED)?,
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
    writeln!(out, "    /// The octets of this PDU, in aligned PER.")?;
    writeln!(out, "    pub fn encode(&self) -> Result<Vec<u8>, crate::Error> {{")?;
    writeln!(out, "        rasn::aper::encode(self).map_err(crate::Error::encoding::<Self>)")?;
    writeln!(out, "    }}")?;
    writeln!(out, "    /// The PDU that `bytes` are, in aligned PER: the octets of one PDU, whole, and no")?;
    writeln!(out, "    /// octet after it.")?;
    writeln!(out, "    pub fn decode(bytes: &[u8]) -> Result<Self, crate::Error> {{")?;
    writeln!(out, "        crate::per::decode_complete(bytes)")?;
    writeln!(out, "    }}")?;
    writeln!(out, "    /// The message that this PDU holds, as its type `T`.")?;
    writeln!(out, "    ///")?;
    writeln!(out, "    /// `T` is the type of the message that `kind()` says. For the type of another")?;
    writeln!(out, "    /// message the error has the kind `WrongMessage`, as the octets of one message")?;
    writeln!(out, "    /// may decode as another:")?;
    writeln!(out, "    ///")?;
    writeln!(out, "    /// ```")?;
    writeln!(out, "    /// use oxirush_ngap::{{ErrorKind, ngap::*}};")?;
    writeln!(out, "    ///")?;
    writeln!(out, "    /// // A UE CONTEXT RELEASE REQUEST")?;
    writeln!(out, "    /// let octets = hex::decode(\"002a4015000003000a0002002a005500020007000f40020500\").unwrap();")?;
    writeln!(out, "    /// let pdu = NGAP_PDU::decode(&octets)?;")?;
    writeln!(out, "    /// let request: UEContextReleaseRequest = pdu.decode_value()?;")?;
    writeln!(out, "    /// assert_eq!(request.protocol_ies.0.len(), 3);")?;
    writeln!(out, "    ///")?;
    writeln!(out, "    /// let other = pdu.decode_value::<NGSetupRequest>().unwrap_err();")?;
    writeln!(out, "    /// assert_eq!(other.kind(), ErrorKind::WrongMessage);")?;
    writeln!(out, "    /// # Ok::<(), oxirush_ngap::Error>(())")?;
    writeln!(out, "    /// ```")?;
    writeln!(out, "    ///")?;
    writeln!(out, "    /// A type that is no message, as that of a transfer or an IE, does not compile.")?;
    writeln!(out, "    /// `decode_open_type` decodes any type from an open type.")?;
    writeln!(out, "    ///")?;
    writeln!(out, "    /// ```compile_fail")?;
    writeln!(out, "    /// use oxirush_ngap::ngap::*;")?;
    writeln!(out, "    ///")?;
    writeln!(out, "    /// fn held(pdu: &NGAP_PDU) {{")?;
    writeln!(out, "    ///     let _ = pdu.decode_value::<PDUSessionResourceSetupRequestTransfer>();")?;
    writeln!(out, "    /// }}")?;
    writeln!(out, "    /// ```")?;
    writeln!(out, "    pub fn decode_value<T: crate::Message>(&self) -> Result<T, crate::Error> {{")?;
    writeln!(out, "        let (direction, code) = (self.direction(), self.procedure_code());")?;
    writeln!(out, "        if (direction, code) != (T::DIRECTION, T::PROCEDURE_CODE) {{")?;
    writeln!(out, "            return Err(crate::Error::wrong_message(T::NAME, direction, code));")?;
    writeln!(out, "        }}")?;
    writeln!(out, "        let value = match self {{")?;
    writeln!(out, "            NGAPPDU::initiatingMessage(message) => &message.value,")?;
    writeln!(out, "            NGAPPDU::successfulOutcome(message) => &message.value,")?;
    writeln!(out, "            NGAPPDU::unsuccessfulOutcome(message) => &message.value,")?;
    writeln!(out, "        }};")?;
    writeln!(out, "        decode_open_type(value)")?;
    writeln!(out, "    }}")?;
    writeln!(out, "    /// Return the procedure code of this PDU.")?;
    writeln!(out, "    pub fn procedure_code(&self) -> u8 {{")?;
    writeln!(out, "        match self {{")?;
    writeln!(out, "            NGAPPDU::initiatingMessage(message) => message.procedure_code.0,")?;
    writeln!(out, "            NGAPPDU::successfulOutcome(message) => message.procedure_code.0,")?;
    writeln!(out, "            NGAPPDU::unsuccessfulOutcome(message) => message.procedure_code.0,")?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "    /// Return the PDU direction as a human-readable string.")?;
    writeln!(out, "    pub fn direction(&self) -> &'static str {{")?;
    writeln!(out, "        match self {{")?;
    writeln!(out, "            NGAPPDU::initiatingMessage(_) => \"InitiatingMessage\",")?;
    writeln!(out, "            NGAPPDU::successfulOutcome(_) => \"SuccessfulOutcome\",")?;
    writeln!(out, "            NGAPPDU::unsuccessfulOutcome(_) => \"UnsuccessfulOutcome\",")?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "    /// Returns `true` for an initiating message.")?;
    writeln!(out, "    pub fn is_initiating(&self) -> bool {{ matches!(self, NGAPPDU::initiatingMessage(_)) }}")?;
    writeln!(out, "    /// Returns `true` for a successful outcome.")?;
    writeln!(out, "    pub fn is_successful(&self) -> bool {{ matches!(self, NGAPPDU::successfulOutcome(_)) }}")?;
    writeln!(out, "    /// Returns `true` for an unsuccessful outcome.")?;
    writeln!(out, "    pub fn is_unsuccessful(&self) -> bool {{ matches!(self, NGAPPDU::unsuccessfulOutcome(_)) }}")?;
    writeln!(out, "}}")?;
    writeln!(out, "impl std::fmt::Display for NGAPPDU {{")?;
    writeln!(out, "    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{")?;
    writeln!(out, "        write!(formatter, \"{{}} {{}} (code={{}})\", self.direction(), self.procedure_name(), self.procedure_code())")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out, "/// The open type of a value: its complete encoding in aligned PER, in which an empty")?;
    writeln!(out, "/// encoding becomes one zero octet (X.691 (02/2021) §11.1.4, §11.2.1).")?;
    writeln!(out, "pub fn encode_open_type<T: rasn::Encode>(value: &T) -> Result<rasn::types::Any, crate::Error> {{")?;
    writeln!(out, "    crate::per::encode_open_type(value)")?;
    writeln!(out, "}}")?;
    writeln!(out, "/// The value of the type `T` that an open type holds, in aligned PER: one value,")?;
    writeln!(out, "/// whole, and no octet after it.")?;
    writeln!(out, "pub fn decode_open_type<T: rasn::Decode>(value: &rasn::types::Any) -> Result<T, crate::Error> {{")?;
    writeln!(out, "    crate::per::decode_complete(value.as_bytes())")?;
    writeln!(out, "}}")?;

    Ok(out)
}
