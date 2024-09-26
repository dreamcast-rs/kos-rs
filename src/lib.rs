#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
pub mod allocator;
#[cfg(not(feature = "std"))]
pub mod panic;
#[cfg(not(feature = "std"))]
pub mod print;

pub mod dbglog;
pub mod mem;
