pub(crate) mod context;
pub(crate) mod entry;
pub(crate) mod interface;
pub(crate) mod stream;

use libc::{c_char, c_int, c_long, c_short, c_uint};
use std::ptr::null;

// Required structs from pthread.h
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PthreadInternalList {
    pub prev: *const PthreadInternalList,
    pub next: *const PthreadInternalList,
}

impl Default for PthreadInternalList {
    fn default() -> Self {
        Self {
            prev: null::<PthreadInternalList>(),
            next: null::<PthreadInternalList>(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PthreadMutexS {
    pub lock: c_int,
    pub count: c_uint,
    pub owner: c_int,
    pub nusers: c_uint,
    pub kind: c_int,
    pub spins: c_short,
    pub elision: c_short,
    pub list: PthreadInternalList,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union PthreadMutexU {
    pub data: PthreadMutexS,
    pub size: [c_char; 40usize],
    pub align: c_long,
}
