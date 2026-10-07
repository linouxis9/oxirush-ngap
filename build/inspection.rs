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

/// `roots` are the types that the registry gives to `Typed::of`: one of them that
/// reaches no open type still has an implementation, which does nothing.
fn generate_open_type_repair(
    generated: &str,
    module: &str,
    roots: &std::collections::BTreeSet<String>,
    out: &mut String,
) -> Result<()> {
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
                let fields: Vec<_> = fields.collect();
                if !fields.is_empty() || roots.contains(&item.ident.to_string()) {
                    let fields = list(fields);
                    writeln!(out, "open_types! {{ {prefix}{} {fields} }}", item.ident)?;
                }
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
                    if roots.contains(&item.ident.to_string()) {
                        writeln!(out, "open_types! {{ {prefix}{} {{}} }}", item.ident)?;
                    }
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

/// The well-known types whose values a tree shows in a readable form: the form, and the
/// names of the types that have it. A name is matched whatever its case and its hyphens:
/// TS 36.413 writes `PLMNidentity` and `Port-Number`.
const READABLE: &[(&str, &[&str])] = &[
    ("Plmn", &["PLMNIdentity"]),
    ("Digits", &["IMSI"]),
    ("Address", &["TransportLayerAddress"]),
    (
        "Number",
        &[
            "AMFPointer",
            "AMFRegionID",
            "AMFSetID",
            "CellIdentity",
            "CI",
            "EPS-TAC",
            "EUTRACellIdentity",
            "FiveG-TMSI",
            "FiveGSTAC",
            "GTP-TEID",
            "LAC",
            "M-TMSI",
            "MME-Code",
            "MME-Group-ID",
            "NRCellIdentity",
            "PortNumber",
            "RAC",
            "SST",
            "TAC",
            "UL-NAS-Count",
        ],
    ),
];

/// The ASN.1 identifier that `rasn` has for a type, a field or an alternative, and
/// whether the attributes of a type say that it is an ENUMERATED.
fn rasn(attributes: &[syn::Attribute]) -> Result<(Option<String>, bool)> {
    use syn::{Meta, Token, punctuated::Punctuated};
    let (mut identifier, mut enumerated) = (None, false);
    for attribute in attributes.iter().filter(|a| a.path().is_ident("rasn")) {
        let metas = attribute.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
        for meta in metas {
            match meta {
                Meta::Path(path) if path.is_ident("enumerated") => enumerated = true,
                Meta::NameValue(pair) if pair.path.is_ident("identifier") => {
                    let syn::Expr::Lit(literal) = pair.value else {
                        continue;
                    };
                    if let syn::Lit::Str(name) = literal.lit {
                        identifier = Some(name.value());
                    }
                }
                _ => {}
            }
        }
    }
    Ok((identifier, enumerated))
}

/// The last segment of a type's path: its name and its generic arguments.
fn named(ty: &syn::Type) -> Option<(String, Vec<&syn::GenericArgument>)> {
    let syn::Type::Path(path) = ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    let arguments = match &segment.arguments {
        syn::PathArguments::AngleBracketed(arguments) => arguments.args.iter().collect(),
        _ => Vec::new(),
    };
    Some((segment.ident.to_string(), arguments))
}

/// The structs of one unnamed member, by name: the ASN.1 name of each and the type that
/// it wraps.
type Wrapped<'a> = BTreeMap<String, (String, &'a syn::Type)>;

/// The number of bits of the fixed OCTET STRING or BIT STRING that `name` is.
fn bits(name: &str, wrapped: &Wrapped) -> Option<u32> {
    let (inner, arguments) = named(wrapped.get(name)?.1)?;
    let size = arguments.iter().find_map(|argument| match argument {
        syn::GenericArgument::Const(syn::Expr::Lit(literal)) => match &literal.lit {
            syn::Lit::Int(size) => size.base10_parse::<u32>().ok(),
            _ => None,
        },
        _ => None,
    });
    match inner.as_str() {
        "FixedOctetString" => Some(size? * 8),
        "FixedBitString" => size,
        _ => bits(&inner, wrapped),
    }
}

