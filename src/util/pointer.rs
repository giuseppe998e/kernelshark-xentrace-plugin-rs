#[macro_export]
macro_rules! from_raw_ptr {
    ($($ptr:ident),+) => {
        unsafe { ($($ptr.as_ref()),+) }
    };
}

#[macro_export]
macro_rules! from_raw_ptr_mut {
    ($($ptr:ident),+) => {
        unsafe { ($($ptr.as_mut()),+) }
    };
}
