//! The values of the bindings as JSON, and JSON as values of the bindings: an encoder
//! and a decoder for the codec traits of `rasn`, which write and read a
//! [`serde_json::Value`] and no text.
//!
//! They are modelled on the codec of the JSON Encoding Rules of rasn 0.29, `rasn::jer`
//! (copyright of the rasn developers, under the MIT or the Apache-2.0 licence, and
//! taken here under the Apache-2.0 licence): a value has the form that it had with
//! that codec, which is the form of ITU-T X.697 (02/2021) but for what follows.
//!
//! | ASN.1 | JSON |
//! | --- | --- |
//! | `BOOLEAN` | `true` or `false` |
//! | `INTEGER` | A number. One that is below -2^63 or above 2^64-1 is a string of its decimal digits, and no other integer is: X.697 §21 has a number of any size, and `serde_json` has no exact number for these without a feature that every user of the crate would have to build with |
//! | `ENUMERATED` | The name of the value, a string |
//! | `BIT STRING` of one size | Its octets in hexadecimal, the last one padded with zero bits |
//! | `BIT STRING` whose size varies | `{"value": HEXADECIMAL, "length": BITS}`. Where the type gives one size and an extension marker, a string of that size is its octets in hexadecimal, as rasn wrote every string of such a type, and one of another size has its length: X.697 §24.3 has every string of such a type with its length |
//! | `OCTET STRING` | Its octets in hexadecimal |
//! | `NULL` | `null` |
//! | `OBJECT IDENTIFIER` | Its arcs in decimal, with a full stop between them |
//! | A character string | A string |
//! | `SEQUENCE` | An object with a member for each component, by the name that ASN.1 gives it, and none for a component that is absent |
//! | `SEQUENCE OF` | A list |
//! | `CHOICE` | An object with one member: the name of the alternative, with its value |
//! | An open type | Its octets in hexadecimal, as rasn wrote it: X.697 §41 has the value that the octets hold, which the views of the crate show beside them |
//!
//! Hexadecimal is written in upper case and read in either. What neither protocol
//! has is not written and not read: `REAL`, the time types, `SET`, `SET OF` and the
//! character strings of other alphabets than those of `UTF8String`, `PrintableString`,
//! `VisibleString`, `IA5String` and `NumericString`.
//!
//! The decoder is strict, as nothing reads its result back to compare: a member that
//! the type does not have, a second alternative of a `CHOICE`, padding bits that are
//! not zero and an integer outside the constraint of its type are each refused, with
//! the names on the way to the value.

use rasn::error::{DecodeError, EncodeError};
use rasn::types::fields::Field;
use rasn::types::{self, Any, BitStr, BitString, Constraints, Identifier, Tag, TagTree};
use rasn::{Codec, Decode, Encode};
use serde_json::{Map, Number, Value};

use crate::inspect_paths::same_name;
use crate::{Error, ErrorKind};

// ── Hexadecimal ─────────────────────────────────────────────────────────────

/// Octets in hexadecimal, in upper case.
pub(crate) fn hex(octets: &[u8]) -> String {
    // A digit is its half octet after `0`, and seven further for one after `9`, which
    // are the characters between `9` and `A`. The loop has no table and no branch, so
    // the compiler writes it for several octets at once.
    let digit = |half: u8| half + b'0' + u8::from(half > 9) * (b'A' - b'9' - 1);
    let mut text = vec![0; octets.len() * 2];
    let (pairs, _) = text.as_chunks_mut::<2>();
    for (pair, octet) in pairs.iter_mut().zip(octets) {
        *pair = [digit(octet >> 4), digit(octet & 15)];
    }
    // The digits are ASCII.
    String::from_utf8(text).unwrap_or_default()
}

/// The octets that `text` writes in hexadecimal, two digits for each, in either case.
pub(crate) fn unhex(text: &str) -> Option<Vec<u8>> {
    /// What is no digit: a value that no half octet has.
    const NONE: u8 = 0x80;
    // The value of a digit, without a branch that leaves the loop: what is no digit
    // is told after it, so that the compiler writes the loop for several octets at
    // once. A letter is taken in lower case, which differs from upper case by one bit.
    let digit = |digit: u8| {
        let (number, letter) = (digit.wrapping_sub(b'0'), (digit | 0x20).wrapping_sub(b'a'));
        match (number < 10, letter < 6) {
            (true, _) => number,
            (false, true) => letter + 10,
            (false, false) => NONE,
        }
    };
    let (pairs, rest) = text.as_bytes().as_chunks::<2>();
    if !rest.is_empty() {
        return None;
    }
    let mut octets = vec![0; pairs.len()];
    let mut other = 0;
    for (octet, [high, low]) in octets.iter_mut().zip(pairs) {
        let (high, low) = (digit(*high), digit(*low));
        other |= high | low;
        *octet = high << 4 | low & 15;
    }
    (other & NONE == 0).then_some(octets)
}

/// The start of what is written, which says which value it is.
fn written(value: &Value) -> String {
    let text = value.to_string();
    match text.char_indices().nth(40) {
        Some((end, _)) => format!("{}…", &text[..end]),
        None => text,
    }
}

// ── From a value to JSON ────────────────────────────────────────────────────

/// What is being written, which the next value goes into.
enum Open {
    /// The members of a SEQUENCE, or the alternative of a CHOICE: the names of those
    /// that are still to come are the last ones of the names of the encoder.
    Members(Map<String, Value>),
    /// The entries of a list.
    Entries(Vec<Value>),
}

/// Writes a value of the bindings as JSON.
pub(crate) struct Encoder {
    /// What is being written, the innermost last.
    open: Vec<Open>,
    /// The names of the members that are still to come, of every SEQUENCE that is
    /// being written: the next one last.
    names: Vec<&'static str>,
    /// The value, once it is written.
    value: Option<Value>,
}

/// The JSON form of `value`.
pub(crate) fn to_value<T: Encode>(value: &T) -> Result<Value, Error> {
    // What does not depend on `T` is in functions that are compiled once.
    let mut encoder = Encoder::new();
    let written = value.encode(&mut encoder);
    finished(written, encoder)
}

/// What an encoder wrote, or why it wrote nothing.
fn finished(written: Result<(), EncodeError>, encoder: Encoder) -> Result<Value, Error> {
    let unwritten =
        |why: String| Error::limit(format!("the JSON forms do not represent this value: {why}"));
    written.map_err(|error| unwritten(crate::error::encode_reason(&error)))?;
    encoder
        .value
        .ok_or_else(|| unwritten("it is absent".to_string()))
}

/// What the JSON forms do not write: a type that neither protocol has.
fn no_form(of: &str) -> EncodeError {
    rasn::enc::Error::custom(format!("they have no {of}"), Codec::Jer)
}

impl Encoder {
    /// `value` is the next value: a member of the SEQUENCE that is being written, by
    /// the next of its names, an entry of the list, or the value itself.
    fn put(&mut self, value: Value) {
        match self.open.last_mut() {
            Some(Open::Members(members)) => {
                if let Some(name) = self.names.pop() {
                    members.insert(name.to_string(), value);
                }
            }
            Some(Open::Entries(entries)) => entries.push(value),
            None => self.value = Some(value),
        }
    }

    /// The next member of the SEQUENCE that is being written is absent.
    fn absent(&mut self) {
        if let Some(Open::Members(_)) = self.open.last() {
            self.names.pop();
        }
    }

    /// An encoder that has written nothing.
    fn new() -> Self {
        Self {
            open: Vec::new(),
            names: Vec::new(),
            value: None,
        }
    }

