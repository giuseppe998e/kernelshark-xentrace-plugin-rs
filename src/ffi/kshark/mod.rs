pub(crate) mod context;
pub(crate) mod entry;
pub(crate) mod interface;
pub(crate) mod stream;

use libc::{c_int, c_ushort};

// Constants
pub(crate) const KS_PLUGIN_UNTOUCHED_MASK: c_ushort = 1 << 7;

pub(crate) const KS_EMPTY_BIN: c_int = -1;