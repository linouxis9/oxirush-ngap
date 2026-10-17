//! The values of a view by their paths, and the IEs of a tree by their names: see the
//! module documentation of [`crate::inspect`]. The rules are those of
//! `oxirush_aper::inspect`, which the two protocols share, with the tables of this one.

use std::collections::HashMap;
use std::sync::LazyLock;

use oxirush_aper::inspect::{self as shared, Protocol};
pub(crate) use oxirush_aper::inspect::{
    EDITED, Kind, Shape, extended, member_shape, of_a_transfer, same_name,
};
use serde_json::{Map, Value};

use crate::Error;
// The name of the protocol, the first segment of the paths of the IEs of the message of
// a PDU, and what the ASN.1 says of the messages, the IEs and the types.
use crate::inspect_registry::{
    CARRIED, ENUMERATIONS, IE_NAMES, IE_TYPES, ITEM_CRITICALITIES, MESSAGE_IE_CRITICALITIES,
    MESSAGES, PROTOCOL, ROOT, TYPES,
};
// The type that an IE of an extension container has where its identifier alone does not
// say it: the one that the type which the container extends gives it.
use crate::registry::EXTENSION_TYPES;

/// The tables of the protocol, as the paths read them.
pub(crate) static TABLES: Protocol = Protocol {
    name: PROTOCOL,
    root: ROOT,
    types: TYPES,
    messages: MESSAGES,
    ie_names: IE_NAMES,
    ie_types: IE_TYPES,
    extension_types: EXTENSION_TYPES,
    carried: CARRIED,
    enumerations: ENUMERATIONS,
    item_criticalities: ITEM_CRITICALITIES,
    message_ie_criticalities: MESSAGE_IE_CRITICALITIES,
    ie_id,
    kind_of,
};

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
        $crate::inspect_paths::Shape::Extensions("")
    };
    (extension) => {
        $crate::inspect_paths::Shape::Extension(None, "")
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
/// bit string. A member is written with `?` when a value may be without it, and
/// its value as `_` when it has no members, `bits` for a bit string whose size
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

/// The number of the IE that ASN.1 names `id-<name>`, however the name is written.
pub(crate) fn ie_id(name: &str) -> Option<u16> {
    /// The identifier of each IE by the letters and the digits of its name, in lower
    /// case, which is how two names are the same: the first IE of a name, as a search
    /// of the names in their order finds it.
    static IDS: LazyLock<HashMap<Vec<u8>, u16>> = LazyLock::new(|| {
        let mut ids = HashMap::new();
        for (id, name) in IE_NAMES {
            ids.entry(letters(name).collect()).or_insert(*id);
        }
        ids
    });
    // The letters of the name, on the stack for a name of the length that an IE has:
    // the longest has 51 of them.
    let mut short = [0; 64];
    let mut length = 0;
    for letter in letters(name) {
        let Some(place) = short.get_mut(length) else {
            let long: Vec<u8> = letters(name).collect();
            return IDS.get(&long).copied();
        };
        *place = letter;
        length += 1;
    }
    IDS.get(&short[..length]).copied()
}

/// What a type of the list of the types is; `None` for a name that the list does not
/// have, which is that of a value without members.
fn kind_of(name: &str) -> Option<&'static Kind> {
    static KINDS: LazyLock<HashMap<&'static str, &'static Kind>> =
        LazyLock::new(|| TYPES.iter().map(|(name, kind)| (*name, kind)).collect());
    KINDS.get(name).copied()
}

/// The values at `path` in `view`, in the order of the view.
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
/// After an IE, a segment that is the name of that IE stays on it: a path may say which
/// IE an entry is, as `0/Cause` for the first IE of a list, and one that ends so selects
/// the IE. The name of another IE, where the value has no such member, is an error that
/// says which IE the entry is.
///
/// The `octets` of an IE or of a transfer are what it was received as: those of one whose
/// value [`set`], [`remove`] or [`insert`] edited since are an error, as `from_view` of
/// the PDU has yet to give it others. So is a path into a value that did not decode.
pub fn select<'a>(view: &'a Value, path: &str) -> Result<Vec<&'a Value>, Error> {
    shared::select(&TABLES, view, path)
}

