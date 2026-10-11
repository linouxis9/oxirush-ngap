//! The values of an inspection tree by their paths, and the IEs of a message by their
//! names: see the module documentation of [`crate::inspect`].

use serde_json::{Map, Value, json};

// The name of the protocol, the first segment of the paths of the IEs of the message of
// a PDU, and what the ASN.1 says of the messages, the IEs and the types.
use crate::inspect_registry::{CARRIED, ENUMERATIONS, IE_TYPES, MESSAGES, PROTOCOL, ROOT, TYPES};
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
/// How deep a message that is written by the names of its IEs is read.
const WRITTEN: usize = 256;

/// What a value of a tree is, as its ASN.1 type has it: what a path can select in it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Shape {
    /// A value that the tables do not describe: it has the members that the tree has.
    Unknown,
    /// The octets of a value that did not decode: nothing is selected in them.
    Undecoded,
    /// The tree of a PDU, of this message when the tables have it.
    Pdu(Option<Sent>),
    /// The message of a PDU.
    Message(Sent),
    /// A value without members.
    Plain,
    /// A string of bits whose size varies: its `value` and its `length`, unless the tree
    /// shows it as it is usually written.
    Bits,
    /// A SEQUENCE or a CHOICE, or a type that is a list or a string of bits, by the name
    /// that the bindings give it. A name that the list of the types does not have is
    /// that of a value without members: an ENUMERATED, which has its names.
    Named(&'static str),
    /// A list.
    List(&'static Shape),
    /// The IEs of a container: those of the object set of this message, or of a set
    /// that the tables do not have.
    Ies(Option<Sent>),
    /// The IEs of an extension container.
    Extensions,
    /// An IE of a container, of this identifier when the path says it.
    Ie(Option<u16>),
    /// An IE of an extension container.
    Extension(Option<u16>),
    /// Octets that contain a value of the type of this name, which the tree shows
    /// decoded beside them. No name: the octets have one of several types.
    Transfer(&'static str),
    /// The octets of the IE of this identifier, which may carry a type that the
    /// specification names and the ASN.1 does not. The tree shows them as they are, or
    /// as the transfer of that type under its name, once they are written or opened
    /// so.
    Carrier(u16),
}

/// A message, as the paths need it: the name that ASN.1 gives it, the name of its type
/// in the bindings, and the IEs of its object set.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Sent {
    name: &'static str,
    ty: &'static str,
    ies: &'static [(u16, bool)],
}

/// A member of a SEQUENCE or an alternative of a CHOICE: the name that ASN.1 gives it,
/// the shape of its value, and whether a value may be without it.
pub(crate) type Member = (&'static str, Shape, bool);

/// What a type of the list of the types is.
pub(crate) enum Kind {
    /// A SEQUENCE, with its members.
    Sequence(&'static [Member]),
    /// A CHOICE, with its alternatives.
    Choice(&'static [Member]),
    /// A type that is a list, a string of bits, or another type.
    Is(Shape),
}

/// The shape that the list of the types writes.
macro_rules! shape {
    (_) => {
        $crate::inspect_paths::Shape::Plain
    };
    (bits) => {
        $crate::inspect_paths::Shape::Bits
    };
    (ies) => {
        $crate::inspect_paths::Shape::Ies(None)
    };
    (ie) => {
        $crate::inspect_paths::Shape::Ie(None)
    };
    (extensions) => {
        $crate::inspect_paths::Shape::Extensions
    };
    (extension) => {
        $crate::inspect_paths::Shape::Extension(None)
    };
    ([$of:tt]) => {
        $crate::inspect_paths::Shape::List(&$crate::inspect_paths::shape!($of))
    };
    (()) => {
        $crate::inspect_paths::Shape::Transfer("")
    };
    (($contained:ident)) => {
        $crate::inspect_paths::Shape::Transfer(stringify!($contained))
    };
    ($name:ident) => {
        $crate::inspect_paths::Shape::Named(stringify!($name))
    };
}
pub(crate) use shape;

/// Whether the list of the types writes a member as one that may be absent.
macro_rules! optional {
    (:) => {
        false
    };
    (?) => {
        true
    };
}
pub(crate) use optional;

/// What the list of the types says a type is.
macro_rules! kind {
    (= $is:tt) => {
        $crate::inspect_paths::Kind::Is($crate::inspect_paths::shape!($is))
    };
    (sequence { $($member:literal $presence:tt $shape:tt),* }) => {
        $crate::inspect_paths::Kind::Sequence(&[$((
            $member,
            $crate::inspect_paths::shape!($shape),
            $crate::inspect_paths::optional!($presence),
        )),*])
    };
    (choice { $($member:literal $presence:tt $shape:tt),* }) => {
        $crate::inspect_paths::Kind::Choice(&[$((
            $member,
            $crate::inspect_paths::shape!($shape),
            $crate::inspect_paths::optional!($presence),
        )),*])
    };
}
pub(crate) use kind;

/// The types of the protocol, which `src/inspect_registry.rs` lists from the bindings in
/// the order of their names: each SEQUENCE with its members and each CHOICE with its
/// alternatives, by the names that ASN.1 gives them, and each type that is a list or a
/// string of bits. A member is written with `?` when a value may be without it, and
/// its value as `_` when it has no members, `bits` for a string of bits whose size
/// varies, the name of its type, a list in brackets, a container by its kind, or in
/// parentheses the type that the octets of a transfer contain.
macro_rules! types {
    ($($name:ident $kind:tt $body:tt;)*) => {
        pub(crate) const TYPES: &[(&str, $crate::inspect_paths::Kind)] = &[
            $((stringify!($name), $crate::inspect_paths::kind!($kind $body)),)*
        ];
    };
}
pub(crate) use types;

/// The letters and the digits of a name, in lower case.
fn letters(name: &str) -> impl Iterator<Item = u8> + '_ {
    let letters = name.bytes().filter(u8::is_ascii_alphanumeric);
    letters.map(|letter| letter.to_ascii_lowercase())
}

/// Whether two names are the same: they have the same letters and digits, whatever their
/// case and whatever is between them. A name without a letter or a digit is no name.
pub(crate) fn same_name(a: &str, b: &str) -> bool {
    letters(a).next().is_some() && letters(a).eq(letters(b))
}

/// The position that a segment is: a number as decimal writes it, without a sign and
/// without a zero before it.
fn position(part: &str) -> Option<usize> {
    let plain = !part.is_empty()
        && part.bytes().all(|digit| digit.is_ascii_digit())
        && (part == "0" || !part.starts_with('0'));
    part.parse().ok().filter(|_| plain)
}

/// The number of the IE that ASN.1 names `id-<name>`.
pub(crate) fn ie_id(name: &str) -> Option<u16> {
    let mut names = crate::inspect::ie_names().iter();
    names
        .find(|(_, known)| same_name(known, name))
        .map(|(id, _)| *id)
}

/// The identifier of an entry of a list of IEs: its number, or the name of its IE as an
/// edit may write it.
fn entry_id(entry: &Value) -> Option<u64> {
    match entry.get("id")? {
        Value::String(name) => ie_id(name).map(u64::from),
        id => id.as_u64(),
    }
}

/// The name that ASN.1 gives the IE of an identifier, or its number.
fn ie_name(id: u16) -> String {
    let mut names = crate::inspect::ie_names().iter();
    match names.find(|(known, _)| *known == id) {
        Some((_, name)) => name.to_string(),
        None => format!("@id={id}"),
    }
}

/// Whether a segment selects among the IEs of a message: a name, a position, `@id=N`,
/// `*`, or `-` for their end.
fn selects_ies(part: &str) -> bool {
    matches!(part, "*" | "-")
        || part.starts_with("@id=")
        || position(part).is_some()
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
    let rooted = same_name(&parts[0], ROOT);
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

/// The last segment of the path of a type in the bindings: its name.
fn type_name(path: &'static str) -> &'static str {
    path.rsplit(':').next().unwrap_or(path).trim()
}

/// The message of a direction and a procedure code, or of a name.
fn sent(of: impl Fn(&'static str, u8, &'static str) -> bool) -> Option<Sent> {
    let mut messages = MESSAGES.iter();
    let message = messages.find(|(direction, code, name, ..)| of(direction, *code, name));
    message.map(|(_, _, name, _, ies, ty)| Sent {
        name,
        ty: type_name(ty),
        ies,
    })
}

/// The shape of a tree: that of a PDU, of the message that its direction and its
/// procedure code say.
fn pdu(tree: &Value) -> Shape {
    let code = tree.get("procedure_code").and_then(Value::as_u64);
    let direction = tree.get("direction").and_then(Value::as_str);
    Shape::Pdu(sent(|of, procedure, _| {
        Some(of) == direction && Some(u64::from(procedure)) == code
    }))
}

/// What a type of the list of the types is; `None` for a name that the list does not
/// have, which is that of a value without members.
fn kind_of(name: &str) -> Option<&'static Kind> {
    let at = TYPES.binary_search_by(|(known, _)| (*known).cmp(name));
    at.ok().map(|at| &TYPES[at].1)
}

/// The names of the values of the ENUMERATED that the bindings name `name`.
fn enumeration(name: &str) -> Option<&'static [&'static str]> {
    let at = ENUMERATIONS.binary_search_by(|(known, _)| (*known).cmp(name));
    at.ok().map(|at| ENUMERATIONS[at].1)
}

/// `shape`, or what the type that it names is.
fn resolved(mut shape: Shape) -> Shape {
    // A type names another one a few times at most.
    for _ in 0..16 {
        shape = match shape {
            Shape::Named(name) => match kind_of(name) {
                Some(Kind::Is(is)) => *is,
                Some(_) => return shape,
                None => return Shape::Plain,
            },
            shape => return shape,
        };
    }
    Shape::Unknown
}

/// The shape of the value of the IE of an identifier.
fn ie_value(id: Option<u16>) -> Shape {
    let Some(id) = id else {
        return Shape::Unknown;
    };
    if CARRIED.iter().any(|(carrier, ..)| *carrier == id) {
        return Shape::Carrier(id);
    }
    match IE_TYPES.iter().find(|(known, ..)| *known == id) {
        Some((_, _, contained)) if !contained.is_empty() => Shape::Transfer(type_name(contained)),
        Some((_, ty, _)) => Shape::Named(type_name(ty)),
        // An identifier that is unknown, or that has several types.
        None => Shape::Unknown,
    }
}

/// The members that a value of `shape` can have, which is `entry` in a tree; `None` for
/// a shape whose members are those that the tree has.
fn members(shape: Shape, entry: Option<&Map<String, Value>>) -> Option<Vec<Member>> {
    let has = |member: &str| entry.is_some_and(|entry| entry.contains_key(member));
    // The entry of an IE in a tree says which IE its value was decoded as.
    let ie = |id: Option<u16>, value: &'static str, raw: &'static str| {
        let of = |member: &str| {
            let id = entry
                .and_then(|entry| entry.get(member))
                .and_then(Value::as_u64);
            id.and_then(|id| u16::try_from(id).ok())
        };
        let decoded = match has("_decode_error") {
            true => Shape::Undecoded,
            false => ie_value(of("_original_id").or(of("id")).or(id)),
        };
        vec![
            ("id", Shape::Plain, false),
            ("criticality", Shape::Plain, false),
            (value, decoded, true),
            (raw, Shape::Plain, true),
            ("_original_id", Shape::Plain, true),
            ("_ie_name", Shape::Plain, true),
            ("_decode_error", Shape::Plain, true),
            (EDITED, Shape::Plain, true),
        ]
    };
    Some(match shape {
        Shape::Unknown | Shape::Undecoded => return None,
        Shape::Pdu(message) => vec![
            ("procedure_code", Shape::Plain, false),
            ("direction", Shape::Plain, false),
            ("criticality", Shape::Plain, false),
            (
                MESSAGE,
                message.map_or(Shape::Unknown, Shape::Message),
                false,
            ),
            ("_raw_message", Shape::Plain, true),
        ],
        // The IEs of the message of a PDU are those of its object set.
        Shape::Message(message) => {
            let mut members = members(Shape::Named(message.ty), entry)?;
            for (name, shape, _) in &mut members {
                if *name == IES {
                    *shape = Shape::Ies(Some(message));
                }
            }
            members
        }
        Shape::Named(name) => match kind_of(name) {
            Some(Kind::Sequence(members) | Kind::Choice(members)) => members.to_vec(),
            Some(Kind::Is(is)) => return self::members(resolved(*is), entry),
            None => Vec::new(),
        },
        Shape::Bits => vec![
            ("value", Shape::Plain, false),
            ("length", Shape::Plain, false),
        ],
        Shape::Ie(id) => ie(id, "value", "_raw_value"),
        Shape::Extension(id) => ie(id, "extensionValue", "_raw_value"),
        Shape::Transfer(contained) => {
            let decoded = match contained.is_empty() {
                true => Shape::Unknown,
                false => Shape::Named(contained),
            };
            vec![
                ("decoded", decoded, true),
                ("_raw_transfer", Shape::Plain, true),
                ("_decode_error", Shape::Plain, true),
                (EDITED, Shape::Plain, true),
            ]
        }
        // Each type that the octets may carry, by the name that ASN.1 gives it.
        Shape::Carrier(id) => {
            let carried = CARRIED.iter().filter(|(carrier, ..)| *carrier == id);
            carried
                .map(|(_, name, ty)| (*name, Shape::Transfer(type_name(ty)), true))
                .collect()
        }
        Shape::Plain | Shape::List(_) | Shape::Ies(_) | Shape::Extensions => Vec::new(),
    })
}

/// The member of `members` that a segment names. A member that starts with `_` is named
/// as it is.
fn member<'m>(members: &'m [Member], part: &str) -> Option<&'m Member> {
    let exact = members.iter().find(|(name, ..)| *name == part);
    exact.or_else(|| {
        let mut named = members.iter().filter(|(name, ..)| !name.starts_with('_'));
        named.find(|(name, ..)| same_name(name, part))
    })
}

/// The member of the entry of a tree that a segment names, where no table says what
/// the entry has.
fn key<'e>(entry: &'e Map<String, Value>, part: &str) -> Option<&'e str> {
    let exact = entry.get_key_value(part).map(|(name, _)| name);
    let named = || {
        let mut names = entry.keys().filter(|name| !name.starts_with('_'));
        names.find(|name| same_name(name, part))
    };
    exact.or_else(named).map(String::as_str)
}

