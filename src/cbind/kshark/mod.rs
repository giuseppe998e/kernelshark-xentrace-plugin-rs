use libc::{c_int, c_ushort, size_t};

pub(crate) mod entry;
pub(crate) mod interface;
pub(crate) mod stream;

// Constants
pub(crate) const KS_PLUGIN_UNTOUCHED_MASK: c_ushort = 1 << 7;

pub(crate) const KS_EMPTY_BIN: c_int = -1;

pub(crate) const KS_DATA_FORMAT_SIZE: size_t = 15;
