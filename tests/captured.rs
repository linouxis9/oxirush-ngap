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
//! that its sender sent. With `inspect`, each is the same octets through its tree, in
//! which every IE is read with its type, and has the name that tshark gives it. The tests of `oracle/` compare what the codec of rasn makes of each.

use std::collections::BTreeMap;

use oxirush_ngap::Direction;
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
        let pdu = match NGAPPDU::decode(&captured.octets) {
            Ok(pdu) => pdu,
            Err(error) => panic!("{name} of {}: {error}", captured.source),
        };
        // What the capture read in its first two octets.
        assert_eq!(pdu.procedure_code(), captured.procedure_code, "{name}");
        let alternative = Direction::ALL
            .iter()
            .position(|direction| *direction == pdu.direction());
        assert_eq!(alternative, Some(captured.alternative.into()), "{name}");

        // Written back, it is the octets that were sent, and so is the PDU that
        // borrows them.
        assert_eq!(pdu.encode().unwrap(), captured.octets, "{name}");
        let borrowed = NGAPPDU::decode(&captured.octets).unwrap();
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
    use oxirush_ngap::Ngap;
    use oxirush_ngap::tree::Value;

    /// Each value of a tree that is its octets, as an IE or a transfer that was not read
    /// with its type is, at any depth.
    fn unread(value: &Value, found: &mut Vec<String>) {
        match value {
            Value::Object(members) if members.len() == 1 && members.contains_key("octets") => {
                found.push(value.to_string())
            }
            Value::Object(members) => members.values().for_each(|member| unread(member, found)),
            Value::Array(entries) => entries.iter().for_each(|entry| unread(entry, found)),
            _ => {}
        }
    }

    #[test]
    fn each_captured_pdu_is_the_same_octets_through_its_tree() {
        let mut named = 0;
        for captured in captured() {
            let name = captured.name;
            let pdu = NGAPPDU::decode(&captured.octets).unwrap();

            // The message has the name that tshark gives it.
            let (message, _) = name.split_once('/').unwrap();
            if message != "-" {
                let set = Ngap::object_set(pdu.direction(), pdu.procedure_code());
                assert_eq!(set.map(|set| set.name), Some(message), "{name}");
                named += 1;
            }

            // Every IE and every transfer of its tree is read with its type, and the tree
            // gives the octets back.
            let tree = pdu.to_tree();
            let mut found = Vec::new();
            unread(&tree, &mut found);
            assert_eq!(found, Vec::<String>::new(), "{name}");
            let again = NGAPPDU::from_tree(&tree).unwrap().encode().unwrap();
            assert_eq!(again, captured.octets, "{name}: its tree");
        }
        assert!(named > 70, "{named} messages that tshark names");
    }
}
