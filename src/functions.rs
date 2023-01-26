use crate::{
    decoder::{
        get_record_info, get_record_name, get_record_task,
        timestamp::{get_env_cpu_freq, tsc_to_ns},
    },
    ffi::libkshark::{DataStream, Entry, KS_EMPTY_BIN, KS_PLUGIN_UNTOUCHED_MASK},
    str_into_ptr,
};
use libc::{c_char, c_int, c_short, c_void, ssize_t};
use xentrace_parser::{record::DomainKind, Trace};

pub fn get_pid(_stream: *mut DataStream, entry: *mut Entry) -> c_int {
    Entry::from_ptr(entry)
        .filter(|entry| entry.visible & KS_PLUGIN_UNTOUCHED_MASK > 0)
        .map(|entry| entry.pid)
        .unwrap_or(KS_EMPTY_BIN)
}

pub fn get_event_id(stream: *mut DataStream, entry: *mut Entry) -> c_int {
    DataStream::from_ptr(stream)
        .and_then(DataStream::get_interface)
        .and_then(|interface| interface.get_data_handler::<Trace>())
        .zip(Entry::from_ptr(entry))
        .and_then(|(trace, entry)| trace.get(entry.offset as usize))
        .and_then(|record| u32::from(record.event().code()).try_into().ok())
        .unwrap_or(0)
}

pub fn get_event_name(stream: *mut DataStream, entry: *mut Entry) -> *mut c_char {
    DataStream::from_ptr(stream)
        .and_then(DataStream::get_interface)
        .and_then(|interface| interface.get_data_handler::<Trace>())
        .zip(Entry::from_ptr(entry))
        .and_then(|(trace, entry)| trace.get(entry.offset as usize))
        .and_then(|record| {
            let name = get_record_name(record.event());
            str_into_ptr!(name)
        })
        .unwrap() // TODO Handle NULL exception
}

pub fn get_task(stream: *mut DataStream, entry: *mut Entry) -> *mut c_char {
    DataStream::from_ptr(stream)
        .and_then(DataStream::get_interface)
        .and_then(|interface| interface.get_data_handler::<Trace>())
        .zip(Entry::from_ptr(entry))
        .and_then(|(trace, entry)| trace.get(entry.offset as usize))
        .and_then(|record| {
            let task = get_record_task(record.domain());
            str_into_ptr!(task)
        })
        .unwrap() // TODO Handle NULL exception
}

pub fn get_info(stream: *mut DataStream, entry: *mut Entry) -> *mut c_char {
    DataStream::from_ptr(stream)
        .and_then(DataStream::get_interface)
        .and_then(|interface| interface.get_data_handler::<Trace>())
        .zip(Entry::from_ptr(entry))
        .and_then(|(trace, entry)| trace.get(entry.offset as usize))
        .and_then(|record| {
            let info = get_record_info(record.event());
            str_into_ptr!(info)
        })
        .unwrap() // TODO Handle NULL exception
}

//pub fn dump_entry(stream: *mut DataStream, entry: *mut Entry) -> *mut c_char {
//    todo!()
//}

pub fn load_entries(
    stream: *mut DataStream,
    _: *mut c_void,
    rows_ptr: *mut *mut *mut Entry,
) -> ssize_t {
    let stream = {
        let stream_opt = DataStream::from_ptr(stream);
        match stream_opt {
            Some(stream) => stream,
            None => return -1,
        }
    };

    let trace = {
        let trace_opt = stream
            .get_interface()
            .and_then(|interface| interface.get_data_handler::<Trace>());

        match trace_opt {
            Some(trace) => trace,
            None => return -1,
        }
    };

    let rows = {
        let cpu_qhz = get_env_cpu_freq();
        let first_tsc = trace.get(0).map(|record| record.event().tsc());

        let default_domid = i32::from(u16::from(DomainKind::default()));
        let _ = stream.add_task_id(default_domid);

        trace
            .iter()
            .zip(0..)
            .map(|(record, index)| {
                let mut entry = Box::<Entry>::default();

                entry.offset = index;
                entry.stream_id = stream.stream_id;
                entry.cpu = record.cpu().try_into().unwrap_or(c_short::MAX);
                entry.ts = tsc_to_ns(record.event().tsc(), cpu_qhz, first_tsc);
                entry.event_id = u32::from(record.event().code())
                    .try_into()
                    .unwrap_or(c_short::MAX);

                entry.pid = match record.domain().kind() {
                    DomainKind::Idle => 0,
                    DomainKind::Default => default_domid,
                    _ => {
                        let task_id = (u32::from(record.domain()) + 1)
                            .try_into()
                            .unwrap_or(c_int::MAX);
                        let _ = stream.add_task_id(task_id);
                        task_id
                    }
                };

                Box::into_raw(entry)
            })
            .collect::<Vec<*mut Entry>>()
            .into_boxed_slice()
    };

    unsafe {
        *rows_ptr = Box::into_raw(rows) as _;
    }

    trace.record_count().try_into().unwrap_or(ssize_t::MAX)
}
