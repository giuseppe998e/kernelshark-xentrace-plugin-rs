use std::ptr::null_mut;

use libc::c_void;

/// Data interface identifier
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum DataInterfaceId /* kshark_data_interface_id */ {
    /// An interface with unknown type.
    InvalidInterface = 0,
    /// Generic interface suitable for Ftrace data.
    GenericDataInterface = 1,
}

/// Structure representing the interface of methods used to
/// operate over the data from a given stream.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GenericStreamInterface /* GenericStreamInterface */ {
    /// Interface version identifier.
    pub interface_id: DataInterfaceId,
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
    pub fn get_data_handler<'a, T>(&'a self) -> Option<&'a T> {
        let handle = self.handle as *mut T;
        unsafe { handle.as_ref::<'a>() }
    }
}

impl Default for GenericStreamInterface {
    fn default() -> Self {
        Self {
            interface_id: DataInterfaceId::GenericDataInterface,
            get_pid: null_mut(),
            get_event_id: null_mut(),
            get_event_name: null_mut(),
            get_task: null_mut(),
            get_info: null_mut(),
            aux_info: null_mut(),
            find_event_id: null_mut(),
            get_all_event_ids: null_mut(),
            dump_entry: null_mut(),
            get_all_event_field_names: null_mut(),
            get_event_field_type: null_mut(),
            read_event_field_int64: null_mut(),
            read_record_field_int64: null_mut(),
            load_entries: null_mut(),
            load_matrix: null_mut(),
            handle: null_mut(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        mem::{align_of, size_of, MaybeUninit},
        ptr::addr_of,
    };

    use super::GenericStreamInterface;

    #[test]
    fn bindgen_layout() {
        const UNINIT: MaybeUninit<GenericStreamInterface> = MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();

        assert_eq!(
            size_of::<GenericStreamInterface>(),
            136usize,
            concat!("Size of: ", stringify!(GenericStreamInterface))
        );
        assert_eq!(
            align_of::<GenericStreamInterface>(),
            8usize,
            concat!("Alignment of ", stringify!(GenericStreamInterface))
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).interface_id) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(interface_id)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).get_pid) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(get_pid)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).get_event_id) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(get_event_id)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).get_event_name) as usize - ptr as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(get_event_name)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).get_task) as usize - ptr as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(get_task)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).get_info) as usize - ptr as usize },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(get_info)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).aux_info) as usize - ptr as usize },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(aux_info)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).find_event_id) as usize - ptr as usize },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(find_event_id)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).get_all_event_ids) as usize - ptr as usize },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(get_all_event_ids)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).dump_entry) as usize - ptr as usize },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(dump_entry)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).get_all_event_field_names) as usize - ptr as usize },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(get_all_event_field_names)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).get_event_field_type) as usize - ptr as usize },
            88usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(get_event_field_type)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).read_event_field_int64) as usize - ptr as usize },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(read_event_field_int64)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).read_record_field_int64) as usize - ptr as usize },
            104usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(read_record_field_int64)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).load_entries) as usize - ptr as usize },
            112usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(load_entries)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).load_matrix) as usize - ptr as usize },
            120usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(load_matrix)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).handle) as usize - ptr as usize },
            128usize,
            concat!(
                "Offset of field: ",
                stringify!(GenericStreamInterface),
                "::",
                stringify!(handle)
            )
        );
    }
}
