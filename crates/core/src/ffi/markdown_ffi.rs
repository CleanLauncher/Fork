use crate::ffi::macros::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
pub extern "C" fn markdown_to_html(markdown_ptr: *const c_char) -> *mut c_char {
    ffi_null_check!(markdown_ptr);
    let markdown_text = ffi_cstr_to_str!(markdown_ptr);
    match markdown::markdown_to_html(markdown_text) {
        Ok(html_output) => ffi_cstring_to_raw!(html_output),
        Err(_) => std::ptr::null_mut(),
    }
}
