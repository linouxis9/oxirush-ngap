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

//! The time and the memory of each operation as the IEs of a message grow in
//! number.
//!
//! ```bash
//! cargo bench --bench scale                  # 1,000, 10,000 and 65,535 IEs
//! cargo bench --bench scale -- 65535 paths   # one size, and the operations whose name has "paths"
//! ```
//!
//! An input is one message of that many IEs of one name. A row is one
//! operation on it, and gives for one call:
//!
//! - `ms`: its time, the least of three calls, or that of one call when it
//!   took more than two seconds;
//! - `allocs`: its allocations;
//! - `peak`: the most octets that were allocated at one time during the call,
//!   beyond what was allocated before it;
//! - `x octets`: that peak, in octets of the message.
//!
//! An operation whose time grows as the number of IEs does takes ten times
//! longer on ten times more of them, and has the same `x octets`.

use benches::counting::{Counting, allocations};
use benches::{filter, many_ies, selected};
use std::time::{Duration, Instant};

#[global_allocator]
static ALLOCATOR: Counting = Counting;

fn main() {
    let words = filter();
    let counts: Vec<usize> = words.iter().filter_map(|word| word.parse().ok()).collect();
    let counts = match counts.is_empty() {
        true => vec![1_000, 10_000, 65_535],
        false => counts,
    };
    let names: Vec<String> = (words.into_iter())
        .filter(|word| word.parse::<usize>().is_err())
        .collect();
    println!(
        "{:<17} {:<10} {:>11} {:>9} {:>11} {:>9}",
        "operation", "input", "ms", "allocs", "peak", "x octets"
    );
    for input in many_ies(&counts) {
        eprintln!("{}: {}", input.name, input.description);
        for operation in input.operations() {
            if !selected(&names, operation.name, input.name) {
                continue;
            }
            let (mut least, mut counted) = (Duration::MAX, [0; 3]);
            for _ in 0..3 {
                // What the call needs is made before it, and is no part of it.
                let mut calls = operation.calls();
                let started = Instant::now();
                counted = allocations(&mut calls[0]);
                least = least.min(started.elapsed());
                if least > Duration::from_secs(2) {
                    break;
                }
            }
            println!(
                "{:<17} {:<10} {:>11.3} {:>9} {:>11} {:>9.1}",
                operation.name,
                input.name,
                least.as_secs_f64() * 1e3,
                counted[0],
                counted[2],
                counted[2] as f64 / input.octets() as f64
            );
        }
    }
}
