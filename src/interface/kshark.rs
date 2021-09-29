use std::os::raw::{c_char, c_int, c_long, c_short, c_uint, c_void};

/* STRUCTs from pthread.h (REQUIRED) */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: c_int,
    pub __count: c_uint,
    pub __owner: c_int,
    pub __nusers: c_uint,
    pub __kind: c_int,
    pub __spins: c_short,
    pub __elision: c_short,
    pub __list: __pthread_internal_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [c_char; 40usize],
    pub __align: c_long,
}

/* STRUCTs from ksharklib.h */
/// Kernel Shark entry contains all information from one trace record needed
/// in order to  visualize the time-series of trace records. The part of the
/// data which is not directly required for the visualization (latency, record
/// info etc.) is available on-demand via the offset into the trace file.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kshark_entry {
    /// Pointer to the next (in time) kshark_entry on the same CPU core.
    pub next: *mut kshark_entry,
    /// A bit mask controlling the visibility of the entry. A value of OxFF
    /// would mean that the entry is visible everywhere. Use
    /// kshark_filter_masks to check the level of visibility/invisibility
    /// of the entry.
    pub visible: u16,
    /// Data stream identifier.
    pub stream_id: i16,
    /// Unique Id of the trace event type.
    pub event_id: i16,
    /// The CPU core of the record.
    pub cpu: i16,
    /// The PID of the task the record was generated.
    pub pid: i32,
    /// The offset into the trace file, used to find the record.
    pub offset: i64,
    /// The time of the record in nano seconds. The value is taken from
    /// the timestamps within the trace data file, which are architecture
    /// dependent. The time usually is the timestamp from when the system
    /// started.
    pub ts: i64,
}

/// Structure representing the interface of methods used to operate over
/// the data from a given stream.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kshark_generic_stream_interface {
    /// Interface version identifier.
    pub type_: c_uint,
    /// Method used to retrieve the Process Id of the entry.
    pub get_pid: *mut (),
    /// Method used to retrieve the Event Id of the entry.
    pub get_event_id: *mut (),
    /// Method used to retrieve the Event name of the entry.
    pub get_event_name: *mut (),
    /// Method used to retrieve the Task name of the entry.
    pub get_task: *mut (),
    /// Method used to retrieve the Info string of the entry.
    pub get_info: *mut (),
    /// Method used to retrieve an unspecified auxiliary info of the trace
    /// record.
    pub aux_info: *mut (),
    /// Method used to retrieve Id of the Event from its name.
    pub find_event_id: *mut (),
    /// Method used to retrieve the array of Ids of all Events.
    pub get_all_event_ids: *mut (),
    /// Method used to dump the entry's content to string.
    pub dump_entry: *mut (),
    /// Method used to retrieve the array of all field names of a given
    /// event.
    pub get_all_event_field_names: *mut (),
    /// Method used to access the type of an event's data field.
    pub get_event_field_type: *mut (),
    /// Method used to access the value of an event's data field.
    pub read_event_field_int64: *mut (),
    /// Method used to access the value of an event's data field.
    pub read_record_field_int64: *mut (),
    /// Method used to load the data in the form of entries.
    pub load_entries: *mut (),
    /// Method used to load the data in matrix form.
    pub load_matrix: *mut (),
    /// Generic data handle.
    pub handle: *mut c_void,
}

/// Structure representing a stream of trace data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kshark_data_stream {
    /// Data stream identifier.
    pub stream_id: u16,
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
    pub input_mutex: pthread_mutex_t,
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
    /// The type of the data.
    pub data_format: [c_char; 15usize],
    /// List of Plugin interfaces.
    pub plugins: *mut c_void, // XXX NOT IMPL - kshark_dpi_list
    /// The number of plugins registered for this stream.
    pub n_plugins: c_int,
    /// System clock calibration function.
    pub calib: *mut (), // XXX NOT IMPL (fn) - time_calib_func
    /// An array of time calibration constants.
    pub calib_array: *mut i64,
    /// The size of the array of time calibration constants.
    pub calib_array_size: usize, // XXX ~ size_t
    /// List of Plugin's Event handlers.
    pub event_handlers: *mut c_void, // XXX NOT IMPL - kshark_event_proc_handler
    /// List of Plugin's Draw handlers.
    pub draw_handlers: *mut c_void, // XXX NOT IMPL - kshark_draw_handler
    /// The interface of methods used to operate over the data from a given
    /// stream.
    pub interface: *mut kshark_generic_stream_interface,
}

/// Structure representing the parameters of the stream descriptor array owned
/// by the kshark session.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kshark_stream_array_descriptor {
    /// The identifier of the Data stream added.
    pub max_stream_id: c_int,
    /// The the next free Data stream identifier (index).
    pub next_free_stream_id: c_int,
    /// The capacity of the array of stream objects (pointers).
    pub array_size: c_int,
}

/// Structure representing a kshark session.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kshark_context {
    /// Array of data stream descriptors.
    pub stream: *mut *mut kshark_data_stream,
    /// The number of data streams.
    pub n_streams: c_int,
    /// Parameters of the stream descriptor array.
    pub stream_info: kshark_stream_array_descriptor,
    /// Bit mask, controlling the visibility of the entries after filtering.
    /// If given bit is set here, all entries which are filtered-out will
    /// have this bit unset in their \"visible\" fields.
    pub filter_mask: u8,
    /// List of Data collections.
    pub collections: *mut c_void, // XXX NOT IMPL - kshark_entry_collection
    /// List of data readout interfaces.
    pub inputs: *mut c_void, // XXX NOT IMPL - kshark_dri_list
    /// The number of readout interfaces.
    pub n_inputs: c_int,
    /// List of Plugins.
    pub plugins: *mut c_void, // XXX NOT IMPL - kshark_plugin_list
    /// The number of plugins.
    pub n_plugins: c_int,
}

/* FUNCTIONs from ksharklib.h */
extern "C" {
    pub fn kshark_hash_id_add(
        hash: *mut c_void, /* XXX NOT IMPL - kshark_hash_id */
        id: c_int,
    ) -> c_int;
}
