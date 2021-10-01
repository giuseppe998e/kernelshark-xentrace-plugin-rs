pub(crate) mod context;
pub(crate) mod entry;
pub(crate) mod interface;
pub(crate) mod stream;

use libc::{c_char, c_int, c_long, c_short, c_uint};
use std::ptr::null;

// Required structs from pthread.h
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *const __pthread_internal_list,
    pub __next: *const __pthread_internal_list,
}

impl Default for __pthread_internal_list {
    fn default() -> Self {
        Self {
            __prev: null::<__pthread_internal_list>(),
            __next: null::<__pthread_internal_list>(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: c_int,
    pub __count: c_uint,
    pub __owner: c_int,
    pub __nusers: c_uint,
    pub __kind: c_int,
    pub __spins: c_short,
    pub __elision: c_short,
    pub __list: __pthread_internal_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [c_char; 40usize],
    pub __align: c_long,
}
