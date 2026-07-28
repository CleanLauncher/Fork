use crate::ffi::macros::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
pub extern "C" fn launcher_zip_entry_names(archive_path_ptr: *const c_char, out_count: *mut usize) -> *mut *mut c_char {
    ffi_null_check!(archive_path_ptr, out_count);
    let archive_path = ffi_cstr_to_str!(archive_path_ptr);
    match archive::zip_entry_names(archive_path) {
        Ok(names) => ffi_string_vec_to_raw!(names, out_count),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn launcher_zip_read_entry(archive_path_ptr: *const c_char, entry_name_ptr: *const c_char, out_len: *mut usize) -> *mut u8 {
    ffi_null_check!(archive_path_ptr, entry_name_ptr, out_len);
    let archive_path = ffi_cstr_to_str!(archive_path_ptr);
    let entry_name = ffi_cstr_to_str!(entry_name_ptr);
    match archive::zip_read_entry(archive_path, entry_name) {
        Ok(entry_data) => ffi_buffer_to_box!(entry_data, out_len),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn launcher_zip_extract_file(
    archive_path_ptr: *const c_char,
    entry_name_ptr: *const c_char,
    target_path_ptr: *const c_char,
) -> bool {
    ffi_false_check!(archive_path_ptr, entry_name_ptr, target_path_ptr);
    let archive_path = ffi_cstr_to_str_false!(archive_path_ptr);
    let entry_name = ffi_cstr_to_str_false!(entry_name_ptr);
    let target_path = ffi_cstr_to_str_false!(target_path_ptr);
    archive::zip_extract_file(archive_path, entry_name, target_path).is_ok()
}

#[no_mangle]
pub extern "C" fn launcher_zip_extract_dir(
    archive_path_ptr: *const c_char,
    subdir_prefix_ptr: *const c_char,
    target_dir_ptr: *const c_char,
    out_count: *mut usize,
) -> *mut *mut c_char {
    ffi_null_check!(archive_path_ptr, target_dir_ptr, out_count);
    let archive_path = ffi_cstr_to_str!(archive_path_ptr);
    let subdir_prefix = if subdir_prefix_ptr.is_null() {
        ""
    } else {
        ffi_cstr_to_str!(subdir_prefix_ptr)
    };
    let target_dir = ffi_cstr_to_str!(target_dir_ptr);
    match archive::zip_extract_dir(archive_path, subdir_prefix, target_dir) {
        Ok(files) => ffi_string_vec_to_raw!(files, out_count),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn launcher_zip_entry_exists(archive_path_ptr: *const c_char, entry_name_ptr: *const c_char) -> bool {
    ffi_false_check!(archive_path_ptr, entry_name_ptr);
    let archive_path = ffi_cstr_to_str_false!(archive_path_ptr);
    let entry_name = ffi_cstr_to_str_false!(entry_name_ptr);
    archive::zip_entry_exists(archive_path, entry_name).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn launcher_tar_entry_names(archive_path_ptr: *const c_char, out_count: *mut usize) -> *mut *mut c_char {
    ffi_null_check!(archive_path_ptr, out_count);
    let archive_path = ffi_cstr_to_str!(archive_path_ptr);
    match archive::tar_list_entries(archive_path) {
        Ok(entries) => {
            let name_list: Vec<String> = entries.into_iter().map(|e| e.entry_name).collect();
            ffi_string_vec_to_raw!(name_list, out_count)
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn launcher_tar_extract_dir(
    archive_path_ptr: *const c_char,
    target_dir_ptr: *const c_char,
    out_count: *mut usize,
) -> *mut *mut c_char {
    ffi_null_check!(archive_path_ptr, target_dir_ptr, out_count);
    let archive_path = ffi_cstr_to_str!(archive_path_ptr);
    let target_dir = ffi_cstr_to_str!(target_dir_ptr);
    match archive::tar_extract_dir(archive_path, target_dir) {
        Ok(files) => ffi_string_vec_to_raw!(files, out_count),
        Err(_) => std::ptr::null_mut(),
    }
}