    /// The members of a SEQUENCE come next, in the order of its components and of its
    /// additions. They are taken as slices: an iterator over the components of a type
    /// would be compiled for each SEQUENCE.
    fn members(&mut self, root: &[Field], extended: &[Field]) {
        let first = self.names.len();
        let fields = root.iter().chain(extended);
        self.names.extend(fields.map(|field| field.name));
        self.names[first..].reverse();
        self.open.push(Open::Members(Map::new()));
    }

    /// One member of this name comes next: the alternative of a CHOICE.
    fn member(&mut self, name: &'static str) {
        self.names.push(name);
        self.open.push(Open::Members(Map::new()));
    }

    /// What was opened last is written whole: it is the next value.
    fn close(&mut self) {
        match self.open.pop() {
            Some(Open::Members(members)) => self.put(Value::Object(members)),
            Some(Open::Entries(entries)) => self.put(Value::Array(entries)),
            None => {}
        }
    }

    /// An integer: a number, or its decimal digits where JSON has no exact number.
    fn integer(&mut self, value: i128) {
        self.put(match (i64::try_from(value), u64::try_from(value)) {
            (Ok(number), _) => Value::Number(Number::from(number)),
            (_, Ok(number)) => Value::Number(Number::from(number)),
            _ => Value::String(value.to_string()),
        });
    }

    /// A bit string: its octets, with its length unless its type gives it.
    fn bits(&mut self, constraints: &Constraints, value: &BitStr) {
        let text = hex(&octets_of(value));
        self.put(match one_size(constraints) {
            Some((size, _)) if size == value.len() => Value::String(text),
            // rasn writes a string of a type of one size without its length.
            Some((_, false)) => Value::String(text),
            _ => {
                let mut members = Map::new();
                members.insert("value".into(), Value::String(text));
                members.insert("length".into(), Value::Number(Number::from(value.len())));
                Value::Object(members)
            }
        });
    }

    /// A string.
    fn text(&mut self, text: &[u8]) -> Result<(), EncodeError> {
        match std::str::from_utf8(text) {
            Ok(text) => {
                self.put(Value::String(text.to_string()));
                Ok(())
            }
            Err(_) => Err(rasn::enc::Error::custom(
                "a character string that is not UTF-8",
                Codec::Jer,
            )),
        }
    }
}

/// The size that a constraint gives a string, when it gives one, and whether a string
/// may have another: the constraint has an extension marker.
fn one_size(constraints: &Constraints) -> Option<(usize, bool)> {
    let size = constraints.size()?;
    let one = size
        .constraint
        .is_fixed()
        .then(|| size.constraint.as_start());
    Some((*one.flatten()?, size.extensible.is_some()))
}

/// The octets of a bit string, the last one padded with zero bits.
fn octets_of(bits: &BitStr) -> Vec<u8> {
    // A string that starts an octet, as every string of the bindings does: its whole
    // octets, then the bits of its last one.
    if let Some((None, whole, last)) = bits.domain().region() {
        let mut octets = Vec::with_capacity(whole.len() + 1);
        octets.extend_from_slice(whole);
        octets.extend(last.map(|last| last.load_value()));
        return octets;
    }
    bit_by_bit(bits)
}

/// The octets of a bit string that starts anywhere in an octet.
fn bit_by_bit(bits: &BitStr) -> Vec<u8> {
    let octet = |bits: &BitStr| {
        let read = bits
            .iter()
            .fold(0u8, |octet, bit| octet << 1 | u8::from(*bit));
        read << (8 - bits.len())
    };
    bits.chunks(8).map(octet).collect()
}

/// The name of the alternative of a CHOICE that has `tag`.
fn alternative_of(
    tag: Tag,
    root: &'static [TagTree],
    extended: Option<&'static [TagTree]>,
    names: &'static [&'static str],
) -> Option<&'static str> {
    let mut tags = Vec::new();
    for tree in root.iter().chain(extended.into_iter().flatten()) {
        leaves(tree, &mut tags);
    }
    let index = tags.iter().position(|known| *known == tag)?;
    names.get(index).copied()
}

/// The tags of a tree, in its order.
fn leaves(tree: &TagTree, tags: &mut Vec<Tag>) {
    match tree {
        TagTree::Leaf(tag) => tags.push(*tag),
        TagTree::Choice(trees) => trees.iter().for_each(|tree| leaves(tree, tags)),
    }
}

