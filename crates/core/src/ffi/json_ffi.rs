use crate::ffi::macros::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
pub extern "C" fn json_validate(data_ptr: *const u8, data_len: usize) -> bool {
    if data_ptr.is_null() {
        return false;
    }
    let input_bytes = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    json::parse(input_bytes).is_ok()
}

#[no_mangle]
pub extern "C" fn json_is_binary(data_ptr: *const u8, data_len: usize) -> bool {
    if data_ptr.is_null() {
        return false;
    }
    let input_bytes = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    json::is_binary_json(input_bytes)
}
