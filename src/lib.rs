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
mod ffi;
mod util;

use ffi::kshark::{
    context::Context, entry::Entry, interface::GenericStreamInterface, stream::DataStream,
};
use libc::{c_char, c_int, c_void};
use std::{
    alloc::{dealloc, Layout, System},
    convert::TryInto,
    fs::File,
    io::Read,
    path::Path,
    ptr::null_mut,
};
use util::{
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
    let entry = from_raw_ptr(entry_ptr).unwrap();
    entry.pid
}

fn get_task(_stream_ptr: *mut DataStream, _entry_ptr: *mut Entry) -> *mut c_char {
    into_str_ptr("TASK")
}

fn get_event_name(_stream_ptr: *mut DataStream, _entry_ptr: *mut Entry) -> *mut c_char {
    into_str_ptr("EVENT")
}

fn get_info(_stream_ptr: *mut DataStream, _entry_ptr: *mut Entry) -> *mut c_char {
    into_str_ptr("INFO")
}

fn dump_entry(_stream_ptr: *mut DataStream, _entry_ptr: *mut Entry) -> *mut c_char {
    into_str_ptr("DUMP")
}

fn load_entries(
    stream_ptr: *mut DataStream,
    _context_ptr: *mut Context,
    rows_ptr: *mut *mut *mut Entry,
) -> isize {
    let stream = from_raw_ptr(stream_ptr).unwrap();
    let xtparser: &Parser = stream.get_interface().get_data_handler().unwrap();

    let first_tsc = xtparser
        .get_records()
        .get(0)
        .map(|r| r.get_event().get_tsc());

    let mut offset = 0;
    let rows: Vec<*mut Entry> = xtparser
        .get_records()
        .iter()
        .map(|r| {
            let mut entry = Entry::new_boxed();

            entry.stream_id = stream.stream_id.try_into().unwrap();
            entry.cpu = r.get_cpu().try_into().unwrap();
            entry.ts = tsc_to_ns(r.get_event().get_tsc(), first_tsc, None);
            entry.event_id = (r.get_event().get_code() % (i16::MAX as u32))
                .try_into()
                .unwrap();

            let domtype = r.get_domain().get_type();
            if domtype != DomainType::Idle {
                let task_id = if domtype == DomainType::Default {
                    DomainType::Default.to_id()
                } else {
                    domtype.to_id() + 1
                }
                .into();

                stream.add_task_id(task_id);
                entry.pid = task_id;
            }

            entry.offset = offset;
            offset += 1;

            Box::into_raw(entry)
        })
        .collect();

    unsafe {
        *rows_ptr = Box::into_raw(rows.into_boxed_slice()) as _;
    }

    xtparser.get_records().len().try_into().unwrap()
}

// KSHARK_INPUT_CHECK @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_check(file_ptr: *mut c_char, _frmt: *mut *mut c_char) -> bool {
    let file_str = from_str_ptr(file_ptr).unwrap();
    let file_path = Path::new(file_str);

    if let Ok(mut fp) = File::open(file_path) {
        let ecode = {
            let mut buf = [0u8; 4];
            fp.read_exact(&mut buf).unwrap_or_default();
            0x0fffffff & u32::from_ne_bytes(buf)
        };

        return xentrace_parser::TRC_TRACE_CPU_CHANGE == ecode; // XXX Must use interface/xen
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
    let xtparser = Box::new(Parser::new(stream.get_file_path()).unwrap());

    stream.idle_pid = 0;
    stream.n_cpus = xtparser.cpu_count().try_into().unwrap();
    stream.n_events = xtparser.get_records().len().try_into().unwrap();

    stream.interface = {
        let mut interface = GenericStreamInterface::new_boxed();

        interface.get_pid = get_pid as _;
        interface.get_event_name = get_event_name as _;
        interface.get_info = get_info as _;
        interface.get_task = get_task as _;
        interface.dump_entry = dump_entry as _;
        interface.load_entries = load_entries as _;
        interface.handle = Box::into_raw(xtparser) as _;

        Box::into_raw(interface)
    };

    0
}

// KSHARK_INPUT_DEINITIALIZER @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_deinitializer(stream_ptr: *mut DataStream) {
    let stream = from_raw_ptr(stream_ptr).unwrap();
    let interface = stream.get_mut_interface();
    let xtparser: Box<Parser> = unsafe { Box::from_raw(interface.handle as _) };

    drop(xtparser);
    interface.handle = null_mut::<c_void>();
}
