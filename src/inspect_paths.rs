//! The values of an inspection tree by their paths: see the module documentation of
//! [`crate::inspect`].

use serde_json::{Value, json};

// The name of the protocol, and the first segment of the paths of the IEs of the
// message of a PDU.
use crate::inspect_registry::{PROTOCOL, ROOT};
/// The member of a tree that holds the message of the PDU.
const MESSAGE: &str = "message";
/// The member of a message that holds its IEs.
const IES: &str = "protocolIEs";
/// The most occurrences that a path selects.
const OCCURRENCES: usize = 4096;
/// The member that marks an IE or a transfer whose value an edit changed: its octets are
/// no longer those of its value.
pub(crate) const EDITED: &str = "_edited";
/// The members that hold the value of an IE or of a transfer.
const VALUES: [&str; 3] = ["value", "extensionValue", "decoded"];

/// Whether two names are the same: whatever their case, with `-`, `_` and space taken as
/// the same.
fn same_name(a: &str, b: &str) -> bool {
    let letter = |c: u8| match c {
        b'-' | b'_' | b' ' => b'-',
        c => c.to_ascii_lowercase(),
    };
    a.len() == b.len()
        && a.bytes()
            .zip(b.bytes())
            .all(|(a, b)| letter(a) == letter(b))
}

/// The number of the IE that ASN.1 names `id-<name>`.
fn ie_id(name: &str) -> Option<u16> {
    let mut names = crate::inspect::ie_names().iter();
    names
        .find(|(_, known)| same_name(known, name))
        .map(|(id, _)| *id)
}

/// Whether a segment selects among the IEs of a message: a name, a position, `@id=N`,
/// `*`, or `-` for their end.
fn selects_ies(part: &str) -> bool {
    matches!(part, "*" | "-")
        || part.starts_with("@id=")
        || part.parse::<usize>().is_ok()
        || ie_id(part).is_some()
}

/// The segments of a path: those of a JSON pointer, and whether the path starts with the
/// root. The segment after the root selects among the IEs of the message.
fn segments(path: &str) -> Result<(Vec<String>, bool), String> {
    if path.len() > 4096 || !path.starts_with('/') {
        return Err("IE path must start with / and be at most 4096 bytes".into());
    }
    let parts = path[1..]
        .split('/')
        .map(|part| {
            let mut decoded = String::new();
            let mut chars = part.chars();
            while let Some(c) = chars.next() {
                decoded.push(if c == '~' {
                    match chars.next() {
                        Some('0') => '~',
                        Some('1') => '/',
                        _ => return Err("invalid JSON-pointer escape".into()),
                    }
                } else {
                    c
                });
            }
            if let Some(id) = decoded.strip_prefix("@id=") {
                id.parse::<u16>().map_err(|_| "IE id must be u16")?;
            }
            Ok(decoded)
        })
        .collect::<Result<Vec<_>, String>>()?;
    if parts.len() > 64 {
        return Err("IE path nesting exceeds 64".into());
    }
    let rooted = parts[0] == ROOT;
    if rooted
        && let Some(ie) = parts.get(1)
        && !selects_ies(ie)
    {
        return Err(format!("{ie:?} is not an IE of {PROTOCOL} at {path}"));
    }
    Ok((parts, rooted))
}

/// The segments of `path` in a tree, with what the root is short for, and whether the
/// IEs on the way are selected by name.
fn resolve(path: &str) -> Result<(Vec<String>, bool), String> {
    let (mut parts, rooted) = segments(path)?;
    if rooted {
        parts.splice(..1, [MESSAGE.to_string(), IES.to_string()]);
    }
    Ok((parts, rooted))
}

