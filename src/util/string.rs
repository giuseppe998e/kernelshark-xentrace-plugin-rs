#[macro_export]
macro_rules! from_str_ptr {
    ($c_buf:expr) => {{
        let c_str = unsafe { std::ffi::CStr::from_ptr($c_buf) };
        c_str.to_str()
    }};
}

#[macro_export]
macro_rules! into_str_ptr {
    ($string:expr) => {{
        let c_string = std::ffi::CString::new($string).unwrap();
        c_string.into_raw()
    }};
}
