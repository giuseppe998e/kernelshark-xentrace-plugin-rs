use super::stream::DataStream;
use libc::{c_int, c_uchar, c_void};
use std::ptr::null;

/// Structure representing a kshark session.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Context /* kshark_context */ {
    /// Array of data stream descriptors.
    pub stream: *const *const DataStream,
    /// The number of data streams.
    pub n_streams: c_int,
    /// Parameters of the stream descriptor array.
    pub stream_info: StreamArrayDescriptor,
    /// Bit mask, controlling the visibility of the entries after filtering.
    /// If given bit is set here, all entries which are filtered-out will
    /// have this bit unset in their \"visible\" fields.
    pub filter_mask: c_uchar,
    /// List of Data collections.
    pub collections: *const c_void, // XXX NOT IMPL - kshark_entry_collection
    /// List of data readout interfaces.
    pub inputs: *const c_void, // XXX NOT IMPL - kshark_dri_list
    /// The number of readout interfaces.
    pub n_inputs: c_int,
    /// List of Plugins.
    pub plugins: *const c_void, // XXX NOT IMPL - kshark_plugin_list
    /// The number of plugins.
    pub n_plugins: c_int,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            stream: null::<*const DataStream>(),
            n_streams: Default::default(),
            stream_info: Default::default(),
            filter_mask: Default::default(),
            collections: null::<c_void>(),
            inputs: null::<c_void>(),
            n_inputs: Default::default(),
            plugins: null::<c_void>(),
            n_plugins: Default::default(),
        }
    }
}

/// Structure representing the parameters of the stream descriptor
/// array owned by the kshark session.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct StreamArrayDescriptor /* kshark_stream_array_descriptor */ {
    /// The identifier of the Data stream added.
    pub max_stream_id: c_int,
    /// The the next free Data stream identifier (index).
    pub next_free_stream_id: c_int,
    /// The capacity of the array of stream objects (pointers).
    pub array_size: c_int,
}