/// Whether a value of `shape` is an IE or a transfer: it has `octets`.
fn has_octets(shape: Shape) -> bool {
    matches!(
        shape,
        Shape::Ie(_) | Shape::Extension(_) | Shape::Transfer(_)
    )
}

/// The member that holds the value of an IE or of a transfer of `shape`, which a path may
/// leave out: a segment that names no member of the IE or of the transfer is taken in
/// that value.
fn value_member(shape: Shape) -> Option<&'static str> {
    match shape {
        Shape::Ie(_) => Some("value"),
        Shape::Extension(_) => Some("extensionValue"),
        Shape::Transfer(_) => Some("decoded"),
        _ => None,
    }
}

/// The members of a value of `shape` that a path names: those that its type has, and the
/// `octets` of an IE or of a transfer.
fn names(shape: Shape, members: &[Member]) -> Vec<&'static str> {
    let mut names: Vec<_> = (members.iter().map(|(name, ..)| *name))
        .filter(|name| shown(name) && *name != "_decode_error")
        .collect();
    if has_octets(shape) {
        names.push("octets");
    }
    names
}

/// `reason`, why a segment selects nothing in the value of an IE or of a transfer of
/// `shape`, with the members that the IE or the transfer has itself: the segment may be
/// one of them, written wrong.
fn or_its_own(reason: String, shape: Shape, members: &[Member]) -> String {
    let of = match shape {
        Shape::Transfer(_) => "a transfer",
        _ => "an IE",
    };
    let own = names(shape, members).join(", ");
    format!("{reason}; {of} itself has {own}")
}

/// Why a segment selects nothing in a value of `shape`: the type has no such member.
fn no_member(shape: Shape, members: &[Member], part: &str, path: &str) -> String {
    // The root stands for the IEs of a message, which a private message has none of.
    if let (Shape::Message(message), IES) = (shape, part) {
        let has: Vec<_> = members.iter().map(|(name, ..)| *name).collect();
        return format!(
            "a message {} has no IEs under /{ROOT}: it has {} at {path}",
            message.name,
            has.join(", ")
        );
    }
    let of = match shape {
        Shape::Named(name) => name.to_string(),
        Shape::Message(message) => message.name.to_string(),
        Shape::Pdu(_) => "a tree".to_string(),
        Shape::Bits => "a string of bits".to_string(),
        Shape::Transfer(_) => "a transfer".to_string(),
        Shape::Ie(_) | Shape::Extension(_) => "an IE".to_string(),
        _ => "this value".to_string(),
    };
    let names = names(shape, members);
    match names.is_empty() {
        true => format!("{part:?} is not a member of {of}, which has none, at {path}"),
        false => format!(
            "{part:?} is not a member of {of}, which has {}, at {path}",
            names.join(", ")
        ),
    }
}

/// The shape of an entry of a list of `shape`.
fn element(shape: Shape) -> Shape {
    match shape {
        Shape::Ies(_) | Shape::List(Shape::Ie(_)) => Shape::Ie(None),
        Shape::Extensions | Shape::List(Shape::Extension(_)) => Shape::Extension(None),
        Shape::List(of) => *of,
        _ => Shape::Unknown,
    }
}

