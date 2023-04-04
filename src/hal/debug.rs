// SPDX-License-Identifier: MIT

pub fn debug_print_cstr(msg: &cstr_core::CStr) {
    unsafe {
        nxdk_sys::hal::debug::debugPrint(msg.as_ptr() as *const libc::c_char);
    }
}

pub fn debug_print_str(msg: &str) {
    // FIXME: Don't use expect()
    let cstr = cstr_core::CString::new(msg).expect("CString failed");
    debug_print_cstr(&cstr);
}