/// Whether `path` can select a value in a message that ASN.1 names `message`: what
/// [`select`] says of a path without a view to select in.
///
/// The name is taken as [`message_named`](crate::inspect::message_named) takes it. Under
/// the root, the name of an IE has to be that of an IE of the object set of the message;
/// under an IE, each segment has to be a member that the type of the value has, through
/// the lists, the alternatives of a CHOICE and the decoded value of a transfer. A position
/// or `*` stands for each entry that a list could have, and `-` for the end of a list.
///
/// An IE of an extension container is read against the type that the type which the
/// container extends gives it, where its identifier alone does not say its type.
///
/// What is not known is not refused: the value of an IE selected by `@id=N` when the
/// identifier is unknown or has no type where it is, the entries of a container that is
/// not that of the message when they are selected by position, a segment under such an
/// IE, which may name a member of its value, and whether a position is within the bounds
/// of its list. Such an entry is the IE that the next segment names, and the path is
/// then read against the type of that IE: unless some value has a member of that name
/// and of another type, which the segment may be.
pub fn check_path(message: &str, path: &str) -> Result<(), Error> {
    shared::check_path(&TABLES, message, path)
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
pub fn enumerated_at(message: &str, path: &str) -> Result<Option<&'static [&'static str]>, Error> {
    shared::enumerated_at(&TABLES, message, path)
}

/// Each value of `view` with the path that selects it, in the order of the view. A list
/// of plain values is one value. The IEs of the message go by name under the root, and
/// those of a message that the view contains by name in its place; an IE goes by its
/// position when it has no name or its identifier is there twice.
///
/// The members that start with `_` repeat what was received and are left out, except
/// `_decode_error`. The `octets` of an IE or of a transfer are listed when it has no
/// value beside them: those of a value repeat it, and are selected all the same.
///
/// What is nested deeper than 64 levels, the most segments that a path has, is one
/// value at the deepest path, whatever it holds: no view of a PDU is that deep, and
/// every path that is listed is one that [`select`] takes.
pub fn paths(view: &Value) -> Vec<(String, Value)> {
    shared::paths(&TABLES, view)
}

/// Give the member or the list entry at `path` the value `value`. `null` takes an
/// optional member out. An IE is written as `{id, criticality, value}` with its typed
/// value, or with its `octets` in hexadecimal in place of `value`; its `id` is a number
/// or the name of the IE. The `octets` of an IE or of a transfer are set in place of
/// its value. An alternative of a CHOICE takes the place of the one that is there.
///
/// A path that selects nothing is an error, and the view is then as it was. So is a
/// name that the type of a value cannot have, a member that its type always has and
/// that would be taken out, and an edit that would send nothing else: the value of an IE
/// or the decoded value of a transfer is not taken out, as the octets received would go
/// out in its place.
pub fn set(view: &mut Value, path: &str, value: Value) -> Result<(), Error> {
    shared::set(&TABLES, view, path, value)
}

/// Take the member or the list entries at `path` out of `view`. A path that selects
/// nothing is an error, and so is a member that its type always has.
pub fn remove(view: &mut Value, path: &str) -> Result<(), Error> {
    shared::remove(&TABLES, view, path)
}

/// Add `value` to a list of `view`: before each entry that `path` selects, or at the end
/// for a path that ends with `-`. An IE is written as [`set`] takes it.
pub fn insert(view: &mut Value, path: &str, value: Value) -> Result<(), Error> {
    shared::insert(&TABLES, view, path, value)
}

/// Show, in `view`, the octets that `path` takes as a type that they may carry as what
/// `decode` makes of them and of the name of that type: see
/// [`open`](crate::inspect::open), which gives the function that decodes.
pub(crate) fn open_with(
    view: &mut Value,
    path: &str,
    decode: &dyn Fn(Value, &str) -> Value,
) -> Result<(), Error> {
    shared::open(&TABLES, view, path, decode)
}

/// Write the entry of an IE as a view has it: the name of the IE that its `id` may be
/// written as becomes its number, and its `octets` what it is sent as in place of a
/// value.
pub(crate) fn written_ie(entry: &mut Map<String, Value>) -> Result<(), Error> {
    shared::written_ie(&TABLES, entry)
}

