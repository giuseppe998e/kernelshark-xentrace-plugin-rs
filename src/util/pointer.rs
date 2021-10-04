pub(crate) fn from_raw_ptr<'a, T>(ptr: *mut T) -> Option<&'a T> {
    unsafe { ptr.as_ref() }
}

pub(crate) fn from_raw_ptr_mut<'a, T>(ptr: *mut T) -> Option<&'a mut T> {
    unsafe { ptr.as_mut() }
}