/// The shape of the IE of an identifier in a list of `shape`; `None` for a list whose
/// entries are no IEs.
fn element_of(shape: Shape, id: u16) -> Option<Shape> {
    match element(shape) {
        Shape::Ie(_) => Some(Shape::Ie(Some(id))),
        Shape::Extension(_) => Some(Shape::Extension(Some(id))),
        Shape::Unknown => Some(Shape::Unknown),
        _ => None,
    }
}

/// A place on the way of a path: a value of the tree, or none where the tree has no
/// value and the type could have one, and the shape of a value there.
type Place<'a> = (Option<&'a Value>, Shape);

/// Why a segment selects nothing at a place: `true` when the value there cannot be read,
/// which no other place makes up for.
type Refusal = (bool, String);

/// Add to `next` what `part` selects at `place`. A member that the type can have and the
/// value does not have is a place without a value.
fn advance<'a>(
    place: Place<'a>,
    part: &str,
    named: bool,
    path: &str,
    next: &mut Vec<Place<'a>>,
) -> Result<(), Refusal> {
    let (value, shape) = (place.0, resolved(place.1));
    let mut push = |value: Option<&'a Value>, shape: Shape| {
        // The places without a value are those of the types: each one once.
        let known = |(other, of): &Place| other.is_none() && *of == shape;
        if value.is_some() || !next.iter().any(known) {
            next.push((value, shape));
        }
    };
    let no = |reason: String| Err((false, reason));
    match value {
        Some(Value::Object(entry)) => {
            let members = members(shape, Some(entry));
            // The IEs of a message that the tree contains are selected in its place.
            if named
                && selects_ies(part)
                && !entry.contains_key(part)
                && let Some(ies @ Value::Array(_)) = entry.get(IES)
            {
                let of = members.as_deref().and_then(|members| member(members, IES));
                let shape = of.map_or(Shape::Ies(None), |(_, shape, _)| *shape);
                return advance((Some(ies), shape), part, named, path, next);
            }
            if part == "*" {
                for (name, value) in entry.iter().filter(|(name, _)| shown(name)) {
                    let known = members.as_deref().and_then(|members| member(members, name));
                    push(
                        Some(value),
                        known.map_or(Shape::Unknown, |(_, shape, _)| *shape),
                    );
                }
                return Ok(());
            }
            // The value of an IE or of a transfer, in which a segment that names none of
            // its members is taken, as if the member that holds it stood before.
            let held = value_member(shape);
            if part.starts_with("@id=") && held.is_none() {
                return no(format!("IE-id selection requires an array at {path}"));
            }
            // `octets` is what an IE or a transfer was received as.
            if same_name(part, "octets") && (has_octets(shape) || received(entry).is_some()) {
                if entry.contains_key(EDITED) {
                    return Err((
                        true,
                        format!("the value was edited since these octets were received at {path}"),
                    ));
                }
                push(received(entry), Shape::Plain);
                return Ok(());
            }
            let Some(members) = members else {
                // No table says what the value can have: it has what the tree has.
                return match key(entry, part) {
                    Some(name) => {
                        push(entry.get(name), Shape::Unknown);
                        Ok(())
                    }
                    None => no(format!(
                        "unknown or unavailable decoded field {part:?} at {path}"
                    )),
                };
            };
            let own = member(&members, part);
            let held = held.and_then(|held| member(&members, held));
            let Some((name, of, _)) = own.or(held) else {
                return no(no_member(shape, &members, part, path));
            };
            let value = match entry.get(*name) {
                None if *name == "decoded" && entry.contains_key("_decode_error") => {
                    let error = entry["_decode_error"].as_str().unwrap_or_default();
                    return Err((
                        true,
                        format!("the transfer did not decode at {path}: {error}"),
                    ));
                }
                value => value,
            };
            if own.is_some() {
                push(value, *of);
                return Ok(());
            }
            let taken = advance((value, *of), part, named, path, next);
            taken.map_err(|(unread, reason)| match unread {
                true => (unread, reason),
                false => (unread, or_its_own(reason, shape, &members)),
            })
        }
        Some(Value::Array(entries)) => {
            let id = match part.strip_prefix("@id=") {
                Some(id) => Some(
                    id.parse::<u16>()
                        .map_err(|_| (false, "invalid IE id".into()))?,
                ),
                None => ie_id(part).filter(|_| named),
            };
            if part == "*" {
                for entry in entries {
                    push(Some(entry), element(shape));
                }
            } else if let Some(id) = id {
                let Some(absent) = element_of(shape, id) else {
                    return no(format!(
                        "{part:?} selects an IE, and the entries of this list go by position at {path}"
                    ));
                };
                let mut found = false;
                for entry in entries {
                    if entry_id(entry) == Some(u64::from(id)) {
                        push(Some(entry), element(shape));
                        found = true;
                    }
                }
                // An IE that is not there: what follows is still one of its paths.
                if !found {
                    push(None, absent);
                }
            } else if let Some(index) = position(part) {
                if let Some(entry) = entries.get(index) {
                    push(Some(entry), element(shape));
                }
            } else {
                return no(format!("array index must be numeric at {path}"));
            }
            Ok(())
        }
        // A value that is not there, by what its type says.
        Some(Value::Null) | None => typed(shape, part, named, path, &mut |shape| push(None, shape)),
        Some(value) => match shape {
            Shape::Undecoded => Err((true, format!("the IE did not decode at {path}"))),
            // A value without members has none to select.
            _ if part == "*" => Ok(()),
            // A string of bits that the tree shows as it is usually written.
            Shape::Bits if ["value", "length"].contains(&part) => Ok(()),
            // A transfer that an edit replaced by its octets.
            Shape::Transfer(_) if same_name(part, "octets") => {
                push(Some(value), Shape::Plain);
                Ok(())
            }
            Shape::Transfer(_) if same_name(part, "decoded") => Ok(()),
            // What is no member of a transfer is one of the decoded value, which a
            // transfer that is its octets has not.
            Shape::Transfer(contained) if !contained.is_empty() => {
                let decoded = Shape::Named(contained);
                let taken = typed(decoded, part, named, path, &mut |shape| push(None, shape));
                let own = members(shape, None).unwrap_or_default();
                taken.map_err(|(unread, reason)| (unread, or_its_own(reason, shape, &own)))
            }
            // Octets that may carry a type are that type once they are opened.
            Shape::Carrier(_) => {
                let carried = members(shape, None).unwrap_or_default();
                match member(&carried, part) {
                    Some((name, ..)) => Err((
                        true,
                        format!(
                            "the octets are not shown as a {name} at {path}: open decodes them"
                        ),
                    )),
                    None => no(no_member(shape, &carried, part, path)),
                }
            }
            Shape::Plain => no(no_member(shape, &[], part, path)),
            _ => no(format!("decoded path traverses a scalar at {path}")),
        },
    }
}

/// Give `push` what `part` selects in a value of `shape` that a tree does not have: the
/// places of the values that it could have.
fn typed(
    shape: Shape,
    part: &str,
    named: bool,
    path: &str,
    push: &mut dyn FnMut(Shape),
) -> Result<(), Refusal> {
    let no = |reason: String| Err((false, reason));
    match shape {
        Shape::Unknown | Shape::Undecoded => push(Shape::Unknown),
        Shape::List(_) | Shape::Ies(_) | Shape::Extensions => {
            let id = match part.strip_prefix("@id=") {
                Some(id) => Some(
                    id.parse::<u16>()
                        .map_err(|_| (false, "invalid IE id".into()))?,
                ),
                None => ie_id(part).filter(|_| named),
            };
            if let Some(id) = id {
                let Some(of) = element_of(shape, id) else {
                    return no(format!(
                        "{part:?} selects an IE, and the entries of this list go by position at {path}"
                    ));
                };
                // A name is that of an IE of the object set, where the tables have it.
                if let Shape::Ies(Some(message)) = shape
                    && !part.starts_with("@id=")
                    && !message.ies.iter().any(|(known, _)| *known == id)
                {
                    let ies: Vec<_> = message.ies.iter().map(|(id, _)| ie_name(*id)).collect();
                    return no(format!(
                        "{part:?} is not an IE of {}, which has {}, at {path}",
                        message.name,
                        ies.join(", ")
                    ));
                }
                push(of);
            } else if matches!(part, "*" | "-") || position(part).is_some() {
                match shape {
                    Shape::Ies(Some(message)) => {
                        for (id, _) in message.ies {
                            push(Shape::Ie(Some(*id)));
                        }
                    }
                    shape => push(element(shape)),
                }
            } else {
                return no(format!("array index must be numeric at {path}"));
            }
        }
        shape => {
            let members = members(shape, None).unwrap_or_default();
            // The IEs of a message that a value contains go in its place.
            if named
                && selects_ies(part)
                && member(&members, part).is_none()
                && let Some((_, ies, _)) = member(&members, IES)
            {
                return typed(*ies, part, named, path, push);
            }
            if part == "*" {
                for (_, of, _) in members.iter().filter(|(name, ..)| shown(name)) {
                    push(*of);
                }
            } else if same_name(part, "octets") && has_octets(shape) {
                push(Shape::Plain);
            } else if let Some((_, of, _)) = member(&members, part) {
                push(*of);
            } else if let Some((_, of, _)) =
                value_member(shape).and_then(|held| member(&members, held))
            {
                // What names no member of an IE or of a transfer is one of its value.
                let taken = typed(resolved(*of), part, named, path, push);
                return taken
                    .map_err(|(unread, reason)| (unread, or_its_own(reason, shape, &members)));
            } else {
                return no(no_member(shape, &members, part, path));
            }
        }
    }
    Ok(())
}

