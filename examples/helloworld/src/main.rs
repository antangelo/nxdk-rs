// SPDX-License-Identifier: GPL-2.0-only

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use core::panic::PanicInfo;

#[global_allocator]
static ALLOCATOR: nxdk_rs::alloc::XboxKernelAlloc = nxdk_rs::alloc::XboxKernelAlloc {};

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let msg = cstr_core::CString::new("Hello from Rust!\n").expect("Unable to alloc string");

    unsafe {
        nxdk_rs::sys::hal::video::XVideoSetMode(640, 480, 32, 0);
        nxdk_rs::sys::hal::debug::debugPrint(msg.as_ptr() as *const libc::c_char);
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
