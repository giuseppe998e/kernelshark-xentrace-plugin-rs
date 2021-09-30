use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    str::Utf8Error,
};

pub(crate) fn from_str_ptr<'a>(c_buf: *const c_char) -> Result<&'a str, Utf8Error> {
    let c_str = unsafe { CStr::from_ptr(c_buf) };
    c_str.to_str()
}

pub(crate) fn into_str_ptr<T: Into<Vec<u8>>>(string: T) -> *const c_char {
    let c_string = CString::new(string).unwrap();
    let str_bytes = c_string.into_bytes_with_nul().into_boxed_slice();
    Box::into_raw(str_bytes) as _
}
