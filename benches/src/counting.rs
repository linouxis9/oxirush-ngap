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

//! An allocator that counts, which stands in for the system allocator in the
//! programs that say what an operation allocates. The numbers are exact, and
//! the same on every run.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

/// The allocator: a program names it as its `#[global_allocator]`.
pub struct Counting;

static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
static REQUESTED: AtomicUsize = AtomicUsize::new(0);
static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

fn allocated(size: usize) {
    ALLOCATIONS.fetch_add(1, Relaxed);
    REQUESTED.fetch_add(size, Relaxed);
    let live = LIVE.fetch_add(size, Relaxed) + size;
    PEAK.fetch_max(live, Relaxed);
}

// The system allocator does the work, and is given what this one is given.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        allocated(layout.size());
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        allocated(layout.size());
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        LIVE.fetch_sub(layout.size(), Relaxed);
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        LIVE.fetch_sub(layout.size(), Relaxed);
        allocated(size);
        unsafe { System.realloc(pointer, layout, size) }
    }
}

/// The allocations of one call of `operation`: how many, the octets they
/// asked for, and the most that were live beyond what was before the call.
pub fn allocations(mut operation: impl FnMut()) -> [usize; 3] {
    let before = [ALLOCATIONS.load(Relaxed), REQUESTED.load(Relaxed)];
    let live = LIVE.load(Relaxed);
    PEAK.store(live, Relaxed);
    operation();
    [
        ALLOCATIONS.load(Relaxed) - before[0],
        REQUESTED.load(Relaxed) - before[1],
        PEAK.load(Relaxed) - live,
    ]
}
