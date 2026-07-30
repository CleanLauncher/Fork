use crate::ffi::macros::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
pub extern "C" fn launcher_settings_new(path_ptr: *const c_char) -> *mut settings::SettingsStore {
    ffi_null_check!(path_ptr);
    let path = ffi_cstr_to_str!(path_ptr);
    Box::into_raw(Box::new(settings::SettingsStore::new(path)))
}

#[no_mangle]
pub extern "C" fn launcher_settings_free(ptr: *mut settings::SettingsStore) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn launcher_settings_load(ptr: *mut settings::SettingsStore) -> bool {
    if ptr.is_null() {
        return false;
    }
    let store = unsafe { &mut *ptr };
    store.load().is_ok()
}

#[no_mangle]
pub extern "C" fn launcher_settings_save(ptr: *mut settings::SettingsStore) -> bool {
    if ptr.is_null() {
        return false;
    }
    let store = unsafe { &mut *ptr };
    store.save().is_ok()
}

#[no_mangle]
pub extern "C" fn launcher_settings_get_string(
    ptr: *const settings::SettingsStore,
    key_ptr: *const c_char,
) -> *mut c_char {
    ffi_null_check!(key_ptr);
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    let store = unsafe { &*ptr };
    let key = ffi_cstr_to_str!(key_ptr);
    let value = store.get_string(key);
    if value.is_empty() && !store.contains(key) {
        return std::ptr::null_mut();
    }
    ffi_cstring_to_raw!(value)
}

#[no_mangle]
pub extern "C" fn launcher_settings_get_int(
    ptr: *const settings::SettingsStore,
    key_ptr: *const c_char,
) -> i64 {
    if key_ptr.is_null() {
        return 0;
    }
    if ptr.is_null() {
        return 0;
    }
    let store = unsafe { &*ptr };
    let key = match unsafe { CStr::from_ptr(key_ptr) }.to_str() {
        Ok(text) => text,
        Err(_) => return 0,
    };
    store.get_int(key)
}

#[no_mangle]
pub extern "C" fn launcher_settings_get_bool(
    ptr: *const settings::SettingsStore,
    key_ptr: *const c_char,
) -> bool {
    if key_ptr.is_null() {
        return false;
    }
    if ptr.is_null() {
        return false;
    }
    let store = unsafe { &*ptr };
    let key = ffi_cstr_to_str_false!(key_ptr);
    store.get_bool(key)
}

#[no_mangle]
pub extern "C" fn launcher_settings_set_string(
    ptr: *mut settings::SettingsStore,
    key_ptr: *const c_char,
    value_ptr: *const c_char,
) {
    if ptr.is_null() || key_ptr.is_null() || value_ptr.is_null() {
        return;
    }
    let store = unsafe { &mut *ptr };
    let key = ffi_cstr_to_str_void!(key_ptr);
    let value = ffi_cstr_to_str_void!(value_ptr);
    store.set_string(key, value);
}

#[no_mangle]
pub extern "C" fn launcher_settings_set_int(
    ptr: *mut settings::SettingsStore,
    key_ptr: *const c_char,
    value: i64,
) {
    if ptr.is_null() || key_ptr.is_null() {
        return;
    }
    let store = unsafe { &mut *ptr };
    let key = ffi_cstr_to_str_void!(key_ptr);
    store.set_int(key, value);
}

#[no_mangle]
pub extern "C" fn launcher_settings_set_bool(
    ptr: *mut settings::SettingsStore,
    key_ptr: *const c_char,
    value: bool,
) {
    if ptr.is_null() || key_ptr.is_null() {
        return;
    }
    let store = unsafe { &mut *ptr };
    let key = ffi_cstr_to_str_void!(key_ptr);
    store.set_bool(key, value);
}

#[no_mangle]
pub extern "C" fn launcher_settings_reset(
    ptr: *mut settings::SettingsStore,
    key_ptr: *const c_char,
) {
    if ptr.is_null() || key_ptr.is_null() {
        return;
    }
    let store = unsafe { &mut *ptr };
    let key = ffi_cstr_to_str_void!(key_ptr);
    store.reset(key);
}

#[no_mangle]
pub extern "C" fn launcher_settings_contains(
    ptr: *const settings::SettingsStore,
    key_ptr: *const c_char,
) -> bool {
    ffi_false_check!(key_ptr);
    if ptr.is_null() {
        return false;
    }
    let store = unsafe { &*ptr };
    let key = ffi_cstr_to_str_false!(key_ptr);
    store.contains(key)
}

#[no_mangle]
pub extern "C" fn launcher_settings_register_default(
    ptr: *mut settings::SettingsStore,
    key_ptr: *const c_char,
    value_ptr: *const c_char,
) {
    if ptr.is_null() || key_ptr.is_null() || value_ptr.is_null() {
        return;
    }
    let store = unsafe { &mut *ptr };
    let key = ffi_cstr_to_str_void!(key_ptr);
    let value = ffi_cstr_to_str_void!(value_ptr);
    store.register_default(key, value);
}

#[no_mangle]
pub extern "C" fn launcher_settings_register_alias(
    ptr: *mut settings::SettingsStore,
    alias_ptr: *const c_char,
    canonical_ptr: *const c_char,
) {
    if ptr.is_null() || alias_ptr.is_null() || canonical_ptr.is_null() {
        return;
    }
    let store = unsafe { &mut *ptr };
    let alias = ffi_cstr_to_str_void!(alias_ptr);
    let canonical = ffi_cstr_to_str_void!(canonical_ptr);
    store.register_alias(alias, canonical);
}

#[no_mangle]
pub extern "C" fn launcher_settings_is_dirty(ptr: *const settings::SettingsStore) -> bool {
    if ptr.is_null() {
        return false;
    }
    let store = unsafe { &*ptr };
    store.is_dirty()
}

#[no_mangle]
pub extern "C" fn launcher_settings_keys(
    ptr: *const settings::SettingsStore,
    out_count: *mut usize,
) -> *mut *mut c_char {
    ffi_null_check!(out_count);
    if ptr.is_null() {
        unsafe {
            *out_count = 0;
        }
        return std::ptr::null_mut();
    }
    let store = unsafe { &*ptr };
    let keys: Vec<String> = store.keys().into_iter().map(String::from).collect();
    ffi_string_vec_to_raw!(keys, out_count)
}
