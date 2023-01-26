use super::GenericStreamInterface;
use crate::{ffi::libkshark_plugin::KS_DATA_FORMAT_SIZE, str_from_ptr};
use libc::{c_char, c_int, c_long, c_short, c_void, size_t};
use std::ptr::null_mut;

const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;

extern "C" {
    fn kshark_hash_id_add(
        hash: *mut c_void, // kshark_hash_id - NOT IMPL!
        id: c_int,
    ) -> c_int;
}

/// Structure representing a stream of trace data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataStream /* DataStream */ {
    /// Data stream identifier.
    pub stream_id: c_short,
    /// The number of CPUs presented in this data stream.
    pub n_cpus: c_int,
    /// Hash table of Idle CPUs.
    pub idle_cpus: *mut c_void, // kshark_hash_id - NOT IMPL!
    /// The number of distinct event types presented in this data stream.
    pub n_events: c_int,
    /// The Process Id of the Idle task.
    pub idle_pid: c_int,
    /// Trace data file pathname.
    pub file: *mut c_char,
    /// Stream name.
    pub name: *mut c_char,
    /// Hash table of task PIDs.
    pub tasks: *mut c_void, // kshark_hash_id - NOT IMPL!
    /// A mutex, used to protect the access to the input file.
    pub _input_mutex_padding: [c_char; __SIZEOF_PTHREAD_MUTEX_T], // pthread_mutex_t - NOT IMPL!
    /// Hash of tasks to filter on.
    pub show_task_filter: *mut c_void, // kshark_hash_id - NOT IMPL!
    /// Hash of tasks to not display.
    pub hide_task_filter: *mut c_void, // kshark_hash_id - NOT IMPL!
    /// Hash of events to filter on.
    pub show_event_filter: *mut c_void, // kshark_hash_id - NOT IMPL!
    /// Hash of events to not display.
    pub hide_event_filter: *mut c_void, // kshark_hash_id - NOT IMPL!
    /// Hash of CPUs to filter on.
    pub show_cpu_filter: *mut c_void, // kshark_hash_id - NOT IMPL!
    /// Hash of CPUs to not display.
    pub hide_cpu_filter: *mut c_void, // kshark_hash_id - NOT IMPL!
    /// Flag showing if some entries are filtered out
    /// (marked as invisible).
    pub filter_is_applied: bool,
    /// The type of the data.
    pub data_format: [c_char; KS_DATA_FORMAT_SIZE],
    /// List of Plugin interfaces.
    pub plugins: *mut c_void, // kshark_dpi_list - NOT IMPL!
    /// The number of plugins registered for this stream.
    pub n_plugins: c_int,
    /// System clock calibration function.
    pub calib: *mut c_void, // time_calib_func - NOT IMPL!
    /// An array of time calibration constants.
    pub calib_array: *mut c_long,
    /// The size of the array of time calibration constants.
    pub calib_array_size: size_t,
    /// List of Plugin's Event handlers.
    pub event_handlers: *mut c_void, // kshark_event_proc_handler - NOT IMPL!
    /// List of Plugin's Draw handlers.
    pub draw_handlers: *mut c_void, // kshark_draw_handler - NOT IMPL!
    /// The interface of methods used to operate over the data
    /// from a given stream.
    pub interface: *mut GenericStreamInterface,
}

impl DataStream {
    #[inline]
    pub fn from_ptr<'a>(ptr: *mut Self) -> Option<&'a Self> {
        unsafe { ptr.as_ref::<'a>() }
    }

    #[inline]
    pub fn from_ptr_mut<'a>(ptr: *mut Self) -> Option<&'a mut Self> {
        unsafe { ptr.as_mut::<'a>() }
    }

    pub fn add_task_id(&self, id: c_int) -> Result<(), c_int> {
        let result = unsafe { kshark_hash_id_add(self.tasks, id) };
        match result {
            0.. => Ok(()),
            n => Err(n),
        }
    }

    #[inline]
    pub fn get_file_path(&self) -> Option<&str> {
        str_from_ptr!(self.file)
    }

    #[inline]
    pub fn get_interface<'a>(&'a self) -> Option<&'a GenericStreamInterface> {
        unsafe { self.interface.as_ref::<'a>() }
    }

    #[inline]
    pub fn get_interface_mut<'a>(&'a mut self) -> Option<&'a mut GenericStreamInterface> {
        unsafe { self.interface.as_mut::<'a>() }
    }
}

