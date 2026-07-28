use crate::ffi::macros::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
pub extern "C" fn launcher_http_set_user_agent(agent_ptr: *const c_char) {
    if agent_ptr.is_null() {
        return;
    }
    let agent = ffi_cstr_to_str_void!(agent_ptr);
    http_client::set_user_agent(agent);
}

#[no_mangle]
pub extern "C" fn launcher_http_set_timeout(timeout_ms: u64) {
    http_client::set_timeout(timeout_ms);
}

#[no_mangle]
pub extern "C" fn launcher_http_set_header(name_ptr: *const c_char, value_ptr: *const c_char) {
    if name_ptr.is_null() || value_ptr.is_null() {
        return;
    }
    let name = ffi_cstr_to_str_void!(name_ptr);
    let value = ffi_cstr_to_str_void!(value_ptr);
    http_client::set_header(name, value);
}

#[no_mangle]
pub extern "C" fn launcher_http_get(url_ptr: *const c_char, out_status: *mut u16, out_len: *mut usize) -> *mut u8 {
    ffi_null_check!(url_ptr, out_status, out_len);
    let url = ffi_cstr_to_str!(url_ptr);
    match http_client::get(url) {
        Ok(response) => {
            unsafe {
                *out_status = response.status;
            }
            ffi_buffer_to_box!(response.body, out_len)
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn launcher_http_get_file(url_ptr: *const c_char, path_ptr: *const c_char, out_status: *mut u16) -> bool {
    ffi_false_check!(url_ptr, path_ptr);
    let url = ffi_cstr_to_str_false!(url_ptr);
    let path = ffi_cstr_to_str_false!(path_ptr);
    match http_client::download_to_file(url, path) {
        Ok(result) => {
            unsafe {
                *out_status = result.status;
            }
            true
        }
        Err(_) => false,
    }
}

#[no_mangle]
pub extern "C" fn launcher_http_get_file_resume(
    url_ptr: *const c_char,
    path_ptr: *const c_char,
    existing_bytes: u64,
    max_retries: u32,
    out_status: *mut u16,
) -> bool {
    ffi_false_check!(url_ptr, path_ptr);
    let url = ffi_cstr_to_str_false!(url_ptr);
    let path = ffi_cstr_to_str_false!(path_ptr);
    let retries = if max_retries == 0 { None } else { Some(max_retries) };
    match http_client::download_to_file_with_resume(url, path, existing_bytes, retries) {
        Ok(result) => {
            unsafe {
                *out_status = result.status;
            }
            true
        }
        Err(_) => false,
    }
}

#[no_mangle]
pub extern "C" fn launcher_http_post_json(
    url_ptr: *const c_char,
    body_ptr: *const u8,
    body_len: usize,
    out_status: *mut u16,
    out_len: *mut usize,
) -> *mut u8 {
    ffi_null_check!(url_ptr, body_ptr, out_status, out_len);
    let url = ffi_cstr_to_str!(url_ptr);
    let body = unsafe { slice::from_raw_parts(body_ptr, body_len) };
    match http_client::post_json(url, body, None) {
        Ok(response) => {
            unsafe {
                *out_status = response.status;
            }
            ffi_buffer_to_box!(response.body, out_len)
        }
        Err(_) => std::ptr::null_mut(),
    }
}