/// The members that `entry`, a value of `shape` in a view, can have, each with its
/// shape.
pub(crate) fn member_shapes(shape: Shape, entry: &Map<String, Value>) -> shared::Members {
    TABLES.member_shapes(shape, entry)
}

/// The shape of each entry of a list of `shape`.
pub(crate) fn entry_shape(shape: Shape) -> Shape {
    TABLES.entry_shape(shape)
}

/// The shape of the message of a direction and a procedure code in a view, where the
/// tables have it.
pub(crate) fn message_shape(direction: &str, code: u8) -> Shape {
    TABLES.message_shape(direction, code)
}

/// The shape of octets that carry the type which ASN.1 names `name`, once a view shows
/// them so: the transfer of that type.
pub(crate) fn carried_shape(name: &str) -> Shape {
    TABLES.carried_shape(name)
}

/// The IEs of the message that ASN.1 names `message`, which a view has as `ies`, as a
/// tree has them: each as its name with its value, and so are the IEs that a value
/// holds, to any depth.
pub(crate) fn named_ies(message: &str, ies: Value) -> Result<Value, Error> {
    shared::named_ies(&TABLES, message, ies)
}

/// The entries that a view has for the IEs of the message that ASN.1 names `message`,
/// which `ies` has as a tree has them: as [`named_ies`] shows them.
pub(crate) fn listed_ies(message: &str, ies: &Value) -> Result<Value, Error> {
    shared::listed_ies(&TABLES, message, ies)
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxirush_aper::inspect::{Member, type_name};
    use serde_json::json;

    /// The member of a view that holds the message of the PDU, and the member of a
    /// message that holds its IEs.
    const MESSAGE: &str = "message";
    const IES: &str = "protocolIEs";
    /// The members that hold the value of an IE or of a transfer.
    const VALUES: [&str; 3] = ["value", "extensionValue", "decoded"];

    /// Every member and every alternative of every type of the list.
    fn all_members() -> impl Iterator<Item = (&'static str, &'static [Member])> {
        TYPES.iter().filter_map(|(name, kind)| match kind {
            Kind::Sequence(members) | Kind::Choice(members) => Some((*name, *members)),
            Kind::Is(_) => None,
        })
    }

    /// The member of `members` that a segment names.
    fn member<'m>(members: &'m [Member], part: &str) -> Option<&'m Member> {
        let exact = members.iter().find(|(name, ..)| *name == part);
        exact.or_else(|| members.iter().find(|(name, ..)| same_name(name, part)))
    }

    /// The name that ASN.1 gives the IE of an identifier.
    fn ie_name(id: u16) -> String {
        TABLES.ie_name_of(id).unwrap().to_string()
    }

    #[test]
    fn the_types_are_listed_in_the_order_of_their_names_and_name_types_of_the_list() {
        assert!(TYPES.windows(2).all(|pair| pair[0].0 < pair[1].0));
        fn check(of: &str, shape: Shape) {
            match shape {
                // A name that the list does not have is that of an ENUMERATED, which
                // has no members and has its names.
                Shape::Named(name) => assert!(
                    kind_of(name).is_some() || TABLES.enumeration(name).is_some(),
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
        // The identifiers of the IEs are in their order, which their names are read in.
        assert!(IE_NAMES.windows(2).all(|pair| pair[0].0 < pair[1].0));
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
        distinct("the IEs", &mut IE_NAMES.iter().map(|(_, name)| *name));
        let mut messages = MESSAGES.iter().map(|(_, _, name, ..)| *name);
        distinct("the messages", &mut messages);
        // The names of an IE and of a member do not meet: a message that a view contains
        // has its IEs alone.
        for (name, members) in all_members() {
            if members.iter().any(|(member, ..)| *member == IES) {
                assert_eq!(members.len(), 1, "{name}");
            }
        }
        // A name is read whatever its case and its separators, and a name of an IE
        // finds its identifier.
        assert!(same_name("AMF-UE-NGAP-ID", "amf_ue ngap.id"));
        assert!(!same_name("-", "_") && !same_name("", ""));
        for (id, name) in IE_NAMES {
            assert_eq!(ie_id(&name.to_lowercase()), Some(*id), "{name}");
        }
    }

    #[test]
    fn no_two_names_of_an_enumerated_are_the_same() {
        // A name is read whatever its case and its separators: no two values of one
        // type are then one name.
        for (ty, names) in ENUMERATIONS {
            for (at, name) in names.iter().enumerate() {
                let other = names[..at].iter().find(|other| same_name(other, name));
                assert!(other.is_none(), "{ty}: {name} and {other:?}");
            }
        }
    }

    /// The view of a message that ASN.1 names `name` and that has `ies` for IEs.
    fn view_of(name: &str, ies: Value) -> Value {
        let mut messages = MESSAGES.iter();
        let (direction, code, ..) = messages.find(|(_, _, known, ..)| *known == name).unwrap();
        json!({
            "direction": direction,
            "procedure_code": code,
            "criticality": "reject",
            MESSAGE: {IES: ies},
        })
    }

    #[test]
    fn a_segment_that_names_no_member_of_an_ie_is_taken_in_its_value() {
        let cause = ie_id("Cause").expect("an IE of the protocol");
        let value = json!({"radioNetwork": "unspecified"});
        let entry = json!({"id": cause, "criticality": "ignore", "value": value});
        let mut view = view_of("UEContextReleaseCommand", json!([entry]));
        let path = |rest: &str| format!("/{ROOT}/Cause/{rest}");
        // The path that leaves `value` out selects what the one that has it selects.
        let short = select(&view, &path("radioNetwork")).unwrap();
        assert_eq!(short, [&json!("unspecified")]);
        assert_eq!(short, select(&view, &path("value/radioNetwork")).unwrap());
        assert!(select(&view, &path("nas")).unwrap().is_empty());
        // What the IE has itself comes first, and `*` is each of its members.
        assert_eq!(select(&view, &path("value")).unwrap(), [&value]);
        assert_eq!(select(&view, &path("id")).unwrap(), [&json!(cause)]);
        assert_eq!(select(&view, &path("*")).unwrap().len(), 3);
        // A name that neither has is refused with the members of both, with a view and
        // without one.
        let refused = select(&view, &path("misspelled")).unwrap_err();
        assert_eq!(refused.kind(), crate::ErrorKind::NoSuchMember);
        let text = refused.to_string();
        assert!(text.contains("is not a member of Cause, which has radioNetwork"));
        assert!(text.contains("; an IE itself has id, criticality, value, octets, at /"));
        let checked = |rest: &str| check_path("UEContextReleaseCommand", &path(rest));
        assert_eq!(checked("misspelled"), Err(refused.clone()));
        assert_eq!(checked("nas"), Ok(()));
        let deeper = checked("radioNetwork/deeper").unwrap_err().to_string();
        assert!(deeper.contains("which has none") && !deeper.contains("itself"));
        // An edit takes the same path, and marks the value that it changed.
        view[MESSAGE][IES][0]["_raw_value"] = json!("0000");
        set(&mut view, &path("nas"), json!("normal-release")).unwrap();
        let edited = &view[MESSAGE][IES][0];
        assert_eq!(edited["value"], json!({"nas": "normal-release"}));
        assert_eq!(edited[EDITED], json!(true));
        let before = view.clone();
        assert_eq!(set(&mut view, &path("misspelled"), json!(1)), Err(refused));
        let kept = remove(&mut view, &path("nas")).unwrap_err();
        assert_eq!(kept.kind(), crate::ErrorKind::Edit);
        assert!(
            kept.to_string().contains("a CHOICE has one alternative"),
            "{kept}"
        );
        assert_eq!(view, before);
    }

    #[test]
    fn the_name_of_the_ie_that_an_entry_is_stays_on_the_entry() {
        let cause = ie_id("Cause").expect("an IE of the protocol");
        let value = json!({"radioNetwork": "unspecified"});
        let entry = json!({"id": cause, "criticality": "ignore", "value": value});
        let mut view = view_of("UEContextReleaseCommand", json!([entry]));
        let path = |rest: &str| format!("/{ROOT}/{rest}");
        // After its position, and after its name, which selects it already.
        for named in ["0/Cause", "0/cause", "Cause/Cause", "*/Cause"] {
            assert_eq!(select(&view, &path(named)).unwrap(), [&entry], "{named}");
            let member = select(&view, &path(&format!("{named}/radioNetwork"))).unwrap();
            assert_eq!(member, [&json!("unspecified")], "{named}");
            let whole = select(&view, &path(&format!("{named}/value"))).unwrap();
            assert_eq!(whole, [&value], "{named}");
        }
        // The name of another IE says which IE the entry is, with a view and without.
        let other = IE_NAMES.iter().find(|(id, _)| *id != cause);
        let other = path(&format!("0/{}/value", other.unwrap().1));
        let refused = select(&view, &other).unwrap_err();
        let text = refused.to_string();
        assert!(text.starts_with("this IE is Cause, not "), "{refused}");
        assert!(text.contains("is not a member of Cause, which has radioNetwork"));
        let checked = |rest: &str| check_path("UEContextReleaseCommand", &path(rest));
        assert_eq!(checked("Cause/Cause/nas"), Ok(()));
        assert!(checked("Cause/Cause/misspelled").is_err());
        let names = |rest: &str| enumerated_at("UEContextReleaseCommand", &path(rest));
        assert_eq!(names("*/Cause/nas"), names("Cause/value/nas"));
        assert!(names("*/Cause/nas").unwrap().is_some());
        // An edit takes the same path, and the entry itself where the path ends with
        // its name.
        assert_eq!(set(&mut view, &other, json!(1)), Err(refused));
        set(&mut view, &path("0/Cause/nas"), json!("normal-release")).unwrap();
        let edited = select(&view, &path("Cause/value")).unwrap();
        assert_eq!(edited, [&json!({"nas": "normal-release"})]);
        insert(&mut view, &path("0/Cause"), entry.clone()).unwrap();
        assert_eq!(select(&view, &path("*/Cause")).unwrap().len(), 2);
        remove(&mut view, &path("1/Cause")).unwrap();
        assert_eq!(select(&view, &path("*/Cause")).unwrap(), [&entry]);
    }

    #[test]
    fn an_entry_that_is_not_known_is_the_ie_that_a_segment_names_where_no_member_is() {
        // Where a view is not there to say which IE an entry is, a segment that names
        // an IE is taken for it, and the path goes on in the type of that IE: unless a
        // value has a member of that name and of another type, which it may be.
        let cause = ie_id("Cause").expect("an IE of the protocol");
        for (shape, stays) in [
            (Shape::Ie(None), Shape::Ie(Some(cause))),
            (
                Shape::Extension(None, ""),
                Shape::Extension(Some(cause), ""),
            ),
        ] {
            assert_eq!(TABLES.named_ie(shape, "cause", true), Some(stays));
            assert_eq!(TABLES.named_ie(shape, "cause", false), None);
        }
        assert_eq!(
            TABLES.named_ie(Shape::Ie(Some(cause + 1)), "Cause", true),
            None
        );
        assert_eq!(TABLES.named_ie(Shape::Plain, "Cause", true), None);
        let (mut taken, mut left, mut spelled) = (0, 0, 0);
        for (of, members) in all_members() {
            for (member, shape, _) in members {
                let Some(id) = ie_id(member) else {
                    continue;
                };
                // A member of the value of an IE that is named as the IE itself is
                // spelled otherwise, which is how a path tells the two.
                let own = match TABLES.resolved(TABLES.ie_value(Some(id), "")) {
                    Shape::Transfer(contained) => Shape::Named(contained),
                    own => own,
                };
                if own == Shape::Named(of) {
                    assert_ne!(*member, ie_name(id), "{of}.{member}");
                    assert!(
                        !TABLES.stays_on(id, "", member) && TABLES.stays_on(id, "", &ie_name(id))
                    );
                    spelled += 1;
                }
                let own = TABLES.resolved(TABLES.ie_value(Some(id), ""));
                match TABLES.named_ie(Shape::Ie(None), member, true) {
                    Some(_) => {
                        let same = TABLES.resolved(*shape) == own;
                        assert!(same, "{of}.{member}");
                        taken += 1;
                    }
                    None => left += 1,
                }
            }
        }
        assert!(taken > 0 && left > 0, "{taken} {left}");
        // TS 36.413 has one such member, and TS 38.413 none.
        assert!(spelled <= 1, "{spelled}");
    }

    #[test]
    fn only_a_string_of_bits_names_a_member_as_the_ie_that_holds_it_does() {
        // A segment under an IE that names a member of the IE is that member, and under
        // a transfer one of the transfer: of the values that they hold, a string of
        // bits whose size varies alone has a member of such a name, its `value`, which
        // a path reaches after the `value` of the IE.
        let mut bits = 0;
        // Each IE with the type of its identifier, and with the one that each type which
        // it extends gives it.
        let of_their_own = IE_TYPES.iter().map(|(id, ..)| (id, ""));
        let extensions = EXTENSION_TYPES
            .iter()
            .map(|(id, extended, _)| (id, *extended));
        for (id, extended) in of_their_own.chain(extensions) {
            let mut shape = TABLES.resolved(TABLES.ie_value(Some(*id), extended));
            if let Shape::Transfer(contained) = shape {
                shape = TABLES.resolved(Shape::Named(contained));
            }
            let members = TABLES.members(shape, None).unwrap_or_default();
            for own in VALUES.into_iter().chain(["id", "criticality", "octets"]) {
                let same = members.iter().any(|(name, ..)| same_name(name, own));
                assert_eq!(same, shape == Shape::Bits && own == "value", "{id}: {own}");
            }
            bits += usize::from(shape == Shape::Bits);
        }
        assert!(bits > 0);
    }

    #[test]
    fn an_extension_has_the_type_that_the_type_it_extends_gives_it() {
        // An extension container extends the type that has it, whatever type that is.
        let mut containers = 0;
        for (name, _) in all_members() {
            let members = TABLES.members(Shape::Named(name), None).unwrap_or_default();
            for (_, shape, _) in members {
                if let Shape::Extensions(extended) = shape {
                    assert_eq!(extended, name);
                    assert_eq!(entry_shape(shape), Shape::Extension(None, name));
                    containers += 1;
                }
            }
        }
        assert!(containers > 0);
        // An IE whose identifier has a type for each type that it extends is of that type
        // there, and of none elsewhere: its identifier alone does not say its type.
        for (id, extended, ty) in EXTENSION_TYPES {
            assert!(kind_of(extended).is_some(), "{extended}");
            assert!(IE_TYPES.iter().all(|(known, ..)| known != id), "{id}");
            assert_eq!(
                TABLES.ie_value(Some(*id), extended),
                Shape::Named(type_name(ty))
            );
            assert_eq!(TABLES.ie_value(Some(*id), ""), Shape::Unknown);
            let extension = Shape::Extension(Some(*id), extended);
            let value = TABLES.members(extension, None).unwrap();
            let (_, value, _) = member(&value, "extensionValue").unwrap();
            assert_eq!(*value, Shape::Named(type_name(ty)), "{id} in {extended}");
            // Its name stays on it, as that of any IE does.
            let named = TABLES.named_ie(Shape::Extension(None, extended), &ie_name(*id), true);
            assert_eq!(named, Some(extension));
        }
    }

    #[test]
    fn the_ies_that_a_value_holds_are_written_by_name_and_read_back() {
        let cause = ie_id("Cause").expect("an IE of the protocol");
        let value = json!({"radioNetwork": "unspecified"});
        // The entry of a view, with what it keeps of what was received, and as it is
        // written.
        let entry = |received: bool| {
            let mut entry = json!({"id": cause, "criticality": "ignore", "value": value});
            if received {
                entry["_raw_value"] = json!("0000");
                entry["_original_id"] = json!(cause);
            }
            entry
        };
        // No table has the criticality of the IEs of a message that is not one of the
        // protocol, as that of a transfer: it is written, and asked for.
        let shown = named_ies("no message", json!([entry(true)])).unwrap();
        assert_eq!(shown, json!([{"Cause": value, "criticality": "ignore"}]));
        let listed = listed_ies("no message", &shown).unwrap();
        assert_eq!(listed, json!([entry(false)]));
        let refused = listed_ies("no message", &json!([{"Cause": value}])).unwrap_err();
        assert_eq!(refused.kind(), crate::ErrorKind::Form);
        let text = refused.to_string();
        assert!(text.contains("says its \"criticality\""), "{refused}");
        // An entry that is written as a view has it stays as it is.
        assert_eq!(listed_ies("no message", &listed), Ok(listed));
        // One that did not decode has its octets, which are sent as they are.
        let mut undecoded = entry(true);
        undecoded["value"] = json!("0000");
        undecoded["_decode_error"] = json!("why");
        let shown = named_ies("no message", json!([undecoded])).unwrap();
        let octets = json!({"Cause": {"octets": "0000"}, "criticality": "ignore"});
        assert_eq!(shown, json!([octets]));
        let listed = listed_ies("no message", &shown).unwrap();
        let octets = json!({"id": cause, "criticality": "ignore", "octets": "0000"});
        assert_eq!(listed, json!([octets]));
        // A tree is not read deeper than a view is encoded.
        let mut deep = json!(1);
        for _ in 0..258 {
            deep = json!([deep]);
        }
        let deep = json!([{"Cause": deep, "criticality": "ignore"}]);
        let refused = listed_ies("no message", &deep).unwrap_err();
        assert_eq!(refused.kind(), crate::ErrorKind::Limit);
        assert!(refused.to_string().contains("nests deeper"), "{refused}");
    }

    #[test]
    fn the_octets_of_an_ie_carry_a_type_of_the_list_by_a_name_that_no_member_has() {
        for (id, name, ty) in CARRIED {
            // The value of the IE is octets, and what they carry a type of the list, by
            // the name that ASN.1 gives it.
            let (_, octets, contained) = IE_TYPES.iter().find(|(known, ..)| known == id).unwrap();
            assert!(contained.is_empty() && kind_of(type_name(octets)).is_none());
            assert!(kind_of(type_name(ty)).is_some() && same_name(name, type_name(ty)));
            assert_eq!(TABLES.ie_value(Some(*id), ""), Shape::Carrier(*id));
            // A view is encoded with any member of that name as that type.
            for (of, members) in all_members() {
                let same = members.iter().any(|(member, ..)| same_name(member, name));
                assert!(!same, "{of}: {name}");
            }
        }
    }

    #[test]
    fn what_is_nested_deeper_than_a_path_goes_is_one_value() {
        // Lists in lists, ten levels deeper than a path goes: the path of the one value
        // has a segment for each level that is walked, and the value is what is under
        // them, whole.
        const SEGMENTS: usize = 64;
        let mut deep = json!([{"a": 1}]);
        for _ in 0..SEGMENTS + 10 {
            deep = json!([deep]);
        }
        let found = paths(&deep);
        assert_eq!(found.len(), 1);
        let (path, value) = &found[0];
        assert_eq!(path.matches('/').count(), SEGMENTS);
        assert_eq!(select(&deep, path), Ok(vec![value]));
        // What is not that deep is listed value by value.
        let mut shallow = json!([{"a": 1}]);
        for _ in 0..SEGMENTS - 10 {
            shallow = json!([shallow]);
        }
        let found = paths(&shallow);
        assert_eq!(found.len(), 1);
        assert!(found[0].0.ends_with("/0/a") && found[0].1 == json!(1));
    }

    #[test]
    fn an_entry_of_a_list_of_ies_is_known_by_its_members() {
        // An IE is written with a number or a name as its `id`: no type has a member of
        // that name with such a value, nor one named as the members of an IE or of a
        // transfer in a view.
        for (name, members) in all_members() {
            for (member, shape, _) in members {
                assert!(
                    !["octets", "decoded", "extensionValue"].contains(member),
                    "{name}.{member}"
                );
                if *member == "id" {
                    assert!(
                        matches!(TABLES.resolved(*shape), Shape::Named(_)),
                        "{name}.id"
                    );
                }
            }
        }
    }
}
