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

//! What the benchmarks measure: the inputs, and the operations on each. The
//! four programs of `benches/` share them:
//!
//! - `codec` times each operation with criterion, and profiles it with
//!   pprof;
//! - `allocations` counts the allocations of one call of each;
//! - `heap` says where the allocations of an operation are made;
//! - `scale` gives the time and the memory of each on a message of a
//!   thousand IEs and more.
//!
//! The inputs are the messages of the fixtures of the crate,
//! `tests/fixtures/messages.tsv`, as one input, and a few messages of the
//! size that a network sends, each an input of its own.
//!
//! This file is the same in oxirush-ngap and oxirush-s1ap. What depends on
//! the protocol is in `protocol.rs`: the type of a PDU, the messages that it
//! writes, and the operations that take the type of a message.

pub mod counting;
mod protocol;

use protocol::codec::inspect;
use protocol::{Borrowed, Pdu};
use serde_json::{Value, json};
use std::hint::black_box;

// ── Inputs ──────────────────────────────────────────────────────────────────

/// A message that `protocol.rs` writes: one of the size that a network sends.
struct Written {
    /// The name of its input, without spaces: `paging`.
    name: &'static str,
    /// What the message is.
    what: &'static str,
    /// The message, as a tree.
    tree: Value,
    /// The path of a value that is deep in its view.
    path: &'static str,
    /// An edit of its view: a path, and the value that it is given.
    edit: (&'static str, Value),
    /// The operations that take the type of the message.
    typed: Typed,
}

/// The operations on a PDU that take the type of its message.
struct Typed {
    /// `decode_value` of the PDU, as the type of its message.
    decode_value: fn(&Pdu),
    /// The message of the PDU is decoded first, and borrows from it: the call
    /// reads each of its IEs with the macro that extracts.
    extract: for<'a> fn(&'a Pdu) -> Call<'a>,
    /// The values of the IEs of the PDU are read first: the call builds the
    /// PDU of these values with the macro that builds.
    build: fn(&Pdu) -> Call<'static>,
    /// The PDU that `build` builds.
    #[cfg_attr(not(test), allow(dead_code))]
    rebuilt: fn(&Pdu) -> Pdu,
}

/// A message of an input, with what the operations on it are given.
struct Message {
    /// The name that ASN.1 gives the message.
    name: &'static str,
    octets: Vec<u8>,
    pdu: Pdu,
    view: Value,
    tree: Value,
    /// The path of a value that is deep in the view.
    path: String,
    /// An edit of the view: a path, and the value that it is given.
    edit: (String, Value),
    typed: Option<Typed>,
}

impl Message {
    /// The message that `octets` are. Without a path, the path is that of the
    /// value that is deepest in its view, and without an edit, the edit is
    /// one that any message takes.
    fn new(
        octets: Vec<u8>,
        path: Option<&str>,
        edit: Option<(&str, Value)>,
        typed: Option<Typed>,
    ) -> Self {
        let pdu = Pdu::decode(&octets).unwrap();
        assert_eq!(pdu.encode().unwrap(), octets);
        let name = inspect::message_name(&pdu).unwrap();
        let view = pdu.to_view().unwrap();
        let tree = pdu.to_tree().unwrap();
        let path = path.map_or_else(|| deepest(&view), String::from);
        let edit = match edit {
            Some((path, value)) => (path.to_string(), value),
            None => another_criticality(&view),
        };
        Self {
            name,
            octets,
            pdu,
            view,
            tree,
            path,
            edit,
            typed,
        }
    }
}

/// The path of the value that is deepest in a view: the first of those that
/// have the most segments.
fn deepest(view: &Value) -> String {
    let paths = inspect::paths(view);
    let depth = |path: &str| path.matches('/').count();
    // The last of the deepest ones, from the end, is the first of them.
    let deepest = paths.iter().rev().max_by_key(|(path, _)| depth(path));
    deepest.map(|(path, _)| path.clone()).unwrap()
}

/// An edit that the view of any message takes: another criticality for its
/// first IE, or for the PDU when the message has no IE.
fn another_criticality(view: &Value) -> (String, Value) {
    let of_the_first_ie = format!("{}/0/criticality", protocol::ROOT);
    let has_an_ie = inspect::select(view, &of_the_first_ie).is_ok_and(|ie| !ie.is_empty());
    let path = match has_an_ie {
        true => of_the_first_ie,
        false => "/criticality".to_string(),
    };
    let other = match inspect::select(view, &path).unwrap()[0].as_str() {
        Some("reject") => "ignore",
        _ => "reject",
    };
    (path, json!(other))
}

/// One or several messages that the operations are measured on.
pub struct Input {
    /// The name of the input, without spaces: `fixtures`.
    pub name: &'static str,
    /// What the input is.
    pub description: String,
    messages: Vec<Message>,
}

