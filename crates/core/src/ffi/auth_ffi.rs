use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use super::macros::{ffi_check_ptr, ffi_check_result, ffi_result_string};

#[no_mangle]
pub extern "C" fn core_auth_microsoft_login(
    client_id: *const c_char,
    redirect_uri: *const c_char,
    auth_code: *const c_char,
) -> *mut c_char {
    let client_id = ffi_check_ptr!(client_id, return std::ptr::null_mut());
    let redirect_uri = ffi_check_ptr!(redirect_uri, return std::ptr::null_mut());
    let auth_code = ffi_check_ptr!(auth_code, return std::ptr::null_mut());

    let client_id = unsafe { CStr::from_ptr(client_id) }
        .to_string_lossy()
        .to_string();
    let redirect_uri = unsafe { CStr::from_ptr(redirect_uri) }
        .to_string_lossy()
        .to_string();
    let auth_code = unsafe { CStr::from_ptr(auth_code) }
        .to_string_lossy()
        .to_string();

    let auth = auth::MicrosoftAuth::new(&client_id, &redirect_uri);
    let result = auth.exchange_code(&auth_code);
    ffi_result_string!(result)
}

#[no_mangle]
pub extern "C" fn core_auth_microsoft_refresh(
    client_id: *const c_char,
    redirect_uri: *const c_char,
    refresh_token: *const c_char,
) -> *mut c_char {
    let client_id = ffi_check_ptr!(client_id, return std::ptr::null_mut());
    let redirect_uri = ffi_check_ptr!(redirect_uri, return std::ptr::null_mut());
    let refresh_token = ffi_check_ptr!(refresh_token, return std::ptr::null_mut());

    let client_id = unsafe { CStr::from_ptr(client_id) }
        .to_string_lossy()
        .to_string();
    let redirect_uri = unsafe { CStr::from_ptr(redirect_uri) }
        .to_string_lossy()
        .to_string();
    let refresh_token = unsafe { CStr::from_ptr(refresh_token) }
        .to_string_lossy()
        .to_string();

    let auth = auth::MicrosoftAuth::new(&client_id, &redirect_uri);
    let result = auth.refresh(&refresh_token);
    ffi_result_string!(result)
}

#[no_mangle]
pub extern "C" fn core_auth_microsoft_url(
    client_id: *const c_char,
    redirect_uri: *const c_char,
    state: *const c_char,
) -> *mut c_char {
    let client_id = ffi_check_ptr!(client_id, return std::ptr::null_mut());
    let redirect_uri = ffi_check_ptr!(redirect_uri, return std::ptr::null_mut());
    let state = ffi_check_ptr!(state, return std::ptr::null_mut());

    let client_id = unsafe { CStr::from_ptr(client_id) }
        .to_string_lossy()
        .to_string();
    let redirect_uri = unsafe { CStr::from_ptr(redirect_uri) }
        .to_string_lossy()
        .to_string();
    let state = unsafe { CStr::from_ptr(state) }
        .to_string_lossy()
        .to_string();

    let auth = auth::MicrosoftAuth::new(&client_id, &redirect_uri);
    let url = auth.authorize_url(&state);
    CString::new(url).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn core_auth_mojang_login(
    username: *const c_char,
    password: *const c_char,
) -> *mut c_char {
    let username = ffi_check_ptr!(username, return std::ptr::null_mut());
    let password = ffi_check_ptr!(password, return std::ptr::null_mut());

    let username = unsafe { CStr::from_ptr(username) }
        .to_string_lossy()
        .to_string();
    let password = unsafe { CStr::from_ptr(password) }
        .to_string_lossy()
        .to_string();

    let result = auth::MojangAuth::authenticate(&username, &password, None);
    ffi_result_string!(result)
}

#[no_mangle]
pub extern "C" fn core_auth_elyby_login(
    username: *const c_char,
    password: *const c_char,
) -> *mut c_char {
    let username = ffi_check_ptr!(username, return std::ptr::null_mut());
    let password = ffi_check_ptr!(password, return std::ptr::null_mut());

    let username = unsafe { CStr::from_ptr(username) }
        .to_string_lossy()
        .to_string();
    let password = unsafe { CStr::from_ptr(password) }
        .to_string_lossy()
        .to_string();

    let result = auth::ElyByAuth::authenticate(&username, &password);
    ffi_result_string!(result)
}

#[no_mangle]
pub extern "C" fn core_token_store_save(
    uuid: *const c_char,
    json: *const c_char,
    storage_dir: *const c_char,
) -> i32 {
    let uuid = ffi_check_ptr!(uuid, return -1);
    let json = ffi_check_ptr!(json, return -1);
    let storage_dir = ffi_check_ptr!(storage_dir, return -1);

    let uuid = unsafe { CStr::from_ptr(uuid) }
        .to_string_lossy()
        .to_string();
    let json = unsafe { CStr::from_ptr(json) }
        .to_string_lossy()
        .to_string();
    let storage_dir = unsafe { CStr::from_ptr(storage_dir) }
        .to_string_lossy()
        .to_string();

    let store = auth::TokenStore::new(&storage_dir);
    if let Ok(storage) = serde_json::from_str::<auth::TokenStorage>(&json) {
        match store.save(&storage) {
            Ok(()) => 0,
            Err(_) => -1,
        }
    } else {
        -1
    }
}

#[no_mangle]
pub extern "C" fn core_token_store_load(
    uuid: *const c_char,
    storage_dir: *const c_char,
) -> *mut c_char {
    let uuid = ffi_check_ptr!(uuid, return std::ptr::null_mut());
    let storage_dir = ffi_check_ptr!(storage_dir, return std::ptr::null_mut());

    let uuid = unsafe { CStr::from_ptr(uuid) }
        .to_string_lossy()
        .to_string();
    let storage_dir = unsafe { CStr::from_ptr(storage_dir) }
        .to_string_lossy()
        .to_string();

    let store = auth::TokenStore::new(&storage_dir);
    match store.load(&uuid) {
        Ok(storage) => match serde_json::to_string(&storage) {
            Ok(json) => CString::new(json).unwrap_or_default().into_raw(),
            Err(_) => std::ptr::null_mut(),
        },
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn core_token_store_remove(uuid: *const c_char, storage_dir: *const c_char) -> i32 {
    let uuid = ffi_check_ptr!(uuid, return -1);
    let storage_dir = ffi_check_ptr!(storage_dir, return -1);

    let uuid = unsafe { CStr::from_ptr(uuid) }
        .to_string_lossy()
        .to_string();
    let storage_dir = unsafe { CStr::from_ptr(storage_dir) }
        .to_string_lossy()
        .to_string();

    let store = auth::TokenStore::new(&storage_dir);
    match store.remove(&uuid) {
        Ok(()) => 0,
        Err(_) => -1,
    }
}
