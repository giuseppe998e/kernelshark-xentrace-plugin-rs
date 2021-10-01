use libc::{c_uint, c_void};
use std::ptr::null;

/// Structure representing the interface of methods used to
/// operate over the data from a given stream.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GenericStreamInterface /* kshark_generic_stream_interface */ {
    /// Interface version identifier.
    pub type_: c_uint,
    /// Method used to retrieve the Process Id of the entry.
    pub get_pid: *const c_void,
    /// Method used to retrieve the Event Id of the entry.
    pub get_event_id: *const c_void,
    /// Method used to retrieve the Event name of the entry.
    pub get_event_name: *const c_void,
    /// Method used to retrieve the Task name of the entry.
    pub get_task: *const c_void,
    /// Method used to retrieve the Info string of the entry.
    pub get_info: *const c_void,
    /// Method used to retrieve an unspecified auxiliary info
    /// of the trace record.
    pub aux_info: *const c_void,
    /// Method used to retrieve Id of the Event from its name.
    pub find_event_id: *const c_void,
    /// Method used to retrieve the array of Ids of all Events.
    pub get_all_event_ids: *const c_void,
    /// Method used to dump the entry's content to string.
    pub dump_entry: *const c_void,
    /// Method used to retrieve the array of all field names
    /// of a given event.
    pub get_all_event_field_names: *const c_void,
    /// Method used to access the type of an event's data field.
    pub get_event_field_type: *const c_void,
    /// Method used to access the value of an event's data field.
    pub read_event_field_int64: *const c_void,
    /// Method used to access the value of an event's data field.
    pub read_record_field_int64: *const c_void,
    /// Method used to load the data in the form of entries.
    pub load_entries: *const c_void,
    /// Method used to load the data in matrix form.
    pub load_matrix: *const c_void,
    /// Generic data handle.
    pub handle: *const c_void,
}

impl Default for GenericStreamInterface {
    fn default() -> Self {
        Self {
            type_: Default::default(),
            get_pid: null::<c_void>(),
            get_event_id: null::<c_void>(),
            get_event_name: null::<c_void>(),
            get_task: null::<c_void>(),
            get_info: null::<c_void>(),
            aux_info: null::<c_void>(),
            find_event_id: null::<c_void>(),
            get_all_event_ids: null::<c_void>(),
            dump_entry: null::<c_void>(),
            get_all_event_field_names: null::<c_void>(),
            get_event_field_type: null::<c_void>(),
            read_event_field_int64: null::<c_void>(),
            read_record_field_int64: null::<c_void>(),
            load_entries: null::<c_void>(),
            load_matrix: null::<c_void>(),
            handle: null::<c_void>(),
        }
    }
}