/// The values at `path` in `tree`, in the order of the tree. An IE that the message does
/// not have selects nothing; a member that a value does not have is an error, whether the
/// type of the value has none of that name or the value has it absent. `*` is each entry
/// of a list or each member of a value, without those that start with `_`. The `octets`
/// of an IE or of a transfer are what it was received as: those of one whose value
/// [`set`], [`remove`] or [`insert`] edited since are an error, as
/// [`encode_pdu`](crate::inspect::encode_pdu) has yet to give it others.
pub fn select<'a>(tree: &'a Value, path: &str) -> Result<Vec<&'a Value>, String> {
    let mut selected = vec![tree];
    let (parts, named) = resolve(path)?;
    for part in parts {
        let mut next = Vec::new();
        for value in selected {
            // The IEs of a message that the tree contains are selected in its place.
            let value = match value.get(IES) {
                Some(ies @ Value::Array(_))
                    if named && value.get(&part).is_none() && selects_ies(&part) =>
                {
                    ies
                }
                _ => value,
            };
            if part == "*" {
                match value {
                    Value::Array(a) => next.extend(a),
                    Value::Object(o) => {
                        next.extend(o.iter().filter(|(name, _)| shown(name)).map(|(_, v)| v))
                    }
                    Value::Null => {}
                    _ => return Err(format!("wildcard cannot traverse a scalar at {path}")),
                }
            } else if let Some(id) = part.strip_prefix("@id=") {
                let id: u64 = id.parse().map_err(|_| "invalid IE id")?;
                if let Some(array) = value.as_array() {
                    next.extend(array.iter().filter(|v| v["id"].as_u64() == Some(id)));
                } else if !value.is_null() {
                    return Err(format!("IE-id selection requires an array at {path}"));
                }
            } else if let Some(object) = value.as_object() {
                // `octets` is what an IE or a transfer was received as.
                let octets = received(object).filter(|_| part == "octets");
                if octets.is_some() && object.contains_key(EDITED) {
                    return Err(format!(
                        "the value was edited since these octets were received at {path}"
                    ));
                }
                if let Some(value) = object.get(&part).or(octets) {
                    next.push(value);
                } else {
                    return Err(format!(
                        "unknown or unavailable decoded field {part:?} at {path}"
                    ));
                }
            } else if let Some(array) = value.as_array() {
                if let Some(id) = ie_id(&part).filter(|_| named) {
                    let id = Some(u64::from(id));
                    next.extend(array.iter().filter(|v| v["id"].as_u64() == id));
                } else {
                    let index = part
                        .parse::<usize>()
                        .map_err(|_| format!("array index must be numeric at {path}"))?;
                    if let Some(value) = array.get(index) {
                        next.push(value);
                    }
                }
            } else if !value.is_null() {
                return Err(format!("decoded path traverses a scalar at {path}"));
            }
            if next.len() > OCCURRENCES {
                return Err(format!("IE selection exceeds {OCCURRENCES} occurrences"));
            }
        }
        selected = next;
    }
    Ok(selected)
}

/// Whether a member is one of a value, and not what a tree keeps of what was received.
fn shown(name: &str) -> bool {
    !name.starts_with('_') || name == "_decode_error"
}

/// The octets that an IE or a transfer was received as.
fn received(entry: &serde_json::Map<String, Value>) -> Option<&Value> {
    entry.get("_raw_value").or(entry.get("_raw_transfer"))
}

/// The member of an entry that holds the octets it was received as.
fn received_member(entry: &serde_json::Map<String, Value>) -> Option<&'static str> {
    ["_raw_value", "_raw_transfer"]
        .into_iter()
        .find(|member| entry.contains_key(*member))
}

/// Each value of `tree` with the path that selects it, in the order of the tree. A list
/// of plain values is one value. The IEs of the message go by name under the root, and
/// those of a message that the tree contains by name in its place; an IE goes by its
/// position when it has no name or its identifier is there twice. The members that start
/// with `_` repeat what was received and are left out, except `_decode_error`: the octets
/// of an IE are at its `octets`.
pub fn paths(tree: &Value) -> Vec<(String, Value)> {
    let mut found = Vec::new();
    walk(tree, &mut String::new(), &mut found);
    found
}

