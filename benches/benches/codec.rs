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

//! The time of each operation, and its CPU profile.
//!
//! ```bash
//! cargo bench --bench codec                     # every operation on every input
//! cargo bench --bench codec -- decode/          # those whose name has "decode/"
//! cargo bench --bench codec -- --profile-time 10 to-view/fixtures
//! go tool pprof -top target/criterion/to-view/fixtures/profile/profile.pb
//! ```
//!
//! A benchmark is `operation/input`, and its time is that of one pass over
//! the messages of the input: criterion also gives it per message, as
//! elements per second. What an operation is given, such as the view that
//! `set` edits, is made before the pass and dropped after it, and is no part
//! of its time. With `--profile-time`, criterion runs the benchmark for that
//! many seconds under pprof, which samples the program a thousand times a
//! second and writes `profile.pb`, a profile that `go tool pprof` reads
//! (`-top`, `-list REGEX`, `-peek REGEX`, `-traces`). With
//! `PROFILE=flamegraph` in the environment it writes `flamegraph.svg`
//! instead.

use benches::{Run, inputs};
use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use pprof::criterion::{Output, PProfProfiler};
use std::time::Duration;

fn operations(criterion: &mut Criterion) {
    for input in inputs() {
        for operation in input.operations() {
            let mut group = criterion.benchmark_group(operation.name);
            group.throughput(Throughput::Elements(operation.messages as u64));
            group.bench_function(input.name, |bencher| match &operation.run {
                Run::Direct(call) => bencher.iter(|| (0..operation.messages).for_each(call)),
                Run::Prepared(_) => bencher.iter_batched_ref(
                    || operation.calls(),
                    |calls| calls.iter_mut().for_each(|call| call()),
                    BatchSize::LargeInput,
                ),
            });
            group.finish();
        }
    }
}

fn configuration() -> Criterion {
    let output = match std::env::var("PROFILE").as_deref() {
        Ok("flamegraph") => Output::Flamegraph(None),
        _ => Output::Protobuf,
    };
    Criterion::default()
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(2))
        .sample_size(50)
        .with_profiler(PProfProfiler::new(1000, output))
}

criterion_group! {
    name = codec;
    config = configuration();
    targets = operations
}
criterion_main!(codec);
