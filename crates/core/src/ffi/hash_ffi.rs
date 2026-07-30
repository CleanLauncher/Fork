use crate::ffi::macros::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
pub extern "C" fn launcher_hash_sha256(data_ptr: *const u8, data_len: usize) -> *mut c_char {
    ffi_null_check!(data_ptr);
    let input_bytes = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    ffi_cstring_to_raw!(hashing::sha256(input_bytes))
}

#[no_mangle]
pub extern "C" fn launcher_hash_sha512(data_ptr: *const u8, data_len: usize) -> *mut c_char {
    ffi_null_check!(data_ptr);
    let input_bytes = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    ffi_cstring_to_raw!(hashing::sha512(input_bytes))
}

#[no_mangle]
pub extern "C" fn launcher_hash_md5(data_ptr: *const u8, data_len: usize) -> *mut c_char {
    ffi_null_check!(data_ptr);
    let input_bytes = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    ffi_cstring_to_raw!(hashing::md5(input_bytes))
}

#[no_mangle]
pub extern "C" fn launcher_hash_sha256_file(path_ptr: *const c_char) -> *mut c_char {
    ffi_null_check!(path_ptr);
    let path_text = ffi_cstr_to_str!(path_ptr);
    match hashing::sha256_file(path_text) {
        Ok(hash) => ffi_cstring_to_raw!(hash),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn launcher_verify_sha256(
    data_ptr: *const u8,
    data_len: usize,
    expected_ptr: *const c_char,
) -> bool {
    ffi_false_check!(data_ptr, expected_ptr);
    let input_bytes = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    let expected = ffi_cstr_to_str_false!(expected_ptr);
    hashing::verify_sha256(input_bytes, expected)
}
