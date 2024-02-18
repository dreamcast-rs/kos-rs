use alloc::alloc::{GlobalAlloc, Layout};
struct KOSAllocator;

// Provided by libc, will be integrated with libc crate later
extern "C" {
    pub fn memalign(alignment: usize, size: usize) -> *mut u8;
    pub fn free(ptr: *mut u8);
}

unsafe impl GlobalAlloc for KOSAllocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        memalign(layout.align(), layout.size()) as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut u8);
    }
}

#[global_allocator]
static ALLOCATOR: KOSAllocator = KOSAllocator;
