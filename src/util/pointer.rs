use std::os::raw::c_void;

pub fn into_raw_ptr<T>(var: *mut T) -> *mut c_void {
    let var_box = Box::new(var);
    Box::into_raw(var_box) as *mut _
}

pub fn from_raw_ptr<T>() -> Result<*mut T, ()> {
    Err(())
}