fn walk(value: &Value, path: &mut String, found: &mut Vec<(String, Value)>) {
    let step = |segment: &str, child: &Value, path: &mut String, found: &mut _| {
        let length = path.len();
        path.push('/');
        path.push_str(&segment.replace('~', "~0").replace('/', "~1"));
        walk(child, path, found);
        path.truncate(length);
    };
    match value {
        Value::Object(members) if !members.is_empty() => {
            for (name, member) in members {
                if !shown(name) {
                    continue;
                }
                let Some(ies) = member.as_array().filter(|_| name == IES) else {
                    step(name, member, path, found);
                    continue;
                };
                // The IEs of a message go by name: under the root for the message of the
                // PDU, and in the place of a message that the tree contains.
                let mut root = format!("/{ROOT}");
                let of_the_pdu = path.strip_prefix('/') == Some(MESSAGE);
                let message = match of_the_pdu {
                    true => &mut root,
                    false => &mut *path,
                };
                for (index, ie) in ies.iter().enumerate() {
                    step(&ie_segment(ies, index), ie, message, found);
                }
                // The message of the PDU says that it has no IE.
                if of_the_pdu && ies.is_empty() {
                    found.push((root, member.clone()));
                }
            }
        }
        Value::Array(entries) if entries.iter().any(|e| e.is_object() || e.is_array()) => {
            for (index, entry) in entries.iter().enumerate() {
                step(&index.to_string(), entry, path, found);
            }
        }
        plain => found.push((path.clone(), plain.clone())),
    }
}

/// The segment that selects the IE at `index` of `ies` alone: its name, or its position
/// when it has no name or its identifier is there twice.
fn ie_segment(ies: &[Value], index: usize) -> String {
    let id = ies[index]["id"].as_u64();
    let mut names = crate::inspect::ie_names().iter();
    match names.find(|(known, _)| Some(u64::from(*known)) == id) {
        Some((_, name)) if ies.iter().filter(|ie| ie["id"].as_u64() == id).count() == 1 => {
            name.to_string()
        }
        _ => index.to_string(),
    }
}

/// What an edit does at its path.
enum Edit {
    Set(Value),
    Remove,
    Insert(Value),
}

/// Give the member or the list entry at `path` the value `value`. `null` takes an
/// optional member out. An IE is written as `{id, criticality, value}` with its typed
/// value, or with its `octets` in hexadecimal in place of `value`; its `id` is a number,
/// or its name under the root. The `octets` of an IE or of a transfer are set in place of
/// its value.
///
/// A path that selects nothing is an error, and the tree is then as it was. So is an
/// edit that would send nothing else: the value of an IE or the decoded value of a
/// transfer is not taken out, as the octets received would go out in its place.
pub fn set(tree: &mut Value, path: &str, value: Value) -> Result<(), String> {
    edit(tree, path, Edit::Set(value))
}

/// Take the member or the list entries at `path` out of `tree`. A path that selects
/// nothing is an error.
pub fn remove(tree: &mut Value, path: &str) -> Result<(), String> {
    edit(tree, path, Edit::Remove)
}

/// Add `value` to a list of `tree`: before each entry that `path` selects, or at the end
/// for a path that ends with `-`. An IE is written as [`set`] takes it.
pub fn insert(tree: &mut Value, path: &str, value: Value) -> Result<(), String> {
    edit(tree, path, Edit::Insert(value))
}

fn edit(tree: &mut Value, path: &str, mut operation: Edit) -> Result<(), String> {
    let (parts, named) = resolve(path)?;
    if let (true, Edit::Set(ie) | Edit::Insert(ie)) = (named, &mut operation)
        && let Some(ie) = ie.as_object_mut()
    {
        if let Some(octets) = ie.remove("octets") {
            if ie.contains_key("value") {
                return Err("an IE is written with its value or its octets, not both".into());
            }
            ie.insert("_raw_value".into(), octets);
        }
        if let Some(name) = ie.get("id").and_then(Value::as_str) {
            let unknown = || format!("{name:?} is not an IE of {PROTOCOL}");
            ie.insert("id".into(), json!(ie_id(name).ok_or_else(unknown)?));
        }
    }
    let mut edited = tree.clone();
    if apply(&mut edited, &parts, &operation, named)? == 0 {
        return Err(format!("IE edit selected no field: {path}"));
    }
    *tree = edited;
    Ok(())
}

