/***
 * KernelSheak XenTrace plugin
 * Copyright (C) 2021 Giuseppe Eletto <peppe.eletto@gmail.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301
 * USA
 */
mod cbind;
use cbind::kshark::{
    context::Context, entry::Entry, interface::GenericStreamInterface, stream::DataStream,
    KS_EMPTY_BIN, KS_PLUGIN_UNTOUCHED_MASK,
};

mod stringify;
use stringify::{get_record_info_str, get_record_name_str, get_record_task_str};

mod util;
use util::{get_record, tsc_to_ns};

use xentrace_parser::{
    record::{Domain, DomainType},
    xentrace_parse, Trace,
};

use libc::{c_char, c_int, c_short, c_uint, c_void, ssize_t};
use std::{alloc::System, convert::TryInto, fs::File, io::Read, path::Path, ptr::null_mut};

// Use System allocator
//#[global_allocator]
//static A: System = System;

static KSHARK_SOURCE_TYPE: &str = "xentrace_bin";

fn get_pid(_stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> c_int {
    let entry = from_raw_ptr!(entry_ptr);
    match entry {
        Some(e) if e.visible & KS_PLUGIN_UNTOUCHED_MASK > 0 => e.pid,
        _ => KS_EMPTY_BIN,
    }
}

fn get_event_id(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> c_int {
    let record = get_record(stream_ptr, entry_ptr);
    record
        .and_then(|r| {
            let code = r.event.code;
            code.into_u32().try_into().ok()
        })
        .unwrap_or(0)
}

fn get_event_name(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> *mut c_char {
    let record = get_record(stream_ptr, entry_ptr);
    let name_str = match record {
        Some(r) => get_record_name_str(&r.event),
        None => "unknown".to_owned(),
    };

    into_str_ptr!(name_str)
}

fn get_task(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> *mut c_char {
    let entry = from_raw_ptr!(entry_ptr);
    let task_str = match entry {
        Some(e) => {
            let record = {
                let stream = from_raw_ptr!(stream_ptr);
                stream.and_then(|s| {
                    let interface = s.get_interface();
                    let trace = interface.get_data_handler::<Trace>().unwrap();
                    trace.records.get(e.offset as usize)
                })
            };

            match record {
                Some(r) => get_record_task_str(&r.domain),
                _ if e.pid != DomainType::Default.into_id().into() => {
                    let dom = Domain::from_u32((e.pid - 1).try_into().unwrap());
                    format!("d{}/v{}", dom.type_.into_id(), dom.vcpu)
                }
                _ => "default/v?".to_owned(),
            }
        }
        None => "unknown".to_owned(),
    };

    into_str_ptr!(task_str)
}

fn get_info(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> *mut c_char {
    let record = get_record(stream_ptr, entry_ptr);
    let info_str = match record {
        Some(r) => get_record_info_str(&r.event),
        None => "unknown".to_owned(),
    };

    into_str_ptr!(info_str)
}

fn dump_entry(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> *mut c_char {
    let record = get_record(stream_ptr, entry_ptr);
    let (name_str, info_str) = match record {
        Some(r) => (
            get_record_name_str(&r.event),
            get_record_info_str(&r.event),
        ),
        None => ("unknown".to_owned(), "unknown".to_owned()),
    };

    into_str_ptr!(format!(
        "Record {{ Name: \"{}\", Info: \"{}\" }}",
        name_str, info_str
    ))
}

fn load_entries(
    stream_ptr: *mut DataStream,
    _context_ptr: *mut Context,
    rows_ptr: *mut *mut *mut Entry,
) -> ssize_t {
    let stream = from_raw_ptr!(stream_ptr).unwrap();
    let trace = stream.get_interface().get_data_handler::<Trace>().unwrap();

    let rows: Vec<*mut Entry> = {
        let first_tsc = trace.records.get(0).map(|r| r.event.tsc);

        let default_domid = DomainType::Default.into_id().into();
        stream.add_task_id(default_domid);

        trace
            .records
            .iter()
            .zip(0..)
            .map(|(r, i)| {
                let mut entry = Entry::new_boxed();

                entry.offset = i;
                entry.stream_id = stream.stream_id;
                entry.cpu = r.cpu.try_into().unwrap_or(c_short::MAX);
                entry.ts = tsc_to_ns(r.event.tsc, first_tsc, None);
                entry.event_id = r
                    .event
                    .code
                    .into_u32()
                    .try_into()
                    .unwrap_or(c_short::MAX);

                let dom = r.domain;
                entry.pid = match dom.type_ {
                    DomainType::Idle => 0,
                    DomainType::Default => default_domid,
                    _ => {
                        let task_id = (dom.into_u32() + 1).try_into().unwrap_or(c_int::MAX);
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

    trace
        .records
        .len()
        .try_into()
        .unwrap_or(ssize_t::MAX)
}

// KSHARK_INPUT_CHECK @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_check(file_ptr: *mut c_char, _frmt: *mut *mut c_char) -> bool {
    if let Ok(fstr) = from_str_ptr!(file_ptr) {
        if let Ok(mut file) = File::open(Path::new(fstr)) {
            let ecode = {
                let mut buf = [0u8; 4];
                let _ = file.read_exact(&mut buf);
                0x0FFFFFFF & c_uint::from_ne_bytes(buf)
            };

            return xentrace_parser::codes::TRC_TRACE_CPU_CHANGE == ecode; // XXX Must use interface/xen
        }
    }

    false
}

// KSHARK_INPUT_FORMAT @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_format() -> *mut c_char {
    KSHARK_SOURCE_TYPE.as_ptr() as _
}

// KSHARK_INPUT_INITIALIZER @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_initializer(stream_ptr: *mut DataStream) -> c_int {
    let mut stream = from_raw_ptr_mut!(stream_ptr).unwrap();
    let trace = Box::new(xentrace_parse(stream.get_file_path()).unwrap());

    stream.idle_pid = 0;
    stream.n_cpus = trace.cpu_count().into();
    stream.n_events = trace.records.len().try_into().unwrap_or(c_int::MAX);

    stream.interface = {
        let mut interface = GenericStreamInterface::new_boxed();

        interface.get_pid = get_pid as _;
        interface.get_event_id = get_event_id as _;
        interface.get_event_name = get_event_name as _;
        interface.get_task = get_task as _;
        interface.get_info = get_info as _;
        interface.dump_entry = dump_entry as _;
        interface.load_entries = load_entries as _;
        interface.handle = Box::into_raw(trace) as _;

        Box::into_raw(interface)
    };

    0
}

// KSHARK_INPUT_DEINITIALIZER @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_deinitializer(stream_ptr: *mut DataStream) {
    let stream = from_raw_ptr!(stream_ptr).unwrap();
    let interface = stream.get_mut_interface();
    let trace = unsafe { Box::<Trace>::from_raw(interface.handle as _) };

    drop(trace);
    interface.handle = null_mut::<c_void>();
}