/// What `part` selects at each of `places`. A segment that selects nothing because a
/// type has no such member is an error when no place has it.
fn advance_all<'a>(
    places: Vec<Place<'a>>,
    part: &str,
    named: bool,
    path: &str,
) -> Result<Vec<Place<'a>>, String> {
    let mut next = Vec::new();
    let mut refusal = None;
    for place in places {
        match advance(place, part, named, path, &mut next) {
            Ok(()) => {}
            Err((true, reason)) => return Err(reason),
            Err((false, reason)) => refusal = refusal.or(Some(reason)),
        }
        if next.len() > OCCURRENCES {
            return Err(format!("IE selection exceeds {OCCURRENCES} occurrences"));
        }
    }
    match refusal {
        Some(reason) if next.is_empty() => Err(reason),
        _ => Ok(next),
    }
}

/// The values at `path` in `tree`, in the order of the tree.
///
/// An IE that the message does not have selects nothing, and so does a member that its
/// type can have and the value does not have: an OPTIONAL member that is absent, or
/// another alternative of a CHOICE. A name that the type of the value cannot have is an
/// error, which says the members that it has. `*` is each entry of a list or each member
/// of a value, without those that start with `_`; after it, a segment is an error when
/// none of the values can have it.
///
/// A segment that names no member of an IE is taken in its value, and one that names no
/// member of a transfer in its decoded value, as if `value` or `decoded` stood before
/// it. A name that neither has is an error, which says the members of both.
///
/// The `octets` of an IE or of a transfer are what it was received as: those of one whose
/// value [`set`], [`remove`] or [`insert`] edited since are an error, as
/// [`encode_pdu`](crate::inspect::encode_pdu) has yet to give it others. So is a path into
/// a value that did not decode.
pub fn select<'a>(tree: &'a Value, path: &str) -> Result<Vec<&'a Value>, String> {
    let (parts, named) = resolve(path)?;
    let mut places = vec![(Some(tree), pdu(tree))];
    for part in &parts {
        places = advance_all(places, part, named, path)?;
    }
    Ok(places.into_iter().filter_map(|(value, _)| value).collect())
}

/// Whether `path` can select a value in a message that ASN.1 names `message`: what
/// [`select`] says of a path without a tree to select in.
///
/// The name is taken as [`message_named`](crate::inspect::message_named) takes it. Under
/// the root, the name of an IE has to be that of an IE of the object set of the message;
/// under an IE, each segment has to be a member that the type of the value has, through
/// the lists, the alternatives of a CHOICE and the decoded value of a transfer. A position
/// or `*` stands for each entry that a list could have, and `-` for the end of a list.
///
/// What is not known is not refused: the value of an IE selected by `@id=N` when the
/// identifier is unknown or has several types, the entries of a container that is not
/// that of the message when they are selected by position, a segment under such an IE,
/// which may name a member of its value, and whether a position is within the bounds of
/// its list.
pub fn check_path(message: &str, path: &str) -> Result<(), String> {
    let known = sent(|_, _, name| same_name(name, message));
    let message = known.ok_or_else(|| format!("{message:?} is not a message of {PROTOCOL}"))?;
    let (parts, named) = resolve(path)?;
    let mut places: Vec<Place> = vec![(None, Shape::Pdu(Some(message)))];
    for part in &parts {
        places = advance_all(places, part, named, path)?;
        if places.is_empty() {
            return Err(format!(
                "{path} selects nothing in a message {}: a value before {part:?} has no members",
                message.name
            ));
        }
    }
    Ok(())
}

/// The names that the value at `path` can have in a message that ASN.1 names `message`,
/// when its type is an ENUMERATED: the names of its values as the specification spells
/// them, in the order of its definition. A name is compared as [`select`] compares one,
/// whatever its case and whatever is between its letters. A name that is not one of them
/// is never the value of a message, so a caller can refuse it before it compares one.
///
/// `None` for a value of another type, and for one whose type is not known: what
/// [`check_path`] does not refuse for that reason. A path that it refuses is an error
/// here too.
pub fn enumerated_at(message: &str, path: &str) -> Result<Option<&'static [&'static str]>, String> {
    let known = sent(|_, _, name| same_name(name, message));
    let message = known.ok_or_else(|| format!("{message:?} is not a message of {PROTOCOL}"))?;
    let (parts, named) = resolve(path)?;
    let mut places: Vec<Place> = vec![(None, Shape::Pdu(Some(message)))];
    for part in &parts {
        places = advance_all(places, part, named, path)?;
    }
    // The value is of one type, wherever the path finds it.
    let mut names = None;
    for (_, mut shape) in places {
        // A type that is another one names it a few times at most.
        for _ in 0..16 {
            match shape {
                Shape::Named(name) => match kind_of(name) {
                    Some(Kind::Is(is)) => shape = *is,
                    _ => break,
                },
                _ => break,
            }
        }
        let Shape::Named(name) = shape else {
            return Ok(None);
        };
        match (enumeration(name), names) {
            (Some(of), None) => names = Some(of),
            (Some(of), Some(other)) if of == other => {}
            _ => return Ok(None),
        }
    }
    Ok(names)
}

/// Whether a member is one of a value, and not what a tree keeps of what was received.
fn shown(name: &str) -> bool {
    !name.starts_with('_') || name == "_decode_error"
}

/// The octets that an IE or a transfer was received as.
fn received(entry: &Map<String, Value>) -> Option<&Value> {
    entry.get("_raw_value").or(entry.get("_raw_transfer"))
}

/// The member of an entry that holds the octets it was received as.
fn received_member(entry: &Map<String, Value>) -> Option<&'static str> {
    ["_raw_value", "_raw_transfer"]
        .into_iter()
        .find(|member| entry.contains_key(*member))
}

/// Each value of `tree` with the path that selects it, in the order of the tree. A list
/// of plain values is one value. The IEs of the message go by name under the root, and
/// those of a message that the tree contains by name in its place; an IE goes by its
/// position when it has no name or its identifier is there twice.
///
/// The members that start with `_` repeat what was received and are left out, except
/// `_decode_error`. The `octets` of an IE or of a transfer are listed when it has no
/// value beside them: those of a value repeat it, and are selected all the same.
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
            // An IE or a transfer that has its octets alone.
            if let Some(octets) = received(members)
                && !VALUES.iter().any(|value| members.contains_key(*value))
            {
                step("octets", octets, path, found);
            }
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
    let id = entry_id(&ies[index]);
    let mut names = crate::inspect::ie_names().iter();
    match names.find(|(known, _)| Some(u64::from(*known)) == id) {
        Some((_, name)) if ies.iter().filter(|ie| entry_id(ie) == id).count() == 1 => {
            name.to_string()
        }
        _ => index.to_string(),
    }
}

/// What an edit does at its path.
enum Edit<'d> {
    Set(Value),
    Remove,
    Insert(Value),
    /// Nothing, but for the octets that the path takes as a type that they may carry:
    /// this function makes of them, and of the name of that type, what a tree shows.
    Open(&'d dyn Fn(Value, &str) -> Value),
}

/// Give the member or the list entry at `path` the value `value`. `null` takes an
/// optional member out. An IE is written as `{id, criticality, value}` with its typed
/// value, or with its `octets` in hexadecimal in place of `value`; its `id` is a number
/// or the name of the IE. The `octets` of an IE or of a transfer are set in place of
/// its value. An alternative of a CHOICE takes the place of the one that is there.
///
/// A path that selects nothing is an error, and the tree is then as it was. So is a
/// name that the type of a value cannot have, a member that its type always has and
/// that would be taken out, and an edit that would send nothing else: the value of an IE
/// or the decoded value of a transfer is not taken out, as the octets received would go
/// out in its place.
pub fn set(tree: &mut Value, path: &str, value: Value) -> Result<(), String> {
    edit(tree, path, Edit::Set(value))
}

