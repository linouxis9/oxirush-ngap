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

//! One operation of hampi on one PDU, as `compare.py` runs it:
//!
//! ```text
//! oxirush-compare-hampi OPERATION ITERATIONS HEX
//! ```
//!
//! `OPERATION` is `decode`, the PDU decoded with every IE of its type, as hampi decodes,
//! or `encode`, that PDU written again. It prints what `compare` of the crate prints.
#![allow(
    dead_code,
    unreachable_patterns,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused,
    clippy::all
)]

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use asn1_codecs::{aper::AperCodec, PerCodecData};
use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::time::Instant;

/// The PDU type of the protocol of the build.
#[cfg(not(s1ap))]
type Pdu = bindings::NGAP_PDU;
#[cfg(s1ap)]
type Pdu = bindings::S1AP_PDU;

/// An allocator that counts, for the allocations of one call.
struct Counting;
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
static OCTETS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Relaxed);
        OCTETS.fetch_add(layout.size(), Relaxed);
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        System.dealloc(pointer, layout)
    }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Relaxed);
        OCTETS.fetch_add(size, Relaxed);
        System.realloc(pointer, layout, size)
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let [_, operation, iterations, octets] = &arguments[..] else {
        panic!("oxirush-compare-hampi OPERATION ITERATIONS HEX");
    };
    let iterations: u32 = iterations.parse().unwrap();
    let octets = hex::decode(octets).unwrap();
    let mut data = PerCodecData::from_slice_aper(&octets);
    let pdu = Pdu::aper_decode(&mut data).expect("hampi decodes the PDU");
    let mut call: Box<dyn FnMut()> = match operation.as_str() {
        "decode" => Box::new(|| {
            let mut data = PerCodecData::from_slice_aper(black_box(&octets));
            black_box(Pdu::aper_decode(&mut data).unwrap());
        }),
        _ => Box::new(|| {
            let mut out = PerCodecData::new_aper();
            black_box(&pdu).aper_encode(&mut out).unwrap();
            black_box(out.into_bytes());
        }),
    };
    for _ in 0..iterations {
        call();
    }
    let before = (ALLOCATIONS.load(Relaxed), OCTETS.load(Relaxed));
    call();
    let (count, bytes) = (ALLOCATIONS.load(Relaxed) - before.0, OCTETS.load(Relaxed) - before.1);
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
