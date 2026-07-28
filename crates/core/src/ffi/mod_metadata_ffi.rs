use crate::ffi::macros::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
pub extern "C" fn launcher_parse_modrinth_project(json_ptr: *const c_char) -> *mut c_char {
    ffi_null_check!(json_ptr);
    let json = ffi_cstr_to_str!(json_ptr);
    match mod_metadata::parse_modrinth_project(json) {
        Ok(project) => {
            let serialized = serde_json::to_string(&project).unwrap_or_default();
            ffi_cstring_to_raw!(serialized)
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn launcher_parse_modrinth_version(json_ptr: *const c_char) -> *mut c_char {
    ffi_null_check!(json_ptr);
    let json = ffi_cstr_to_str!(json_ptr);
    match mod_metadata::parse_modrinth_version(json) {
        Ok(version) => {
            let serialized = serde_json::to_string(&version).unwrap_or_default();
            ffi_cstring_to_raw!(serialized)
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn launcher_parse_curseforge_project(json_ptr: *const c_char) -> *mut c_char {
    ffi_null_check!(json_ptr);
    let json = ffi_cstr_to_str!(json_ptr);
    match mod_metadata::parse_curseforge_project(json) {
        Ok(project) => {
            let serialized = serde_json::to_string(&project).unwrap_or_default();
            ffi_cstring_to_raw!(serialized)
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn launcher_parse_curseforge_version(json_ptr: *const c_char) -> *mut c_char {
    ffi_null_check!(json_ptr);
    let json = ffi_cstr_to_str!(json_ptr);
    match mod_metadata::parse_curseforge_version(json) {
        Ok(version) => {
            let serialized = serde_json::to_string(&version).unwrap_or_default();
            ffi_cstring_to_raw!(serialized)
        }
        Err(_) => std::ptr::null_mut(),
    }
}
