use crate::ffi::macros::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
pub extern "C" fn launcher_free_buffer(raw_ptr: *mut u8, length: usize) {
    if !raw_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(std::ptr::slice_from_raw_parts_mut(raw_ptr, length));
        }
    }
}

#[no_mangle]
pub extern "C" fn launcher_free_string(raw_ptr: *mut c_char) {
    if !raw_ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(raw_ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn launcher_free_string_list(ptr: *mut *mut c_char, count: usize) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let slice = std::slice::from_raw_parts_mut(ptr, count);
        for item in slice.iter_mut() {
            if !item.is_null() {
                let _ = CString::from_raw(*item);
            }
        }
        let _ = Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, count));
    }
}
