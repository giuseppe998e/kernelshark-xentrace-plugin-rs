use super::{interface::GenericStreamInterface, KS_DATA_FORMAT_SIZE};
use crate::from_str_ptr;
use libc::{c_char, c_int, c_long, c_short, c_uint, c_void, size_t};
use std::ptr::null_mut;

extern "C" {
    fn kshark_hash_id_add(
        hash: *mut c_void, // XXX NOT IMPL - kshark_hash_id
        id: c_int,
    ) -> c_int;
}

/// Structure representing a stream of trace data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataStream /* kshark_data_stream */ {
    /// Data stream identifier.
    pub stream_id: c_short,
    /// The number of CPUs presented in this data stream.
    pub n_cpus: c_int,
    /// The number of distinct event types presented in this data stream.
    pub n_events: c_int,
    /// The Process Id of the Idle task.
    pub idle_pid: c_int,
    /// Trace data file pathname.
    pub file: *mut c_char,
    /// Stream name.
    pub name: *mut c_char,
    /// Hash table of task PIDs.
    pub tasks: *mut c_void, // XXX NOT IMPL - kshark_hash_id
    /// A mutex, used to protect the access to the input file.
    pub input_mutex: PthreadMutexU,
    /// Hash of tasks to filter on.
    pub show_task_filter: *mut c_void, // XXX NOT IMPL - kshark_hash_id
    /// Hash of tasks to not display.
    pub hide_task_filter: *mut c_void, // XXX NOT IMPL - kshark_hash_id
    /// Hash of events to filter on.
    pub show_event_filter: *mut c_void, // XXX NOT IMPL - kshark_hash_id
    /// Hash of events to not display.
    pub hide_event_filter: *mut c_void, // XXX NOT IMPL - kshark_hash_id
    /// Hash of CPUs to filter on.
    pub show_cpu_filter: *mut c_void, // XXX NOT IMPL - kshark_hash_id
    /// Hash of CPUs to not display.
    pub hide_cpu_filter: *mut c_void, // XXX NOT IMPL - kshark_hash_id
    /// Flag showing if some entries are filtered out
    /// (marked as invisible).
    pub filter_is_applied: bool,
    /// The type of the data.
    pub data_format: [c_char; KS_DATA_FORMAT_SIZE],
    /// List of Plugin interfaces.
    pub plugins: *mut c_void, // XXX NOT IMPL - kshark_dpi_list
    /// The number of plugins registered for this stream.
    pub n_plugins: c_int,
    /// System clock calibration function.
    pub calib: *mut c_void, // XXX NOT IMPL - time_calib_func
    /// An array of time calibration constants.
    pub calib_array: *mut c_long,
    /// The size of the array of time calibration constants.
    pub calib_array_size: size_t,
    /// List of Plugin's Event handlers.
    pub event_handlers: *mut c_void, // XXX NOT IMPL - kshark_event_proc_handler
    /// List of Plugin's Draw handlers.
    pub draw_handlers: *mut c_void, // XXX NOT IMPL - kshark_draw_handler
    /// The interface of methods used to operate over the data
    /// from a given stream.
    pub interface: *mut GenericStreamInterface,
}

impl DataStream {
    pub fn add_task_id(&self, id: c_int) -> c_int {
        unsafe { kshark_hash_id_add(self.tasks, id) }
    }

    pub fn get_file_path(&self) -> &str {
        from_str_ptr!(self.file).unwrap_or_default()
    }

    pub fn get_interface(&self) -> &GenericStreamInterface {
        unsafe { self.interface.as_ref().unwrap() }
    }

    pub fn get_mut_interface(&self) -> &mut GenericStreamInterface {
        unsafe { self.interface.as_mut().unwrap() }
    }
}

impl Default for DataStream {
    fn default() -> Self {
        Self {
            stream_id: Default::default(),
            n_cpus: Default::default(),
            n_events: Default::default(),
            idle_pid: Default::default(),
            file: null_mut::<c_char>(),
            name: null_mut::<c_char>(),
            tasks: null_mut::<c_void>(),
            input_mutex: PthreadMutexU { align: 0 },
            show_task_filter: null_mut::<c_void>(),
            hide_task_filter: null_mut::<c_void>(),
            show_event_filter: null_mut::<c_void>(),
            hide_event_filter: null_mut::<c_void>(),
            show_cpu_filter: null_mut::<c_void>(),
            hide_cpu_filter: null_mut::<c_void>(),
            filter_is_applied: false,
            data_format: Default::default(),
            plugins: null_mut::<c_void>(),
            n_plugins: Default::default(),
            calib: null_mut::<c_void>(),
            calib_array: null_mut::<c_long>(),
            calib_array_size: Default::default(),
            event_handlers: null_mut::<c_void>(),
            draw_handlers: null_mut::<c_void>(),
            interface: null_mut::<GenericStreamInterface>(),
        }
    }
}

// Required structs from pthread.h
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PthreadInternalList {
    pub prev: *mut PthreadInternalList,
    pub next: *mut PthreadInternalList,
}

impl Default for PthreadInternalList {
    fn default() -> Self {
        Self {
            prev: null_mut::<PthreadInternalList>(),
            next: null_mut::<PthreadInternalList>(),
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
