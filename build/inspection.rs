//! Generate dynamic open-type dispatch from the same ASN.1 as the codec.
use std::collections::BTreeMap;
use std::fmt::Write;

use anyhow::{Result, ensure};
use regex::Regex;
use syn::visit::Visit;

use crate::registry::{Ie, Registry, alone, canonical};

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
    let (identifier, enumerated, _) = rasn_kind(attributes)?;
    Ok((identifier, enumerated))
}

/// Whether the attributes of a string give it one size: JER then writes a string of bits
/// as its octets alone, without its length.
fn one_size(attributes: &[syn::Attribute]) -> Result<bool> {
    use syn::{Meta, Token, punctuated::Punctuated};
    for attribute in attributes.iter().filter(|a| a.path().is_ident("rasn")) {
        let metas = attribute.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
        for meta in metas {
            if let Meta::List(list) = meta
                && list.path.is_ident("size")
            {
                let size = list.tokens.to_string();
                let size = size.split(',').next().unwrap_or_default().trim();
                let digits = size.trim_matches('"');
                return Ok(!digits.is_empty() && digits.bytes().all(|digit| digit.is_ascii_digit()));
            }
        }
    }
    Ok(false)
}

/// What [`rasn`] gives, and whether the attributes of a member give it a default value.
fn rasn_kind(attributes: &[syn::Attribute]) -> Result<(Option<String>, bool, bool)> {
    use syn::{Meta, Token, punctuated::Punctuated};
    let (mut identifier, mut enumerated, mut default) = (None, false, false);
    for attribute in attributes.iter().filter(|a| a.path().is_ident("rasn")) {
        let metas = attribute.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
        for meta in metas {
            match meta {
                Meta::Path(path) if path.is_ident("enumerated") => enumerated = true,
                Meta::Path(path) if path.is_ident("default") => default = true,
                Meta::NameValue(pair) if pair.path.is_ident("default") => default = true,
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
    Ok((identifier, enumerated, default))
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
fn generate_readable(generated: &str, ies: &BTreeMap<u16, Ie>, out: &mut String) -> Result<()> {
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
                eprintln!("warning: the members named {name} have several types: not readable");
            }
            _ => {}
        }
    }
    let mut by_ie = Lists::new();
    for (id, ie) in ies {
        if let Some(ty) = alone(&ie.types)
            && let Some((asn, form)) = readable(&syn::parse_str(&ty)?, &wrapped, 0)
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

/// The containers of the protocol, which a tree shows as IEs, and how the list of the
/// types writes each.
const CONTAINERS: &[(&str, &str)] = &[
    ("ProtocolIEContainer", "ies"),
    ("ProtocolIEField", "ie"),
    ("ProtocolExtensionContainer", "extensions"),
    ("ProtocolExtensionField", "extension"),
];

/// How the list of the types writes a value of type `ty`, and whether a member of that
/// type may be absent: `_` for a value without members, `bits` for a string of bits whose
/// size varies, the name of a SEQUENCE, of a CHOICE or of a type that is a list, a list
/// in brackets and a container by its kind.
fn shape(ty: &syn::Type, declared: &BTreeMap<String, &syn::Item>, depth: u8) -> (String, bool) {
    let plain = || ("_".to_string(), false);
    let Some((name, arguments)) = named(ty).filter(|_| depth < 16) else {
        return plain();
    };
    let inner = arguments.iter().find_map(|argument| match argument {
        syn::GenericArgument::Type(inner) => Some(inner),
        _ => None,
    });
    if let Some((_, written)) = CONTAINERS.iter().find(|(container, _)| *container == name) {
        return (written.to_string(), false);
    }
    match (name.as_str(), inner, declared.get(&name)) {
        ("Option", Some(inner), _) => (shape(inner, declared, depth + 1).0, true),
        ("Box", Some(inner), _) => shape(inner, declared, depth + 1),
        ("SequenceOf" | "SetOf" | "Vec", Some(inner), _) => {
            (format!("[{}]", shape(inner, declared, depth + 1).0), false)
        }
        ("BitString" | "SizedBitString", ..) => ("bits".to_string(), false),
        // A type that only wraps another is what it wraps, when that has no members.
        (_, _, Some(syn::Item::Struct(item))) => match &item.fields {
            syn::Fields::Named(_) => (name.clone(), false),
            syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                match shape(&fields.unnamed[0].ty, declared, depth + 1).0.as_str() {
                    "_" => plain(),
                    "bits" if one_size(&item.attrs).unwrap_or(false) => plain(),
                    _ => (name.clone(), false),
                }
            }
            _ => plain(),
        },
        (_, _, Some(syn::Item::Enum(item))) => match rasn(&item.attrs) {
            Ok((_, false)) => (name.clone(), false),
            _ => plain(),
        },
        _ => plain(),
    }
}

/// The list of the types: one line for each SEQUENCE and each CHOICE, with its members
/// or its alternatives by the names that ASN.1 gives them, and one for each type that is
/// a list or a string of bits. A member that holds a transfer has the type that its
/// octets contain in parentheses.
fn generate_types(
    generated: &str,
    transfers: &BTreeMap<String, std::collections::BTreeSet<String>>,
    out: &mut String,
) -> Result<()> {
    use syn::ext::IdentExt;
    let syntax = syn::parse_file(generated)?;
    let mut declarations = Vec::new();
    items(&syntax.items, "", &mut declarations);
    let mut declared = BTreeMap::new();
    for (_, item) in &declarations {
        let name = match item {
            syn::Item::Struct(item) => &item.ident,
            syn::Item::Enum(item) => &item.ident,
            _ => unreachable!(),
        };
        ensure!(
            declared.insert(name.to_string(), *item).is_none(),
            "{name}: two types of the bindings have this name"
        );
    }
    let member = |attributes: &[syn::Attribute], ident: &syn::Ident, ty: &syn::Type| {
        let (name, _, default) = rasn_kind(attributes)?;
        let name = name.unwrap_or_else(|| ident.unraw().to_string());
        let (mut written, optional) = shape(ty, &declared, 0);
        if written == "bits" && one_size(attributes)? {
            written = "_".to_string();
        }
        // The octets of a transfer are shown as the value that they contain.
        if let Some(contained) = transfers.get(&name) {
            ensure!(written == "_", "{name}: a transfer is a string of octets");
            let contained = alone(contained).unwrap_or_default();
            written = format!("({})", contained.rsplit("::").next().unwrap_or_default());
        }
        let presence = match optional || default {
            true => '?',
            false => ':',
        };
        Ok(format!("{name:?}{presence} {written}"))
    };
    // rustfmt leaves the lines of a macro called with braces as they are.
    writeln!(out, "types! {{")?;
    for (name, item) in &declared {
        if CONTAINERS.iter().any(|(container, _)| container == name) {
            continue;
        }
        match item {
            syn::Item::Struct(item) => match &item.fields {
                syn::Fields::Named(fields) => {
                    let members = fields.named.iter().map(|field| {
                        member(&field.attrs, field.ident.as_ref().expect("a name"), &field.ty)
                    });
                    let members = members.collect::<Result<Vec<_>>>()?;
                    writeln!(out, "    {name} sequence {{ {} }};", members.join(", "))?;
                }
                syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    let (written, _) = shape(&fields.unnamed[0].ty, &declared, 0);
                    if written != "_" && !(written == "bits" && one_size(&item.attrs)?) {
                        writeln!(out, "    {name} = {written};")?;
                    }
                }
                _ => {}
            },
            syn::Item::Enum(item) if !rasn(&item.attrs)?.1 => {
                let alternatives = item.variants.iter().map(|alternative| {
                    match alternative.fields.iter().next() {
                        Some(field) => member(&alternative.attrs, &alternative.ident, &field.ty),
                        None => {
                            let name = rasn(&alternative.attrs)?.0;
                            let name = name.unwrap_or_else(|| alternative.ident.unraw().to_string());
                            Ok(format!("{name:?}: _"))
                        }
                    }
                });
                let alternatives = alternatives.collect::<Result<Vec<_>>>()?;
                writeln!(out, "    {name} choice {{ {} }};", alternatives.join(", "))?;
            }
            _ => {}
        }
    }
    writeln!(out, "}}")?;
    Ok(())
}

/// The tables that only the inspection has: the transfers, the readable forms, the
/// members of the types and the open types. The messages and the IEs are those of the
/// registry.
pub(super) fn generate(
    protocol: &str,
    generated: &str,
    asn: &str,
    registry: &Registry,
) -> Result<String> {
    let module = protocol.to_ascii_lowercase();
    let contained = Regex::new(
        r"(?m)^\s*([A-Za-z][A-Za-z0-9-]*)\s+OCTET\s+STRING\s*\(\s*CONTAINING\s+([A-Za-z][A-Za-z0-9-]*)\s*\)",
    )?;
    let mut transfers: BTreeMap<String, std::collections::BTreeSet<String>> = BTreeMap::new();
    for c in contained.captures_iter(asn) {
        if let Some(ty) = registry.resolve(&c[2]) {
            transfers.entry(c[1].to_string()).or_default().insert(ty);
        }
    }
    // One line for each transfer: rustfmt leaves the lines of a macro called with braces
    // as they are.
    let mut out = String::from(
        "// Auto-generated by build/inspection.rs from ASN.1; do not edit.\nuse crate::inspect::{Form, Typed, transfers};\nuse crate::inspect_paths::types;\npub(crate) use crate::registry::{IE_NAMES, IE_TYPES, MESSAGES, ie, ie_contents};\n",
    );
    writeln!(
        out,
        "pub(crate) const PROTOCOL: &str = {protocol:?};\npub(crate) const ROOT: &str = {module:?};"
    )?;
    // The member that holds a transfer, and the type that its octets contain.
    writeln!(out, "transfers! {{")?;
    for (field, alternatives) in &transfers {
        match alone(alternatives) {
            Some(ty) => writeln!(out, "    {field:?} {ty};")?,
            None => writeln!(out, "    {field:?};")?,
        }
    }
    writeln!(out, "}}")?;
    generate_readable(generated, &registry.ies, &mut out)?;
    generate_types(generated, &transfers, &mut out)?;
    // The types that the registry and the transfers have the functions of.
    let messages = registry.procedures.values().flat_map(|p| p.messages.iter());
    let roots = (messages.flatten().map(|message| message.ty.clone()))
        .chain(registry.ies.values().filter_map(|ie| alone(&ie.types)))
        .chain(registry.ies.values().filter_map(|ie| alone(&ie.contents)))
        .chain(transfers.values().filter_map(alone))
        .filter_map(|ty| Some(ty.rsplit("::").next()?.to_string()))
        .collect();
    generate_open_type_repair(generated, &module, &roots, &mut out)?;
    Ok(out)
}
