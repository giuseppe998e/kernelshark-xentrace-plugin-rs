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
mod stringify;
mod util;

use cbind::kshark::{
    context::Context, entry::Entry, interface::GenericStreamInterface, stream::DataStream,
    KS_EMPTY_BIN, KS_PLUGIN_UNTOUCHED_MASK,
};
use libc::{c_char, c_int, c_void};
use std::{
    alloc::System, collections::HashMap, convert::TryInto, fs::File, io::Read, path::Path,
    ptr::null_mut,
};
use stringify::{get_record_info_str, get_record_name_str, get_record_task_str};
use util::{
    get_record,
    pointer::{from_raw_ptr, from_raw_ptr_mut},
    string::{from_str_ptr, into_str_ptr},
    tsc_to_ns,
};
use xentrace_parser::{DomainType, Parser};

// Use System allocator (malloc on Linux)
#[global_allocator]
static A: System = System;

static KSHARK_SOURCE_TYPE: &str = "xentrace_bin";

fn get_pid(_stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> c_int {
    let entry = from_raw_ptr(entry_ptr);
    match entry {
        Some(e) if e.visible & KS_PLUGIN_UNTOUCHED_MASK > 0 => e.pid,
        _ => KS_EMPTY_BIN,
    }
}

fn get_task(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> *mut c_char {
    let record = get_record(stream_ptr, entry_ptr);
    let task_str = match record {
        Some(r) => get_record_task_str(r),
        None => "unknown".to_owned(),
    };

    into_str_ptr(task_str)
}

fn get_event_name(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> *mut c_char {
    let record = get_record(stream_ptr, entry_ptr);
    let ename_str = match record {
        Some(r) => get_record_name_str(r),
        None => "unknown".to_owned(),
    };

    into_str_ptr(ename_str)
}

fn get_info(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> *mut c_char {
    let record = get_record(stream_ptr, entry_ptr);
    let einfo_str = match record {
        Some(r) => get_record_info_str(r),
        None => "unknown".to_owned(),
    };

    into_str_ptr(einfo_str)
}

fn dump_entry(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> *mut c_char {
    let record = get_record(stream_ptr, entry_ptr);
    let (ename_str, einfo_str) = match record {
        Some(r) => (get_record_name_str(r), get_record_info_str(r)),
        None => ("unknown".to_owned(), "unknown".to_owned()),
    };

    into_str_ptr(format!(
        "Record {{ Name: \"{}\", Info: \"{}\" }}",
        ename_str, einfo_str
    ))
}

fn load_entries(
    stream_ptr: *mut DataStream,
    _context_ptr: *mut Context,
    rows_ptr: *mut *mut *mut Entry,
) -> isize {
    let stream = from_raw_ptr(stream_ptr).unwrap();
    let parser: &Parser = stream.get_interface().get_data_handler().unwrap();

    stream.add_task_id(DomainType::Default.to_id().into()); /* "pidmap" is probably impossible to reach
                                                            this number of entries (dom:vcpu pairs) */

    let rows: Vec<*mut Entry> = {
        let mut pidmap = HashMap::<u32, i32>::new();
        let first_tsc = parser.get_records().get(0).map(|r| r.get_event().get_tsc());

        parser
            .get_records()
            .iter()
            .zip(0..)
            .map(|(r, i)| {
                let mut entry = Entry::new_boxed();

                entry.stream_id = stream.stream_id;
                entry.cpu = r.get_cpu().try_into().unwrap_or(i16::MAX);
                entry.ts = tsc_to_ns(r.get_event().get_tsc(), first_tsc, None);
                entry.event_id = r.get_event().get_code().try_into().unwrap_or(i16::MAX);
                entry.offset = i;

                let dom = r.get_domain();
                entry.pid = match dom.get_type() {
                    DomainType::Idle => 0,
                    DomainType::Default => DomainType::Default.to_id().into(),
                    _ => {
                        let task_id = (pidmap.len() + 1).try_into().unwrap_or(i32::MAX);
                        *pidmap.entry(dom.as_u32()).or_insert_with(|| {
                            stream.add_task_id(task_id);
                            task_id
                        })
                    }
                };

                Box::into_raw(entry)
            })
            .collect()
    };

    unsafe {
        *rows_ptr = Box::into_raw(rows.into_boxed_slice()) as _;
    }

    parser.get_records().len().try_into().unwrap_or(isize::MAX)
}

// KSHARK_INPUT_CHECK @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_check(file_ptr: *mut c_char, _frmt: *mut *mut c_char) -> bool {
    if let Ok(fstr) = from_str_ptr(file_ptr) {
        if let Ok(mut file) = File::open(Path::new(fstr)) {
            let ecode = {
                let mut buf = [0u8; 4];
                let _ = file.read_exact(&mut buf);
                0x0fffffff & u32::from_ne_bytes(buf)
            };

            return xentrace_parser::TRC_TRACE_CPU_CHANGE == ecode; // XXX Must use interface/xen
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
    let mut stream = from_raw_ptr_mut(stream_ptr).unwrap();
    let parser = Box::new(Parser::new(stream.get_file_path()).unwrap());

    stream.idle_pid = 0;
    stream.n_cpus = parser.cpu_count().into();
    stream.n_events = parser.get_records().len().try_into().unwrap_or(i32::MAX);

    stream.interface = {
        let mut interface = GenericStreamInterface::new_boxed();

        interface.get_pid = get_pid as _;
        interface.get_task = get_task as _;
        interface.get_event_name = get_event_name as _;
        interface.get_info = get_info as _;
        interface.dump_entry = dump_entry as _;
        interface.load_entries = load_entries as _;
        interface.handle = Box::into_raw(parser) as _;

        Box::into_raw(interface)
    };

    0
}

// KSHARK_INPUT_DEINITIALIZER @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_deinitializer(stream_ptr: *mut DataStream) {
    let stream = from_raw_ptr(stream_ptr).unwrap();
    let interface = stream.get_mut_interface();
    let parser: Box<Parser> = unsafe { Box::from_raw(interface.handle as _) };

    drop(parser);
    interface.handle = null_mut::<c_void>();
}
