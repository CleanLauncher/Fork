use process_launcher::ProcessLauncher;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn launch_instance(instance_path: *const c_char) -> i32 {
    if instance_path.is_null() {
        return -1;
    }
    let c_str = unsafe { CStr::from_ptr(instance_path) };
    let path = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    // Simulating launcher FFI call
    log::info!("Launching instance from FFI at {}", path);
    0
}
