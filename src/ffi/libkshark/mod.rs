// FFI derived from https://github.com/yordan-karadzhov/kernel-shark/blob/b5ad5393ebf014db40a565e2e64b9dcd88246d8b/src/libkshark.h
mod data_stream;
mod entry;
mod generic_stream_interface;

// Structs & Enums
#[allow(unused_imports)]
pub use self::{
    data_stream::DataStream,
    entry::Entry,
    generic_stream_interface::{DataInterfaceId, GenericStreamInterface},
};

// Constants
pub const KS_EMPTY_BIN: libc::c_int = -1;
pub const KS_PLUGIN_UNTOUCHED_MASK: libc::c_ushort = 1 << 7;
