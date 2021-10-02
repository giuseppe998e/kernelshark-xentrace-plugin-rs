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
use libc::{c_char, c_int};
use std::{alloc::System, convert::TryInto, fs::File, io::Read, path::Path};
use util::{
    pointer::{from_raw_ptr, from_raw_ptr_mut},
    string::{from_str_ptr, into_str_ptr},
};

// Use System allocator (malloc on Linux)
#[global_allocator]
static A: System = System;

const TEST_EVENTS_NUM: usize = 50000;
const TEST_CPUS_NUM: usize = 8;

static KSHARK_SOURCE_TYPE: &str = "xentrace_bin";

fn get_pid(_stream_ptr: *const DataStream, entry_ptr: *const Entry) -> c_int {
    let entry = from_raw_ptr(entry_ptr).unwrap();
    entry.pid
}

fn get_task(_stream_ptr: *const DataStream, _entry_ptr: *const Entry) -> *const c_char {
    into_str_ptr("TASK")
}

fn get_event_name(_stream_ptr: *const DataStream, _entry_ptr: *const Entry) -> *const c_char {
    into_str_ptr("EVENT")
}

fn get_info(_stream_ptr: *const DataStream, _entry_ptr: *const Entry) -> *const c_char {
    into_str_ptr("INFO")
}

fn dump_entry(_stream_ptr: *const DataStream, _entry_ptr: *const Entry) -> *const c_char {
    into_str_ptr("DUMP")
}

fn load_entries(
    stream_ptr: *const DataStream,
    _context_ptr: *const Context,
    rows_ptr: *mut *mut *mut Entry,
) -> isize {
    let stream = from_raw_ptr(stream_ptr).unwrap();
    let mut rows = Vec::with_capacity(TEST_EVENTS_NUM);

    for i in 0..TEST_EVENTS_NUM {
        let mut entry = Entry::new_boxed();

        entry.stream_id = stream.stream_id as _;
        entry.event_id = (i % 5) as _;
        entry.cpu = (i % TEST_CPUS_NUM) as _;
        entry.pid = (10 + i % 2) as _;
        entry.ts = (1000000 + i * 10000) as _;
        entry.offset = i as _;

        rows.push(Box::into_raw(entry));
    }

    unsafe {
        *rows_ptr = Box::into_raw(rows.into_boxed_slice()) as _;
    }

    TEST_EVENTS_NUM.try_into().unwrap()
}

// KSHARK_INPUT_CHECK @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_check(file_ptr: *const c_char, _frmt: *const *const c_char) -> bool {
    let file_str = from_str_ptr(file_ptr).unwrap();
    let file_path = Path::new(file_str);

    let ecode = match File::open(file_path) {
        Ok(mut fp) => {
            let mut buf = [0u8; 4];
            fp.read_exact(&mut buf).unwrap_or_default();
            0x0fffffff & u32::from_ne_bytes(buf)
        }
        Err(_) => 0,
    };

    xentrace_parser::TRC_TRACE_CPU_CHANGE == ecode // XXX Must use interface/xen
}

// KSHARK_INPUT_FORMAT @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_format() -> *const c_char {
    KSHARK_SOURCE_TYPE.as_ptr() as _
}

// KSHARK_INPUT_INITIALIZER @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_initializer(stream_ptr: *mut DataStream) -> c_int {
    let mut interface = GenericStreamInterface::new_boxed();

    interface.get_pid = get_pid as _;
    interface.get_event_name = get_event_name as _;
    interface.get_info = get_info as _;
    interface.get_task = get_task as _;
    interface.dump_entry = dump_entry as _;
    interface.load_entries = load_entries as _;

    let mut stream = from_raw_ptr_mut(stream_ptr).unwrap();
    stream.interface = Box::into_raw(interface);

    stream.n_cpus = TEST_CPUS_NUM.try_into().unwrap();
    stream.n_events = TEST_EVENTS_NUM.try_into().unwrap();
    stream.idle_pid = 0;

    stream.add_task_id(10);
    stream.add_task_id(11);

    0
}

// KSHARK_INPUT_DEINITIALIZER @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_deinitializer(_stream_ptr: *const DataStream) {}