impl rasn::Encoder<'_> for Encoder {
    type Ok = ();
    type Error = EncodeError;
    type AnyEncoder<'this, const R: usize, const E: usize> = Encoder;

    fn codec(&self) -> Codec {
        Codec::Jer
    }

    fn encode_any(&mut self, _: Tag, value: &Any, _: Identifier) -> Result<(), EncodeError> {
        self.put(Value::String(hex(value.as_bytes())));
        Ok(())
    }

    fn encode_bool(&mut self, _: Tag, value: bool, _: Identifier) -> Result<(), EncodeError> {
        self.put(Value::Bool(value));
        Ok(())
    }

    fn encode_bit_string(
        &mut self,
        _: Tag,
        constraints: &Constraints,
        value: &BitStr,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        self.bits(constraints, value);
        Ok(())
    }

    fn encode_enumerated<E: types::Enumerated>(
        &mut self,
        _: Tag,
        value: &E,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        self.put(Value::String(value.identifier().to_string()));
        Ok(())
    }

    fn encode_object_identifier(
        &mut self,
        _: Tag,
        value: &[u32],
        _: Identifier,
    ) -> Result<(), EncodeError> {
        let arcs: Vec<_> = value.iter().map(u32::to_string).collect();
        self.put(Value::String(arcs.join(".")));
        Ok(())
    }

    fn encode_integer<I: types::IntegerType>(
        &mut self,
        _: Tag,
        _: &Constraints,
        value: &I,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        match value.clone().try_into() {
            Ok(value) => self.integer(value),
            // An integer of more than 127 bits.
            Err(_) => self.put(Value::String(value.to_string())),
        }
        Ok(())
    }

    fn encode_real<R: types::RealType>(
        &mut self,
        _: Tag,
        _: &Constraints,
        _: &R,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        Err(no_form("REAL"))
    }

    fn encode_null(&mut self, _: Tag, _: Identifier) -> Result<(), EncodeError> {
        self.put(Value::Null);
        Ok(())
    }

    fn encode_octet_string(
        &mut self,
        _: Tag,
        _: &Constraints,
        value: &[u8],
        _: Identifier,
    ) -> Result<(), EncodeError> {
        self.put(Value::String(hex(value)));
        Ok(())
    }

    fn encode_general_string(
        &mut self,
        _: Tag,
        _: &Constraints,
        _: &types::GeneralString,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        Err(no_form("GeneralString"))
    }

    fn encode_graphic_string(
        &mut self,
        _: Tag,
        _: &Constraints,
        _: &types::GraphicString,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        Err(no_form("GraphicString"))
    }

    fn encode_utf8_string(
        &mut self,
        _: Tag,
        _: &Constraints,
        value: &str,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        self.put(Value::String(value.to_string()));
        Ok(())
    }

    fn encode_visible_string(
        &mut self,
        _: Tag,
        _: &Constraints,
        value: &types::VisibleString,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        self.text(value.as_iso646_bytes())
    }

    fn encode_ia5_string(
        &mut self,
        _: Tag,
        _: &Constraints,
        value: &types::Ia5String,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        self.text(value.as_iso646_bytes())
    }

    fn encode_printable_string(
        &mut self,
        _: Tag,
        _: &Constraints,
        value: &types::PrintableString,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        self.text(value.as_bytes())
    }

    fn encode_numeric_string(
        &mut self,
        _: Tag,
        _: &Constraints,
        value: &types::NumericString,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        self.text(value.as_bytes())
    }

    fn encode_teletex_string(
        &mut self,
        _: Tag,
        _: &Constraints,
        _: &types::TeletexString,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        Err(no_form("TeletexString"))
    }

    fn encode_bmp_string(
        &mut self,
        _: Tag,
        _: &Constraints,
        _: &types::BmpString,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        Err(no_form("BMPString"))
    }

    fn encode_generalized_time(
        &mut self,
        _: Tag,
        _: &types::GeneralizedTime,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        Err(no_form("GeneralizedTime"))
    }

    fn encode_utc_time(
        &mut self,
        _: Tag,
        _: &types::UtcTime,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        Err(no_form("UTCTime"))
    }

    fn encode_date(&mut self, _: Tag, _: &types::Date, _: Identifier) -> Result<(), EncodeError> {
        Err(no_form("DATE"))
    }

    fn encode_explicit_prefix<V: Encode>(
        &mut self,
        _: Tag,
        value: &V,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        value.encode(self)
    }

    fn encode_sequence<'b, const RL: usize, const EL: usize, C, F>(
        &'b mut self,
        _: Tag,
        encoder_scope: F,
        _: Identifier,
    ) -> Result<(), EncodeError>
    where
        C: types::Constructed<RL, EL>,
        F: FnOnce(&mut Self::AnyEncoder<'b, RL, EL>) -> Result<(), EncodeError>,
    {
        let (root, extended) = (C::FIELDS, C::EXTENDED_FIELDS);
        self.members(&root, extended.as_deref().unwrap_or_default());
        encoder_scope(self)?;
        self.close();
        Ok(())
    }

    fn encode_sequence_of<E: Encode>(
        &mut self,
        _: Tag,
        value: &[E],
        _: &Constraints,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        self.open
            .push(Open::Entries(Vec::with_capacity(value.len())));
        for entry in value {
            entry.encode(self)?;
        }
        self.close();
        Ok(())
    }

    fn encode_set<'b, const RL: usize, const EL: usize, C, F>(
        &'b mut self,
        _: Tag,
        _: F,
        _: Identifier,
    ) -> Result<(), EncodeError>
    where
        C: types::Constructed<RL, EL>,
        F: FnOnce(&mut Self::AnyEncoder<'b, RL, EL>) -> Result<(), EncodeError>,
    {
        Err(no_form("SET"))
    }

    fn encode_set_of<E: Encode + Eq + core::hash::Hash>(
        &mut self,
        _: Tag,
        _: &types::SetOf<E>,
        _: &Constraints,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        Err(no_form("SET OF"))
    }

    fn encode_some<E: Encode>(&mut self, value: &E, _: Identifier) -> Result<(), EncodeError> {
        value.encode(self)
    }

    fn encode_some_with_tag_and_constraints<E: Encode>(
        &mut self,
        _: Tag,
        _: &Constraints,
        value: &E,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        value.encode(self)
    }

    fn encode_none<E: Encode>(&mut self, _: Identifier) -> Result<(), EncodeError> {
        self.absent();
        Ok(())
    }

    fn encode_none_with_tag(&mut self, _: Tag, _: Identifier) -> Result<(), EncodeError> {
        self.absent();
        Ok(())
    }

    fn encode_choice<E: Encode + types::Choice>(
        &mut self,
        _: &Constraints,
        tag: Tag,
        encode_fn: impl FnOnce(&mut Self) -> Result<Tag, EncodeError>,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        let name = alternative_of(tag, E::VARIANTS, E::EXTENDED_VARIANTS, E::IDENTIFIERS);
        let name = name.ok_or_else(|| EncodeError::variant_not_in_choice(Codec::Jer))?;
        self.member(name);
        encode_fn(self)?;
        self.close();
        Ok(())
    }

    fn encode_extension_addition<E: Encode>(
        &mut self,
        _: Tag,
        _: &Constraints,
        value: E,
        _: Identifier,
    ) -> Result<(), EncodeError> {
        value.encode(self)
    }

    fn encode_extension_addition_group<const RL: usize, const EL: usize, E>(
        &mut self,
        value: Option<&E>,
        _: Identifier,
    ) -> Result<(), EncodeError>
    where
        E: Encode + types::Constructed<RL, EL>,
    {
        match value {
            Some(value) => value.encode(self),
            None => {
                self.absent();
                Ok(())
            }
        }
    }
}

// ── From JSON to a value ────────────────────────────────────────────────────

/// What a value that is read is named by, in what holds it.
#[derive(Clone, Copy)]
enum Name {
    /// The value that is read first has no name.
    Root,
    /// A member of a SEQUENCE or the alternative of a CHOICE, by the name that ASN.1
    /// gives it.
    Member(&'static str),
    /// An entry of a list, by its position.
    Entry(usize),
}

/// Why JSON is not a value of its type: the kind of the error, the names on the way to
/// where the fault is, and what is wrong there.
struct Refusal {
    kind: ErrorKind,
    at: Vec<String>,
    why: String,
}

/// Reads JSON as a value of the bindings.
pub(crate) struct Decoder<'v> {
    /// What is still to be read, the next one last: a value with its name, or no value
    /// for a member that its SEQUENCE is without.
    values: Vec<(Name, Option<&'v Value>)>,
    /// The names on the way to the value that is being read.
    at: Vec<Name>,
    /// Why the JSON is refused, once it is: the first fault.
    refusal: Option<Refusal>,
}

/// The value of the type `T` that `value` writes.
pub(crate) fn from_value<T: Decode>(value: &Value) -> Result<T, Error> {
    // What does not depend on `T` is in functions that are compiled once.
    let mut decoder = Decoder::new(value);
    match T::decode(&mut decoder) {
        Ok(value) => Ok(value),
        Err(error) => Err(refused(
            error,
            decoder,
            T::IDENTIFIER.0.unwrap_or("its type"),
        )),
    }
}

/// Why a decoder read no value of the type that ASN.1 names `name`.
fn refused(error: DecodeError, decoder: Decoder, name: &str) -> Error {
    let (kind, at, why) = match decoder.refusal {
        Some(refusal) => (refusal.kind, refusal.at, refusal.why),
        // What the bindings refuse themselves, as a string of another size than its
        // type has.
        None => {
            let (fields, why) = crate::error::decode_reason(&error);
            let spelled = |field: &&str| {
                let spelling = crate::inspect_paths::member_spelling(field);
                spelling.unwrap_or(field).to_string()
            };
            (ErrorKind::Value, fields.iter().map(spelled).collect(), why)
        }
    };
    let what = match at.is_empty() {
        true => format!("this is not a value of {name}"),
        false => format!("this is not a value of {name}, at {}", at.join("/")),
    };
    Error::new(kind, what).because(why)
}

/// What the JSON forms do not read: a type that neither protocol has.
fn unread(of: &str) -> DecodeError {
    rasn::de::Error::custom(format!("the JSON forms have no {of}"), Codec::Jer)
}

impl<'v> Decoder<'v> {
    /// A decoder of `value`, which has read nothing.
    fn new(value: &'v Value) -> Self {
        Self {
            values: vec![(Name::Root, Some(value))],
            at: Vec::new(),
            refusal: None,
        }
    }

    /// The error of a value that is refused: `why`, at the value that `name` names in
    /// what is being read. The first refusal is the one that is told.
    fn refuse(&mut self, kind: ErrorKind, name: Name, why: String) -> DecodeError {
        if self.refusal.is_none() {
            let segment = |name: &Name| match name {
                Name::Root => None,
                Name::Member(name) => Some(name.to_string()),
                Name::Entry(position) => Some(position.to_string()),
            };
            let at = self.at.iter().chain([&name]).filter_map(segment).collect();
            self.refusal = Some(Refusal { kind, at, why });
        }
        rasn::de::Error::custom("the JSON is not a value of its type", Codec::Jer)
    }

