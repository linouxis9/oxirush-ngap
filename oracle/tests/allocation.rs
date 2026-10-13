//! What the list codec that rasn derives reserves for a count that a peer claims, beside
//! what the reader of the crate reserves for it: the reason why the oracle has a list
//! codec of its own, and what `tests/allocation.rs` of the crate holds the crate to.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use codec::aper::decode;
use codec::ngap::CellIDCancelledNR;
use oracle::bindings::CellIDCancelledNRItem;
use oracle::rasn::{self, prelude::*};

struct PeakAllocator;

static CURRENT: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for PeakAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let current = CURRENT.fetch_add(layout.size(), Ordering::SeqCst) + layout.size();
        PEAK.fetch_max(current, Ordering::SeqCst);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        CURRENT.fetch_sub(layout.size(), Ordering::SeqCst);
        unsafe { System.dealloc(pointer, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: PeakAllocator = PeakAllocator;

/// `CellIDCancelledNR` of the oracle, with the list codec that rasn derives for it in
/// place of the one that the oracle has.
#[derive(AsnType, Decode)]
#[rasn(delegate, size("1..=65535"))]
struct Derived(#[allow(dead_code)] SequenceOf<CellIDCancelledNRItem>);

/// The most memory that is reserved at once while `read` reads octets that are no
/// value, and refuses them.
fn reserved(read: impl FnOnce() -> bool) -> usize {
    let before = CURRENT.load(Ordering::SeqCst);
    PEAK.store(before, Ordering::SeqCst);
    assert!(!read());
    PEAK.load(Ordering::SeqCst) - before
}

// One test: the counters are those of the process.
#[test]
fn the_list_codec_of_rasn_reserves_for_a_count_and_the_crate_does_not() {
    // CellIDCancelledNR is SIZE(1..65535): a count of 65535, then 8192 octets that are
    // no element.
    let octets = [&[0xff, 0xfe][..], &[0xff; 8192]].concat();
    // The list codec of rasn reserves for the count that it reads, as far as the
    // octets after it have bits.
    let derived = reserved(|| rasn::aper::decode::<Derived>(&octets).is_ok());
    assert!(
        derived > 1024 * 1024,
        "the codec of rasn reserved {derived} bytes"
    );
    // The reader of the crate reserves a little, and grows the list with the elements
    // that it reads.
    let cells = reserved(|| decode::<CellIDCancelledNR>(&octets).is_ok());
    assert!(cells < 64 * 1024, "reserved {cells} bytes");
}
