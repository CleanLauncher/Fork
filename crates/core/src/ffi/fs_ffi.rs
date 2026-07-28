use crate::ffi::macros::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
pub extern "C" fn launcher_fs_read(path_ptr: *const c_char, output_length: *mut usize) -> *mut u8 {
    ffi_null_check!(path_ptr, output_length);
    let path_text = ffi_cstr_to_str!(path_ptr);
    match filesystem::read(path_text) {
        Ok(file_payload) => ffi_buffer_to_box!(file_payload, output_length),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn launcher_fs_write(path_ptr: *const c_char, data_ptr: *const u8, data_len: usize) -> bool {
    ffi_false_check!(path_ptr, data_ptr);
    let path_text = ffi_cstr_to_str_false!(path_ptr);
    let input_bytes = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    filesystem::write(path_text, input_bytes).is_ok()
}

#[no_mangle]
pub extern "C" fn launcher_fs_delete_path(path_ptr: *const c_char) -> bool {
    ffi_false_check!(path_ptr);
    let path_text = ffi_cstr_to_str_false!(path_ptr);
    filesystem::delete_path(path_text).is_ok()
}

#[no_mangle]
pub extern "C" fn launcher_fs_remove_invalid_filename_chars(input_ptr: *const c_char, replace_with: c_char) -> *mut c_char {
    ffi_null_check!(input_ptr);
    let input_text = ffi_cstr_to_str!(input_ptr);
    let sanitized = filesystem::remove_invalid_filename_chars(input_text, replace_with as u8 as char);
    ffi_cstring_to_raw!(sanitized)
}
