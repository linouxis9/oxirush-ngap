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

//! One operation of a codec on one PDU, as `compare/compare.py` runs it for each codec:
//!
//! ```text
//! compare CODEC OPERATION ITERATIONS HEX
//! ```
//!
//! `CODEC` is `ours`, the crate, or `rasn`, the rasn 0.29 bindings of `oracle/`.
//! `OPERATION` is
//!
//! - `envelope`: the PDU decoded, its message left as octets, as `decode_borrowed` of
//!   the crate reads it;
//! - `decode`: the PDU decoded, its message and the value of each of its IEs, each of
//!   the type of its identifier (a transfer stays its octets);
//! - `encode`: the PDU written from the typed value of each of its IEs.
//!
//! It runs the operation `ITERATIONS` times as a warm-up, then three times `ITERATIONS`
//! times, and prints `{"ns": N, "allocations": A, "bytes": B}`: the time of a call in
//! the fastest of the three, and the allocations and the octets allocated of a call, and
//! `"held"`, the most memory that the program held, in KiB.
//! This file is the same in oxirush-ngap and oxirush-s1ap.

use benches::counting::{Counting, allocations};
use benches::{Pdu, TheirMessage, TheirPdu, codec, our_pdu, their_message, their_pdu};
use oracle::rasn;
use std::hint::black_box;
use std::time::Instant;

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// A typed value of the crate, which writes itself.
pub trait Ours {
    fn write(&self, writer: &mut codec::aper::Writer<'_>) -> Result<(), codec::Error>;
}

impl<T: codec::Encode> Ours for T {
    fn write(&self, writer: &mut codec::aper::Writer<'_>) -> Result<(), codec::Error> {
        self.encode(writer)
    }
}

/// A typed value of the rasn bindings, which writes itself in an open type.
pub trait Theirs {
    fn open(&self) -> rasn::types::Any;
}

impl<T: rasn::Encode> Theirs for T {
    fn open(&self) -> rasn::types::Any {
        rasn::types::Any::new(oracle::encode(self).unwrap())
    }
}

fn ours_typed<'o, T: codec::Decode<'o> + codec::Encode + 'o>(
    octets: &'o [u8],
) -> Result<Box<dyn Ours + 'o>, codec::Error> {
    Ok(Box::new(codec::aper::decode::<T>(octets)?))
}

fn theirs_typed<T: rasn::Decode + rasn::Encode + 'static>(
    octets: &[u8],
) -> Result<Box<dyn Theirs>, String> {
    Ok(Box::new(oracle::decode::<T>(octets)?))
}

fn ours_read<'o, T: codec::Decode<'o> + 'o>(octets: &'o [u8]) -> Result<(), codec::Error> {
    black_box(codec::aper::decode::<T>(octets)?);
    Ok(())
}

