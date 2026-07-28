use crate::ffi::macros::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
pub extern "C" fn launcher_natural_compare(left_ptr: *const c_char, right_ptr: *const c_char, case_insensitive: bool) -> i32 {
    if left_ptr.is_null() || right_ptr.is_null() {
        return 0;
    }
    let left_str = match unsafe { CStr::from_ptr(left_ptr) }.to_str() {
        Ok(text) => text,
        Err(_) => return 0,
    };
    let right_str = match unsafe { CStr::from_ptr(right_ptr) }.to_str() {
        Ok(text) => text,
        Err(_) => return 0,
    };
    match string_utils::natural_compare(left_str, right_str, case_insensitive) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

#[no_mangle]
pub extern "C" fn launcher_human_readable_file_size(raw_byte_count: f64, use_si_units: bool, decimal_points: usize) -> *mut c_char {
    let formatted_size = string_utils::human_readable_file_size(raw_byte_count, use_si_units, decimal_points);
    ffi_cstring_to_raw!(formatted_size)
}
