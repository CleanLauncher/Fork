use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use super::macros::{ffi_check_ptr, ffi_result_string};

#[no_mangle]
pub extern "C" fn core_download_file(
    url: *const c_char,
    dest: *const c_char,
    expected_hash: *const c_char,
) -> i32 {
    let url = ffi_check_ptr!(url, return -1);
    let dest = ffi_check_ptr!(dest, return -1);

    let url_str = unsafe { CStr::from_ptr(url) }.to_string_lossy().to_string();
    let dest_str = unsafe { CStr::from_ptr(dest) }
        .to_string_lossy()
        .to_string();
    let hash = if expected_hash.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(expected_hash) }
                .to_string_lossy()
                .to_string(),
        )
    };

    let task = download_manager::DownloadTask {
        url: url_str,
        destination: dest_str,
        expected_sha256: hash,
        size: None,
    };

    match download_manager::Downloader::new().download(&task) {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "C" fn core_download_verify(path: *const c_char, expected_sha256: *const c_char) -> i32 {
    let path = ffi_check_ptr!(path, return -1);
    let expected_sha256 = ffi_check_ptr!(expected_sha256, return -1);

    let path_str = unsafe { CStr::from_ptr(path) }
        .to_string_lossy()
        .to_string();
    let hash_str = unsafe { CStr::from_ptr(expected_sha256) }
        .to_string_lossy()
        .to_string();

    match download_manager::Downloader::verify_download(&path_str, &hash_str) {
        Ok(true) => 1,
        Ok(false) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "C" fn core_download_batch(json_tasks: *const c_char) -> *mut c_char {
    let json_tasks = ffi_check_ptr!(json_tasks, return std::ptr::null_mut());
    let json = unsafe { CStr::from_ptr(json_tasks) }
        .to_string_lossy()
        .to_string();

    let tasks: Vec<download_manager::DownloadTask> = match serde_json::from_str(&json) {
        Ok(t) => t,
        Err(_) => return std::ptr::null_mut(),
    };

    let results = download_manager::Downloader::new().download_parallel(&tasks);
    let json_results: Vec<serde_json::Value> = results
        .iter()
        .map(|r| match r {
            Ok(()) => serde_json::json!({"success": true}),
            Err(e) => serde_json::json!({"success": false, "error": e.to_string()}),
        })
        .collect();

    match serde_json::to_string(&json_results) {
        Ok(json) => CString::new(json).unwrap_or_default().into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}
