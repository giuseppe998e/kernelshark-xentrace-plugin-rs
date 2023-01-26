// CString macros
mod cstring {
    #[macro_export]
    macro_rules! str_from_ptr {
        ($ptr:expr) => {
            match $ptr.is_null() {
                true => None,
                false => {
                    let cstr = unsafe { std::ffi::CStr::from_ptr($ptr) };
                    cstr.to_str().ok()
                }
            }
        };
    }

    #[macro_export]
    macro_rules! str_into_ptr {
        ($str:expr) => {{
            use std::ffi::CString;
            CString::new($str).map(CString::into_raw).ok()
        }};
    }
}
