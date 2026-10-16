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

//! The PDUs of `tests/fixtures/captured.tsv`, which other implementations sent:
//! Open5GS, free5GC and PacketRusher, captured on the wire. Nothing of them was
//! written or chosen with this crate. Each is read, and written back as the octets
//! that its sender sent. With `inspect`, each is the same octets through its tree and
//! through its view, in which every IE is read with its type, and has the name that
//! tshark gives it. The tests of `oracle/` compare what the codec of rasn makes of each.

use std::collections::BTreeMap;

use oxirush_ngap::ngap::*;

/// A PDU of the fixtures.
struct Captured {
    /// The name that tshark gives its message with the place of the PDU in the file, as
    /// `DownlinkNASTransport/captured-3`, or `-` before the place where tshark names none.
    name: &'static str,
    octets: Vec<u8>,
    /// The procedure code and the alternative of the PDU, read from its first two octets
    /// where it was captured.
    procedure_code: u8,
    alternative: u8,
    /// The implementation that sent it, with its version.
    source: &'static str,
}

fn captured() -> Vec<Captured> {
    let lines = include_str!("fixtures/captured.tsv").lines();
    let lines = lines.filter(|line| !line.starts_with('#') && !line.is_empty());
    let captured = lines.map(|line| {
        let fields: Vec<&str> = line.split('\t').collect();
        let [name, octets, code, alternative, source, _, _] = fields[..] else {
            panic!("{line} has not the seven columns of the file");
        };
        Captured {
            name,
            octets: hex::decode(octets).unwrap(),
            procedure_code: code.parse().unwrap(),
            alternative: alternative.parse().unwrap(),
            source,
        }
    });
    captured.collect()
}

#[test]
fn each_captured_pdu_is_read_and_written_as_the_octets_that_its_sender_sent() {
    let mut senders: BTreeMap<&str, usize> = BTreeMap::new();
    for captured in captured() {
        let name = captured.name;
        let pdu = match NGAP_PDU::decode(&captured.octets) {
            Ok(pdu) => pdu,
            Err(error) => panic!("{name} of {}: {error}", captured.source),
        };
        // What the capture read in its first two octets.
        assert_eq!(pdu.procedure_code(), captured.procedure_code, "{name}");
        let alternatives = [
            pdu.is_initiating(),
            pdu.is_successful(),
            pdu.is_unsuccessful(),
        ];
        let alternative = alternatives.iter().position(|is| *is);
        assert_eq!(alternative, Some(captured.alternative.into()), "{name}");

        // Written back, it is the octets that were sent, and so is the PDU that
        // borrows them.
        assert_eq!(pdu.encode().unwrap(), captured.octets, "{name}");
        let borrowed = NGAP_PDU::decode_borrowed(&captured.octets).unwrap();
        assert_eq!(borrowed, pdu, "{name}");
        assert_eq!(borrowed.encode().unwrap(), captured.octets, "{name}");

        // Its IEs, read whatever its message is and put back, are its message.
        let ies = pdu.protocol_ies().unwrap().into_owned();
        let mut again = pdu.clone();
        again.set_protocol_ies(ies).unwrap();
        assert_eq!(again.encode().unwrap(), captured.octets, "{name}");

        // Its sender wrote it by the clauses of X.691: the PDU, its message and the
        // value of each IE have each their one form. No implementation of the captures
        // is evidence for an encoding that a reader takes in another form.
        let strict = oxirush_ngap::aper::decode_strict::<NGAPPDU>(&captured.octets);
        assert_eq!(strict.as_ref(), Ok(&pdu), "{name}");
        let deviations = pdu.check().unwrap();
        let other = |deviation: &&oxirush_ngap::Deviation| {
            matches!(deviation, oxirush_ngap::Deviation::NotCanonical { .. })
        };
        assert_eq!(deviations.iter().find(other), None, "{name}");

        let sender = captured.source.split(' ').next().unwrap();
        *senders.entry(sender).or_default() += 1;
    }
    // Each PDU of each sender was read.
    let senders: Vec<_> = senders.into_iter().collect();
    assert_eq!(
        senders,
        [("Open5GS", 85), ("PacketRusher", 13), ("free5GC", 79)]
    );
}

#[cfg(feature = "inspect")]
mod forms {
    use super::*;
    use oxirush_ngap::inspect;
    use serde_json::Value;

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

    /// Why an IE or a transfer of a view was not read with its type, at any depth.
    fn unread(value: &Value, found: &mut Vec<String>) {
        match value {
            Value::Object(members) => {
                found.extend(members.get("_decode_error").map(Value::to_string));
                members.values().for_each(|member| unread(member, found));
            }
            Value::Array(entries) => entries.iter().for_each(|entry| unread(entry, found)),
            _ => {}
        }
    }

    #[test]
    fn each_captured_pdu_is_the_same_octets_through_its_tree_and_its_view() {
        let mut named = 0;
        for captured in captured() {
            let name = captured.name;
            let pdu = NGAP_PDU::decode(&captured.octets).unwrap();

            // The message has the name that tshark gives it.
            let (message, _) = name.split_once('/').unwrap();
            if message != "-" {
                assert_eq!(inspect::message_name(&pdu), Some(message), "{name}");
                named += 1;
            }

            // Every IE and every transfer of its view is read with its type.
            let view = pdu.to_view().unwrap();
            let mut found = Vec::new();
            unread(&view, &mut found);
            assert_eq!(found, Vec::<String>::new(), "{name}");

            // The view gives the octets back, and so do its values alone, without the
            // octets that it keeps of what was received.
            let octets = |view: &Value| NGAP_PDU::from_view(view).unwrap().encode().unwrap();
            assert_eq!(octets(&view), captured.octets, "{name}: its view");
            assert_eq!(
                octets(&written(&view)),
                captured.octets,
                "{name}: its values"
            );

            // And so does its tree.
            let tree = pdu.to_tree().unwrap();
            let again = NGAP_PDU::from_tree(&tree).unwrap().encode().unwrap();
            assert_eq!(again, captured.octets, "{name}: its tree");
        }
        assert!(named > 70, "{named} messages that tshark names");
    }
}
