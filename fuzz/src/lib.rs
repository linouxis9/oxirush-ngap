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

//! What the fuzz targets do with the octets that they are given: the three programs
//! of `fuzz_targets/` call these functions.
//!
//! A target does not only look for a panic. It gives the same octets to the crate and
//! to the oracle of its tests, which has the same types with the codec of rasn, another
//! implementation of aligned PER, and compares what the two make of them: the PDU, the
//! protocol IEs of its message, the message that `decode_value` reads, and the JSON
//! form of each value that a view decodes. What one reads the other reads, as a value
//! that both write as the same octets, and what one refuses the other refuses. And
//! what was read is written and read again as the same.
//!
//! The crate differs from the oracle in one thing, which the comparison of the oracle
//! names where it meets it: its bindings hold an INTEGER in 64 bits.
//!
//! This file is the same in oxirush-ngap and oxirush-s1ap. What depends on the protocol
//! is in `protocol.rs`.

mod protocol;

use protocol::{ANY_MESSAGE, Borrowed, Error, ErrorKind, PDU_TYPE, Pdu, ROOT};
use protocol::{aper, fuzzing, inspect};
use serde_json::Value;

/// What the crate wrote of a value that it read, or why it read none, for the oracle.
fn ours(written: &Result<Vec<u8>, Error>) -> Result<&[u8], String> {
    let written = written.as_ref().map(Vec::as_slice);
    written.map_err(ToString::to_string)
}

/// The octets of the open type that holds the message of a PDU.
fn message(pdu: &Pdu) -> &[u8] {
    match pdu {
        Pdu::initiatingMessage(message) => &message.value,
        Pdu::successfulOutcome(message) => &message.value,
        Pdu::unsuccessfulOutcome(message) => &message.value,
        _ => panic!("a PDU of an alternative that the bindings do not have"),
    }
}

/// A PDU, the protocol IEs of its message and the message as its type: each as the
/// crate reads and writes it, and as the codec of rasn does.
pub fn decode(octets: &[u8]) {
    // The PDU, which both read or both refuse, and write back as the same octets.
    let read = Pdu::decode(octets);
    let written = read.as_ref().map_err(Clone::clone).and_then(Pdu::encode);
    oracle::same_octets(PDU_TYPE, octets, ours(&written));
    let pdu = match read {
        Ok(pdu) => pdu,
        // What `decode` refuses, the codec of the type refuses in the same words.
        Err(error) => {
            assert_eq!(aper::decode::<Borrowed>(octets), Err(error));
            return;
        }
    };
    let written = match written {
        Ok(written) => written,
        Err(error) => panic!("a PDU that was decoded does not encode: {error}: {pdu:?}"),
    };
    assert_eq!(Pdu::decode(&written).as_ref(), Ok(&pdu), "{written:02x?}");

    // A PDU that borrows its message from the octets is the same PDU.
    let borrowed = Borrowed::decode_borrowed(octets);
    assert_eq!(borrowed.as_ref(), Ok(&pdu), "{octets:02x?}");
    assert_eq!(borrowed.unwrap().into_owned(), pdu);
    // `decode` and `encode` take a PDU of the usual layout octet by octet: the codec of
    // the type gives the same PDU and the same octets.
    assert_eq!(aper::decode::<Borrowed>(octets).as_ref(), Ok(&pdu));
    assert_eq!(aper::encode(&pdu).as_ref(), Ok(&written), "{pdu:?}");

    // What the octets of the message have no part in.
    let _ = (pdu.to_string(), pdu.procedure_name(), pdu.kind());

    // The protocol IEs of the message, whatever the message is: rasn reads them as
    // those of a message that is a container of protocol IEs and nothing else.
    let theirs = oracle::read(ANY_MESSAGE, message(&pdu)).unwrap();
    match pdu.protocol_ies() {
        Ok(ies) => {
            // Put back, they are the message that rasn writes of them.
            let ies = ies.into_owned();
            let mut again = pdu.clone();
            again.set_protocol_ies(ies.clone()).unwrap();
            match &theirs {
                Ok(theirs) => assert_eq!(message(&again), &theirs.octets[..], "{pdu:?}"),
                Err(error) => panic!("{pdu:?} has protocol IEs here, and by rasn: {error}"),
            }
            assert_eq!(again.protocol_ies(), Ok(ies));
            let _ = pdu.check();
        }
        // The private message has no protocol IEs, and its octets are not looked at.
        Err(error) if error.kind() == ErrorKind::WrongMessage => {}
        Err(error) => assert!(theirs.is_err(), "{error}: {pdu:?}"),
    }

    // The message as its type.
    if let Some((name, written)) = fuzzing::message_read_and_written(&pdu) {
        oracle::same_octets(name, message(&pdu), ours(&written));
    }
}

/// `value` without what a view keeps of what was received: the members whose name
/// starts with `_`, at any depth.
fn written(value: &Value) -> Value {
    match value {
        Value::Object(members) => {
            let kept = members.iter().filter(|(name, _)| !name.starts_with('_'));
            let kept = kept.map(|(name, value)| (name.clone(), written(value)));
            Value::Object(kept.collect())
        }
        Value::Array(entries) => Value::Array(entries.iter().map(written).collect()),
        other => other.clone(),
    }
}