fn theirs_read<T: rasn::Decode>(octets: &[u8]) -> Result<(), String> {
    black_box(oracle::decode::<T>(octets)?);
    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/ies.rs"));

/// Decode a PDU and the value of each of its IEs with the crate, and drop them.
fn ours_decode(octets: &[u8]) {
    let pdu = Pdu::decode_borrowed(octets).unwrap();
    for ie in pdu.protocol_ies().unwrap().0 {
        ours_dropped(ie.id.0, &ie.value).unwrap().unwrap();
    }
}

/// Decode a PDU and the value of each of its IEs with the rasn bindings, and drop them.
fn theirs_decode(octets: &[u8]) {
    let pdu: TheirPdu = oracle::decode(octets).unwrap();
    let message: TheirMessage = oracle::decode(their_message(&pdu)).unwrap();
    for ie in message.protocol_ies.0 {
        theirs_dropped(ie.id.0, ie.value.as_bytes())
            .unwrap()
            .unwrap();
    }
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let [_, codec, operation, iterations, octets] = &arguments[..] else {
        panic!("compare CODEC OPERATION ITERATIONS HEX");
    };
    let iterations: u32 = iterations.parse().unwrap();
    let octets = hex::decode(octets).unwrap();
    let octets = &octets[..];
    let mut call: Box<dyn FnMut()> = match (codec.as_str(), operation.as_str()) {
        ("ours", "envelope") => Box::new(|| {
            black_box(Pdu::decode_borrowed(black_box(octets)).unwrap());
        }),
        ("ours", "decode") => Box::new(|| ours_decode(black_box(octets))),
        ("ours", "encode") => {
            // What the typed values borrow lives as long as the program.
            let pdu: &'static Pdu = Box::leak(Box::new(Pdu::decode(octets).unwrap()));
            let ies = Box::leak(Box::new(pdu.protocol_ies().unwrap().0));
            let typed: Vec<_> = (ies.iter())
                .map(|ie| {
                    (
                        ie.id,
                        ie.criticality,
                        ours(ie.id.0, &ie.value).unwrap().unwrap(),
                    )
                })
                .collect();
            // As the macro that builds a PDU writes one: each IE in its open type, in the
            // octets of the message, then the PDU of the message.
            Box::new(move || {
                let mut octets = Vec::with_capacity(256);
                let mut writer = codec::aper::Writer::new(&mut octets);
                writer.bit(false);
                writer.size(typed.len(), 0, 65535).unwrap();
                for (id, criticality, value) in &typed {
                    writer.unsigned(id.0.into(), 0, 65535).unwrap();
                    writer.index(*criticality as u64, 3, false);
                    let start = writer.begin_open_type();
                    value.write(&mut writer).unwrap();
                    writer.end_open_type(start);
                }
                let written = our_pdu(pdu, codec::Octets::owned(octets));
                black_box(written.encode().unwrap());
            })
        }
        ("rasn", "envelope") => Box::new(|| {
            black_box(oracle::decode::<TheirPdu>(black_box(octets)).unwrap());
        }),
        ("rasn", "decode") => Box::new(|| theirs_decode(black_box(octets))),
        ("rasn", "encode") => {
            let pdu: TheirPdu = oracle::decode(octets).unwrap();
            let message: TheirMessage = oracle::decode(their_message(&pdu)).unwrap();
            let typed: Vec<_> = (message.protocol_ies.0.iter())
                .map(|ie| {
                    (
                        ie.clone(),
                        theirs(ie.id.0, ie.value.as_bytes()).unwrap().unwrap(),
                    )
                })
                .collect();
            Box::new(move || {
                let fields = typed
                    .iter()
                    .map(|(ie, value)| oracle::bindings::ProtocolIEField {
                        value: value.open(),
                        ..ie.clone()
                    });
                let mut message = message.clone();
                message.protocol_ies.0 = fields.collect();
                let message = rasn::types::Any::new(oracle::encode(&message).unwrap());
                black_box(oracle::encode(&their_pdu(&pdu, message)).unwrap());
            })
        }
        (codec, operation) => panic!("no {operation} of {codec}"),
    };
    for _ in 0..iterations {
        call();
    }
    let [count, bytes, _] = allocations(&mut call);
    let mut fastest = f64::MAX;
    for _ in 0..3 {
        let started = Instant::now();
        for _ in 0..iterations {
            call();
        }
        fastest = fastest.min(started.elapsed().as_secs_f64() * 1e9 / f64::from(iterations));
    }
    let held = held();
    println!(r#"{{"ns": {fastest:.1}, "allocations": {count}, "bytes": {bytes}, "held": {held}}}"#);
}

/// The most memory that the program held, in KiB: `VmHWM` of `/proc/self/status`, of
/// this program alone.
fn held() -> u64 {
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let line = status.lines().find(|line| line.starts_with("VmHWM:"));
    let number = line.and_then(|line| line.split_whitespace().nth(1));
    number.and_then(|number| number.parse().ok()).unwrap_or(0)
}