impl Default for DataStream {
    fn default() -> Self {
        Self {
            stream_id: Default::default(),
            n_cpus: Default::default(),
            idle_cpus: null_mut(),
            n_events: Default::default(),
            idle_pid: Default::default(),
            file: null_mut(),
            name: null_mut(),
            tasks: null_mut(),
            _input_mutex_padding: [0; __SIZEOF_PTHREAD_MUTEX_T],
            show_task_filter: null_mut(),
            hide_task_filter: null_mut(),
            show_event_filter: null_mut(),
            hide_event_filter: null_mut(),
            show_cpu_filter: null_mut(),
            hide_cpu_filter: null_mut(),
            filter_is_applied: false,
            data_format: Default::default(),
            plugins: null_mut(),
            n_plugins: Default::default(),
            calib: null_mut(),
            calib_array: null_mut(),
            calib_array_size: Default::default(),
            event_handlers: null_mut(),
            draw_handlers: null_mut(),
            interface: null_mut(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::DataStream;
    use std::{
        mem::{align_of, size_of, MaybeUninit},
        ptr::addr_of,
    };

    #[test]
    fn bindgen_layout() {
        const UNINIT: MaybeUninit<DataStream> = MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();

        assert_eq!(
            size_of::<DataStream>(),
            216usize,
            concat!("Size of: ", stringify!(DataStream))
        );
        assert_eq!(
            align_of::<DataStream>(),
            8usize,
            concat!("Alignment of ", stringify!(DataStream))
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).stream_id) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(stream_id)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).n_cpus) as usize - ptr as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(n_cpus)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).idle_cpus) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(idle_cpus)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).n_events) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(n_events)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).idle_pid) as usize - ptr as usize },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(idle_pid)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).file) as usize - ptr as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(file)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).name) as usize - ptr as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(name)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).tasks) as usize - ptr as usize },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(tasks)
            )
        );
        //assert_eq!(
        //    unsafe { addr_of!((*ptr)._input_mutex_padding) as usize - ptr as usize },
        //    48usize,
        //    concat!(
        //        "Offset of field: ",
        //        stringify!(DataStream),
        //        "::",
        //        stringify!(input_mutex)
        //    )
        //);
        assert_eq!(
            unsafe { addr_of!((*ptr).show_task_filter) as usize - ptr as usize },
            88usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(show_task_filter)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).hide_task_filter) as usize - ptr as usize },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(hide_task_filter)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).show_event_filter) as usize - ptr as usize },
            104usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(show_event_filter)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).hide_event_filter) as usize - ptr as usize },
            112usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(hide_event_filter)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).show_cpu_filter) as usize - ptr as usize },
            120usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(show_cpu_filter)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).hide_cpu_filter) as usize - ptr as usize },
            128usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(hide_cpu_filter)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).filter_is_applied) as usize - ptr as usize },
            136usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(filter_is_applied)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).data_format) as usize - ptr as usize },
            137usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(data_format)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).plugins) as usize - ptr as usize },
            152usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(plugins)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).n_plugins) as usize - ptr as usize },
            160usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(n_plugins)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).calib) as usize - ptr as usize },
            168usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(calib)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).calib_array) as usize - ptr as usize },
            176usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(calib_array)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).calib_array_size) as usize - ptr as usize },
            184usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(calib_array_size)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).event_handlers) as usize - ptr as usize },
            192usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(event_handlers)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).draw_handlers) as usize - ptr as usize },
            200usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(draw_handlers)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).interface) as usize - ptr as usize },
            208usize,
            concat!(
                "Offset of field: ",
                stringify!(DataStream),
                "::",
                stringify!(interface)
            )
        );
    }
}
