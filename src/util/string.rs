use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub fn ptr_to_str(c_buf: *const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(c_buf) };
    let str_slice = c_str.to_str().unwrap();
    str_slice.to_owned()
}

pub fn str_to_ptr(string: &str) -> *const c_char {
    let c_string = CString::new(string).unwrap();
    let str_bytes = c_string.as_bytes_with_nul();

    let str_box = Box::new(str_bytes);
    Box::into_raw(str_box) as *const _
}