/// Take the member or the list entries at `path` out of `tree`. A path that selects
/// nothing is an error, and so is a member that its type always has.
pub fn remove(tree: &mut Value, path: &str) -> Result<(), String> {
    edit(tree, path, Edit::Remove)
}

/// Add `value` to a list of `tree`: before each entry that `path` selects, or at the end
/// for a path that ends with `-`. An IE is written as [`set`] takes it.
pub fn insert(tree: &mut Value, path: &str, value: Value) -> Result<(), String> {
    edit(tree, path, Edit::Insert(value))
}

/// Show, in `tree`, the octets that `path` takes as a type that they may carry as what
/// `decode` makes of them and of the name of that type: see
/// [`open`](crate::inspect::open), which gives the function that decodes.
pub(crate) fn open_with(
    tree: &mut Value,
    path: &str,
    decode: &dyn Fn(Value, &str) -> Value,
) -> Result<(), String> {
    let (parts, named) = resolve(path)?;
    let mut opened = tree.clone();
    let shape = pdu(&opened);
    apply(&mut opened, shape, &parts, &Edit::Open(decode), named, path)?;
    *tree = opened;
    Ok(())
}

/// Write the entry of an IE as a tree has it: the name of the IE that its `id` may be
/// written as becomes its number, and its `octets` what it is sent as in place of a
/// value. An entry of a list of IEs has an `id`, which no member of an ASN.1 type is
/// named beside a string or a number.
pub(crate) fn written_ie(entry: &mut Map<String, Value>) -> Result<(), String> {
    let id = match entry.get("id") {
        Some(Value::String(name)) => {
            let unknown = || format!("{name:?} is not an IE of {PROTOCOL}");
            Some(json!(ie_id(name).ok_or_else(unknown)?))
        }
        Some(Value::Number(_)) => None,
        _ => return Ok(()),
    };
    if let Some(id) = id {
        entry.insert("id".into(), id);
    }
    if let Some(octets) = entry.remove("octets") {
        if VALUES.iter().any(|value| entry.contains_key(*value)) {
            return Err("an IE is written with its value or its octets, not both".into());
        }
        entry.insert("_raw_value".into(), octets);
    }
    Ok(())
}

fn edit(tree: &mut Value, path: &str, mut operation: Edit) -> Result<(), String> {
    let (parts, named) = resolve(path)?;
    if let Edit::Set(ie) | Edit::Insert(ie) = &mut operation
        && let Some(ie) = ie.as_object_mut()
    {
        written_ie(ie)?;
    }
    let mut edited = tree.clone();
    let shape = pdu(&edited);
    if apply(&mut edited, shape, &parts, &operation, named, path)? == 0 {
        return Err(format!("IE edit selected no field: {path}"));
    }
    *tree = edited;
    Ok(())
}

/// Mark `entry` when `member`, which an edit changed or changed something under, holds
/// the value of an IE or of a transfer that was received.
fn mark(entry: &mut Map<String, Value>, member: &str) {
    if VALUES.contains(&member) && received_member(entry).is_some() {
        entry.insert(EDITED.into(), true.into());
    }
}

/// Apply `operation` at `parts` under `tree`, a value of `shape`; the number of places
/// that it changed.
fn apply(
    tree: &mut Value,
    shape: Shape,
    parts: &[String],
    operation: &Edit,
    named: bool,
    path: &str,
) -> Result<usize, String> {
    let (part, rest) = parts.split_first().ok_or("empty edit path")?;
    let shape = resolved(shape);
    // Octets that the path takes as a type that they may carry are shown as it, when
    // that is what is asked.
    if let (Shape::Carrier(_), Edit::Open(decode)) = (shape, operation)
        && tree.is_string()
        && let Some(carried) = members(shape, None)
        && let Some((name, ..)) = member(&carried, part)
    {
        *tree = json!({*name: decode(tree.take(), name)});
    }
    let members = members(shape, tree.as_object());
    // The IEs of a message that the tree contains are edited in its place.
    if named
        && tree.get(part).is_none()
        && selects_ies(part)
        && let Some(ies @ Value::Array(_)) = tree.get_mut(IES)
    {
        let of = members.as_deref().and_then(|members| member(members, IES));
        let shape = of.map_or(Shape::Ies(None), |(_, shape, _)| *shape);
        return apply(ies, shape, parts, operation, named, path);
    }
    if let Some(object) = tree.as_object_mut() {
        // What names no member of an IE or of a transfer is edited in its value, as if
        // the member that holds it stood before.
        if let Some(members) = &members
            && part != "*"
            && !same_name(part, "octets")
            && member(members, part).is_none()
            && let Some((name, of, _)) = value_member(shape).and_then(|held| member(members, held))
        {
            let changed = match object.get_mut(*name) {
                Some(value) => apply(value, *of, parts, operation, named, path),
                None => Ok(0),
            };
            if matches!(changed, Ok(1..)) {
                mark(object, name);
                return changed;
            }
            // An edit that changes nothing says what the value cannot have, as a
            // selection does: the segment may be a member of the IE or of the transfer,
            // written wrong.
            let place = (object.get(*name), *of);
            return match advance(place, part, named, path, &mut Vec::new()) {
                Err((false, reason)) => Err(or_its_own(reason, shape, members)),
                Err((true, reason)) => Err(reason),
                Ok(()) => changed,
            };
        }
        if part == "*" || part.starts_with("@id=") {
            return Err("array selection used on an object".into());
        }
        // The octets of an IE or of a transfer are sent in place of its value.
        let raw = received_member(object).or(match shape {
            Shape::Ie(_) | Shape::Extension(_) => Some("_raw_value"),
            Shape::Transfer(_) => Some("_raw_transfer"),
            _ => None,
        });
        if let Some(raw) = raw.filter(|_| rest.is_empty() && same_name(part, "octets")) {
            let octets = match operation {
                Edit::Set(octets @ Value::String(_)) => octets,
                Edit::Open(_) => return Ok(0),
                _ => return Err("octets are set, in hexadecimal".into()),
            };
            object.insert(raw.into(), octets.clone());
            for value in VALUES.into_iter().chain(["_decode_error", EDITED]) {
                object.remove(value);
            }
            return Ok(1);
        }
        // The member as its type names it, and what the type says of it.
        let (name, of, optional) = match &members {
            Some(members) => match member(members, part) {
                Some((name, of, optional)) => (name.to_string(), *of, *optional),
                None => return Err(no_member(shape, members, part, path)),
            },
            None => (
                key(object, part).unwrap_or(part).to_string(),
                Shape::Unknown,
                true,
            ),
        };
        let choice =
            matches!(shape, Shape::Named(name) if matches!(kind_of(name), Some(Kind::Choice(_))));
        if rest.is_empty() {
            let removes = matches!(operation, Edit::Remove | Edit::Set(Value::Null));
            // What holds the value of an IE or of a transfer: without it the octets
            // received would be sent, and the edit would change nothing.
            let value = match name.as_str() {
                "value" | "extensionValue" => object.contains_key("id"),
                "decoded" => object.contains_key("_raw_transfer"),
                _ => false,
            };
            if removes && value {
                return Err(format!(
                    "{name} is not taken out: remove what has it, or set its octets"
                ));
            }
            if removes && choice {
                return Err(format!(
                    "{name} is not taken out: a CHOICE has one alternative, set another in its place"
                ));
            }
            if removes && !optional {
                return Err(format!("{name} is not taken out: its type always has it"));
            }
            return match operation {
                // Null takes a member out, as a removal does: one that is not there is
                // not selected.
                Edit::Remove | Edit::Set(Value::Null) => {
                    Ok(usize::from(object.remove(&name).is_some()))
                }
                Edit::Set(value) => {
                    // A CHOICE has one alternative.
                    if choice {
                        object.clear();
                    }
                    object.insert(name.clone(), value.clone());
                    mark(object, &name);
                    Ok(1)
                }
                Edit::Insert(_) => Err("insert adds to a list".into()),
                Edit::Open(_) => Ok(0),
            };
        }
        let changed = match object.get_mut(&name) {
            Some(child) => apply(child, of, rest, operation, named, path)?,
            None => 0,
        };
        if changed > 0 {
            mark(object, &name);
        }
        return Ok(changed);
    }
    let Some(array) = tree.as_array_mut() else {
        // Octets that were not opened say so, as they do where they are selected.
        let unopened = match shape {
            Shape::Carrier(_) => advance((Some(tree), shape), part, named, path, &mut Vec::new()),
            _ => Ok(()),
        };
        return match unopened {
            Err((true, reason)) => Err(reason),
            _ => Ok(0),
        };
    };
    if let (Edit::Insert(value), "-", []) = (operation, part.as_str(), rest) {
        array.push(value.clone());
        return Ok(1);
    }
    let id = match ie_id(part).filter(|_| named) {
        Some(id) => Some(id),
        None => part
            .strip_prefix("@id=")
            .map(|id| id.parse().map_err(|_| "invalid IE id"))
            .transpose()?,
    };
    if let Some(id) = id
        && element_of(shape, id).is_none()
    {
        return Err(format!(
            "{part:?} selects an IE, and the entries of this list go by position at {path}"
        ));
    }
    let indexes: Vec<_> = if part == "*" {
        (0..array.len()).collect()
    } else if let Some(id) = id {
        let entries = array.iter().enumerate();
        entries
            .filter_map(|(index, value)| (entry_id(value) == Some(u64::from(id))).then_some(index))
            .collect()
    } else {
        vec![position(part).ok_or("array index must be numeric")?]
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
                Edit::Open(_) => continue,
            }
            changed += 1;
        } else {
            changed += apply(
                &mut array[index],
                element(shape),
                rest,
                operation,
                named,
                path,
            )?;
        }
    }
    Ok(changed)
}

