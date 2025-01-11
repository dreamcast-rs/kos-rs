// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

#![doc(html_logo_url = "https://kos-rs.dreamcast.wiki/kos-rs_logo.png")]
#![doc(html_favicon_url = "https://kos-rs.dreamcast.wiki/kos-rs_favicon.ico")]

//! Bindings to KallistiOS for Sega Dreamcast.
//!
//! See [dreamcast.rs](https://dreamcast.rs) or the [dreamcast.wiki](https://dreamcast.wiki)
//! for more information on setting up KallistiOS and Rust to use this crate.

pub mod dbglog;
pub mod mem;

// Re-exports
pub use kos_sys as ffi;
pub use kos_sys::KOS_INIT_FLAGS as INIT_FLAGS;
#[doc(hidden)]
pub use kos_sys::libc as libc;
