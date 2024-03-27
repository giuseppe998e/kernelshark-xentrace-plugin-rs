// CString macros
mod cstring {
    #[macro_export]
    macro_rules! str_from_ptr {
        ($ptr:expr) => {
            if !$ptr.is_null() {
                let cstr = unsafe { std::ffi::CStr::from_ptr($ptr) };
                cstr.to_str().ok()
            } else {
                None
            }
        };
    }

    #[macro_export]
    macro_rules! str_into_raw {
        ($str:expr) => {
            std::ffi::CString::new($str)
                .map(std::ffi::CString::into_raw)
                .ok()
        };
    }
}
