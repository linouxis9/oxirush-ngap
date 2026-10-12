//! Decoding reserves memory for the elements it reads, not for the count a
//! length determinant claims.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use oxirush_ngap::ngap::{
    CellIDCancelledNR, CellIDCancelledNRItem, MBSIntendedServiceAreaList,
    UEAssociatedLogicalNGConnectionList,
};
use rasn::prelude::*;

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

/// `CellIDCancelledNR` with the list codec that rasn derives for it, in place of the
/// one of the crate.
#[derive(AsnType, Decode)]
#[rasn(delegate, size("1..=65535"))]
struct Derived(#[allow(dead_code)] SequenceOf<CellIDCancelledNRItem>);

/// The most memory that is reserved at once while `octets`, which are no `T`, are
/// decoded as one.
fn reserved<T: Decode>(octets: &[u8]) -> usize {
    let before = CURRENT.load(Ordering::SeqCst);
    PEAK.store(before, Ordering::SeqCst);
    assert!(rasn::aper::decode::<T>(octets).is_err());
    PEAK.load(Ordering::SeqCst) - before
}

// One test: the counters are those of the process.
#[test]
fn a_claimed_list_length_reserves_no_memory() {
    // After a count, 8192 octets that are no element: the first does not decode.
    let after = |count: &[u8]| [count, &[0xff; 8192]].concat();

    // CellIDCancelledNR is SIZE(1..65535): a count of 65535, then no element, then
    // octets that are none.
    for octets in [vec![0xff, 0xfe], after(&[0xff, 0xfe])] {
        let cells = reserved::<CellIDCancelledNR>(&octets);
        assert!(cells < 64 * 1024, "reserved {cells} bytes");
    }
    // The list codec of rasn reserves for the count that it reads, as far as the
    // octets after it have bits: the codec of the crate is in its place.
    let derived = reserved::<Derived>(&after(&[0xff, 0xfe]));
    assert!(
        derived > 1024 * 1024,
        "the codec of rasn reserved {derived} bytes"
    );

    // The two lists of SIZE(1..65536) have an unconstrained count: 16383 in two
    // octets, and a fragment of 65536.
    for count in [&[0xbf, 0xff][..], &[0xc4]] {
        let connections = reserved::<UEAssociatedLogicalNGConnectionList>(&after(count));
        assert!(connections < 64 * 1024, "reserved {connections} bytes");
        let areas = reserved::<MBSIntendedServiceAreaList>(&after(count));
        assert!(areas < 64 * 1024, "reserved {areas} bytes");
    }
}
