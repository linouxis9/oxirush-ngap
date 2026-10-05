//! Generate dynamic open-type dispatch from the same ASN.1 as the codec.
use std::collections::BTreeMap;
use std::fmt::Write;

use anyhow::{Result, ensure};
use regex::Regex;
use syn::visit::Visit;

struct TypeNames(std::collections::BTreeSet<String>);
impl<'ast> Visit<'ast> for TypeNames {
    fn visit_type_path(&mut self, path: &'ast syn::TypePath) {
        if let Some(segment) = path.path.segments.last() {
            self.0.insert(segment.ident.to_string());
        }
        syn::visit::visit_type_path(self, path);
    }
}

fn references(ty: &syn::Type) -> std::collections::BTreeSet<String> {
    let mut visitor = TypeNames(Default::default());
    visitor.visit_type(ty);
    visitor.0
}

fn items<'a>(input: &'a [syn::Item], prefix: &str, output: &mut Vec<(String, &'a syn::Item)>) {
    for item in input {
        match item {
            syn::Item::Mod(module) => {
                if let Some((_, inner)) = &module.content {
                    items(inner, &format!("{prefix}{}::", module.ident), output);
                }
            }
            syn::Item::Struct(_) | syn::Item::Enum(_) => output.push((prefix.to_string(), item)),
            _ => {}
        }
    }
}

