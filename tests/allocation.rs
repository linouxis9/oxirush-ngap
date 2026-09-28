//! Decoding reserves memory for the elements it reads, not for the count a
//! length determinant claims.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use oxirush_ngap::ngap::CellIDCancelledNR;

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

#[test]
fn a_claimed_list_length_reserves_no_memory() {
    // CellIDCancelledNR is SIZE(1..65535): a count of 65535, then no element.
    let before = CURRENT.load(Ordering::SeqCst);
    PEAK.store(before, Ordering::SeqCst);
    assert!(rasn::aper::decode::<CellIDCancelledNR>(&[0xff, 0xfe]).is_err());
    let reserved = PEAK.load(Ordering::SeqCst) - before;
    assert!(reserved < 64 * 1024, "reserved {reserved} bytes");
}
