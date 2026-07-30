use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

macro_rules! ffi_check_ptr {
    ($ptr:expr, $on_fail:expr) => {
        if $ptr.is_null() {
            return $on_fail;
        }
        unsafe { CStr::from_ptr($ptr) }
    };
}

macro_rules! ffi_check_result {
    ($result:expr, $on_ok:expr, $on_err:expr) => {
        match $result {
            Ok(val) => $on_ok(val),
            Err(_) => $on_err,
        }
    };
}

macro_rules! ffi_result_string {
    ($result:expr) => {{
        match $result {
            Ok(val) => {
                let json = serde_json::to_string(&val).unwrap_or_default();
                ffi_cstring_to_raw!(json)
            }
            Err(e) => {
                let error_json = serde_json::json!({"error": e.to_string()});
                ffi_cstring_to_raw!(error_json.to_string())
            }
        }
    }};
}
macro_rules! ffi_null_check {
    ($($ptr:expr),+ $(,)?) => {
        if $($ptr.is_null())||+ {
            return std::ptr::null_mut();
        }
    };
}
macro_rules! ffi_false_check {
    ($($ptr:expr),+ $(,)?) => {
        if $($ptr.is_null())||+ {
            return false;
        }
    };
}
macro_rules! ffi_cstr_to_str {
    ($ptr:expr) => {
        match unsafe { CStr::from_ptr($ptr) }.to_str() {
            Ok(text) => text,
            Err(_) => return std::ptr::null_mut(),
        }
    };
}
macro_rules! ffi_cstr_to_str_false {
    ($ptr:expr) => {
        match unsafe { CStr::from_ptr($ptr) }.to_str() {
            Ok(text) => text,
            Err(_) => return false,
        }
    };
}
macro_rules! ffi_cstr_to_str_void {
    ($ptr:expr) => {
        match unsafe { CStr::from_ptr($ptr) }.to_str() {
            Ok(text) => text,
            Err(_) => return,
        }
    };
}
macro_rules! ffi_buffer_to_box {
    ($data:expr, $output_length:expr) => {{
        let mut boxed_buffer = $data.into_boxed_slice();
        unsafe {
            *$output_length = boxed_buffer.len();
        }
        let raw_ptr = boxed_buffer.as_mut_ptr();
        Box::leak(boxed_buffer);
        raw_ptr
    }};
}
macro_rules! ffi_cstring_to_raw {
    ($s:expr) => {
        CString::new($s).unwrap_or_default().into_raw()
    };
}
macro_rules! ffi_string_vec_to_raw {
    ($vec:expr, $out_count:expr) => {{
        let count = $vec.len();
        unsafe {
            *$out_count = count;
        }
        let c_string_vec: Vec<*mut c_char> = $vec
            .into_iter()
            .filter_map(|name| CString::new(name).ok().map(|cs| cs.into_raw()))
            .collect();
        let mut boxed_slice = c_string_vec.into_boxed_slice();
        let raw_ptr = boxed_slice.as_mut_ptr();
        Box::leak(boxed_slice);
        raw_ptr
    }};
}
