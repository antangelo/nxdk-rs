// SPDX-License-Identifier: MIT

use nxdk_sys::clib;
pub struct XboxKernelAlloc {}

unsafe impl core::alloc::GlobalAlloc for XboxKernelAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        clib::stdlib::malloc(layout.size() as libc::c_uint) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        clib::stdlib::free(ptr as *mut libc::c_void)
    }

    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        let csize = layout.size() as libc::c_uint;
        let mem = clib::stdlib::malloc(csize);
        if mem == core::ptr::null_mut() {
            return core::ptr::null_mut();
        }

        clib::string::memset(mem, 0, csize);
        mem as *mut u8
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: core::alloc::Layout, new_size: usize) -> *mut u8 {
        clib::stdlib::realloc(ptr as *mut libc::c_void, new_size as libc::c_uint) as *mut u8
    }
}
