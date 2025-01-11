// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

/// Prints current memory usage stats to stdout.
pub fn malloc_stats() {
    unsafe { kos_sys::malloc::malloc_stats(); }
}
