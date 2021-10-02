use super::{interface::GenericStreamInterface, PthreadMutexU};
use crate::util::string::from_str_ptr;
use libc::{c_char, c_int, c_long, c_ushort, c_void, size_t};
use std::ptr::null;

extern "C" {
    fn kshark_hash_id_add(
        hash: *const c_void, /* XXX NOT IMPL - kshark_hash_id */
        id: c_int,
    ) -> c_int;
}

/// Structure representing a stream of trace data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataStream /* kshark_data_stream */ {
    /// Data stream identifier.
    pub stream_id: c_ushort,
    /// The number of CPUs presented in this data stream.
    pub n_cpus: c_int,
    /// The number of distinct event types presented in this data stream.
    pub n_events: c_int,
    /// The Process Id of the Idle task.
    pub idle_pid: c_int,
    /// Trace data file pathname.
    pub file: *const c_char,
    /// Stream name.
    pub name: *const c_char,
    /// Hash table of task PIDs.
    pub tasks: *const c_void, // XXX NOT IMPL - kshark_hash_id
    /// A mutex, used to protect the access to the input file.
    pub input_mutex: PthreadMutexU,
    /// Hash of tasks to filter on.
    pub show_task_filter: *const c_void, // XXX NOT IMPL - kshark_hash_id
    /// Hash of tasks to not display.
    pub hide_task_filter: *const c_void, // XXX NOT IMPL - kshark_hash_id
    /// Hash of events to filter on.
    pub show_event_filter: *const c_void, // XXX NOT IMPL - kshark_hash_id
    /// Hash of events to not display.
    pub hide_event_filter: *const c_void, // XXX NOT IMPL - kshark_hash_id
    /// Hash of CPUs to filter on.
    pub show_cpu_filter: *const c_void, // XXX NOT IMPL - kshark_hash_id
    /// Hash of CPUs to not display.
    pub hide_cpu_filter: *const c_void, // XXX NOT IMPL - kshark_hash_id
    /// The type of the data.
    pub data_format: [c_char; 15usize],
    /// List of Plugin interfaces.
    pub plugins: *const c_void, // XXX NOT IMPL - kshark_dpi_list
    /// The number of plugins registered for this stream.
    pub n_plugins: c_int,
    /// System clock calibration function.
    pub calib: *const c_void, // XXX NOT IMPL - time_calib_func
    /// An array of time calibration constants.
    pub calib_array: *const c_long,
    /// The size of the array of time calibration constants.
    pub calib_array_size: size_t,
    /// List of Plugin's Event handlers.
    pub event_handlers: *const c_void, // XXX NOT IMPL - kshark_event_proc_handler
    /// List of Plugin's Draw handlers.
    pub draw_handlers: *const c_void, // XXX NOT IMPL - kshark_draw_handler
    /// The interface of methods used to operate over the data
    /// from a given stream.
    pub interface: *const GenericStreamInterface,
}

impl DataStream {
    pub fn add_task_id(&self, id: c_int) -> c_int {
        unsafe { kshark_hash_id_add(self.tasks, id) }
    }

    pub fn get_file_path(&self) -> &str {
        from_str_ptr(self.file).unwrap_or_default()
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
            file: null::<c_char>(),
            name: null::<c_char>(),
            tasks: null::<c_void>(),
            input_mutex: PthreadMutexU { align: 0 },
            show_task_filter: null::<c_void>(),
            hide_task_filter: null::<c_void>(),
            show_event_filter: null::<c_void>(),
            hide_event_filter: null::<c_void>(),
            show_cpu_filter: null::<c_void>(),
            hide_cpu_filter: null::<c_void>(),
            data_format: Default::default(),
            plugins: null::<c_void>(),
            n_plugins: Default::default(),
            calib: null::<c_void>(),
            calib_array: null::<c_long>(),
            calib_array_size: Default::default(),
            event_handlers: null::<c_void>(),
            draw_handlers: null::<c_void>(),
            interface: null::<GenericStreamInterface>(),
        }
    }
}