    /// The error of a value that is not written as its type is: `expected` says what is
    /// expected, at the value `found` that `name` names.
    fn mismatch(&mut self, name: Name, expected: &str, found: &Value) -> DecodeError {
        let why = format!("{expected}, and {} is written", written(found));
        self.refuse(ErrorKind::Value, name, why)
    }

    /// The next value, with its name. A member that its SEQUENCE is without is an
    /// error: its type always has it.
    fn next(&mut self) -> Result<(Name, &'v Value), DecodeError> {
        match self.values.pop() {
            Some((name, Some(value))) => Ok((name, value)),
            Some((name, None)) => {
                let why = "the member is missing".to_string();
                Err(self.refuse(ErrorKind::Value, name, why))
            }
            None => Err(rasn::de::Error::custom(
                "a value is read where none is",
                Codec::Jer,
            )),
        }
    }

    /// Whether the next value is there: a member that its SEQUENCE is without is taken
    /// out of what is to be read.
    fn present(&mut self) -> bool {
        match self.values.last() {
            Some((_, None)) => {
                self.values.pop();
                false
            }
            _ => true,
        }
    }

    /// The next value, a string, with its name: `expected` says what it writes.
    fn string(&mut self, expected: &str) -> Result<(Name, &'v str), DecodeError> {
        let (name, value) = self.next()?;
        match value.as_str() {
            Some(text) => Ok((name, text)),
            None => Err(self.mismatch(name, expected, value)),
        }
    }

    /// The next value, octets in hexadecimal.
    fn octets(&mut self) -> Result<Vec<u8>, DecodeError> {
        const OCTETS: &str = "octets in hexadecimal, two digits for each, are expected";
        let (name, value) = self.next()?;
        match value.as_str().and_then(unhex) {
            Some(octets) => Ok(octets),
            None => Err(self.mismatch(name, OCTETS, value)),
        }
    }

    /// The next value, a character string.
    fn text(&mut self) -> Result<(Name, &'v str), DecodeError> {
        self.string("a string is expected")
    }

    /// The next value, an integer: a number, or the decimal digits of one that JSON
    /// has no exact number for. One that is outside the constraint of its type, which
    /// has no extension marker, is refused.
    fn integer(&mut self, constraints: &Constraints) -> Result<Integral, DecodeError> {
        let (name, value) = self.next()?;
        let read = match value {
            Value::Number(number) => (number.as_i64().map(i128::from))
                .or_else(|| number.as_u64().map(i128::from))
                .map(Integral::Small),
            // The digits of an integer that JSON has no exact number for, and of no other.
            Value::String(digits) => decimal(digits).filter(|read| match read {
                Integral::Small(value) => {
                    i64::try_from(*value).is_err() && u64::try_from(*value).is_err()
                }
                Integral::Large(_) => true,
            }),
            _ => None,
        };
        let Some(read) = read else {
            return Err(self.mismatch(name, "a number is expected", value));
        };
        if let Some(constraint) = constraints
            .value()
            .filter(|value| value.extensible.is_none())
        {
            let (start, end) = (
                constraint.constraint.as_start(),
                constraint.constraint.as_end(),
            );
            let inside = match &read {
                Integral::Small(value) => {
                    start.is_none_or(|start| start <= value) && end.is_none_or(|end| value <= end)
                }
                // More than 127 bits: within no constraint that has a bound.
                Integral::Large(_) => start.is_none() && end.is_none(),
            };
            if !inside {
                let bound = |bound: Option<&i128>| bound.map(i128::to_string).unwrap_or_default();
                let why = format!(
                    "{} is outside its constraint ({}..{})",
                    written(value),
                    bound(start),
                    bound(end)
                );
                return Err(self.refuse(ErrorKind::Value, name, why));
            }
        }
        Ok(read)
    }

    /// The next value, a bit string of a type of these constraints.
    fn bits(&mut self, constraints: &Constraints) -> Result<BitString, DecodeError> {
        const WITH_LENGTH: &str = "a bit string whose size varies is expected, which is \
                                   written as {\"value\": HEXADECIMAL, \"length\": BITS}";
        let (name, value) = self.next()?;
        let size = one_size(constraints);
        let (digits, length) = match (value, size) {
            // A string of the one size of its type is its octets.
            (Value::String(digits), Some((size, _))) => (digits.as_str(), size),
            (Value::Object(members), None | Some((_, true))) => {
                let digits = members.get("value").and_then(Value::as_str);
                let length = members.get("length").and_then(Value::as_u64);
                let length = length.and_then(|length| usize::try_from(length).ok());
                match digits.zip(length).filter(|_| members.len() == 2) {
                    Some(read) => read,
                    None => return Err(self.mismatch(name, WITH_LENGTH, value)),
                }
            }
            (_, Some((size, false))) => {
                let expected = format!("a bit string of {size} bits is expected, in hexadecimal");
                return Err(self.mismatch(name, &expected, value));
            }
            (_, Some((size, true))) => {
                let expected = format!(
                    "a bit string of {size} bits is expected, in hexadecimal, or one of another \
                     size as {{\"value\": HEXADECIMAL, \"length\": BITS}}"
                );
                return Err(self.mismatch(name, &expected, value));
            }
            (_, None) => return Err(self.mismatch(name, WITH_LENGTH, value)),
        };
        let Some(octets) = unhex(digits) else {
            let why = format!("{digits:?} is not hexadecimal, two digits for each octet");
            return Err(self.refuse(ErrorKind::Value, name, why));
        };
        if octets.len() != length.div_ceil(8) {
            let why = format!(
                "{} octets are written for a bit string of {length} bits, which has {}",
                octets.len(),
                length.div_ceil(8)
            );
            return Err(self.refuse(ErrorKind::Value, name, why));
        }
        let padding = octets.len() * 8 - length;
        if octets
            .last()
            .is_some_and(|last| last.trailing_zeros() < padding as u32)
        {
            let why = format!(
                "the last {padding} bits of {digits:?} are after the {length} bits of the bit \
                 string, and are not zero"
            );
            return Err(self.refuse(ErrorKind::Value, name, why));
        }
        let mut bits = BitString::from_vec(octets);
        bits.truncate(length);
        Ok(bits)
    }

    /// The next value, the name of a value of an ENUMERATED that has these names: the
    /// position of the name among them. A name is its letters and its digits, whatever
    /// their case and whatever is between them.
    fn named(&mut self, names: &'static [&'static str]) -> Result<usize, DecodeError> {
        let (name, text) = self.string("the name of a value of the ENUMERATED is expected")?;
        let exact = names.iter().position(|known| *known == text);
        let found = exact.or_else(|| names.iter().position(|known| same_name(known, text)));
        found.ok_or_else(|| {
            let why = format!("{text:?} is no value of the ENUMERATED");
            self.refuse(ErrorKind::Value, name, why)
        })
    }

    /// The next value, a SEQUENCE: its members come next, by the names of its components
    /// and of its additions in this order, and a member of another name is refused. They
    /// are taken as slices: an iterator over the components of a type would be compiled
    /// for each SEQUENCE.
    fn members(&mut self, root: &[Field], extended: &[Field]) -> Result<(), DecodeError> {
        let (name, value) = self.next()?;
        let Some(members) = value.as_object() else {
            return Err(self.mismatch(name, "an object is expected", value));
        };
        let first = self.values.len();
        let mut read = 0;
        for known in root.iter().chain(extended).map(|field| field.name) {
            let member = members.get(known);
            read += usize::from(member.is_some());
            self.values.push((Name::Member(known), member));
        }
        if read != members.len() {
            let known = &self.values[first..];
            let has = |member: &str| {
                let mut names = known.iter();
                names.any(|(name, _)| matches!(name, Name::Member(known) if *known == member))
            };
            if let Some(other) = members.keys().find(|member| !has(member)) {
                let why = format!("{other:?} is not a member that its type has");
                self.values.truncate(first);
                self.at.push(name);
                return Err(self.refuse(ErrorKind::NoSuchMember, Name::Root, why));
            }
        }
        self.values[first..].reverse();
        self.at.push(name);
        Ok(())
    }