/// Whether a view has an IE or a transfer that was not read with its type, and is its
/// octets beside the reason.
fn undecoded(value: &Value) -> bool {
    match value {
        Value::Object(members) => {
            members.contains_key("_decode_error") || members.values().any(undecoded)
        }
        Value::Array(entries) => entries.iter().any(undecoded),
        _ => false,
    }
}

/// What a view decoded as the type that the bindings name `name` is what the oracle
/// reads in the same octets: the same JSON form, or no value for both.
fn observed(name: &'static str, octets: &[u8], ours: &Result<Value, Error>) {
    oracle::same_json(name, octets, ours.as_ref().map_err(ToString::to_string));
}

/// The JSON forms of a PDU: its view, which gives the PDU back and is written from its
/// values, its tree, and the paths of its view. Each value that a view shows is
/// compared with what the JSON Encoding Rules of rasn write of the same octets.
pub fn view(octets: &[u8]) {
    let Ok(pdu) = Pdu::decode(octets) else {
        return;
    };
    fuzzing::observe(observed);
    let Ok(view) = pdu.to_view() else {
        // A message that does not decode has no view, and no tree.
        assert!(pdu.to_tree().is_err(), "{pdu:?}");
        return;
    };

    // A view that nothing changed is the PDU, octet for octet.
    assert_eq!(pdu.with_view(&view).as_ref(), Ok(&pdu), "{view}");
    assert_eq!(Pdu::from_view(&view).as_ref(), Ok(&pdu), "{view}");

    // Without the octets received, a view in which everything was read with its type
    // is written from its values, when they are values that their types write: the PDU
    // that is written has a view, which is written to the same PDU.
    if !undecoded(&view)
        && let Ok(first) = Pdu::from_view(&written(&view))
    {
        let again = match first.to_view() {
            Ok(again) => again,
            Err(error) => panic!("a PDU that a view wrote has no view: {error}: {first:?}"),
        };
        let second = Pdu::from_view(&written(&again));
        assert_eq!(second.as_ref(), Ok(&first), "{view}");
    }

    // A tree is what a message is written from, and the message that is written from
    // it has the same tree.
    if let Ok(tree) = pdu.to_tree()
        && let Ok(first) = Pdu::from_tree(&tree)
    {
        assert_eq!(first.to_tree().as_ref(), Ok(&tree), "{pdu:?}");
    }

    // Each path that the view has selects its value.
    for (path, value) in inspect::paths(&view).into_iter().take(64) {
        assert_eq!(inspect::select(&view, &path), Ok(vec![&value]), "{path}");
    }
}

/// The functions that take a path, on the view of a PDU: `select`, `set`, `insert`,
/// `remove`, `open`, `check_path` and `enumerated_at`, with a path and a value of the
/// octets. No path and no value makes one of them panic, and a view that one of them
/// changed is a PDU or an error.
///
/// The first octets say what is done: the function, a path of the view to start from
/// and where it is cut, then a length and that many octets to add to the path, and a
/// length and that many octets of JSON for the value. The rest is the PDU.
pub fn paths(octets: &[u8]) {
    let [function, pick, cut, ref rest @ ..] = *octets else {
        return;
    };
    let Some((extra, rest)) = counted(rest) else {
        return;
    };
    let Some((value, rest)) = counted(rest) else {
        return;
    };
    let Ok(pdu) = Pdu::decode(rest) else {
        return;
    };
    let Ok(mut view) = pdu.to_view() else {
        return;
    };

    // A path that the view has, cut short, with what the octets add to it.
    let known = inspect::paths(&view);
    let start = match known.get(usize::from(pick) % known.len().max(1)) {
        Some((path, _)) => path.as_str(),
        None => ROOT,
    };
    let start: String = start.chars().take(usize::from(cut)).collect();
    let path = format!("{start}{}", String::from_utf8_lossy(extra));
    let value = match serde_json::from_slice(value) {
        Ok(value) => value,
        Err(_) => Value::String(String::from_utf8_lossy(value).into_owned()),
    };

    let name = inspect::message_name(&pdu).unwrap_or("no message");
    let changed = match function % 7 {
        0 => inspect::select(&view, &path).is_err(),
        1 => inspect::set(&mut view, &path, value).is_ok(),
        2 => inspect::insert(&mut view, &path, value).is_ok(),
        3 => inspect::remove(&mut view, &path).is_ok(),
        4 => inspect::open(&mut view, &path).is_ok(),
        5 => inspect::check_path(name, &path).is_err(),
        _ => inspect::enumerated_at(name, &path).is_err(),
    };
    if !changed {
        return;
    }

    // What an edit left is a PDU or an error, and a PDU that it is encodes or is an
    // error: an edit may write a value that its type does not have.
    let _ = inspect::paths(&view);
    if let Ok(edited) = pdu.with_view(&view) {
        assert_eq!(Pdu::from_view(&view).as_ref(), Ok(&edited), "{path}");
        if let Ok(octets) = edited.encode() {
            assert_eq!(Pdu::decode(&octets).as_ref(), Ok(&edited), "{path}");
        }
    }
}

/// The octets that a length of one octet counts, and those after them.
fn counted(octets: &[u8]) -> Option<(&[u8], &[u8])> {
    let (length, rest) = octets.split_first()?;
    rest.split_at_checked(usize::from(*length))
}
