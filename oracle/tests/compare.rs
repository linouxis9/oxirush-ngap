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

//! The crate against its oracle: the same types with the codec of rasn, another
//! implementation of aligned PER. Each vector of the fixtures, and what a peer
//! may send in its place, is read and written by both, as a PDU, as its message, and
//! with the `inspect` feature as each IE and each transfer that a view decodes.
//!
//! What both read is the same value, by the octets that each writes of it and by its
//! JSON form, and what one refuses the other refuses. The bindings differ from the
//! oracle in one thing, which the comparison names where it meets it: they hold an
//! INTEGER in 64 bits, where rasn holds a number of any size.
//!
//! The tests take the type of a PDU, the message of a PDU as its type and each value
//! that a view decodes from the module `hooks` of the crate, which it has with the
//! configuration name of its hooks: cargo sets it when it is run from the directory
//! of this package. This file is the same for oxirush-ngap and oxirush-s1ap, which it
//! names `codec`.

#[cfg(not(any(oxirush_ngap_hooks, oxirush_s1ap_hooks)))]
compile_error!(
    "the crate is built without its hooks: run cargo from the directory of the oracle, whose \
     .cargo/config.toml sets them, or set RUSTFLAGS to the --cfg that this file names"
);

use codec::hooks::{
    BorrowedPdu, PDU_TYPE, Pdu, message_octets, message_octets_both_ways, message_read_and_written,
};

