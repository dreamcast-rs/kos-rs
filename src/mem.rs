/// Prints current memory usage stats to stdout.
pub fn malloc_stats() {
    extern "C" { fn malloc_stats(); }
    unsafe { malloc_stats(); }
}
