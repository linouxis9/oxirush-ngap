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

//! The allocations of each operation.
//!
//! ```bash
//! cargo bench --bench allocations             # every operation on every input
//! cargo bench --bench allocations -- decode/  # those whose name has "decode/"
//! ```
//!
//! A row is one operation on one input, and gives for one message:
//!
//! - `allocs`: the allocations of one call;
//! - `bytes`: the octets that those allocations asked for;
//! - `peak`: the most octets that were allocated at one time during the call,
//!   beyond what was allocated before it.
//!
//! For an input of several messages, `allocs` and `bytes` are the mean over
//! its messages and `peak` the largest. The numbers are exact, and the same
//! on every run: an allocator that counts stands in for the system allocator
//! in this program, which makes no measure of time.

use benches::counting::{Counting, allocations};
use benches::{filter, inputs, selected};

#[global_allocator]
static ALLOCATOR: Counting = Counting;

fn main() {
    let filter = filter();
    println!(
        "{:<17} {:<35} {:>9} {:>11} {:>11}",
        "operation", "input", "allocs", "bytes", "peak"
    );
    for input in inputs() {
        for operation in input.operations() {
            if !selected(&filter, operation.name, input.name) {
                continue;
            }
            let mut counted = [0; 3];
            for mut call in operation.calls() {
                let one = allocations(&mut call);
                counted[0] += one[0];
                counted[1] += one[1];
                counted[2] = counted[2].max(one[2]);
            }
            let mean = |total: usize| total as f64 / operation.messages as f64;
            println!(
                "{:<17} {:<35} {:>9.1} {:>11.0} {:>11}",
                operation.name,
                input.name,
                mean(counted[0]),
                mean(counted[1]),
                counted[2]
            );
        }
    }
}