/// Whether a transfer has a member of this name in a tree: its `decoded` value, its
/// `octets`, or what a tree keeps of what was received. Where a transfer is written, an
/// object that has none of them is its decoded value.
pub(crate) fn of_a_transfer(name: &str) -> bool {
    name.starts_with('_') || ["decoded", "octets"].contains(&name)
}

/// The criticality that ASN.1 assigns to the IE `id` in a container of `shape`, where the
/// tables have it: that of the object set of a message for its IEs, and that of the
/// single containers for an IE that a value holds alone or as an entry of a list. They
/// have none for the IEs of a message that a transfer contains, nor for those of an
/// extension container.
fn assigned(container: Shape, id: u16) -> Option<&'static str> {
    match container {
        Shape::Ies(Some(message)) => {
            let ies = crate::inspect::message_ie_criticalities(message.name)?;
            let ie = ies.iter().find(|(known, _)| *known == id);
            ie.map(|(_, criticality)| *criticality)
        }
        Shape::Ie(_) | Shape::List(Shape::Ie(_)) => crate::inspect::item_criticality(id),
        _ => None,
    }
}

/// `value`, a value of `shape` of a tree, as a message is written: a transfer as its
/// decoded value, or as its `octets` when it has none, and each IE of a container as
/// [`ie_by_name`] shows it.
fn by_name(value: &Value, shape: Shape) -> Result<Value, String> {
    let shape = resolved(shape);
    match value {
        Value::Object(entry) => match shape {
            Shape::Transfer(_) => {
                let members = members(shape, Some(entry)).unwrap_or_default();
                match (entry.get("decoded"), member(&members, "decoded")) {
                    (Some(decoded), Some((_, of, _))) => by_name(decoded, *of),
                    _ => Ok(json!({"octets": received(entry)})),
                }
            }
            // An IE that a value holds alone.
            Shape::Ie(_) | Shape::Extension(_) => ie_by_name(value, shape, shape),
            _ => {
                let members = members(shape, Some(entry)).unwrap_or_default();
                let shown = entry.iter().map(|(name, value)| {
                    let of = member(&members, name).map_or(Shape::Unknown, |(_, of, _)| *of);
                    Ok((name.clone(), by_name(value, of)?))
                });
                Ok(Value::Object(shown.collect::<Result<_, String>>()?))
            }
        },
        Value::Array(entries) => {
            let shown = entries.iter().map(|entry| match element(shape) {
                of @ (Shape::Ie(_) | Shape::Extension(_)) => ie_by_name(entry, of, shape),
                of => by_name(entry, of),
            });
            Ok(Value::Array(shown.collect::<Result<_, String>>()?))
        }
        plain => Ok(plain.clone()),
    }
}

/// The entry of an IE of `shape` in a container of `container`, as its name with its
/// value: the name that ASN.1 gives the IE, or its identifier in digits when it has
/// none. One that did not decode, or that contains a type and octets that are not of
/// that type, has the octets that it was received as, as `{"octets": "…"}`. Its
/// `criticality` is beside its name when it is not the one that the specification
/// assigns it there, and where the tables do not have that one.
fn ie_by_name(entry: &Value, shape: Shape, container: Shape) -> Result<Value, String> {
    let id = entry_id(entry).and_then(|id| u16::try_from(id).ok());
    let id = id.ok_or("an IE has its identifier")?;
    let mut names = crate::inspect::ie_names().iter();
    let name = match names.find(|(known, _)| *known == id) {
        Some((_, name)) => name.to_string(),
        None => id.to_string(),
    };
    let field = value_member(shape).unwrap_or("value");
    let undecoded = [entry, &entry[field]]
        .iter()
        .any(|of| of.get("_decode_error").is_some());
    let value = match undecoded {
        true => json!({"octets": entry["_raw_value"]}),
        false => by_name(&entry[field], ie_value(Some(id)))?,
    };
    let mut shown = Map::new();
    shown.insert(name, value);
    if entry["criticality"].as_str() != assigned(container, id) {
        shown.insert("criticality".into(), entry["criticality"].clone());
    }
    Ok(Value::Object(shown))
}

/// `value`, a value of `shape` as a message is written, as a tree has it: each IE of a
/// container as the entry that [`entry_of`] gives it. A transfer keeps the form that it
/// is written in, its decoded value alone or as its `decoded` member, which
/// [`encode_pdu`](crate::inspect::encode_pdu) takes.
fn written(value: &Value, shape: Shape, depth: usize) -> Result<Value, String> {
    if depth > WRITTEN {
        return Err(format!("a written message nests deeper than {WRITTEN}"));
    }
    let shape = resolved(shape);
    match value {
        Value::Object(entry) => match shape {
            Shape::Transfer(_) => {
                let members = members(shape, None).unwrap_or_default();
                let decoded = member(&members, "decoded");
                let decoded = decoded.map_or(Shape::Unknown, |(_, of, _)| *of);
                if !entry.keys().any(|name| of_a_transfer(name)) {
                    return written(value, decoded, depth + 1);
                }
                let mut entry = entry.clone();
                if let Some(value) = entry.get_mut("decoded") {
                    *value = written(value, decoded, depth + 1)?;
                }
                Ok(Value::Object(entry))
            }
            // An IE that a value holds alone.
            Shape::Ie(_) | Shape::Extension(_) => entry_of(value, shape, shape, depth),
            _ => {
                let members = members(shape, Some(entry)).unwrap_or_default();
                let listed = entry.iter().map(|(name, value)| {
                    let of = member(&members, name).map_or(Shape::Unknown, |(_, of, _)| *of);
                    Ok((name.clone(), written(value, of, depth + 1)?))
                });
                Ok(Value::Object(listed.collect::<Result<_, String>>()?))
            }
        },
        Value::Array(entries) => {
            let listed = entries.iter().map(|entry| match element(shape) {
                of @ (Shape::Ie(_) | Shape::Extension(_)) => entry_of(entry, of, shape, depth),
                of => written(entry, of, depth + 1),
            });
            Ok(Value::Array(listed.collect::<Result<_, String>>()?))
        }
        plain => Ok(plain.clone()),
    }
}

/// The entry that a tree has for an IE of `shape` in a container of `container`, which
/// is written as its name with its value: the name that ASN.1 gives the IE or its
/// identifier in digits, and `{"octets": "…"}` for the octets that it is sent as. Its
/// `criticality` is the one beside its name, or the one that the specification assigns
/// it there. An entry that is written as a tree has it, with its `id`, stays as it is,
/// and the IEs that its value holds are read the same way.
fn entry_of(ie: &Value, shape: Shape, container: Shape, depth: usize) -> Result<Value, String> {
    const IE: &str = "each entry of a container is one IE: its name with its value";
    let field = value_member(shape).unwrap_or("value");
    let entry = ie.as_object().ok_or(IE)?;
    if entry.contains_key("id") {
        let id = entry_id(ie).and_then(|id| u16::try_from(id).ok());
        let mut entry = entry.clone();
        if let Some(value) = entry.get_mut(field) {
            *value = written(value, ie_value(id), depth + 1)?;
        }
        return Ok(Value::Object(entry));
    }
    let mut members = entry.iter().filter(|(member, _)| *member != "criticality");
    let (Some((name, value)), None) = (members.next(), members.next()) else {
        return Err(IE.into());
    };
    let id = match name.parse::<u16>() {
        Ok(id) => id,
        Err(_) => ie_id(name).ok_or_else(|| format!("{name:?} is not an IE of {PROTOCOL}"))?,
    };
    let criticality = match (entry.get("criticality"), assigned(container, id), container) {
        (Some(criticality), ..) => criticality.clone(),
        (None, Some(criticality), _) => json!(criticality),
        (None, None, Shape::Ies(Some(message))) => {
            return Err(format!(
                "{} has no IE {name}: one that is sent in it says its \"criticality\"",
                message.name
            ));
        }
        (None, None, _) => {
            return Err(format!(
                "{name} says its \"criticality\" here: the tables have none for it"
            ));
        }
    };
    let octets = value.as_object().filter(|value| value.len() == 1);
    Ok(match octets.and_then(|value| value.get("octets")) {
        Some(octets) => json!({"id": id, "criticality": criticality, "octets": octets}),
        None => {
            let value = written(value, ie_value(Some(id)), depth + 1)?;
            json!({"id": id, "criticality": criticality, field: value})
        }
    })
}

