#![doc(html_logo_url = "https://kos-rs.dreamcast.wiki/kos-rs_logo.png")]
#![doc(html_favicon_url = "https://kos-rs.dreamcast.wiki/kos-rs_favicon.ico")]

//! Bindings to KallistiOS v2.1.0 for Sega Dreamcast.
//!
//! See [dreamcast.rs](https://dreamcast.rs) or the [dreamcast.wiki](https://dreamcast.wiki)
//! for more information on setting up KallistiOS and Rust to use this crate.
//!
//! This crate uses the `std` feature to enable use of the Rust `std` library. This is enabled
//! by default. Use `default-features = false` when adding this crate to your `Cargo.toml` if
//! you are building a `no_std` project. In this case, `kos-rs` will provide a panic handler
//! and memory allocator in order to facilitate the use of memory allocating functions and
//! types.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[doc(hidden)]
#[cfg(not(feature = "std"))]
pub mod allocator;
#[doc(hidden)]
#[cfg(not(feature = "std"))]
pub mod panic;
#[doc(hidden)]
#[cfg(not(feature = "std"))]
pub mod print;

pub mod dbglog;
pub mod mem;