/// The octets that hexadecimal writes.
fn unhex(text: &str) -> Vec<u8> {
    let digits = text.as_bytes().chunks(2);
    digits
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

/// The octets of each vector of a file of the fixtures.
fn vectors(file: &str) -> Vec<Vec<u8>> {
    let vectors = file.lines().filter(|line| !line.starts_with('#'));
    vectors
        .map(|line| unhex(line.split('\t').nth(1).unwrap()))
        .collect()
}

/// The vectors of the fixtures: the messages, the vectors with content, the messages
/// of a later release, and the PDUs that other implementations sent.
fn fixtures() -> Vec<Vec<u8>> {
    let mut fixtures = vectors(include_str!("../../tests/fixtures/messages.tsv"));
    fixtures.extend(vectors(include_str!("../../tests/fixtures/vectors.tsv")));
    fixtures.extend(vectors(include_str!("../../tests/fixtures/later.tsv")));
    fixtures.extend(vectors(include_str!("../../tests/fixtures/captured.tsv")));
    fixtures
}

/// Give `read` the `octets`, then what a peer may send in their place: the octets
/// with one more, and for a place among them the octets up to it and the octets
/// with each bit of it changed. The places are each of the first 32 octets, where
/// the layout of a short message is, then one in 13, and each of the last 8. Octets
/// that are more than 2048 are cut at their first 32 and their last 8 places, and
/// none of their bits is changed: what they have more is the octets of one value.
fn variants(octets: &[u8], mut read: impl FnMut(&[u8])) {
    let long = octets.len() > 2048;
    let chosen = |at: &usize| match *at {
        0..32 => true,
        at if at + 8 >= octets.len() => true,
        at => !long && at % 13 == 0,
    };
    read(octets);
    let mut other = [octets, &[0]].concat();
    read(&other);
    other.pop();
    for at in (0..octets.len()).filter(chosen) {
        read(&octets[..at]);
        for bit in (0..8).filter(|_| !long) {
            other[at] ^= 0x80 >> bit;
            read(&other);
            other[at] ^= 0x80 >> bit;
        }
    }
}

/// `octets` as the type that the bindings name `name`: what the crate made of them,
/// which is the octets that it writes of the value or why it reads none, is what the
/// oracle makes of them. Says whether both read a value.
#[track_caller]
fn same(name: &str, octets: &[u8], ours: &Result<Vec<u8>, codec::Error>) -> bool {
    let ours = ours.as_ref().map(Vec::as_slice);
    oracle::same_octets(name, octets, ours.map_err(ToString::to_string))
}

/// A PDU and its message, as octets that are those of a PDU or that are none.
fn pdu_and_message(octets: &[u8]) -> (usize, usize) {
    let pdu = Pdu::decode(octets);
    let written = pdu.as_ref().map_err(Clone::clone).and_then(Pdu::encode);
    // `decode` and `encode` take a PDU of the usual layout octet by octet. The codec of
    // the type, which they stand for, gives the same PDU, the same error and the same
    // octets, and so does the PDU that borrows.
    let as_its_type = codec::aper::decode::<BorrowedPdu>(octets);
    assert_eq!(as_its_type, pdu, "{octets:02x?}");
    assert_eq!(BorrowedPdu::decode_borrowed(octets), pdu, "{octets:02x?}");
    if let Ok(pdu) = &pdu {
        assert_eq!(codec::aper::encode(pdu), written, "{pdu:?}");
        // The protocol IEs of its message are put back octet by octet too.
        if let Ok(ies) = pdu.protocol_ies() {
            let (octet_by_octet, as_its_type) = message_octets_both_ways(&ies);
            assert_eq!(octet_by_octet, as_its_type, "{pdu:?}");
        }
    }
    if !same(PDU_TYPE, octets, &written) {
        return (0, 0);
    }
    let Ok(pdu) = pdu else {
        return (0, 0);
    };
    // The message as its type, for a procedure that the protocol has in that direction.
    let Some((name, message)) = message_read_and_written(&pdu) else {
        return (1, 0);
    };
    (1, usize::from(same(name, message_octets(&pdu), &message)))
}

#[test]
fn a_pdu_and_its_message_are_read_and_written_as_the_oracle_reads_and_writes_them() {
    let (mut pdus, mut messages, mut refused) = (0, 0, 0);
    for fixture in fixtures() {
        // A vector is read by both, and written back by both as the same octets.
        assert_eq!(pdu_and_message(&fixture), (1, 1), "{fixture:02x?}");
        variants(&fixture, |octets| {
            let (pdu, message) = pdu_and_message(octets);
            pdus += pdu;
            messages += message;
            refused += 1 - pdu;
        });
    }
    // Both ways are taken: what is read, and what is refused.
    assert!(
        pdus > 10_000 && messages > 5_000 && refused > 10_000,
        "{pdus} PDUs and {messages} messages read, {refused} refused"
    );
}

mod values {
    use super::*;
    use serde_json::Value;
    use std::cell::Cell;

    thread_local! {
        /// How many values this thread compared, and how many of them both read.
        static COMPARED: Cell<(usize, usize)> = const { Cell::new((0, 0)) };
    }

    /// What a view decoded as the type that the bindings name `name` is what the
    /// oracle reads in the same octets: the same JSON form, or no value for both.
    fn observed(name: &'static str, octets: &[u8], ours: &Result<Value, codec::Error>) {
        let ours = ours.as_ref().map_err(ToString::to_string);
        let read = oracle::same_json(name, octets, ours);
        let (compared, both) = COMPARED.get();
        COMPARED.set((compared + 1, both + usize::from(read)));
    }

    #[test]
    fn each_value_of_a_view_is_what_the_oracle_reads() {
        codec::hooks::observe(Some(observed));
        let mut views = 0;
        for (at, fixture) in fixtures().iter().enumerate() {
            // A vector has a view, whose values are compared, and which gives its
            // octets back when no addition of a later release was skipped.
            let pdu = Pdu::decode(fixture).unwrap();
            let view = pdu.to_view().unwrap();
            let again = Pdu::from_view(&view).unwrap();
            assert_eq!(again.encode().unwrap(), *fixture);
            // What a peer may send in the place of one vector in five: the fuzz
            // targets make the same comparison on every input that they find.
            if at % 5 != 0 {
                continue;
            }
            variants(fixture, |octets| {
                if let Ok(pdu) = Pdu::decode(octets) {
                    views += usize::from(pdu.to_view().is_ok());
                }
            });
        }
        codec::hooks::observe(None);
        let (compared, read) = COMPARED.get();
        assert!(
            views > 2_000 && compared > 10_000 && read > 6_000,
            "{compared} values of {views} views compared, {read} read by both"
        );
    }
}