/// The messages of the fixtures that have protocol IEs: all but the private
/// message, which has no tree.
fn fixtures() -> Vec<Message> {
    let lines = protocol::FIXTURES.lines();
    let octets = lines
        .filter(|line| !line.starts_with('#'))
        .map(|line| hex::decode(line.split('\t').nth(1).unwrap()).unwrap());
    let with_ies = octets.filter(|octets| Pdu::decode(octets).unwrap().protocol_ies().is_ok());
    let messages: Vec<_> = with_ies
        .map(|octets| Message::new(octets, None, None, None))
        .collect();
    assert!(messages.len() >= 100, "{} fixtures", messages.len());
    messages
}

/// The inputs: the fixtures, then the messages that `protocol.rs` writes.
pub fn inputs() -> Vec<Input> {
    let fixtures = fixtures();
    let mut inputs = vec![Input {
        name: "fixtures",
        description: format!(
            "the {} messages of messages.tsv that have protocol IEs",
            fixtures.len()
        ),
        messages: fixtures,
    }];
    for written in protocol::written() {
        let pdu = Pdu::from_tree(&written.tree)
            .unwrap_or_else(|error| panic!("{}: {error}", written.name));
        let octets = pdu.encode().unwrap();
        let description = format!("{}, {} octets", written.what, octets.len());
        let message = Message::new(
            octets,
            Some(written.path),
            Some(written.edit),
            Some(written.typed),
        );
        inputs.push(Input {
            name: written.name,
            description,
            messages: vec![message],
        });
    }
    inputs
}

/// Inputs of one message each, which has `counts` IEs of one name: what an
/// operation costs as the IEs of a message grow in number. A container has
/// 65,535 at most. The path and the edit of such a message are of the value
/// of its last IE.
pub fn many_ies(counts: &[usize]) -> Vec<Input> {
    let input = |count: &usize| {
        let octets = protocol::many_ies(*count).encode().unwrap();
        let description = format!("{count} IEs of one name, {} octets", octets.len());
        let last = format!("{}/{}/value", protocol::ROOT, count - 1);
        let message = Message::new(octets, Some(&last), Some((&last, json!(2))), None);
        Input {
            name: format!("ies-{count}").leak(),
            description,
            messages: vec![message],
        }
    };
    counts.iter().map(input).collect()
}

// ── Operations ──────────────────────────────────────────────────────────────

/// One call of an operation, with what it needs already made. It is called
/// once; what it was given is dropped with it, after the call.
pub type Call<'a> = Box<dyn FnMut() + 'a>;

/// How an operation is called on the message `index` of its input.
pub enum Run<'a> {
    /// The operation needs the message alone.
    Direct(Box<dyn Fn(usize) + 'a>),
    /// The operation is given something that is made first and is no part
    /// of it, such as the view that `set` edits: this makes it, and returns
    /// the call.
    Prepared(Box<dyn Fn(usize) -> Call<'a> + 'a>),
}

/// An operation on the messages of an input. What a call returns is dropped
/// in it.
pub struct Operation<'a> {
    /// The name of the operation: `decode`.
    pub name: &'static str,
    /// How many messages the input has.
    pub messages: usize,
    /// How the operation is called.
    pub run: Run<'a>,
}

impl Operation<'_> {
    /// The calls of the operation on every message of the input, each with
    /// what it needs already made.
    pub fn calls(&self) -> Vec<Call<'_>> {
        let call = |index| -> Call<'_> {
            match &self.run {
                Run::Direct(call) => Box::new(move || call(index)),
                Run::Prepared(prepare) => prepare(index),
            }
        };
        (0..self.messages).map(call).collect()
    }
}

/// What an operation returns, used and dropped. An operation that fails is
/// not one that is measured.
fn done<T, E: std::fmt::Debug>(result: Result<T, E>) {
    drop(black_box(result.unwrap()));
}

/// The operation `name` on the `messages` messages of an input: `call` on
/// the message `index`.
fn operation<'a>(name: &'static str, messages: usize, call: impl Fn(usize) + 'a) -> Operation<'a> {
    Operation {
        name,
        messages,
        run: Run::Direct(Box::new(call)),
    }
}

/// The operation `name` on the `messages` messages of an input: `prepare`
/// makes what the call on the message `index` needs, and returns the call.
fn prepared<'a, C: FnMut() + 'a>(
    name: &'static str,
    messages: usize,
    prepare: impl Fn(usize) -> C + 'a,
) -> Operation<'a> {
    Operation {
        name,
        messages,
        run: Run::Prepared(Box::new(move |index| Box::new(prepare(index)))),
    }
}

impl Input {
    /// The octets of the messages of this input, together.
    pub fn octets(&self) -> usize {
        self.messages
            .iter()
            .map(|message| message.octets.len())
            .sum()
    }

    /// The operations on this input, in the order of the reports: those of
    /// the codec, those that take the type of a message, for an input whose
    /// messages have them, and those of the JSON forms.
    pub fn operations(&self) -> Vec<Operation<'_>> {
        let messages = &self.messages;
        let count = messages.len();
        let message = move |at: usize| black_box(&messages[at]);

