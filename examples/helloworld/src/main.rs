// SPDX-License-Identifier: MIT

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use core::panic::PanicInfo;

#[macro_use]
extern crate alloc;

#[global_allocator]
static ALLOCATOR: nxdk_rs::alloc::XboxKernelAlloc = nxdk_rs::alloc::XboxKernelAlloc {};

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let msg = format!("Hello from {}!", "Rust");

    nxdk_rs::hal::video::xvideo_set_mode(640, 480, 32, nxdk_rs::hal::video::RefreshRate::Default);
    nxdk_rs::hal::debug::debug_print_str(&msg);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
