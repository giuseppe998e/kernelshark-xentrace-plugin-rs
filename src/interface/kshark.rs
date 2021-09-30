use std::{
    os::raw::{c_char, c_int, c_long, c_short, c_uint, c_void},
    ptr::null,
};

/* STRUCTs from pthread.h (REQUIRED) */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *const __pthread_internal_list,
    pub __next: *const __pthread_internal_list,
}

impl Default for __pthread_internal_list {
    fn default() -> Self {
        Self {
            __prev: null::<__pthread_internal_list>(),
            __next: null::<__pthread_internal_list>(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
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
    pub next: *const kshark_entry,
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

impl Default for kshark_entry {
    fn default() -> Self {
        Self {
            next: null::<kshark_entry>(),
            visible: Default::default(),
            stream_id: Default::default(),
            event_id: Default::default(),
            cpu: Default::default(),
            pid: Default::default(),
            offset: Default::default(),
            ts: Default::default(),
        }
    }
}

/// Structure representing the interface of methods used to operate over
/// the data from a given stream.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kshark_generic_stream_interface {
    /// Interface version identifier.
    pub type_: c_uint,
    /// Method used to retrieve the Process Id of the entry.
    pub get_pid: *const (),
    /// Method used to retrieve the Event Id of the entry.
    pub get_event_id: *const (),
    /// Method used to retrieve the Event name of the entry.
    pub get_event_name: *const (),
    /// Method used to retrieve the Task name of the entry.
    pub get_task: *const (),
    /// Method used to retrieve the Info string of the entry.
    pub get_info: *const (),
    /// Method used to retrieve an unspecified auxiliary info of the trace
    /// record.
    pub aux_info: *const (),
    /// Method used to retrieve Id of the Event from its name.
    pub find_event_id: *const (),
    /// Method used to retrieve the array of Ids of all Events.
    pub get_all_event_ids: *const (),
    /// Method used to dump the entry's content to string.
    pub dump_entry: *const (),
    /// Method used to retrieve the array of all field names of a given
    /// event.
    pub get_all_event_field_names: *const (),
    /// Method used to access the type of an event's data field.
    pub get_event_field_type: *const (),
    /// Method used to access the value of an event's data field.
    pub read_event_field_int64: *const (),
    /// Method used to access the value of an event's data field.
    pub read_record_field_int64: *const (),
    /// Method used to load the data in the form of entries.
    pub load_entries: *const (),
    /// Method used to load the data in matrix form.
    pub load_matrix: *const (),
    /// Generic data handle.
    pub handle: *const c_void,
}

impl Default for kshark_generic_stream_interface {
    fn default() -> Self {
        Self {
            type_: Default::default(),
            get_pid: null::<()>(),
            get_event_id: null::<()>(),
            get_event_name: null::<()>(),
            get_task: null::<()>(),
            get_info: null::<()>(),
            aux_info: null::<()>(),
            find_event_id: null::<()>(),
            get_all_event_ids: null::<()>(),
            dump_entry: null::<()>(),
            get_all_event_field_names: null::<()>(),
            get_event_field_type: null::<()>(),
            read_event_field_int64: null::<()>(),
            read_record_field_int64: null::<()>(),
            load_entries: null::<()>(),
            load_matrix: null::<()>(),
            handle: null::<c_void>(),
        }
    }
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
    pub file: *const c_char,
    /// Stream name.
    pub name: *const c_char,
    /// Hash table of task PIDs.
    pub tasks: *const c_void, // XXX NOT IMPL - kshark_hash_id
    /// A mutex, used to protect the access to the input file.
    pub input_mutex: pthread_mutex_t,
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
    pub calib: *const (), // XXX NOT IMPL (fn) - time_calib_func
    /// An array of time calibration constants.
    pub calib_array: *const i64,
    /// The size of the array of time calibration constants.
    pub calib_array_size: usize, // XXX ~ size_t
    /// List of Plugin's Event handlers.
    pub event_handlers: *const c_void, // XXX NOT IMPL - kshark_event_proc_handler
    /// List of Plugin's Draw handlers.
    pub draw_handlers: *const c_void, // XXX NOT IMPL - kshark_draw_handler
    /// The interface of methods used to operate over the data from a given
    /// stream.
    pub interface: *const kshark_generic_stream_interface,
}

impl Default for kshark_data_stream {
    fn default() -> Self {
        Self {
            stream_id: Default::default(),
            n_cpus: Default::default(),
            n_events: Default::default(),
            idle_pid: Default::default(),
            file: null::<i8>(),
            name: null::<i8>(),
            tasks: null::<c_void>(),
            input_mutex: pthread_mutex_t { __align: 0 },
            show_task_filter: null::<c_void>(),
            hide_task_filter: null::<c_void>(),
            show_event_filter: null::<c_void>(),
            hide_event_filter: null::<c_void>(),
            show_cpu_filter: null::<c_void>(),
            hide_cpu_filter: null::<c_void>(),
            data_format: Default::default(),
            plugins: null::<c_void>(),
            n_plugins: Default::default(),
            calib: null::<()>(),
            calib_array: null::<i64>(),
            calib_array_size: Default::default(),
            event_handlers: null::<c_void>(),
            draw_handlers: null::<c_void>(),
            interface: null::<kshark_generic_stream_interface>(),
        }
    }
}

/// Structure representing the parameters of the stream descriptor array owned
/// by the kshark session.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
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
    pub stream: *const *const kshark_data_stream,
    /// The number of data streams.
    pub n_streams: c_int,
    /// Parameters of the stream descriptor array.
    pub stream_info: kshark_stream_array_descriptor,
    /// Bit mask, controlling the visibility of the entries after filtering.
    /// If given bit is set here, all entries which are filtered-out will
    /// have this bit unset in their \"visible\" fields.
    pub filter_mask: u8,
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

impl Default for kshark_context {
    fn default() -> Self {
        Self {
            stream: null::<*const kshark_data_stream>(),
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

/* FUNCTIONs from ksharklib.h */
extern "C" {
    pub fn kshark_hash_id_add(
        hash: *const c_void, /* XXX NOT IMPL - kshark_hash_id */
        id: c_int,
    ) -> c_int;
}