        let mut operations = vec![
            operation("decode", count, move |at| {
                done(Pdu::decode(&message(at).octets))
            }),
            operation("decode-borrowed", count, move |at| {
                done(Borrowed::decode_borrowed(&message(at).octets))
            }),
            operation("encode", count, move |at| done(message(at).pdu.encode())),
            operation("protocol-ies", count, move |at| {
                done(message(at).pdu.protocol_ies())
            }),
            prepared("set-protocol-ies", count, move |at| {
                let mut pdu = message(at).pdu.clone();
                let mut ies = Some(pdu.protocol_ies().unwrap().into_owned());
                move || done(pdu.set_protocol_ies(ies.take().unwrap()))
            }),
            operation("check", count, move |at| done(message(at).pdu.check())),
        ];

        if messages.iter().all(|message| message.typed.is_some()) {
            let typed = move |at: usize| message(at).typed.as_ref().unwrap();
            operations.extend([
                operation("decode-value", count, move |at| {
                    (typed(at).decode_value)(&message(at).pdu)
                }),
                prepared("extract", count, move |at| {
                    (typed(at).extract)(&message(at).pdu)
                }),
                prepared("build", count, move |at| {
                    (typed(at).build)(&message(at).pdu)
                }),
            ]);
        }

        operations.extend([
            operation("to-view", count, move |at| done(message(at).pdu.to_view())),
            operation("to-tree", count, move |at| done(message(at).pdu.to_tree())),
            operation("from-tree", count, move |at| {
                done(Pdu::from_tree(&message(at).tree))
            }),
            operation("from-view", count, move |at| {
                done(Pdu::from_view(&message(at).view))
            }),
            operation("paths", count, move |at| {
                drop(black_box(inspect::paths(&message(at).view)))
            }),
            operation("select", count, move |at| {
                let message = message(at);
                let selected = inspect::select(&message.view, &message.path).unwrap();
                assert!(!selected.is_empty(), "{}", message.path);
                drop(black_box(selected));
            }),
            operation("check-path", count, move |at| {
                let message = message(at);
                done(inspect::check_path(message.name, &message.path))
            }),
            prepared("set", count, move |at| {
                let (path, value) = &message(at).edit;
                let (mut view, mut value) = (message(at).view.clone(), Some(value.clone()));
                move || done(inspect::set(&mut view, path, value.take().unwrap()))
            }),
            prepared("with-view", count, move |at| {
                let message = message(at);
                let mut view = message.view.clone();
                inspect::set(&mut view, &message.edit.0, message.edit.1.clone()).unwrap();
                move || done(message.pdu.with_view(&view))
            }),
        ]);
        operations
    }
}

// ── What the three programs are asked for ───────────────────────────────────

/// Whether the benchmark `operation` on `input` is one that `filter` asks
/// for: one of the words of the filter is in its name `operation/input`, or
/// the filter has no word.
pub fn selected(filter: &[String], operation: &str, input: &str) -> bool {
    let name = format!("{operation}/{input}");
    filter.is_empty() || filter.iter().any(|word| name.contains(word.as_str()))
}

/// The arguments of a benchmark program that are no options of cargo or of
/// the program: the words of a filter.
pub fn filter() -> Vec<String> {
    (std::env::args().skip(1))
        .filter(|argument| !argument.starts_with("--"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The messages that stand for what a network sends are ones that it
    /// may send: `check()` reports nothing in them, every IE of each has a
    /// value in its tree, and the macros read and build each as it is.
    #[test]
    fn the_written_messages_are_messages_of_the_specification() {
        let inputs = inputs();
        assert!(inputs.len() >= 5);
        for input in &inputs[1..] {
            let [message] = &input.messages[..] else {
                panic!("{} is one message", input.name);
            };
            assert_eq!(message.pdu.check().unwrap(), [], "{}", input.name);
            let tree = message.tree.to_string();
            assert!(!tree.contains("\"octets\""), "{}: {tree}", input.name);
            let typed = message.typed.as_ref().unwrap();
            let rebuilt = (typed.rebuilt)(&message.pdu).encode().unwrap();
            assert_eq!(rebuilt, message.octets, "{}", input.name);
        }
    }

    /// Each operation runs on each message of each input, and the edit of a
    /// message gives another PDU than the one that was received.
    #[test]
    fn every_operation_runs_on_every_input() {
        for input in inputs() {
            let operations = input.operations();
            let typed = input.messages[0].typed.is_some();
            assert_eq!(operations.len(), if typed { 18 } else { 15 });
            for operation in operations {
                assert_eq!(operation.messages, input.messages.len());
                for mut call in operation.calls() {
                    call();
                }
            }
            for message in &input.messages {
                let mut view = message.view.clone();
                inspect::set(&mut view, &message.edit.0, message.edit.1.clone()).unwrap();
                let edited = message.pdu.with_view(&view).unwrap().encode().unwrap();
                assert_ne!(edited, message.octets, "{}", message.name);
                assert_eq!(edited.len(), message.octets.len(), "{}", message.name);
            }
        }
    }
}
