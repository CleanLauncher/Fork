use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use super::macros::{ffi_check_ptr, ffi_check_result, ffi_result_string};

#[no_mangle]
pub extern "C" fn core_instance_resolve(version_id: *const c_char) -> *mut c_char {
    let version_id = ffi_check_ptr!(version_id, return std::ptr::null_mut());
    let version_id = unsafe { CStr::from_ptr(version_id) }
        .to_string_lossy()
        .to_string();

    let result = instance_manager::ManifestLoader::resolve_version(&version_id);
    ffi_result_string!(result)
}

#[no_mangle]
pub extern "C" fn core_instance_fetch_manifest() -> *mut c_char {
    let result = instance_manager::ManifestLoader::fetch_manifest();
    ffi_result_string!(result)
}

#[no_mangle]
pub extern "C" fn core_instance_list_versions(filter_type: *const c_char) -> *mut c_char {
    let filter = if filter_type.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(filter_type) }
                .to_string_lossy()
                .to_string(),
        )
    };

    let result = instance_manager::ManifestLoader::list_versions(filter.as_deref());
    ffi_result_string!(result)
}

#[no_mangle]
pub extern "C" fn core_instance_resolve_latest_release() -> *mut c_char {
    let result = instance_manager::ManifestLoader::resolve_latest_release();
    ffi_result_string!(result)
}

#[no_mangle]
pub extern "C" fn core_instance_resolve_latest_snapshot() -> *mut c_char {
    let result = instance_manager::ManifestLoader::resolve_latest_snapshot();
    ffi_result_string!(result)
}

#[no_mangle]
pub extern "C" fn core_instance_config_create(
    config_json: *const c_char,
    instances_dir: *const c_char,
) -> *mut c_char {
    let config_json = ffi_check_ptr!(config_json, return std::ptr::null_mut());
    let instances_dir = ffi_check_ptr!(instances_dir, return std::ptr::null_mut());

    let config_json = unsafe { CStr::from_ptr(config_json) }
        .to_string_lossy()
        .to_string();
    let instances_dir = unsafe { CStr::from_ptr(instances_dir) }
        .to_string_lossy()
        .to_string();

    let manager = instance_manager::InstanceConfigManager::new(&instances_dir);
    match serde_json::from_str::<instance_manager::InstanceConfig>(&config_json) {
        Ok(config) => {
            let name = config.name.clone();
            let result = manager.create(&name, config);
            ffi_result_string!(result)
        }
        Err(e) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn core_instance_config_load(
    name: *const c_char,
    instances_dir: *const c_char,
) -> *mut c_char {
    let name = ffi_check_ptr!(name, return std::ptr::null_mut());
    let instances_dir = ffi_check_ptr!(instances_dir, return std::ptr::null_mut());

    let name = unsafe { CStr::from_ptr(name) }
        .to_string_lossy()
        .to_string();
    let instances_dir = unsafe { CStr::from_ptr(instances_dir) }
        .to_string_lossy()
        .to_string();

    let manager = instance_manager::InstanceConfigManager::new(&instances_dir);
    let result = manager.load(&name);
    ffi_result_string!(result)
}

#[no_mangle]
pub extern "C" fn core_instance_config_list(instances_dir: *const c_char) -> *mut c_char {
    let instances_dir = ffi_check_ptr!(instances_dir, return std::ptr::null_mut());
    let instances_dir = unsafe { CStr::from_ptr(instances_dir) }
        .to_string_lossy()
        .to_string();

    let manager = instance_manager::InstanceConfigManager::new(&instances_dir);
    let result = manager.list();
    ffi_result_string!(result)
}

#[no_mangle]
pub extern "C" fn core_instance_resolve_component_graph(config_json: *const c_char) -> *mut c_char {
    let config_json = ffi_check_ptr!(config_json, return std::ptr::null_mut());
    let config_json = unsafe { CStr::from_ptr(config_json) }
        .to_string_lossy()
        .to_string();

    match serde_json::from_str::<instance_manager::InstanceConfig>(&config_json) {
        Ok(config) => {
            let result = instance_manager::ComponentResolver::resolve_config(&config);
            ffi_result_string!(result)
        }
        Err(e) => std::ptr::null_mut(),
    }
}
