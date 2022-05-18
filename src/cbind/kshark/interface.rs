use libc::c_void;
use std::ptr::null_mut;

/// Data interface identifier
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum DataInterfaceId /* kshark_data_interface_id */ {
    /// An interface with unknown type.
    InvalidInterface = 0,
    /// Generic interface suitable for Ftrace data.
    GenericDataInterface = 1
}

/// Structure representing the interface of methods used to
/// operate over the data from a given stream.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GenericStreamInterface /* kshark_generic_stream_interface */ {
    /// Interface version identifier.
    pub type_: DataInterfaceId,
    /// Method used to retrieve the Process Id of the entry.
    pub get_pid: *mut c_void,
    /// Method used to retrieve the Event Id of the entry.
    pub get_event_id: *mut c_void,
    /// Method used to retrieve the Event name of the entry.
    pub get_event_name: *mut c_void,
    /// Method used to retrieve the Task name of the entry.
    pub get_task: *mut c_void,
    /// Method used to retrieve the Info string of the entry.
    pub get_info: *mut c_void,
    /// Method used to retrieve an unspecified auxiliary info
    /// of the trace record.
    pub aux_info: *mut c_void,
    /// Method used to retrieve Id of the Event from its name.
    pub find_event_id: *mut c_void,
    /// Method used to retrieve the array of Ids of all Events.
    pub get_all_event_ids: *mut c_void,
    /// Method used to dump the entry's content to string.
    pub dump_entry: *mut c_void,
    /// Method used to retrieve the array of all field names
    /// of a given event.
    pub get_all_event_field_names: *mut c_void,
    /// Method used to access the type of an event's data field.
    pub get_event_field_type: *mut c_void,
    /// Method used to access the value of an event's data field.
    pub read_event_field_int64: *mut c_void,
    /// Method used to access the value of an event's data field.
    pub read_record_field_int64: *mut c_void,
    /// Method used to load the data in the form of entries.
    pub load_entries: *mut c_void,
    /// Method used to load the data in matrix form.
    pub load_matrix: *mut c_void,
    /// Generic data handle.
    pub handle: *mut c_void,
}

impl GenericStreamInterface {
    pub fn new_boxed() -> Box<Self> {
        Box::new(GenericStreamInterface::default())
    }

    pub fn get_data_handler<T>(&self) -> Option<&T> {
        let handle = self.handle as *mut T;
        unsafe { handle.as_ref() }
    }
}

impl Default for GenericStreamInterface {
    fn default() -> Self {
        Self {
            type_: DataInterfaceId::GenericDataInterface,
            get_pid: null_mut::<c_void>(),
            get_event_id: null_mut::<c_void>(),
            get_event_name: null_mut::<c_void>(),
            get_task: null_mut::<c_void>(),
            get_info: null_mut::<c_void>(),
            aux_info: null_mut::<c_void>(),
            find_event_id: null_mut::<c_void>(),
            get_all_event_ids: null_mut::<c_void>(),
            dump_entry: null_mut::<c_void>(),
            get_all_event_field_names: null_mut::<c_void>(),
            get_event_field_type: null_mut::<c_void>(),
            read_event_field_int64: null_mut::<c_void>(),
            read_record_field_int64: null_mut::<c_void>(),
            load_entries: null_mut::<c_void>(),
            load_matrix: null_mut::<c_void>(),
            handle: null_mut::<c_void>(),
        }
    }
}
