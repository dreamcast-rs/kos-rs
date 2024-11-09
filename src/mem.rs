/// Prints current memory usage stats to stdout.
pub fn malloc_stats() {
    unsafe { kos_sys::malloc::malloc_stats(); }
}
