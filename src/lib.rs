#![no_std]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
pub mod allocator;
#[cfg(not(feature = "std"))]
pub mod print;

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(feature = "std")]
extern crate std;

pub mod dbglog;
pub mod mem;
