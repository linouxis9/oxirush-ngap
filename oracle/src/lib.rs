/*
   OxiRush
   Copyright 2025-2026 Valentin D'Emmanuele

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! The oracle of the tests: the types of the protocol with the codec of another
//! implementation of aligned PER, `rasn`.
//!
//! The generator of the crate writes two sets of bindings from what rasn-compiler
//! resolves of the ASN.1: those of the crate, with the codec of oxirush-aper, and
//! these, which derive the codec of rasn. Both have the same types, by the same names.
//! The tests and the fuzz targets of the crate give the same octets to both, and
//! compare what they read and what they write back.
//!
//! rasn 0.29 does not follow ITU-T X.691 everywhere, so a few codecs of these bindings
//! are written here in its place, each with its clause: `per` has the SEQUENCE and the
//! list, `sized` the strings whose length is octet-aligned, and the generator the
//! others. The vectors of the crate, which pycrate wrote and tshark reads, hold for
//! both.
//!
//! [`read`] decodes octets as a type that it is given by name, and gives the JSON form
//! of the JSON Encoding Rules of rasn and the octets that rasn writes back.
//! [`same_octets`] and [`same_json`] compare that with what the crate made of the same
//! octets, and say where the two differ by design: the tests and the fuzz targets call
//! them. The bindings themselves are in [`bindings`], for a test that needs a value.

#[allow(clippy::all, missing_docs)]
pub mod bindings;
pub mod per;
pub mod sized;
mod types;

pub use rasn;
pub use types::{NAMES, read};

use serde_json::{Map, Value};

/// What rasn makes of octets.
#[derive(Debug, Clone, PartialEq)]
pub struct Read {
    /// The octets that rasn writes of the value that it read.
    pub octets: Vec<u8>,
    /// The JSON form that the JSON Encoding Rules of rasn give the value, or why they
    /// give none: rasn 0.29 writes no integer of 2^63 or more.
    pub json: Result<serde_json::Value, String>,
}

/// The value of the type `T` that `octets` are the complete encoding of, in aligned PER:
/// one value, whole, and no octet after it.
pub fn decode<T: rasn::Decode>(octets: &[u8]) -> Result<T, String> {
    if octets.is_empty() {
        return Err("there are no octets".into());
    }
    match rasn::aper::decode_with_remainder::<T>(octets) {
        // A value of no bit has exactly one zero octet as its complete encoding.
        Ok((value, rest)) if rest.is_empty() || (octets == [0] && rest == octets) => Ok(value),
        Ok((_, rest)) => Err(format!("the value is followed by {} octets", rest.len())),
        Err(error) => Err(error.to_string()),
    }
}

/// The complete encoding of `value` in aligned PER, in which an empty encoding is one
/// zero octet (X.691 (02/2021) §11.1.4).
pub fn encode<T: rasn::Encode>(value: &T) -> Result<Vec<u8>, String> {
    let mut octets = rasn::aper::encode(value).map_err(|error| error.to_string())?;
    if octets.is_empty() {
        octets.push(0);
    }
    Ok(octets)
}

/// What rasn makes of `octets` as a value of the type `T`.
fn read_as<T: rasn::Decode + rasn::Encode>(octets: &[u8]) -> Result<Read, String> {
    let value: T = decode(octets)?;
    let json = rasn::jer::encode(&value)
        .map_err(|error| format!("{error:?}"))
        .and_then(|text| serde_json::from_str(&text).map_err(|error| error.to_string()));
    Ok(Read {
        octets: encode(&value)?,
        json,
    })
}

/// Why the bindings of the crate refuse what rasn reads, when it is the one thing that
/// they do not hold: an INTEGER of more than 64 bits, or a negative one where its type
/// has none.
fn beyond_the_bindings(error: &str) -> bool {
    error.contains("the number does not fit in 64 bits")
}

/// `octets` as the type that the bindings name `name`: what the crate made of them,
/// which is the octets that it writes of the value or why it reads none, is what rasn
/// makes of them. Says whether both read a value.
///
/// # Panics
///
/// When one reads a value that the other refuses, or when both read a value that they
/// write as other octets: the two codecs disagree.
#[track_caller]
pub fn same_octets(name: &str, octets: &[u8], ours: Result<&[u8], String>) -> bool {
    let theirs = read(name, octets).unwrap_or_else(|| panic!("no {name} in the oracle"));
    match (ours, &theirs) {
        (Ok(ours), Ok(theirs)) => {
            assert_eq!(ours, theirs.octets, "{name} of {octets:02x?}");
            true
        }
        (Err(_), Err(_)) => false,
        (Err(error), Ok(_)) if beyond_the_bindings(&error) => false,
        (ours, theirs) => panic!("{name} of {octets:02x?}: {ours:?} here, {theirs:?} by rasn"),
    }
}

/// Whether `ours` is what rasn writes, `theirs`, but for the two bit strings whose
/// length the codec of rasn has otherwise (X.697 (02/2021) clause 24). A string whose
/// type has one size is a JSON string: in a member that may be absent, rasn does not
/// look at the size that the member gives it, and writes its length. A string of
/// another size than the one that its type has beside an extension marker is written
/// with its length: rasn writes its octets alone, and reads another string back.
fn same_but_for_a_length(ours: &Value, theirs: &Value) -> bool {
    // The octets and the length of a bit string that is written with its length.
    let with_length = |members: &Map<String, Value>| {
        let length = members.get("length").and_then(Value::as_u64)?;
        let octets = members.get("value").and_then(Value::as_str)?;
        let whole = length.div_ceil(8) == octets.len() as u64 / 2;
        (members.len() == 2 && whole).then(|| octets.to_string())
    };
    match (ours, theirs) {
        (Value::String(octets), Value::Object(members)) => {
            with_length(members).as_ref() == Some(octets)
        }
        (Value::Object(members), Value::String(octets)) => {
            with_length(members).as_ref() == Some(octets)
        }
        (Value::Object(ours), Value::Object(theirs)) => {
            let same = |(name, value)| {
                let other = theirs.get::<String>(name);
                other.is_some_and(|other| same_but_for_a_length(value, other))
            };
            ours.len() == theirs.len() && ours.iter().all(same)
        }
        (Value::Array(ours), Value::Array(theirs)) => {
            let mut pairs = ours.iter().zip(theirs);
            ours.len() == theirs.len()
                && pairs.all(|(ours, theirs)| same_but_for_a_length(ours, theirs))
        }
        _ => ours == theirs,
    }
}

/// `octets` as the type that the bindings name `name`, or as an OCTET STRING for the
/// name `Octets`: the JSON form that the crate gives the value that it read, or why it
/// reads none, is what rasn makes of the octets. Says whether both read a value.
///
/// # Panics
///
/// When one reads a value that the other refuses, or when both read a value that they
/// give another JSON form: the two codecs disagree.
#[track_caller]
pub fn same_json(name: &str, octets: &[u8], ours: Result<&Value, String>) -> bool {
    // An IE whose type is an OCTET STRING is its octets, after their length.
    let theirs = match name {
        "Octets" => decode::<rasn::types::OctetString>(octets).map(|value| {
            let digits = value.iter().map(|octet| format!("{octet:02X}"));
            Read {
                octets: encode(&value).unwrap(),
                json: Ok(Value::String(digits.collect())),
            }
        }),
        name => read(name, octets).unwrap_or_else(|| panic!("no {name} in the oracle")),
    };
    match (ours, &theirs) {
        (Ok(ours), Ok(theirs)) => {
            match &theirs.json {
                Ok(theirs) => assert!(
                    same_but_for_a_length(ours, theirs),
                    "{name} of {octets:02x?}: {ours} here, {theirs} by rasn"
                ),
                // The codec of rasn writes no integer of 2^63 or more: there is
                // nothing to compare a value that holds one with.
                Err(error) => assert!(error.contains("ExceedsSupportedIntSize"), "{error}"),
            }
            true
        }
        (Err(_), Err(_)) => false,
        (Err(error), Ok(_)) if beyond_the_bindings(&error) => false,
        (ours, theirs) => panic!("{name} of {octets:02x?}: {ours:?} here, {theirs:?} by rasn"),
    }
}

/// The types that [`read`] decodes: those that the crate decodes by a name or by an
/// identifier, which are the PDU, the messages, the IEs, and what octets contain.
macro_rules! oracle_types {
    ($($name:ident)*) => {
        /// The names of the types that `read` decodes, as the bindings name them.
        pub const NAMES: &[&str] = &[$(stringify!($name),)*];

        /// What rasn makes of `octets` as a value of the type that the bindings name
        /// `name`; `None` for a name that is none of [`NAMES`].
        pub fn read(name: &str, octets: &[u8]) -> Option<Result<crate::Read, String>> {
            match name {
                $(stringify!($name) => Some(crate::read_as::<crate::bindings::$name>(octets)),)*
                _ => None,
            }
        }
    };
}
pub(crate) use oracle_types;
