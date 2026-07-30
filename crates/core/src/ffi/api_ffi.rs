use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use super::macros::{ffi_check_ptr, ffi_result_string};

fn get_client(platform: &str, api_key: Option<&str>) -> Option<Box<dyn launcher_api::ApiClient>> {
    match platform {
        "modrinth" => Some(Box::new(launcher_api::ModrinthClient::new())),
        "curseforge" => api_key.map(|key| {
            Box::new(launcher_api::CurseForgeClient::new(key)) as Box<dyn launcher_api::ApiClient>
        }),
        "ftb" => Some(Box::new(launcher_api::FTBClient::new())),
        "technic" => Some(Box::new(launcher_api::TechnicClient::new())),
        "atlauncher" => Some(Box::new(launcher_api::ATLauncherClient::new())),
        _ => None,
    }
}

#[no_mangle]
pub extern "C" fn core_api_search_packs(
    platform: *const c_char,
    query: *const c_char,
    limit: u32,
    api_key: *const c_char,
) -> *mut c_char {
    let platform = ffi_check_ptr!(platform, return std::ptr::null_mut());
    let query = ffi_check_ptr!(query, return std::ptr::null_mut());
    let platform = unsafe { CStr::from_ptr(platform) }
        .to_string_lossy()
        .to_string();
    let query = unsafe { CStr::from_ptr(query) }
        .to_string_lossy()
        .to_string();
    let api_key = if api_key.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(api_key) }
                .to_string_lossy()
                .to_string(),
        )
    };

    match get_client(&platform, api_key.as_deref()) {
        Some(client) => {
            let result = client.search_packs(&query, limit);
            ffi_result_string!(result)
        }
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn core_api_get_pack(
    platform: *const c_char,
    pack_id: *const c_char,
    api_key: *const c_char,
) -> *mut c_char {
    let platform = ffi_check_ptr!(platform, return std::ptr::null_mut());
    let pack_id = ffi_check_ptr!(pack_id, return std::ptr::null_mut());
    let platform = unsafe { CStr::from_ptr(platform) }
        .to_string_lossy()
        .to_string();
    let pack_id = unsafe { CStr::from_ptr(pack_id) }
        .to_string_lossy()
        .to_string();
    let api_key = if api_key.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(api_key) }
                .to_string_lossy()
                .to_string(),
        )
    };

    match get_client(&platform, api_key.as_deref()) {
        Some(client) => {
            let result = client.get_pack(&pack_id);
            ffi_result_string!(result)
        }
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn core_api_get_versions(
    platform: *const c_char,
    pack_id: *const c_char,
    api_key: *const c_char,
) -> *mut c_char {
    let platform = ffi_check_ptr!(platform, return std::ptr::null_mut());
    let pack_id = ffi_check_ptr!(pack_id, return std::ptr::null_mut());
    let platform = unsafe { CStr::from_ptr(platform) }
        .to_string_lossy()
        .to_string();
    let pack_id = unsafe { CStr::from_ptr(pack_id) }
        .to_string_lossy()
        .to_string();
    let api_key = if api_key.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(api_key) }
                .to_string_lossy()
                .to_string(),
        )
    };

    match get_client(&platform, api_key.as_deref()) {
        Some(client) => {
            let result = client.get_versions(&pack_id);
            ffi_result_string!(result)
        }
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn core_api_download_url(
    platform: *const c_char,
    pack_id: *const c_char,
    version_id: *const c_char,
    api_key: *const c_char,
) -> *mut c_char {
    let platform = ffi_check_ptr!(platform, return std::ptr::null_mut());
    let pack_id = ffi_check_ptr!(pack_id, return std::ptr::null_mut());
    let version_id = ffi_check_ptr!(version_id, return std::ptr::null_mut());
    let platform = unsafe { CStr::from_ptr(platform) }
        .to_string_lossy()
        .to_string();
    let pack_id = unsafe { CStr::from_ptr(pack_id) }
        .to_string_lossy()
        .to_string();
    let version_id = unsafe { CStr::from_ptr(version_id) }
        .to_string_lossy()
        .to_string();
    let api_key = if api_key.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(api_key) }
                .to_string_lossy()
                .to_string(),
        )
    };

    match get_client(&platform, api_key.as_deref()) {
        Some(client) => {
            let result = client.download_url(&pack_id, &version_id);
            ffi_result_string!(result)
        }
        None => std::ptr::null_mut(),
    }
}
