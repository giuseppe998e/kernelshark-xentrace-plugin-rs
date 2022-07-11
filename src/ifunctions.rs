use crate::{
    cbind::kshark::{entry::Entry, stream::DataStream, KS_EMPTY_BIN, KS_PLUGIN_UNTOUCHED_MASK},
    from_raw_ptr, into_str_ptr,
    stringify::{get_record_info_str, get_record_name_str, get_record_task_str},
    util::{get_record, tsc_to_ns},
};

use libc::{c_char, c_int, c_short, c_void, ssize_t};
use xentrace_parser::{record::DomainKind, Trace};

pub(crate) fn get_pid(_stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> c_int {
    let entry = from_raw_ptr!(entry_ptr);
    match entry {
        Some(e) if e.visible & KS_PLUGIN_UNTOUCHED_MASK > 0 => e.pid,
        _ => KS_EMPTY_BIN,
    }
}

pub(crate) fn get_event_id(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> c_int {
    let record = get_record(stream_ptr, entry_ptr);
    record
        .and_then(|r| u32::from(r.event().code()).try_into().ok())
        .unwrap_or(0)
}

pub(crate) fn get_event_name(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> *mut c_char {
    let record = get_record(stream_ptr, entry_ptr);
    let name_str = record
        .map(|r| get_record_name_str(r.event()))
        .unwrap_or_default();

    into_str_ptr!(name_str)
}

pub(crate) fn get_task(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> *mut c_char {
    let record = get_record(stream_ptr, entry_ptr);
    let task_str = record
        .map(|r| get_record_task_str(r.domain()))
        .unwrap_or_default();

    into_str_ptr!(task_str)
}

pub(crate) fn get_info(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> *mut c_char {
    let record = get_record(stream_ptr, entry_ptr);
    let info_str = record
        .map(|r| get_record_info_str(r.event()))
        .unwrap_or_default();

    into_str_ptr!(info_str)
}

pub(crate) fn dump_entry(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> *mut c_char {
    let record = get_record(stream_ptr, entry_ptr);
    let dump_str = record
        .map(|r| {
            (
                get_record_name_str(r.event()),
                get_record_task_str(r.domain()),
                get_record_info_str(r.event()),
            )
        })
        .map(|(name_str, task_str, info_str)| {
            format!(
                "Record {{ Name: \"{}\", Task: \"{}\", Info: \"{}\" }}",
                name_str, task_str, info_str
            )
        })
        .unwrap_or_default();

    into_str_ptr!(dump_str)
}

pub(crate) fn load_entries(
    stream_ptr: *mut DataStream,
    _context_ptr: *mut c_void, // not implemented
    rows_ptr: *mut *mut *mut Entry,
) -> ssize_t {
    let stream = from_raw_ptr!(stream_ptr).unwrap();
    let trace = stream.get_interface().get_data_handler::<Trace>().unwrap();

    let rows: Vec<*mut Entry> = {
        let first_tsc = trace.get(0).map(|r| r.event().tsc());

        let default_domid = DomainKind::Default.into();
        stream.add_task_id(default_domid);

        trace
            .iter()
            .zip(0..)
            .map(|(r, i)| {
                let mut entry = Entry::boxed();

                entry.offset = i;
                entry.stream_id = stream.stream_id;
                entry.cpu = r.cpu().try_into().unwrap_or(c_short::MAX);
                entry.ts = tsc_to_ns(r.event().tsc(), first_tsc, None);
                entry.event_id = u32::from(r.event().code()).try_into().unwrap_or(c_short::MAX);

                entry.pid = match r.domain().kind() {
                    DomainKind::Idle => 0,
                    DomainKind::Default => default_domid,
                    _ => {
                        let task_id = (u32::from(*r.domain()) + 1).try_into().unwrap_or(c_int::MAX);
                        stream.add_task_id(task_id);
                        task_id
                    }
                };

                Box::into_raw(entry)
            })
            .collect()
    };

    unsafe {
        *rows_ptr = Box::into_raw(rows.into_boxed_slice()) as _;
    }

    trace.record_count().try_into().unwrap_or(ssize_t::MAX)
}