fn generate_open_type_repair(generated: &str, module: &str, out: &mut String) -> Result<()> {
    let syntax = syn::parse_file(generated)?;
    let mut declarations = Vec::new();
    items(
        &syntax.items,
        &format!("crate::{module}::"),
        &mut declarations,
    );
    fn fields(item: &syn::Item) -> Vec<&syn::Field> {
        match item {
            syn::Item::Struct(item) => item.fields.iter().collect(),
            syn::Item::Enum(item) => item.variants.iter().flat_map(|v| v.fields.iter()).collect(),
            _ => unreachable!(),
        }
    }
    let mut affected: std::collections::BTreeSet<String> = ["Any".to_string()].into();
    loop {
        let before = affected.len();
        for (_, item) in &declarations {
            let name = match item {
                syn::Item::Struct(item) => &item.ident,
                syn::Item::Enum(item) => &item.ident,
                _ => unreachable!(),
            };
            if fields(item)
                .iter()
                .any(|f| !references(&f.ty).is_disjoint(&affected))
            {
                affected.insert(name.to_string());
            }
        }
        if affected.len() == before {
            break;
        }
    }
    // One line per type: the fields, or the CHOICE alternatives, through which it reaches an
    // open type. rustfmt leaves the lines of a macro called with braces as they are.
    let list = |names: Vec<String>| match names.is_empty() {
        true => "{}".to_string(),
        false => format!("{{ {} }}", names.join(", ")),
    };
    writeln!(out, "use crate::inspect::open_types;")?;
    for (prefix, item) in declarations {
        let open = |field: &syn::Field| !references(&field.ty).is_disjoint(&affected);
        match item {
            syn::Item::Struct(item) => {
                let fields = item.fields.iter().enumerate().filter(|(_, f)| open(f));
                let fields = fields.map(|(index, field)| match &field.ident {
                    Some(name) => name.to_string(),
                    None => index.to_string(),
                });
                writeln!(
                    out,
                    "open_types! {{ {prefix}{} {} }}",
                    item.ident,
                    list(fields.collect())
                )?;
            }
            syn::Item::Enum(item) => {
                let (with, without): (Vec<_>, Vec<_>) = item
                    .variants
                    .iter()
                    .partition(|v| v.fields.iter().any(open));
                ensure!(
                    with.iter().all(|variant| variant.fields.len() == 1),
                    "{}: an alternative with an open type has one value",
                    item.ident
                );
                let names = |variants: Vec<&syn::Variant>| {
                    list(variants.iter().map(|v| v.ident.to_string()).collect())
                };
                if with.is_empty() {
                    writeln!(out, "open_types! {{ {prefix}{} {{}} }}", item.ident)?;
                } else {
                    writeln!(
                        out,
                        "open_types! {{ {prefix}{} {} else {} }}",
                        item.ident,
                        names(with),
                        names(without)
                    )?;
                }
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}

fn canonical(name: &str) -> String {
    name.chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect()
}

pub(super) fn generate(protocol: &str, generated: &str, asn: &str) -> Result<String> {
    let module = protocol.to_ascii_lowercase();
    let declaration = Regex::new(r"(?m)^    pub (?:struct|enum|type) ([A-Za-z][A-Za-z0-9_]*)")?;
    let mut types = BTreeMap::new();
    for c in declaration.captures_iter(generated) {
        types
            .entry(canonical(&c[1]))
            .or_insert_with(|| c[1].to_string());
    }
    let resolve = |name: &str| types.get(&canonical(name));
    let constant = Regex::new(
        r"(?m)^\s*(id-[A-Za-z][A-Za-z0-9-]*)\s+(?:ProtocolIE-ID|ProtocolExtensionID)\s+::=\s+(\d+)",
    )?;
    let ids: BTreeMap<_, u16> = constant
        .captures_iter(asn)
        .map(|c| Ok((c[1].to_string(), c[2].parse()?)))
        .collect::<Result<_>>()?;
    let object = Regex::new(
        r"(?s)\{\s*ID\s+(id-[A-Za-z][A-Za-z0-9-]*)\s+CRITICALITY\s+[A-Za-z-]+\s+(?:TYPE|EXTENSION)\s+(OCTET\s+STRING(?:\s*\(CONTAINING\s+[A-Za-z][A-Za-z0-9-]*\s*\))?|[A-Za-z][A-Za-z0-9-]*)\s+PRESENCE",
    )?;
    let mut ies: BTreeMap<u16, (String, std::collections::BTreeSet<String>)> = BTreeMap::new();
    for c in object.captures_iter(asn) {
        let Some(id) = ids.get(&c[1]).copied() else {
            continue;
        };
        let ty = if c[2].starts_with("OCTET") {
            "rasn::types::OctetString".to_string()
        } else {
            let Some(ty) = resolve(&c[2]) else { continue };
            format!("crate::{module}::{ty}")
        };
        ies.entry(id)
            .or_insert_with(|| {
                (
                    c[1].trim_start_matches("id-").to_string(),
                    Default::default(),
                )
            })
            .1
            .insert(ty);
    }
    let proc_constant =
        Regex::new(r"(?m)^\s*(id-[A-Za-z][A-Za-z0-9-]*)\s+ProcedureCode\s+::=\s+(\d+)")?;
    let proc_ids: BTreeMap<_, u8> = proc_constant
        .captures_iter(asn)
        .map(|c| Ok((c[1].to_string(), c[2].parse()?)))
        .collect::<Result<_>>()?;
    let block = Regex::new(&format!(
        r"(?ms)^[\t ]*[A-Za-z][A-Za-z0-9-]*\s+{protocol}-ELEMENTARY-PROCEDURE\s+::=\s*\{{(.*?)^[\t ]*\}}"
    ))?;
    let proc_ref = Regex::new(r"PROCEDURE CODE\s+(id-[A-Za-z][A-Za-z0-9-]*)")?;
    let mut messages = BTreeMap::new();
    for c in block.captures_iter(asn) {
        let Some(proc_ref) = proc_ref.captures(&c[1]) else {
            continue;
        };
        let Some(code) = proc_ids.get(&proc_ref[1]).copied() else {
            continue;
        };
        for (label, direction) in [
            ("INITIATING MESSAGE", "InitiatingMessage"),
            ("SUCCESSFUL OUTCOME", "SuccessfulOutcome"),
            ("UNSUCCESSFUL OUTCOME", "UnsuccessfulOutcome"),
        ] {
            let name = Regex::new(&format!(r"\b{label}\s+([A-Za-z][A-Za-z0-9-]*)"))?;
            if let Some(name) = name.captures(&c[1]) {
                let ty = resolve(&name[1])
                    .ok_or_else(|| anyhow::anyhow!("missing message {}", &name[1]))?;
                messages.insert((direction, code), format!("crate::{module}::{ty}"));
            }
        }
    }
    let contained = Regex::new(
        r"(?m)^\s*([A-Za-z][A-Za-z0-9-]*)\s+OCTET\s+STRING\s*\(\s*CONTAINING\s+([A-Za-z][A-Za-z0-9-]*)\s*\)",
    )?;
    let mut transfers: BTreeMap<String, std::collections::BTreeSet<String>> = BTreeMap::new();
    for c in contained.captures_iter(asn) {
        if let Some(ty) = resolve(&c[2]) {
            transfers
                .entry(c[1].to_string())
                .or_default()
                .insert(format!("crate::{module}::{ty}"));
        }
    }
    let mut out =
        String::from("// Auto-generated by build/inspection.rs from ASN.1; do not edit.\n");
    for operation in ["decode", "encode"] {
        let (input, output, call) = if operation == "decode" {
            ("raw: &[u8]", "serde_json::Value", "raw")
        } else {
            ("value: &serde_json::Value", "Vec<u8>", "value")
        };
        writeln!(
            out,
            "pub(crate) fn {operation}_message(direction: &str, code: u8, {input}) -> Result<{output}, String> {{ match (direction, code) {{"
        )?;
        for ((direction, code), ty) in &messages {
            writeln!(
                out,
                "({direction:?}, {code}) => crate::inspect::{operation}_typed::<{ty}>({call}),"
            )?;
        }
        writeln!(
            out,
            "_ => Err(format!(\"unknown {protocol} direction/procedure {{direction}}/{{code}}\")), }} }}"
        )?;
        writeln!(
            out,
            "pub(crate) fn {operation}_ie(id: u16, {input}) -> Result<{output}, String> {{"
        )?;
        writeln!(out, "match id {{")?;
        for (id, (_, alternatives)) in &ies {
            if alternatives.len() != 1 {
                continue;
            }
            let ty = alternatives.first().expect("one type");
            writeln!(
                out,
                "{id} => crate::inspect::{operation}_typed::<{ty}>({call}),"
            )?;
        }
        writeln!(
            out,
            "_ => Err(format!(\"{protocol} IE {{id}} is unknown or has several types\")), }} }}"
        )?;
        let has_transfers = transfers
            .values()
            .any(|alternatives| alternatives.len() == 1);
        let transfer_input = if has_transfers {
            input.to_string()
        } else {
            input.replacen(call, &format!("_{call}"), 1)
        };
        writeln!(
            out,
            "pub(crate) fn {operation}_transfer(field: &str, {transfer_input}) -> Result<{output}, String> {{"
        )?;
        if has_transfers {
            writeln!(out, "match field {{")?;
        }
        for (field, alternatives) in &transfers {
            if alternatives.len() == 1 {
                let ty = alternatives.first().expect("one transfer type");
                writeln!(
                    out,
                    "{field:?} => crate::inspect::{operation}_typed::<{ty}>({call}),"
                )?;
            }
        }
        if has_transfers {
            writeln!(out, "_ =>")?;
        }
        writeln!(
            out,
            "Err(format!(\"{protocol} contained transfer {{field}} is unknown or has several types\"))"
        )?;
        if has_transfers {
            writeln!(out, "}}")?;
        }
        writeln!(out, "}}")?;
    }
    writeln!(out, "pub(crate) const TRANSFER_FIELDS: &[&str] = &[")?;
    for field in transfers.keys() {
        writeln!(out, "{field:?},")?;
    }
    writeln!(out, "];")?;
    writeln!(out, "pub(crate) const IE_NAMES: &[(u16, &str)] = &[")?;
    for (id, (name, _)) in &ies {
        writeln!(out, "({id}, {name:?}),")?;
    }
    writeln!(out, "];")?;
    ensure!(
        !messages.is_empty() && !ies.is_empty(),
        "empty inspection registry"
    );
    generate_open_type_repair(generated, &module, &mut out)?;
    Ok(out)
}