/// Mark `entry` when `member`, which an edit changed or changed something under, holds
/// the value of an IE or of a transfer that was received.
fn mark(entry: &mut serde_json::Map<String, Value>, member: &str) {
    if VALUES.contains(&member) && received_member(entry).is_some() {
        entry.insert(EDITED.into(), true.into());
    }
}

/// Apply `operation` at `parts` under `tree`; the number of places that it changed.
fn apply(
    tree: &mut Value,
    parts: &[String],
    operation: &Edit,
    named: bool,
) -> Result<usize, String> {
    let (part, rest) = parts.split_first().ok_or("empty edit path")?;
    // The IEs of a message that the tree contains are edited in its place.
    if named
        && tree.get(part).is_none()
        && selects_ies(part)
        && let Some(ies @ Value::Array(_)) = tree.get_mut(IES)
    {
        return apply(ies, parts, operation, named);
    }
    if let Some(object) = tree.as_object_mut() {
        if part == "*" || part.starts_with("@id=") {
            return Err("array selection used on an object".into());
        }
        if rest.is_empty() {
            // The octets of an IE or of a transfer are sent in place of its value.
            if let Some(member) = received_member(object).filter(|_| part == "octets") {
                let Edit::Set(octets @ Value::String(_)) = operation else {
                    return Err("octets are set, in hexadecimal".into());
                };
                object.insert(member.into(), octets.clone());
                for value in VALUES.into_iter().chain(["_decode_error", EDITED]) {
                    object.remove(value);
                }
                return Ok(1);
            }
            let removes = matches!(operation, Edit::Remove | Edit::Set(Value::Null));
            // What holds the value of an IE or of a transfer: without it the octets
            // received would be sent, and the edit would change nothing.
            let value = match part.as_str() {
                "value" | "extensionValue" => object.contains_key("id"),
                "decoded" => object.contains_key("_raw_transfer"),
                _ => false,
            };
            if removes && value {
                return Err(format!(
                    "{part} is not taken out: remove what has it, or set its octets"
                ));
            }
            return match operation {
                // Null takes a member out, as a removal does: one that is not there is
                // not selected.
                Edit::Remove | Edit::Set(Value::Null) => {
                    Ok(usize::from(object.remove(part).is_some()))
                }
                Edit::Set(value) => {
                    object.insert(part.clone(), value.clone());
                    mark(object, part);
                    Ok(1)
                }
                Edit::Insert(_) => Err("insert adds to a list".into()),
            };
        }
        let changed = match object.get_mut(part) {
            Some(child) => apply(child, rest, operation, named)?,
            None => 0,
        };
        if changed > 0 {
            mark(object, part);
        }
        return Ok(changed);
    }
    let Some(array) = tree.as_array_mut() else {
        return Ok(0);
    };
    if let (Edit::Insert(value), "-", []) = (operation, part.as_str(), rest) {
        array.push(value.clone());
        return Ok(1);
    }
    let id = match ie_id(part).filter(|_| named) {
        Some(id) => Some(u64::from(id)),
        None => part
            .strip_prefix("@id=")
            .map(|id| id.parse().map_err(|_| "invalid IE id"))
            .transpose()?,
    };
    let indexes: Vec<_> = if part == "*" {
        (0..array.len()).collect()
    } else if let Some(id) = id {
        let entries = array.iter().enumerate();
        entries
            .filter_map(|(index, value)| (value["id"].as_u64() == Some(id)).then_some(index))
            .collect()
    } else {
        vec![
            part.parse::<usize>()
                .map_err(|_| "array index must be numeric")?,
        ]
    };
    if indexes.len() > OCCURRENCES {
        return Err(format!(
            "IE edit selects more than {OCCURRENCES} occurrences"
        ));
    }
    let mut changed = 0;
    for index in indexes.into_iter().rev() {
        if index >= array.len() {
            continue;
        }
        if rest.is_empty() {
            match operation {
                Edit::Set(value) => array[index] = value.clone(),
                Edit::Remove => {
                    array.remove(index);
                }
                Edit::Insert(value) => array.insert(index, value.clone()),
            }
            changed += 1;
        } else {
            changed += apply(&mut array[index], rest, operation, named)?;
        }
    }
    Ok(changed)
}