/// The IEs of the message that ASN.1 names `message`, which a tree has as `ies`, as a
/// message is written: each as its name with its value, and so are the IEs that a value
/// holds, to any depth.
pub(crate) fn named_ies(message: &str, ies: &Value) -> Result<Value, String> {
    let message = sent(|_, _, name| same_name(name, message));
    by_name(ies, Shape::Ies(message))
}

/// The entries that a tree has for the IEs of the message that ASN.1 names `message`,
/// which `ies` writes as [`named_ies`] shows them.
pub(crate) fn listed_ies(message: &str, ies: &Value) -> Result<Value, String> {
    let message = sent(|_, _, name| same_name(name, message));
    written(ies, Shape::Ies(message), 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every member and every alternative of every type of the list.
    fn all_members() -> impl Iterator<Item = (&'static str, &'static [Member])> {
        TYPES.iter().filter_map(|(name, kind)| match kind {
            Kind::Sequence(members) | Kind::Choice(members) => Some((*name, *members)),
            Kind::Is(_) => None,
        })
    }

    #[test]
    fn the_types_are_listed_in_the_order_of_their_names_and_name_types_of_the_list() {
        assert!(TYPES.windows(2).all(|pair| pair[0].0 < pair[1].0));
        fn check(of: &str, shape: Shape) {
            match shape {
                // A name that the list does not have is that of an ENUMERATED, which
                // has no members and has its names.
                Shape::Named(name) => assert!(
                    kind_of(name).is_some() || enumeration(name).is_some(),
                    "{of}: {name}"
                ),
                Shape::Transfer(name) if !name.is_empty() => {
                    assert!(kind_of(name).is_some(), "{of}: {name}")
                }
                Shape::List(inner) => check(of, *inner),
                _ => {}
            }
        }
        for (name, kind) in TYPES {
            match kind {
                Kind::Is(shape) => check(name, *shape),
                Kind::Sequence(members) | Kind::Choice(members) => {
                    members.iter().for_each(|(_, shape, _)| check(name, *shape))
                }
            }
        }
        // The types of the messages and of the IEs are types of the bindings.
        for (.., ty) in MESSAGES {
            assert!(kind_of(type_name(ty)).is_some(), "{ty}");
        }
        assert!(
            IE_TYPES
                .iter()
                .any(|(_, ty, _)| kind_of(type_name(ty)).is_some())
        );
    }

    #[test]
    fn an_enumerated_value_has_the_names_of_its_type_wherever_a_path_finds_it() {
        assert!(ENUMERATIONS.windows(2).all(|pair| pair[0].0 < pair[1].0));
        let names = |message: &str, path: &str| enumerated_at(message, &format!("/{ROOT}/{path}"));
        let release = |path: &str| names("UEContextReleaseCommand", path).unwrap();
        // An alternative of a CHOICE, however the path is written.
        let nas = release("Cause/value/nas").expect("an ENUMERATED");
        assert!(nas.contains(&"normal-release") && !nas.contains(&"unspecified-failure"));
        assert_eq!(release("cause/VALUE/Nas"), Some(nas));
        // The value of the IE may be left out of the path.
        assert_eq!(release("Cause/nas"), Some(nas));
        // A CHOICE, and the identifier of an IE.
        assert_eq!(release("Cause/value"), None);
        assert_eq!(release("Cause/id"), None);
        // A member of the entries of a list, through an optional member.
        let error = names(
            "ErrorIndication",
            "CriticalityDiagnostics/value/iEsCriticalityDiagnostics/0/typeOfError",
        );
        assert_eq!(error.unwrap(), Some(&["not-understood", "missing"][..]));
        // What `check_path` refuses is refused.
        assert!(names("UEContextReleaseCommand", "Cause/value/nsa").is_err());
        assert!(names("NoSuchMessage", "Cause/value/nas").is_err());
    }

    #[test]
    fn a_name_is_the_same_whatever_its_case_and_what_is_between_its_letters() {
        assert!(same_name("AMF-UE-NGAP-ID", "amf_ue ngap.id"));
        assert!(same_name("pLMNIdentity", "plmnidentity"));
        assert!(!same_name("pLMNIdentity", "plmn"));
        assert!(!same_name("-", "_") && !same_name("", ""));
        assert_eq!((position("0"), position("12")), (Some(0), Some(12)));
        assert_eq!(
            (position("+1"), position("01"), position("")),
            (None, None, None)
        );
    }

    #[test]
    fn no_two_names_that_a_path_takes_are_the_same() {
        let distinct = |of: &str, names: &mut dyn Iterator<Item = &str>| {
            let names: Vec<_> = names.collect();
            for (at, name) in names.iter().enumerate() {
                let other = names[..at].iter().find(|other| same_name(other, name));
                assert!(other.is_none(), "{of}: {name} and {other:?}");
            }
        };
        for (name, members) in all_members() {
            distinct(name, &mut members.iter().map(|(member, ..)| *member));
        }
        let ies = crate::inspect::ie_names();
        distinct("the IEs", &mut ies.iter().map(|(_, name)| *name));
        let mut messages = MESSAGES.iter().map(|(_, _, name, ..)| *name);
        distinct("the messages", &mut messages);
        // The names of an IE and of a member do not meet: a message that a tree contains
        // has its IEs alone.
        for (name, members) in all_members() {
            if members.iter().any(|(member, ..)| *member == IES) {
                assert_eq!(members.len(), 1, "{name}");
            }
        }
    }

    #[test]
    fn an_enumerated_value_has_two_spellings_at_most() {
        // A name that one type does not have is tried as the other types spell it.
        let names = crate::inspect_registry::ENUMERATED;
        for name in names {
            let same = names.iter().filter(|other| same_name(other, name));
            assert!(same.count() <= 2, "{name}");
        }
    }

    /// The tree of a message that ASN.1 names `name` and that has `ies` for IEs.
    fn tree_of(name: &str, ies: Value) -> Value {
        let mut messages = MESSAGES.iter();
        let (direction, code, ..) = messages.find(|(_, _, known, ..)| *known == name).unwrap();
        json!({
            "direction": direction,
            "procedure_code": code,
            "criticality": "reject",
            "message": {IES: ies},
        })
    }

    #[test]
    fn a_segment_that_names_no_member_of_an_ie_is_taken_in_its_value() {
        let cause = ie_id("Cause").expect("an IE of the protocol");
        let value = json!({"radioNetwork": "unspecified"});
        let entry = json!({"id": cause, "criticality": "ignore", "value": value});
        let mut tree = tree_of("UEContextReleaseCommand", json!([entry]));
        let path = |rest: &str| format!("/{ROOT}/Cause/{rest}");
        // The path that leaves `value` out selects what the one that has it selects.
        let short = select(&tree, &path("radioNetwork")).unwrap();
        assert_eq!(short, [&json!("unspecified")]);
        assert_eq!(short, select(&tree, &path("value/radioNetwork")).unwrap());
        assert!(select(&tree, &path("nas")).unwrap().is_empty());
        // What the IE has itself comes first, and `*` is each of its members.
        assert_eq!(select(&tree, &path("value")).unwrap(), [&value]);
        assert_eq!(select(&tree, &path("id")).unwrap(), [&json!(cause)]);
        assert_eq!(select(&tree, &path("*")).unwrap().len(), 3);
        // A name that neither has is refused with the members of both, with a tree and
        // without one.
        let refused = select(&tree, &path("misspelled")).unwrap_err();
        assert!(refused.contains("is not a member of Cause, which has radioNetwork"));
        assert!(refused.ends_with("; an IE itself has id, criticality, value, octets"));
        let checked = |rest: &str| check_path("UEContextReleaseCommand", &path(rest));
        assert_eq!(checked("misspelled"), Err(refused.clone()));
        assert_eq!(checked("nas"), Ok(()));
        let deeper = checked("radioNetwork/deeper").unwrap_err();
        assert!(deeper.contains("which has none") && !deeper.contains("itself"));
        // An edit takes the same path, and marks the value that it changed.
        tree[MESSAGE][IES][0]["_raw_value"] = json!("0000");
        set(&mut tree, &path("nas"), json!("normal-release")).unwrap();
        let edited = &tree[MESSAGE][IES][0];
        assert_eq!(edited["value"], json!({"nas": "normal-release"}));
        assert_eq!(edited[EDITED], json!(true));
        let before = tree.clone();
        assert_eq!(set(&mut tree, &path("misspelled"), json!(1)), Err(refused));
        let kept = remove(&mut tree, &path("nas")).unwrap_err();
        assert!(kept.contains("a CHOICE has one alternative"), "{kept}");
        assert_eq!(tree, before);
    }

    #[test]
    fn a_segment_that_names_no_member_of_a_transfer_is_taken_in_its_decoded_value() {
        // A transfer that contains a cause, which is a type of the protocol.
        let shape = Shape::Transfer("Cause");
        let at = |transfer: &Value, part: &str| {
            let mut next = Vec::new();
            advance((Some(transfer), shape), part, true, "/a/path", &mut next)?;
            Ok::<_, Refusal>(next.into_iter().map(|(value, _)| value.cloned()).collect())
        };
        let some = |value: Value| Ok::<Vec<_>, Refusal>(vec![Some(value)]);
        let transfer = json!({"_raw_transfer": "00", "decoded": {"radioNetwork": "unspecified"}});
        assert_eq!(at(&transfer, "radioNetwork"), some(json!("unspecified")));
        assert_eq!(at(&transfer, "nas"), Ok(vec![None]));
        // What the transfer has itself comes first.
        assert_eq!(at(&transfer, "decoded"), some(transfer["decoded"].clone()));
        assert_eq!(at(&transfer, "octets"), some(json!("00")));
        let (unread, refused) = at(&transfer, "misspelled").unwrap_err();
        assert!(!unread && refused.contains("is not a member of Cause, which has radioNetwork"));
        assert!(refused.ends_with("; a transfer itself has decoded, octets"));
        // The types say the same without a tree.
        let mut shapes = Vec::new();
        let mut push = |shape| shapes.push(shape);
        let wrong = typed(shape, "misspelled", true, "/a/path", &mut push);
        assert_eq!(wrong, Err((false, refused)));
        assert_eq!(typed(shape, "nas", true, "/a/path", &mut push), Ok(()));
        assert!(matches!(shapes[..], [Shape::Named(nas)] if enumeration(nas).is_some()));
        // One that did not decode cannot be read, and one that is its octets has no
        // decoded value: the name is still one that the type has.
        let undecoded = json!({"_raw_transfer": "00", "_decode_error": "why"});
        assert!(matches!(at(&undecoded, "radioNetwork"), Err((true, _))));
        assert_eq!(at(&json!("00"), "radioNetwork"), Ok(vec![None]));
        assert!(matches!(at(&json!("00"), "misspelled"), Err((false, _))));
        // An edit is made in the decoded value, whose octets are then others.
        let mut edited = transfer.clone();
        let set = Edit::Set(json!("normal-release"));
        let changed = apply(&mut edited, shape, &["nas".into()], &set, true, "/a/path");
        assert_eq!(changed, Ok(1));
        assert_eq!(edited["decoded"], json!({"nas": "normal-release"}));
        assert_eq!(edited[EDITED], json!(true));
    }

    #[test]
    fn only_a_string_of_bits_names_a_member_as_the_ie_that_holds_it_does() {
        // A segment under an IE that names a member of the IE is that member, and under
        // a transfer one of the transfer: of the values that they hold, a string of
        // bits whose size varies alone has a member of such a name, its `value`, which
        // a path reaches after the `value` of the IE.
        let mut bits = 0;
        for (id, ..) in IE_TYPES {
            let mut shape = resolved(ie_value(Some(*id)));
            if let Shape::Transfer(contained) = shape {
                shape = resolved(Shape::Named(contained));
            }
            let members = members(shape, None).unwrap_or_default();
            for own in VALUES.into_iter().chain(["id", "criticality", "octets"]) {
                let same = members.iter().any(|(name, ..)| same_name(name, own));
                assert_eq!(same, shape == Shape::Bits && own == "value", "{id}: {own}");
            }
            bits += usize::from(shape == Shape::Bits);
        }
        assert!(bits > 0);
    }

    #[test]
    fn the_ies_that_a_value_holds_are_written_by_name_and_read_back() {
        let cause = ie_id("Cause").expect("an IE of the protocol");
        let value = json!({"radioNetwork": "unspecified"});
        // The entry of a tree, with what it keeps of what was received, and as it is
        // written.
        let entry = |field: &str, received: bool| {
            let mut entry = json!({"id": cause, "criticality": "ignore", field: value});
            if received {
                entry["_raw_value"] = json!("0000");
                entry["_original_id"] = json!(cause);
            }
            entry
        };
        // No table has the criticality of the IEs of a message that a transfer contains,
        // nor of those of an extension container: it is written, and asked for.
        for (container, field) in [
            (Shape::Ies(None), "value"),
            (Shape::Extensions, "extensionValue"),
        ] {
            let shown = by_name(&json!([entry(field, true)]), container).unwrap();
            assert_eq!(shown, json!([{"Cause": value, "criticality": "ignore"}]));
            let listed = written(&shown, container, 0).unwrap();
            assert_eq!(listed, json!([entry(field, false)]));
            let refused = written(&json!([{"Cause": value}]), container, 0).unwrap_err();
            assert!(refused.contains("says its \"criticality\""), "{refused}");
            // An entry that is written as a tree has it stays as it is.
            assert_eq!(written(&listed, container, 0), Ok(listed));
        }
        // One that did not decode has its octets, which are sent as they are.
        let mut undecoded = entry("value", true);
        undecoded["value"] = json!("0000");
        undecoded["_decode_error"] = json!("why");
        let shown = by_name(&json!([undecoded]), Shape::Ies(None)).unwrap();
        let octets = json!({"Cause": {"octets": "0000"}, "criticality": "ignore"});
        assert_eq!(shown, json!([octets]));
        let listed = written(&shown, Shape::Ies(None), 0).unwrap();
        let octets = json!({"id": cause, "criticality": "ignore", "octets": "0000"});
        assert_eq!(listed, json!([octets]));
        // A transfer is its decoded value, or its octets when it has none, and is read
        // in the form that it is written in.
        let transfer = Shape::Transfer("Cause");
        let decoded = json!({"_raw_transfer": "00", "decoded": value});
        assert_eq!(by_name(&decoded, transfer), Ok(value.clone()));
        assert_eq!(written(&value, transfer, 0), Ok(value.clone()));
        let undecoded = json!({"_raw_transfer": "00", "_decode_error": "why"});
        let octets = json!({"octets": "00"});
        assert_eq!(by_name(&undecoded, transfer), Ok(octets.clone()));
        assert_eq!(written(&octets, transfer, 0), Ok(octets));
        // A message is not read deeper than a tree is encoded.
        let mut deep = json!(1);
        for _ in 0..WRITTEN + 2 {
            deep = json!([deep]);
        }
        let refused = written(&deep, Shape::Unknown, 0).unwrap_err();
        assert!(refused.contains("nests deeper"), "{refused}");
    }

    #[test]
    fn the_octets_of_an_ie_carry_a_type_of_the_list_by_a_name_that_no_member_has() {
        for (id, name, ty) in CARRIED {
            // The value of the IE is octets, and what they carry a type of the list, by
            // the name that ASN.1 gives it.
            let (_, octets, contained) = IE_TYPES.iter().find(|(known, ..)| known == id).unwrap();
            assert!(contained.is_empty() && kind_of(type_name(octets)).is_none());
            assert!(kind_of(type_name(ty)).is_some() && same_name(name, type_name(ty)));
            assert_eq!(ie_value(Some(*id)), Shape::Carrier(*id));
            // A tree is encoded with any member of that name as that type.
            for (of, members) in all_members() {
                let same = members.iter().any(|(member, ..)| same_name(member, name));
                assert!(!same, "{of}: {name}");
            }
        }
    }

    #[test]
    fn an_entry_of_a_list_of_ies_is_known_by_its_members() {
        // An IE is written with a number or a name as its `id`: no type has a member of
        // that name with such a value, nor one named as the members of an IE or of a
        // transfer in a tree.
        for (name, members) in all_members() {
            for (member, shape, _) in members {
                assert!(
                    !["octets", "decoded", "extensionValue"].contains(member),
                    "{name}.{member}"
                );
                if *member == "id" {
                    assert!(matches!(resolved(*shape), Shape::Named(_)), "{name}.id");
                }
            }
        }
    }
}
