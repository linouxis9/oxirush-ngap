//! Decoding reserves memory for the elements it reads, not for the count a
//! length determinant claims.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use oxirush_ngap::aper::{Decode, decode};
use oxirush_ngap::ngap::{
    CellIDCancelledNR, MBSIntendedServiceAreaList, UEAssociatedLogicalNGConnectionList,
};

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

/// The most memory that is reserved at once while `read` reads octets that are no
/// value, and refuses them.
fn reserved(read: impl FnOnce() -> bool) -> usize {
    let before = CURRENT.load(Ordering::SeqCst);
    PEAK.store(before, Ordering::SeqCst);
    assert!(!read());
    PEAK.load(Ordering::SeqCst) - before
}

/// The same for octets that the crate decodes as `T`.
fn reserved_as<'a, T: Decode<'a>>(octets: &'a [u8]) -> usize {
    reserved(|| decode::<T>(octets).is_ok())
}

// One test: the counters are those of the process.
#[test]
fn a_claimed_list_length_reserves_no_memory() {
    // After a count, 8192 octets that are no element: the first does not decode.
    let after = |count: &[u8]| [count, &[0xff; 8192]].concat();

    // CellIDCancelledNR is SIZE(1..65535): a count of 65535, then no element, then
    // octets that are none.
    for octets in [vec![0xff, 0xfe], after(&[0xff, 0xfe])] {
        let cells = reserved_as::<CellIDCancelledNR>(&octets);
        assert!(cells < 64 * 1024, "reserved {cells} bytes");
    }
    // The two lists of SIZE(1..65536) have an unconstrained count: 16383 in two
    // octets, and a fragment of 65536.
    for count in [&[0xbf, 0xff][..], &[0xc4]] {
        let octets = after(count);
        let connections = reserved_as::<UEAssociatedLogicalNGConnectionList>(&octets);
        assert!(connections < 64 * 1024, "reserved {connections} bytes");
        let areas = reserved_as::<MBSIntendedServiceAreaList>(&octets);
        assert!(areas < 64 * 1024, "reserved {areas} bytes");
    }

    // A fragment of 65536 connections of four bits each, which is the longest list
    // that the type has, and the memory that this list takes.
    let fragment = [&[0xc4][..], &[0x00; 32_768]].concat();
    let longest = [&fragment[..], &[0x00]].concat();
    let before = CURRENT.load(Ordering::SeqCst);
    PEAK.store(before, Ordering::SeqCst);
    let list = decode::<UEAssociatedLogicalNGConnectionList>(&longest).unwrap();
    let whole = PEAK.load(Ordering::SeqCst) - before;
    assert_eq!(list.0.len(), 65_536);
    drop(list);
    // Sixteen such fragments (X.691 clause 11.9.3.8) are sixteen times too long a
    // list. It is refused when the length of
    // its second fragment is read, with no more memory than the longest list takes,
    // and not after the 524,288 octets of its elements are read.
    let beyond = [fragment.repeat(16), vec![0x00]].concat();
    let refused = reserved_as::<UEAssociatedLogicalNGConnectionList>(&beyond);
    assert!(
        refused <= whole,
        "reserved {refused} bytes, and the longest list takes {whole}"
    );
}
