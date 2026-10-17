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
fn the_additions_that_the_clauses_give_no_encoding_for_are_refused_by_both() {
    // ULCPSecurityInformation, which both protocols have: the extension bit, no
    // iE-Extensions, a MAC of 16 bits and a count of 5, then the additions of a later
    // version. The first is what a differential run against the codec of rasn found:
    // the crate and the oracle read it as a value without additions.
    let name = "ULCPSecurityInformation";
    type Value = codec::hooks::bindings::ULCPSecurityInformation;
    for (octets, read) in [
        // A bitmap of no bit, in the form of more than 64 bits (X.691 §19.8).
        (&[0x80, 0x02, 0x01, 0x00][..], false),
        // A bitmap of one addition that is not present (§19.1).
        (&[0x80, 0x02, 0x00, 0x00], false),
        // One addition, counted in the form of more than 64 bits (§11.9.3.4).
        (&[0x80, 0x02, 0x01, 0x01, 0x80, 0x01, 0xaa], false),
        // One addition of no octet (§11.1.3.1).
        (&[0x80, 0x02, 0x00, 0x02, 0x00], false),
        // One addition of one octet, and the value without its extension bit.
        (&[0x80, 0x02, 0x00, 0x02, 0x01, 0x00], true),
        (&[0x00, 0x02, 0x00], true),
    ] {
        let ours = codec::aper::decode::<Value>(octets);
        let written = ours.and_then(|value| codec::aper::encode(&value));
        assert_eq!(written.is_ok(), read, "{octets:02x?}: {written:?}");
        assert_eq!(same(name, octets, &written), read, "{octets:02x?}");
    }
}

/// The identifier of a UE that a node gives it: a whole number of a range above 64K,
/// by the name that each protocol gives its type. 42 is one octet after the number of
/// its octets, which the octets start with: three bits in NGAP and two in S1AP.
#[cfg(oxirush_ngap_hooks)]
mod ue_id {
    pub type Id = codec::hooks::bindings::AMFUENGAPID;
    pub const NAME: &str = "AMFUENGAPID";
    pub const IN_TWO_OCTETS: &[u8] = &[0x20, 0x00, 0x2a];
}
#[cfg(oxirush_s1ap_hooks)]
mod ue_id {
    pub type Id = codec::hooks::bindings::MMEUES1APID;
    pub const NAME: &str = "MMEUES1APID";
    pub const IN_TWO_OCTETS: &[u8] = &[0x40, 0x00, 0x2a];
}

#[test]
fn a_value_in_another_form_than_the_canonical_one_is_read_by_both_and_noted_here() {
    use codec::aper::{Decode, Encode, Fault, decode_strict, decode_with_faults, encode};
    use codec::hooks::bindings::{NASPDU, RRCEstablishmentCause};

    // `canonical` is the encoding that the clause of X.691 gives a value, and `other`
    // the same value in the other form: both codecs read both, and write the canonical
    // one. The crate notes the fault of the other form, and refuses it when it is
    // asked to be strict.
    fn both<'a, T: Decode<'a> + Encode + PartialEq + core::fmt::Debug>(
        name: &str,
        canonical: &'a [u8],
        other: &'a [u8],
        fault: Fault,
    ) {
        let (value, none) = decode_with_faults::<T>(canonical).unwrap();
        assert!(none.is_empty(), "{name}: {none:?}");
        assert_eq!(encode(&value).unwrap(), canonical, "{name}");
        assert!(
            oracle::same_octets(name, canonical, Ok(canonical)),
            "{name}"
        );
        let (same, faults) = decode_with_faults::<T>(other).unwrap();
        assert_eq!(same, value, "{name}");
        let noted: Vec<Fault> = faults.iter().map(|noted| noted.fault).collect();
        assert_eq!(noted, [fault], "{name}");
        assert!(
            oracle::same_octets(name, other, Ok(canonical)),
            "{name}: {fault}"
        );
        assert_eq!(decode_strict::<T>(canonical).unwrap(), value, "{name}");
        assert!(decode_strict::<T>(other).is_err(), "{name}: {fault}");
    }

    // §11.9.3.6: a length of 127 or less is one octet. A NAS-PDU of one octet.
    both::<NASPDU>(
        "NASPDU",
        &[0x01, 0xaa],
        &[0x80, 0x01, 0xaa],
        Fault::LengthInTwoOctets,
    );
    // §11.9.3.8.1: a fragment before another is the largest. A NAS-PDU of 32768 octets
    // in one fragment of twice 16K, and in two of 16K.
    let one = [&[0xc2][..], &[0x5a; 32768], &[0x00]].concat();
    let two = [
        &[0xc1][..],
        &[0x5a; 16384],
        &[0xc1],
        &[0x5a; 16384],
        &[0x00],
    ]
    .concat();
    both::<NASPDU>("NASPDU", &one, &two, Fault::SmallFragment);
    // §11.6.1: a normally small number of 63 or less is six bits after a zero bit. The
    // first value that RRCEstablishmentCause has after its extension marker: the
    // extension bit, then the index 0.
    let name = "RRCEstablishmentCause";
    both::<RRCEstablishmentCause>(
        name,
        &[0x80],
        &[0xc0, 0x01, 0x00],
        Fault::SmallNumberAsLarge,
    );
    // §11.3.6: a whole number is in the least octets that hold it. The identifier 42 in
    // one octet, and in two.
    both::<ue_id::Id>(
        ue_id::NAME,
        &[0x00, 0x2a],
        ue_id::IN_TWO_OCTETS,
        Fault::LongNumber,
    );
    // §11.1.4: padding bits are zero. The same identifier, with a one bit before its
    // octet.
    both::<ue_id::Id>(ue_id::NAME, &[0x00, 0x2a], &[0x01, 0x2a], Fault::Padding);

    // X.690 §8.19.2: an arc of an object identifier is the fewest octets that hold it.
    // {1 2 840}, and the same with 840 after an octet that holds nothing. No type that
    // both read by its name has an object identifier alone: the field is read by each.
    use oracle::rasn::types::ObjectIdentifier;
    let arcs = |octets: &[u8]| {
        let mut reader = codec::aper::Reader::noting(octets);
        let read = reader.object_identifier().unwrap().arcs().to_vec();
        let faults: Vec<Fault> = reader.faults().iter().map(|noted| noted.fault).collect();
        let theirs = oracle::rasn::aper::decode::<ObjectIdentifier>(octets).unwrap();
        let theirs: Vec<u32> = theirs.iter().copied().collect();
        assert_eq!(read, theirs, "{octets:02x?}");
        faults
    };
    assert_eq!(arcs(&[3, 0x2a, 0x86, 0x48]), []);
    assert_eq!(arcs(&[4, 0x2a, 0x80, 0x86, 0x48]), [Fault::LongArc]);
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
    use codec::tree::Value;
    use std::cell::Cell;

    thread_local! {
        /// How many values this thread compared, and how many of them both read.
        static COMPARED: Cell<(usize, usize)> = const { Cell::new((0, 0)) };
    }

    /// What a view decoded as the type that the bindings name `name` is what the
    /// oracle reads in the same octets: the same JSON form, or no value for both.
    fn observed(name: &'static str, octets: &[u8], ours: &Result<Value, codec::Error>) {
        let ours = ours.as_ref().map(serde_json::Value::from);
        let read = oracle::same_json(name, octets, ours.as_ref().map_err(ToString::to_string));
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
