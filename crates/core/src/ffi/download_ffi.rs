use download_manager::DownloadManager;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn start_download(url: *const c_char, dest: *const c_char) -> i32 {
    if url.is_null() || dest.is_null() {
        return -1;
    }
    let url_str = unsafe { CStr::from_ptr(url) }.to_string_lossy();
    let dest_str = unsafe { CStr::from_ptr(dest) }.to_string_lossy();

    // Simulating download manager FFI call
    log::info!("Starting download from FFI: {} -> {}", url_str, dest_str);
    0
}
