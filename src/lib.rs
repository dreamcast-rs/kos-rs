#![no_std]
extern crate alloc;
pub mod allocator;
pub mod dbglog;
pub mod print;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
