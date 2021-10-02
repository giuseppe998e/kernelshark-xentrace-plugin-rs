use libc::c_char;
use std::{
    ffi::{CStr, CString},
    str::Utf8Error,
};

pub(crate) fn from_str_ptr<'a>(c_buf: *mut c_char) -> Result<&'a str, Utf8Error> {
    let c_str = unsafe { CStr::from_ptr(c_buf) };
    c_str.to_str()
}

pub(crate) fn into_str_ptr<T: Into<Vec<u8>>>(string: T) -> *mut c_char {
    let c_string = CString::new(string).unwrap();
    c_string.into_raw()
}