/// The well-known type that a member of type `ty` holds, alone or in a list: its ASN.1
/// name and its form.
fn readable(ty: &syn::Type, wrapped: &Wrapped, depth: u8) -> Option<(String, String)> {
    let (name, arguments) = named(ty)?;
    if matches!(name.as_str(), "Option" | "Box" | "Vec" | "SequenceOf") {
        let inner = arguments.iter().find_map(|argument| match argument {
            syn::GenericArgument::Type(inner) => Some(inner),
            _ => None,
        });
        return readable(inner?, wrapped, depth + 1);
    }
    let (asn, inner) = wrapped.get(&name)?;
    for (form, types) in READABLE {
        if types.iter().any(|known| canonical(known) == canonical(asn)) {
            let form = match *form {
                "Number" => format!("Form::Number({})", bits(&name, wrapped)?),
                form => format!("Form::{form}"),
            };
            return Some((asn.clone(), form));
        }
    }
    // A type that only wraps another, or a list.
    if depth < 8 {
        readable(inner, wrapped, depth + 1)
    } else {
        None
    }
}

/// The members and the IEs of the well-known types, by type, and the names of the
/// ENUMERATED values.
fn generate_readable(
    generated: &str,
    ies: &BTreeMap<u16, (String, std::collections::BTreeSet<String>)>,
    out: &mut String,
) -> Result<()> {
    use syn::ext::IdentExt;
    let syntax = syn::parse_file(generated)?;
    let mut declarations = Vec::new();
    items(&syntax.items, "", &mut declarations);
    let mut wrapped = Wrapped::new();
    for (_, item) in &declarations {
        if let syn::Item::Struct(item) = item
            && let syn::Fields::Unnamed(fields) = &item.fields
            && fields.unnamed.len() == 1
        {
            let asn = rasn(&item.attrs)?.0;
            let asn = asn.unwrap_or_else(|| item.ident.to_string());
            wrapped.insert(item.ident.to_string(), (asn, &fields.unnamed[0].ty));
        }
    }
    // The forms of the members of each name, with the type that has each. A name that
    // some type gives a member of another kind has several, and so has no form.
    let mut members: BTreeMap<String, BTreeMap<Option<String>, String>> = BTreeMap::new();
    let mut enumerated = std::collections::BTreeSet::new();
    for (_, item) in &declarations {
        let mut member = |attributes: &[syn::Attribute], ident: &syn::Ident, ty| -> Result<()> {
            let name = rasn(attributes)?.0;
            let name = name.unwrap_or_else(|| ident.unraw().to_string());
            let (asn, form) = readable(ty, &wrapped, 0).unzip();
            let forms = members.entry(name).or_default();
            forms.insert(form, asn.unwrap_or_default());
            Ok(())
        };
        match item {
            syn::Item::Struct(item) => {
                for field in &item.fields {
                    if let Some(ident) = &field.ident {
                        member(&field.attrs, ident, &field.ty)?;
                    }
                }
            }
            syn::Item::Enum(item) if rasn(&item.attrs)?.1 => {
                for value in &item.variants {
                    let name = rasn(&value.attrs)?.0;
                    enumerated.insert(name.unwrap_or_else(|| value.ident.unraw().to_string()));
                }
            }
            syn::Item::Enum(item) => {
                for alternative in &item.variants {
                    if let Some(field) = alternative.fields.iter().next() {
                        member(&alternative.attrs, &alternative.ident, &field.ty)?;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    // One list for each well-known type: what selects it in a `match`, and its form.
    type Lists = BTreeMap<String, (Vec<String>, String)>;
    let mut by_member = Lists::new();
    for (name, forms) in &members {
        match (forms.len(), forms.first_key_value()) {
            (1, Some((Some(form), asn))) => {
                let list = by_member.entry(asn.clone()).or_default();
                list.0.push(format!("{name:?}"));
                list.1.clone_from(form);
            }
            _ if forms.keys().any(Option::is_some) => {
                println!("cargo:warning=the members named {name} have several types: not readable");
            }
            _ => {}
        }
    }
    let mut by_ie = Lists::new();
    for (id, (_, types)) in ies {
        if let (1, Some(ty)) = (types.len(), types.first())
            && let Some((asn, form)) = readable(&syn::parse_str(ty)?, &wrapped, 0)
        {
            let list = by_ie.entry(asn).or_default();
            list.0.push(id.to_string());
            list.1 = form;
        }
    }
    for (function, selector, lists) in [
        ("member_form", "name: &str", &by_member),
        ("ie_form", "id: u16", &by_ie),
    ] {
        let selected = selector.split(':').next().expect("a name");
        if lists.is_empty() {
            let unused = selector.replace(selected, "_");
            writeln!(
                out,
                "pub(crate) fn {function}({unused}) -> Option<Form> {{ None }}"
            )?;
            continue;
        }
        writeln!(
            out,
            "pub(crate) fn {function}({selector}) -> Option<Form> {{ Some(match {selected} {{"
        )?;
        for (asn, (selectors, form)) in lists {
            writeln!(out, "// {asn}\n{} => {form},", selectors.join(" | "))?;
        }
        writeln!(out, "_ => return None, }}) }}")?;
    }
    writeln!(out, "pub(crate) const ENUMERATED: &[&str] = &[")?;
    for name in &enumerated {
        writeln!(out, "{name:?},")?;
    }
    writeln!(out, "];")?;
    Ok(())
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
    // The type in the OCTET STRING of an IE declared `OCTET STRING (CONTAINING ...)`.
    let containing = Regex::new(r"CONTAINING\s+([A-Za-z][A-Za-z0-9-]*)")?;
    let mut contents: BTreeMap<u16, std::collections::BTreeSet<String>> = BTreeMap::new();
    for c in object.captures_iter(asn) {
        let Some(id) = ids.get(&c[1]).copied() else {
            continue;
        };
        if let Some(ty) = containing.captures(&c[2]).and_then(|c| resolve(&c[1])) {
            contents
                .entry(id)
                .or_default()
                .insert(format!("crate::{module}::{ty}"));
        }
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
                messages.insert(
                    (direction, code),
                    (name[1].to_string(), format!("crate::{module}::{ty}")),
                );
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
    // The one type of what has one.
    let alone = |alternatives: &std::collections::BTreeSet<String>| {
        Some(alternatives.first()?.clone()).filter(|_| alternatives.len() == 1)
    };
    // One line for each message, IE and transfer: rustfmt leaves the lines of a macro
    // called with braces as they are.
    let mut out = String::from(
        "// Auto-generated by build/inspection.rs from ASN.1; do not edit.\nuse crate::inspect::{Form, Typed, ies, messages, transfers};\n",
    );
    writeln!(
        out,
        "pub(crate) const PROTOCOL: &str = {protocol:?};\npub(crate) const ROOT: &str = {module:?};"
    )?;
    // The direction, the procedure code, the name that ASN.1 gives it and the type.
    writeln!(out, "messages! {{")?;
    for ((direction, code), (name, ty)) in &messages {
        writeln!(out, "    {direction} {code} {name:?} {ty};")?;
    }
    writeln!(out, "}}")?;
    // The identifier, the name and the type, then the type that the octets of the IE
    // contain. An identifier that has several types has its name alone.
    writeln!(out, "ies! {{")?;
    for (id, (name, alternatives)) in &ies {
        match (alone(alternatives), contents.get(id).and_then(alone)) {
            (Some(ty), Some(contained)) => writeln!(out, "    {id} {name:?} {ty}, {contained};")?,
            (Some(ty), None) => writeln!(out, "    {id} {name:?} {ty};")?,
            (None, None) => writeln!(out, "    {id} {name:?};")?,
            (None, Some(_)) => anyhow::bail!("{name} contains a type and has several"),
        }
    }
    writeln!(out, "}}")?;
    // The member that holds a transfer, and the type that its octets contain.
    writeln!(out, "transfers! {{")?;
    for (field, alternatives) in &transfers {
        match alone(alternatives) {
            Some(ty) => writeln!(out, "    {field:?} {ty};")?,
            None => writeln!(out, "    {field:?};")?,
        }
    }
    writeln!(out, "}}")?;
    ensure!(
        !messages.is_empty() && !ies.is_empty(),
        "empty inspection registry"
    );
    generate_readable(generated, &ies, &mut out)?;
    // The types that the registry has the functions of.
    let roots = (messages.values().map(|(_, ty)| ty.clone()))
        .chain(ies.values().filter_map(|(_, types)| alone(types)))
        .chain(contents.values().filter_map(alone))
        .chain(transfers.values().filter_map(alone))
        .filter_map(|ty| Some(ty.rsplit("::").next()?.to_string()))
        .collect();
    generate_open_type_repair(generated, &module, &roots, &mut out)?;
    Ok(out)
}
