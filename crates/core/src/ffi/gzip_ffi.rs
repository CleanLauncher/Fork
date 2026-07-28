use crate::ffi::macros::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
pub extern "C" fn gzip_unzip(compressed_ptr: *const u8, compressed_len: usize, output_length: *mut usize) -> *mut u8 {
    ffi_null_check!(compressed_ptr, output_length);
    let input_bytes = unsafe { slice::from_raw_parts(compressed_ptr, compressed_len) };
    match gzip::unzip(input_bytes) {
        Ok(decompressed_payload) => ffi_buffer_to_box!(decompressed_payload, output_length),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn gzip_zip(uncompressed_ptr: *const u8, uncompressed_len: usize, output_length: *mut usize) -> *mut u8 {
    ffi_null_check!(uncompressed_ptr, output_length);
    let input_bytes = unsafe { slice::from_raw_parts(uncompressed_ptr, uncompressed_len) };
    match gzip::zip(input_bytes) {
        Ok(compressed_payload) => ffi_buffer_to_box!(compressed_payload, output_length),
        Err(_) => std::ptr::null_mut(),
    }
}