    /// The next value, a list: its entries come next, and this is how many it has.
    fn entries(&mut self) -> Result<usize, DecodeError> {
        let (name, value) = self.next()?;
        let Some(entries) = value.as_array() else {
            return Err(self.mismatch(name, "a list is expected", value));
        };
        let listed = entries.iter().enumerate().rev();
        self.values
            .extend(listed.map(|(position, entry)| (Name::Entry(position), Some(entry))));
        self.at.push(name);
        Ok(entries.len())
    }

    /// The next value, a CHOICE: the tag of its alternative, whose value comes next.
    fn alternative(
        &mut self,
        root: &'static [TagTree],
        extended: Option<&'static [TagTree]>,
        names: &'static [&'static str],
    ) -> Result<Tag, DecodeError> {
        const ONE: &str = "an object with one member is expected, the name of an alternative \
                           with its value";
        let (name, value) = self.next()?;
        let mut members = value.as_object().into_iter().flatten();
        let (Some((written, held)), None) = (members.next(), members.next()) else {
            return Err(self.mismatch(name, ONE, value));
        };
        let mut tags = Vec::new();
        for tree in root.iter().chain(extended.into_iter().flatten()) {
            leaves(tree, &mut tags);
        }
        let position = names.iter().position(|known| known == written);
        let Some((known, tag)) = position.and_then(|at| Some((names[at], *tags.get(at)?))) else {
            self.at.push(name);
            let why = format!(
                "{written:?} is not an alternative that its type has: it has {}",
                names.join(", ")
            );
            return Err(self.refuse(ErrorKind::NoSuchMember, Name::Root, why));
        };
        self.values.push((Name::Member(known), Some(held)));
        self.at.push(name);
        Ok(tag)
    }

    /// What was read last, a SEQUENCE, a list or a CHOICE, is read whole.
    fn close(&mut self) {
        self.at.pop();
    }
}

/// An integer that JSON writes.
enum Integral {
    /// One of 127 bits at most.
    Small(i128),
    /// A larger one, as its octets in two's complement, the most significant first.
    Large(Vec<u8>),
}

/// The integer that `digits` are in decimal, with `-` before those of a negative one.
fn decimal(digits: &str) -> Option<Integral> {
    let (negative, magnitude) = match digits.strip_prefix('-') {
        Some(magnitude) => (true, magnitude),
        None => (false, digits),
    };
    // A number is written one way: no sign of its own, and no zero before it.
    let canonical = magnitude.bytes().all(|digit| digit.is_ascii_digit())
        && !magnitude.is_empty()
        && (magnitude == "0" || !magnitude.starts_with('0'))
        && !(negative && magnitude == "0");
    if !canonical {
        return None;
    }
    if let Ok(small) = digits.parse::<i128>() {
        return Some(Integral::Small(small));
    }
    // The magnitude in base 256, the least significant octet first.
    let mut octets = vec![0u8];
    for digit in magnitude.bytes() {
        let mut carry = u32::from(digit - b'0');
        for octet in &mut octets {
            let value = u32::from(*octet) * 10 + carry;
            *octet = value as u8;
            carry = value >> 8;
        }
        if carry != 0 {
            octets.push(carry as u8);
        }
    }
    // An octet for the sign, then two's complement for a negative number.
    octets.push(0);
    if negative {
        let mut carry = true;
        for octet in &mut octets {
            let (value, overflow) = (!*octet).overflowing_add(u8::from(carry));
            *octet = value;
            carry = overflow;
        }
    }
    octets.reverse();
    Some(Integral::Large(octets))
}

