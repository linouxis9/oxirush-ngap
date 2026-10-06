//! One Rust type for each protocol container.
//!
//! The containers of the Containers module take an object set as a
//! parameter, and rasn-compiler resolves every use of one to types of its
//! own: a field SEQUENCE, its criticality ENUMERATED and their SEQUENCE OF,
//! for each message and each extensible type. An open type is opaque here, so
//! all the fields of one kind have the same members and the same encoding.
//! Each kind becomes one type, and the resolved names are re-exports of it.
use std::collections::BTreeMap;

use anyhow::{Result, anyhow, bail, ensure};
use regex::{Captures, Regex};

const SHARED: &str = r#"    #[doc = " `ProtocolIE-Field`: an IE of a protocol IE container, and a `ProtocolIE-SingleContainer`."]
    #[doc = " The object set of a container does not show in its type: `value` is an open type."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "ProtocolIE-Field")]
    pub struct ProtocolIEField {
        pub id: ProtocolIEID,
        pub criticality: Criticality,
        pub value: Any,
    }
    impl ProtocolIEField {
        pub fn new(id: impl Into<ProtocolIEID>, criticality: Criticality, value: Any) -> Self {
            Self {
                id: id.into(),
                criticality,
                value,
            }
        }
    }
    #[doc = " `ProtocolIE-Container`: the IEs of a message."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("0..=65535"), identifier = "ProtocolIE-Container")]
    pub struct ProtocolIEContainer(pub SequenceOf<ProtocolIEField>);
    #[doc = " `ProtocolExtensionField`: an IE of a protocol extension container."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(automatic_tags, identifier = "ProtocolExtensionField")]
    pub struct ProtocolExtensionField {
        pub id: ProtocolExtensionID,
        pub criticality: Criticality,
        #[rasn(identifier = "extensionValue")]
        pub extension_value: Any,
    }
    impl ProtocolExtensionField {
        pub fn new(
            id: impl Into<ProtocolExtensionID>,
            criticality: Criticality,
            extension_value: Any,
        ) -> Self {
            Self {
                id: id.into(),
                criticality,
                extension_value,
            }
        }
    }
    #[doc = " `ProtocolExtensionContainer`: the `iE-Extensions` of a type."]
    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
    #[rasn(delegate, size("1..=65535"), identifier = "ProtocolExtensionContainer")]
    pub struct ProtocolExtensionContainer(pub SequenceOf<ProtocolExtensionField>);
"#;

/// Replace the resolved containers of `generated` by the types above, which
/// go into its `containers` module. `common` is the module of `Criticality`.
pub fn share(generated: &str, containers: &str, common: &str) -> Result<String> {
    // The resolved name of each replaced type, and the type it now names.
    let mut shared: BTreeMap<String, &str> = BTreeMap::new();

    // rustfmt splits a long attribute or member over lines.
    let field = Regex::new(
        r#"(?ms)^(?:    #\[doc = "[^"\n]*"\]\n)*    #\[derive\(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash\)\]
    #\[rasn\(\s*automatic_tags(?:,\s*identifier = "[^"]+")?,?\s*\)\]
    pub struct ([A-Za-z0-9_]+) \{
(?:        #\[rasn\(value\("0\.\.=65535"\)\)\]
)?        pub id: (u16|ProtocolIEID|ProtocolExtensionID),
        pub criticality:\s+[A-Za-z0-9_]+,
(?:        #\[rasn\(identifier = "extensionValue"\)\]
)?        pub (value|extension_value): Any,
    \}
    impl ([A-Za-z0-9_]+) \{
.*?^    \}
"#,
    )?;
    let mut failure = None;
    let generated = field.replace_all(generated, |captures: &Captures<'_>| {
        let (name, id) = (&captures[1], &captures[2]);
        let (kind, other_id) = match &captures[3] {
            "value" => ("ProtocolIEField", "ProtocolExtensionID"),
            _ => ("ProtocolExtensionField", "ProtocolIEID"),
        };
        if name != &captures[4] || id == other_id {
            failure.get_or_insert_with(|| format!("unrecognized container field {name}"));
        }
        shared.insert(name.to_string(), kind);
        format!("    pub use super::{containers}::{kind} as {name};\n")
    });
    if let Some(failure) = failure {
        bail!(failure);
    }

    let criticality = Regex::new(
        r#"(?m)^(?:    #\[doc = "[^"\n]*"\]\n)*    #\[derive\(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy\)\]
    #\[rasn\(enumerated\)\]
    pub enum ([A-Za-z0-9_]+Criticality) \{
        reject = 0,
        ignore = 1,
        notify = 2,
    \}
"#,
    )?;
    let generated = criticality.replace_all(&generated, |captures: &Captures<'_>| {
        format!(
            "    pub use super::{common}::Criticality as {};\n",
            &captures[1]
        )
    });

    let container = Regex::new(
        r#"(?m)^(?:    #\[doc = "[^"\n]*"\]\n)*    #\[derive\(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash\)\]
    #\[rasn\(delegate, size\("([01])\.\.=65535"\)\)\]
    pub struct ([A-Za-z0-9_]+)\(\s*pub\s+SequenceOf<\s*([A-Za-z0-9_]+),?\s*>,?\s*\);
"#,
    )?;
    let mut lists = Vec::new();
    let generated = container.replace_all(&generated, |captures: &Captures<'_>| {
        let kind = match (&captures[1], shared.get(&captures[3])) {
            ("0", Some(&"ProtocolIEField")) => "ProtocolIEContainer",
            ("1", Some(&"ProtocolExtensionField")) => "ProtocolExtensionContainer",
            _ => return captures[0].to_string(),
        };
        lists.push((captures[2].to_string(), kind));
        format!(
            "    pub use super::{containers}::{kind} as {};\n",
            &captures[2]
        )
    });
    ensure!(
        !shared.is_empty() && !lists.is_empty(),
        "no protocol containers found"
    );
    shared.extend(lists);

    // What refers to a resolved name refers to the shared type. The names are
    // identifiers of rasn-compiler's own making: no string has one.
    let identifier = Regex::new(r"[A-Za-z_][A-Za-z0-9_]*")?;
    let mut output = String::with_capacity(generated.len());
    for line in generated.split_inclusive('\n') {
        if line.starts_with("    pub use ") || line.contains('"') {
            output.push_str(line);
        } else {
            output.push_str(&identifier.replace_all(line, |captures: &Captures<'_>| {
                let name = &captures[0];
                shared.get(name).copied().unwrap_or(name).to_string()
            }));
        }
    }
    let leftover = Regex::new(
        r"pub id: (?:u16|ProtocolIEID|ProtocolExtensionID),\s+pub criticality:|SequenceOf<\s*ProtocolExtensionField\b",
    )?;
    ensure!(
        !leftover.is_match(&output),
        "a protocol container keeps a type of its own"
    );

    // A module imports the parameterized containers by name, or all of them.
    let imports = Regex::new(&format!(
        r"(?ms)^    use super::{containers}::\{{.*?^    \}};\n"
    ))?;
    let mut output = imports
        .replace_all(&output, format!("    use super::{containers}::*;\n"))
        .into_owned();
    let module = Regex::new(&format!(r"(?ms)^pub mod {containers} \{{\n.*?^\}}\n"))?;
    let end = module
        .find(&output)
        .ok_or_else(|| anyhow!("no {containers} module"))?
        .end();
    output.insert_str(end - "}\n".len(), SHARED);
    Ok(output)
}
