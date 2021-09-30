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
mod interface;
mod util;

use interface::kshark::{
    kshark_context, kshark_data_stream, kshark_entry, kshark_generic_stream_interface,
    kshark_hash_id_add,
};
use std::{
    alloc::System,
    fs::File,
    io::Read,
    os::raw::{c_char, c_int},
    path::Path,
    ptr::null,
};
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

fn get_pid(_stream: *mut kshark_data_stream, entry_ptr: *mut kshark_entry) -> c_int {
    let entry = from_raw_ptr(entry_ptr).unwrap();
    entry.pid
}

fn get_task(_stream: *mut kshark_data_stream, _entry: *mut kshark_entry) -> *const c_char {
    into_str_ptr("TASK")
}

fn get_event_name(_stream: *mut kshark_data_stream, _entry: *mut kshark_entry) -> *const c_char {
    into_str_ptr("EVENT")
}

fn get_info(_stream: *mut kshark_data_stream, _entry: *mut kshark_entry) -> *const c_char {
    into_str_ptr("INFO")
}

fn dump_entry(_stream: *mut kshark_data_stream, _entry: *mut kshark_entry) -> *const c_char {
    into_str_ptr("DUMP")
}

fn load_entries(
    stream_ptr: *mut kshark_data_stream,
    _context: *const kshark_context,
    data_rows: *mut *mut *mut kshark_entry,
) -> isize {
    let stream = from_raw_ptr(stream_ptr).unwrap();
    let mut rows = Box::new([null::<kshark_entry>(); TEST_EVENTS_NUM]);

    for i in 0..TEST_EVENTS_NUM {
        let mut entry = Box::new(kshark_entry::default());

        entry.visible = 0xff;
        entry.stream_id = stream.stream_id as i16;
        entry.event_id = (i % 5) as i16;
        entry.cpu = (i % TEST_CPUS_NUM) as i16;
        entry.pid = (10 + i % 2) as i32;
        entry.ts = (1000000 + i * 10000) as i64;
        entry.offset = i as i64;

        rows[i] = Box::into_raw(entry);
    }

    unsafe {
        *data_rows = Box::into_raw(rows) as _;
    }

    TEST_EVENTS_NUM as isize
}

// KSHARK_INPUT_CHECK @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_check(file_ptr: *const c_char, _frmt: *mut *mut c_char) -> bool {
    let file_str = from_str_ptr(file_ptr).unwrap();
    let file_path = Path::new(file_str);

    let hdr = {
        let mut file_buf = File::open(file_path).unwrap();
        let mut buf = [0u8; 4];
        file_buf.read_exact(&mut buf).unwrap();
        u32::from_ne_bytes(buf)
    };

    0x0001f003 == (hdr & 0x0fffffff) // XXX Must use interface/xen
}

// KSHARK_INPUT_FORMAT @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_format() -> *const c_char {
    KSHARK_SOURCE_TYPE.as_ptr() as *const _
}

// KSHARK_INPUT_INITIALIZER @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_initializer(stream_ptr: *mut kshark_data_stream) -> c_int {
    let mut interface = Box::new(kshark_generic_stream_interface::default());

    interface.type_ = 1; // KS_GENERIC_DATA_INTERFACE
    interface.get_pid = get_pid as _;
    interface.get_event_name = get_event_name as _;
    interface.get_info = get_info as _;
    interface.get_task = get_task as _;
    interface.dump_entry = dump_entry as _;
    interface.load_entries = load_entries as _;

    let mut stream = from_raw_ptr_mut(stream_ptr).unwrap();

    stream.interface = Box::into_raw(interface);
    stream.n_cpus = TEST_CPUS_NUM as i32;
    stream.n_events = TEST_EVENTS_NUM as i32;
    stream.idle_pid = 0;

    unsafe {
        kshark_hash_id_add(stream.tasks, 10);
        kshark_hash_id_add(stream.tasks, 11);
    }

    0
}

// KSHARK_INPUT_DEINITIALIZER @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_deinitializer(_stream: *mut kshark_data_stream) {}