impl rasn::Decoder for Decoder<'_> {
    type Ok = ();
    type Error = DecodeError;
    type AnyDecoder<const R: usize, const E: usize> = Self;

    fn codec(&self) -> Codec {
        Codec::Jer
    }

    fn decode_any(&mut self, _: Tag) -> Result<Any, DecodeError> {
        self.octets().map(Any::new)
    }

    fn decode_bit_string(
        &mut self,
        _: Tag,
        constraints: &Constraints,
    ) -> Result<BitString, DecodeError> {
        self.bits(constraints)
    }

    fn decode_bool(&mut self, _: Tag) -> Result<bool, DecodeError> {
        let (name, value) = self.next()?;
        match value.as_bool() {
            Some(read) => Ok(read),
            None => Err(self.mismatch(name, "true or false is expected", value)),
        }
    }

    fn decode_enumerated<E: types::Enumerated>(&mut self, _: Tag) -> Result<E, DecodeError> {
        let position = self.named(E::IDENTIFIERS)?;
        let root = E::VARIANTS.len();
        let value = match position.checked_sub(root) {
            None => E::VARIANTS.get(position),
            Some(extended) => E::EXTENDED_VARIANTS.and_then(|values| values.get(extended)),
        };
        value.copied().ok_or_else(|| {
            rasn::de::Error::custom("an ENUMERATED has a name without a value", Codec::Jer)
        })
    }

    fn decode_integer<I: types::IntegerType>(
        &mut self,
        _: Tag,
        constraints: &Constraints,
    ) -> Result<I, DecodeError> {
        let read = match self.integer(constraints)? {
            Integral::Small(value) => I::try_from(value).ok(),
            Integral::Large(octets) => I::try_from_signed_bytes(&octets, Codec::Jer).ok(),
        };
        // A type of the bindings holds every integer of its constraint.
        read.ok_or_else(|| DecodeError::integer_overflow(I::WIDTH, Codec::Jer))
    }

    fn decode_real<R: types::RealType>(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<R, DecodeError> {
        Err(unread("REAL"))
    }

    fn decode_null(&mut self, _: Tag) -> Result<(), DecodeError> {
        let (name, value) = self.next()?;
        match value.is_null() {
            true => Ok(()),
            false => Err(self.mismatch(name, "null is expected", value)),
        }
    }

    fn decode_object_identifier(&mut self, _: Tag) -> Result<types::ObjectIdentifier, DecodeError> {
        const ARCS: &str = "an object identifier is expected, its arcs in decimal with a full \
                            stop between them";
        let (name, value) = self.next()?;
        let arcs = value.as_str().and_then(|arcs| {
            let arcs: Result<Vec<u32>, _> = arcs.split('.').map(str::parse).collect();
            types::ObjectIdentifier::new(arcs.ok()?)
        });
        arcs.ok_or_else(|| self.mismatch(name, ARCS, value))
    }

    fn decode_sequence<const RC: usize, const EC: usize, D, DF, F>(
        &mut self,
        _: Tag,
        _: Option<DF>,
        decode_fn: F,
    ) -> Result<D, DecodeError>
    where
        D: types::Constructed<RC, EC>,
        DF: FnOnce() -> D,
        F: FnOnce(&mut Self) -> Result<D, DecodeError>,
    {
        let (root, extended) = (D::FIELDS, D::EXTENDED_FIELDS);
        self.members(&root, extended.as_deref().unwrap_or_default())?;
        let value = decode_fn(self)?;
        self.close();
        Ok(value)
    }

    fn decode_sequence_of<D: Decode>(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<Vec<D>, DecodeError> {
        let count = self.entries()?;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(D::decode(self)?);
        }
        self.close();
        Ok(values)
    }

    fn decode_set_of<D: Decode + Eq + core::hash::Hash>(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<types::SetOf<D>, DecodeError> {
        Err(unread("SET OF"))
    }

    fn decode_octet_string<'b, T: From<&'b [u8]> + From<Vec<u8>>>(
        &'b mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<T, DecodeError> {
        self.octets().map(T::from)
    }

    fn decode_utf8_string(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<types::Utf8String, DecodeError> {
        self.text().map(|(_, text)| text.to_string())
    }

    fn decode_visible_string(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<types::VisibleString, DecodeError> {
        let (name, text) = self.text()?;
        types::VisibleString::try_from(text).map_err(|_| {
            let why = format!("{text:?} has a character that a VisibleString has not");
            self.refuse(ErrorKind::Value, name, why)
        })
    }

    fn decode_general_string(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<types::GeneralString, DecodeError> {
        Err(unread("GeneralString"))
    }

    fn decode_graphic_string(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<types::GraphicString, DecodeError> {
        Err(unread("GraphicString"))
    }

    fn decode_ia5_string(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<types::Ia5String, DecodeError> {
        let (name, text) = self.text()?;
        types::Ia5String::try_from(text).map_err(|_| {
            let why = format!("{text:?} has a character that an IA5String has not");
            self.refuse(ErrorKind::Value, name, why)
        })
    }

    fn decode_printable_string(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<types::PrintableString, DecodeError> {
        let (name, text) = self.text()?;
        types::PrintableString::try_from(text).map_err(|_| {
            let why = format!("{text:?} has a character that a PrintableString has not");
            self.refuse(ErrorKind::Value, name, why)
        })
    }

    fn decode_numeric_string(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<types::NumericString, DecodeError> {
        let (name, text) = self.text()?;
        types::NumericString::try_from(text).map_err(|_| {
            let why = format!("{text:?} has a character that a NumericString has not");
            self.refuse(ErrorKind::Value, name, why)
        })
    }

    fn decode_teletex_string(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<types::TeletexString, DecodeError> {
        Err(unread("TeletexString"))
    }

    fn decode_bmp_string(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<types::BmpString, DecodeError> {
        Err(unread("BMPString"))
    }

    fn decode_explicit_prefix<D: Decode>(&mut self, _: Tag) -> Result<D, DecodeError> {
        D::decode(self)
    }

    fn decode_optional_with_explicit_prefix<D: Decode>(
        &mut self,
        _: Tag,
    ) -> Result<Option<D>, DecodeError> {
        self.decode_optional()
    }

    fn decode_utc_time(&mut self, _: Tag) -> Result<types::UtcTime, DecodeError> {
        Err(unread("UTCTime"))
    }

    fn decode_generalized_time(&mut self, _: Tag) -> Result<types::GeneralizedTime, DecodeError> {
        Err(unread("GeneralizedTime"))
    }

    fn decode_date(&mut self, _: Tag) -> Result<types::Date, DecodeError> {
        Err(unread("DATE"))
    }

    fn decode_set<const RC: usize, const EC: usize, FIELDS, SET, D, F>(
        &mut self,
        _: Tag,
        _: D,
        _: F,
    ) -> Result<SET, DecodeError>
    where
        SET: Decode + types::Constructed<RC, EC>,
        FIELDS: Decode,
        D: Fn(&mut Self, usize, Tag) -> Result<FIELDS, DecodeError>,
        F: FnOnce(Vec<FIELDS>) -> Result<SET, DecodeError>,
    {
        Err(unread("SET"))
    }

    fn decode_choice<D: types::DecodeChoice>(&mut self, _: &Constraints) -> Result<D, DecodeError> {
        let tag = self.alternative(D::VARIANTS, D::EXTENDED_VARIANTS, D::IDENTIFIERS)?;
        let value = D::from_tag(self, tag)?;
        self.close();
        Ok(value)
    }

    fn decode_optional<D: Decode>(&mut self) -> Result<Option<D>, DecodeError> {
        match self.present() {
            true => D::decode(self).map(Some),
            false => Ok(None),
        }
    }

    fn decode_optional_with_tag<D: Decode>(&mut self, _: Tag) -> Result<Option<D>, DecodeError> {
        self.decode_optional()
    }

    fn decode_optional_with_constraints<D: Decode>(
        &mut self,
        _: &Constraints,
    ) -> Result<Option<D>, DecodeError> {
        self.decode_optional()
    }

    fn decode_optional_with_tag_and_constraints<D: Decode>(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<Option<D>, DecodeError> {
        self.decode_optional()
    }

    fn decode_extension_addition_with_explicit_tag_and_constraints<D: Decode>(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<Option<D>, DecodeError> {
        self.decode_optional()
    }

    fn decode_extension_addition_with_tag_and_constraints<D: Decode>(
        &mut self,
        _: Tag,
        _: &Constraints,
    ) -> Result<Option<D>, DecodeError> {
        self.decode_optional()
    }

    fn decode_extension_addition_group<
        const RC: usize,
        const EC: usize,
        D: Decode + types::Constructed<RC, EC>,
    >(
        &mut self,
    ) -> Result<Option<D>, DecodeError> {
        self.decode_optional()
    }
}

// ── What rasn writes ────────────────────────────────────────────────────────

#[cfg(test)]
thread_local! {
    /// How many values this thread compared with what rasn writes of them; `None`
    /// for a thread that compares none, as every test but one is.
    pub(crate) static COMPARED: std::cell::Cell<Option<usize>> =
        const { std::cell::Cell::new(None) };
}

/// In a test that asks for it: `value` has the JSON form that the codec of the JSON
/// Encoding Rules of rasn gives it, and is read back from it as it is.
#[cfg(test)]
pub(crate) fn as_rasn_writes_it<T: Encode + Decode + PartialEq + core::fmt::Debug>(value: &T) {
    let Some(compared) = COMPARED.get() else {
        return;
    };
    let ours = to_value(value).unwrap();
    let text = rasn::jer::encode(value).unwrap();
    let theirs: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(ours, theirs, "{value:?}");
    assert_eq!(from_value::<T>(&ours).as_ref(), Ok(value));
    COMPARED.set(Some(compared + 1));
}

#[cfg(test)]
#[allow(non_camel_case_types, non_snake_case)]
mod tests {
    use super::*;
    use rasn::types::{
        FixedBitString, Integer, ObjectIdentifier, OctetString, PrintableString, SequenceOf,
        Utf8String, VisibleString,
    };
    // The derived codec of a member that is an extension addition calls these traits.
    use rasn::{AsnType, Decode, Decoder as _, Encode, Encoder as _};
    use serde_json::json;

    #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
    #[rasn(enumerated)]
    #[non_exhaustive]
    enum Kind {
        #[rasn(identifier = "user-inactivity")]
        user_inactivity = 0,
        unspecified = 1,
        #[rasn(extension_addition, identifier = "not-in-the-root")]
        not_in_the_root = 2,
    }

    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(choice, automatic_tags)]
    enum Either {
        number(u8),
        kind(Kind),
        #[rasn(size("28"), identifier = "home-ID")]
        home_ID(BitString),
        nothing(()),
    }

    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, size("16", extensible))]
    struct Mask(BitString);

    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, value("0..=1099511627775"))]
    struct Identifier40(u64);

    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(delegate, value("0..=4000000000000", extensible))]
    struct Rate(Integer);

    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    #[rasn(automatic_tags)]
    #[non_exhaustive]
    struct Sample {
        flag: bool,
        id: Identifier40,
        rate: Rate,
        #[rasn(value("-8..=7"))]
        small: i8,
        kind: Kind,
        either: Either,
        octets: OctetString,
        #[rasn(identifier = "fixed-bits")]
        fixed_bits: FixedBitString<12>,
        mask: Mask,
        #[rasn(size("1..=160", extensible))]
        address: BitString,
        #[rasn(size("7"), identifier = "day-of-week")]
        day_of_week: Option<BitString>,
        name: Utf8String,
        printable: PrintableString,
        visible: VisibleString,
        object: ObjectIdentifier,
        open: Any,
        list: SequenceOf<Either>,
        absent: Option<u8>,
        #[rasn(extension_addition)]
        later: Option<Kind>,
    }

    fn bits(octets: &[u8], length: usize) -> BitString {
        let mut bits = BitString::from_slice(octets);
        bits.truncate(length);
        bits
    }

    fn sample() -> Sample {
        let mut fixed = FixedBitString::<12>::ZERO;
        fixed[..12].copy_from_bitslice(&bits(&[0xab, 0xc0], 12));
        Sample {
            flag: true,
            id: Identifier40(1_099_511_627_775),
            rate: Rate(4_000_000_000_000u64.into()),
            small: -8,
            kind: Kind::user_inactivity,
            either: Either::home_ID(bits(&[0x12, 0x34, 0x56, 0x70], 28)),
            octets: vec![0x00, 0xff].into(),
            fixed_bits: fixed,
            mask: Mask(bits(&[0xc0, 0x00], 16)),
            address: bits(&[10, 0, 0, 1], 32),
            day_of_week: Some(bits(&[0xfe], 7)),
            name: "gnb-1".into(),
            printable: PrintableString::try_from("AMF 1").unwrap(),
            visible: VisibleString::try_from("visible").unwrap(),
            object: ObjectIdentifier::new(vec![1, 2, 840]).unwrap(),
            open: Any::new(vec![0x05, 0x00]),
            list: vec![
                Either::number(7),
                Either::kind(Kind::unspecified),
                Either::nothing(()),
            ],
            absent: None,
            later: Some(Kind::not_in_the_root),
        }
    }

    fn written() -> Value {
        json!({
            "flag": true,
            "id": 1_099_511_627_775u64,
            "rate": 4_000_000_000_000u64,
            "small": -8,
            "kind": "user-inactivity",
            "either": {"home-ID": "12345670"},
            "octets": "00FF",
            "fixed-bits": "ABC0",
            "mask": "C000",
            "address": {"value": "0A000001", "length": 32},
            "day-of-week": {"value": "FE", "length": 7},
            "name": "gnb-1",
            "printable": "AMF 1",
            "visible": "visible",
            "object": "1.2.840",
            "open": "0500",
            "list": [{"number": 7}, {"kind": "unspecified"}, {"nothing": null}],
            "later": "not-in-the-root",
        })
    }

    /// Why `value` is not a `Sample`: the kind of the error and what it says.
    fn refused(value: &Value) -> (ErrorKind, String) {
        let error = from_value::<Sample>(value).unwrap_err();
        (error.kind(), error.to_string())
    }

    /// The sample with `value` for its member `name`.
    fn with(name: &str, value: Value) -> Value {
        let mut sample = written();
        sample[name] = value;
        sample
    }

    #[test]
    fn each_construct_has_its_form_and_is_read_back() {
        let sample = sample();
        assert_eq!(to_value(&sample).unwrap(), written());
        assert_eq!(from_value::<Sample>(&written()), Ok(sample.clone()));

        // It is the form that rasn gives the value.
        let theirs: Value = serde_json::from_str(&rasn::jer::encode(&sample).unwrap()).unwrap();
        assert_eq!(theirs, written());
    }

    #[test]
    fn hexadecimal_is_written_in_upper_case_and_read_in_either() {
        assert_eq!(hex(&[0x00, 0x9a, 0xff]), "009AFF");
        assert_eq!(unhex("009aFF"), Some(vec![0x00, 0x9a, 0xff]));
        assert_eq!(unhex(""), Some(vec![]));
        // Each octet, alone and among others, and each character that is no digit.
        let every: Vec<u8> = (0..=255).collect();
        let written: String = every.iter().map(|octet| format!("{octet:02X}")).collect();
        assert_eq!(hex(&every), written);
        assert_eq!(unhex(&written), Some(every.clone()));
        assert_eq!(unhex(&written.to_lowercase()), Some(every.clone()));
        for length in 0..=every.len() {
            assert_eq!(
                unhex(&hex(&every[..length])).as_deref(),
                Some(&every[..length])
            );
        }
        for other in (0..=255u8).filter(|other| !other.is_ascii_hexdigit()) {
            for at in [0, 1, 62, 63] {
                let mut text = written.clone().into_bytes();
                text[at] = other;
                if let Ok(text) = String::from_utf8(text) {
                    assert_eq!(unhex(&text), None, "{other:#04x} at {at}");
                }
            }
        }
        for text in ["0", "0g", "000", " 00", "é0"] {
            assert_eq!(unhex(text), None, "{text}");
        }
        let lower = with("octets", json!("00ff"));
        assert_eq!(from_value::<Sample>(&lower), Ok(sample()));
        let (kind, why) = refused(&with("octets", json!("0")));
        assert_eq!(kind, ErrorKind::Value);
        assert_eq!(
            why,
            "this is not a value of Sample, at octets: octets in hexadecimal, two digits for \
             each, are expected, and \"0\" is written"
        );
    }

    #[test]
    fn an_integer_is_a_number_of_any_size() {
        let rate = |value: Integer| to_value(&Rate(value)).unwrap();
        let read = |value: Value| from_value::<Rate>(&value).map(|rate| rate.0);
        let largest = u64::MAX;
        assert_eq!(rate(largest.into()), json!(largest));
        assert_eq!(read(json!(largest)), Ok(largest.into()));
        assert_eq!(rate((1u64 << 63).into()), json!(1u64 << 63));
        assert_eq!(rate(i64::MIN.into()), json!(i64::MIN));

        // JSON has no exact number above 2^64-1 or below -2^63: the decimal digits.
        let above: Integer = (i128::from(largest) + 1).into();
        assert_eq!(rate(above.clone()), json!("18446744073709551616"));
        assert_eq!(read(json!("18446744073709551616")), Ok(above));
        let below: Integer = (i128::from(i64::MIN) - 1).into();
        assert_eq!(rate(below.clone()), json!("-9223372036854775809"));
        assert_eq!(read(json!("-9223372036854775809")), Ok(below));

        // More than 127 bits, both ways.
        let digits = "340282366920938463463374607431768211456";
        let huge = read(json!(digits)).unwrap();
        assert_eq!(huge.to_string(), digits);
        assert_eq!(rate(huge), json!(digits));
        let negative = format!("-{digits}");
        let huge = read(json!(negative)).unwrap();
        assert_eq!(huge.to_string(), negative);
        assert_eq!(rate(huge), json!(negative));

        // A number is written one way, and one that JSON has a number for is that
        // number: its digits are refused.
        for other in ["+1", "01", "-0", "", "1e3", "5", "18446744073709551615"] {
            assert!(read(json!(other)).is_err(), "{other}");
        }
        assert!(read(json!(1.5)).is_err());
        assert!(read(json!("-9223372036854775808")).is_err());
    }

    #[test]
    fn an_integer_outside_its_constraint_is_refused_where_it_is() {
        let (kind, why) = refused(&with("id", json!(1u64 << 40)));
        assert_eq!(kind, ErrorKind::Value);
        assert_eq!(
            why,
            "this is not a value of Sample, at id: 1099511627776 is outside its constraint \
             (0..1099511627775)"
        );
        let (_, why) = refused(&with("small", json!(8)));
        assert!(
            why.ends_with("at small: 8 is outside its constraint (-8..7)"),
            "{why}"
        );
        let (_, why) = refused(&with("id", json!("one")));
        assert!(
            why.ends_with("at id: a number is expected, and \"one\" is written"),
            "{why}"
        );
        // A constraint that has an extension marker takes any integer.
        let beyond = with("rate", json!(4_000_000_000_001u64));
        assert!(from_value::<Sample>(&beyond).is_ok());
        // The constraint of the type of a member that may be absent.
        let (_, why) = refused(&with("absent", json!(256)));
        assert!(
            why.ends_with("at absent: 256 is outside its constraint (0..255)"),
            "{why}"
        );
    }

    #[test]
    fn a_name_of_an_enumerated_is_taken_whatever_its_case_and_separators() {
        for name in ["user-inactivity", "User_Inactivity", "userinactivity"] {
            let sample = from_value::<Sample>(&with("kind", json!(name))).unwrap();
            assert_eq!(sample.kind, Kind::user_inactivity, "{name}");
        }
        let extended = from_value::<Sample>(&with("kind", json!("NOT in the root"))).unwrap();
        assert_eq!(extended.kind, Kind::not_in_the_root);
        let (kind, why) = refused(&with("kind", json!("no-such-kind")));
        assert_eq!(kind, ErrorKind::Value);
        assert_eq!(
            why,
            "this is not a value of Sample, at kind: \"no-such-kind\" is no value of the \
             ENUMERATED"
        );
        let (_, why) = refused(&with("kind", json!(0)));
        assert!(
            why.ends_with("the name of a value of the ENUMERATED is expected, and 0 is written"),
            "{why}"
        );
    }

    #[test]
    fn a_bit_string_has_its_length_unless_its_type_gives_it() {
        // A type of one size and an extension marker: a string of that size is its
        // octets, and one of another size has its length.
        let mask = |bits: BitString| to_value(&Mask(bits)).unwrap();
        let read = |value: Value| from_value::<Mask>(&value).map(|mask| mask.0);
        assert_eq!(mask(bits(&[0xc0, 0x00], 16)), json!("C000"));
        let longer = json!({"value": "C00000", "length": 20});
        assert_eq!(mask(bits(&[0xc0, 0x00, 0x00], 20)), longer);
        assert_eq!(read(longer), Ok(bits(&[0xc0, 0x00, 0x00], 20)));
        assert_eq!(read(json!("c000")), Ok(bits(&[0xc0, 0x00], 16)));
        // The size of the type is also read with its length.
        let sized = json!({"value": "C000", "length": 16});
        assert_eq!(read(sized), Ok(bits(&[0xc0, 0x00], 16)));
        assert!(read(json!("C00000")).is_err());

        // A string that does not start an octet is read bit by bit.
        let within = bits(&[0x0f, 0xf0], 16);
        assert_eq!(hex(&octets_of(&within[4..12])), "FF");
        assert_eq!(hex(&octets_of(&within[4..9])), "F8");
        assert_eq!(hex(&octets_of(&within[..0])), "");

        // What is not the string that it says.
        for (member, value, why) in [
            (
                "fixed-bits",
                json!("ABC"),
                "\"ABC\" is not hexadecimal, two digits for each octet",
            ),
            (
                "fixed-bits",
                json!("ABC0FF"),
                "3 octets are written for a bit string of 12 bits, which has 2",
            ),
            (
                "fixed-bits",
                json!("ABC1"),
                "the last 4 bits of \"ABC1\" are after the 12 bits of the bit string, and are \
                 not zero",
            ),
            (
                "fixed-bits",
                json!({"value": "ABC0", "length": 12}),
                "a bit string of 12 bits is expected, in hexadecimal, and \
                 {\"length\":12,\"value\":\"ABC0\"} is written",
            ),
            (
                "address",
                json!("0A000001"),
                "a bit string whose size varies is expected, which is written as {\"value\": \
                 HEXADECIMAL, \"length\": BITS}, and \"0A000001\" is written",
            ),
            (
                "address",
                json!({"value": "0A000001", "length": 32, "other": 1}),
                "a bit string whose size varies is expected, which is written as {\"value\": \
                 HEXADECIMAL, \"length\": BITS}, and {\"length\":32,\"other\":1,\"value\":\"0A000001… \
                 is written",
            ),
        ] {
            let (kind, text) = refused(&with(member, value));
            assert_eq!(kind, ErrorKind::Value);
            let expected = format!("this is not a value of Sample, at {member}: {why}");
            assert_eq!(text, expected);
        }
    }

    #[test]
    fn a_member_that_the_type_has_not_and_one_that_it_lacks_are_refused() {
        let (kind, why) = refused(&with("other", json!(1)));
        assert_eq!(kind, ErrorKind::NoSuchMember);
        assert_eq!(
            why,
            "this is not a value of Sample: \"other\" is not a member that its type has"
        );

        let mut without = written();
        without.as_object_mut().unwrap().remove("flag");
        let (kind, why) = refused(&without);
        assert_eq!(kind, ErrorKind::Value);
        assert_eq!(
            why,
            "this is not a value of Sample, at flag: the member is missing"
        );

        // A member that may be absent is.
        without = written();
        without.as_object_mut().unwrap().remove("later");
        assert_eq!(from_value::<Sample>(&without).unwrap().later, None);

        let (_, why) = refused(&json!([1]));
        assert_eq!(
            why,
            "this is not a value of Sample: an object is expected, and [1] is written"
        );
    }

    #[test]
    fn a_choice_is_one_alternative_that_its_type_has() {
        let (kind, why) = refused(&with("either", json!({"other": 1})));
        assert_eq!(kind, ErrorKind::NoSuchMember);
        assert_eq!(
            why,
            "this is not a value of Sample, at either: \"other\" is not an alternative that \
             its type has: it has number, kind, home-ID, nothing"
        );
        for other in [
            json!({}),
            json!({"number": 1, "kind": "unspecified"}),
            json!(1),
        ] {
            let (kind, why) = refused(&with("either", other));
            assert_eq!(kind, ErrorKind::Value);
            assert!(
                why.starts_with(
                    "this is not a value of Sample, at either: an object with one member is \
                     expected, the name of an alternative with its value, and "
                ),
                "{why}"
            );
        }
        // The fault of an entry of a list is at its position.
        let list = json!([{"number": 7}, {"number": 256}]);
        let (_, why) = refused(&with("list", list));
        assert_eq!(
            why,
            "this is not a value of Sample, at list/1/number: 256 is outside its constraint \
             (0..255)"
        );
        let (_, why) = refused(&with("list", json!({})));
        assert!(
            why.ends_with("at list: a list is expected, and {} is written"),
            "{why}"
        );
    }

    #[test]
    fn the_other_values_are_refused_as_what_they_are_not() {
        for (member, value, why) in [
            (
                "flag",
                json!(1),
                "true or false is expected, and 1 is written",
            ),
            ("name", json!(1), "a string is expected, and 1 is written"),
            (
                "printable",
                json!("a_b"),
                "\"a_b\" has a character that a PrintableString has not",
            ),
            (
                "object",
                json!("1.x"),
                "an object identifier is expected, its arcs in decimal with a full stop \
                 between them, and \"1.x\" is written",
            ),
            (
                "open",
                json!({"number": 1}),
                "octets in hexadecimal, two digits for each, are expected, and {\"number\":1} \
                 is written",
            ),
        ] {
            let (kind, text) = refused(&with(member, value));
            assert_eq!(kind, ErrorKind::Value);
            let expected = format!("this is not a value of Sample, at {member}: {why}");
            assert_eq!(text, expected);
        }
        let (_, why) = refused(&with("either", json!({"nothing": 0})));
        assert!(
            why.ends_with("at either/nothing: null is expected, and 0 is written"),
            "{why}"
        );
    }
}
