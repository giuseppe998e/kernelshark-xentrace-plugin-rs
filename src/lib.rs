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
    KS_EMPTY_BIN, KS_PLUGIN_UNTOUCHED_MASK,
};
use libc::{c_char, c_int, c_void};
use std::{alloc::System, convert::TryInto, fs::File, io::Read, path::Path, ptr::null_mut};
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
    let entry = from_raw_ptr(entry_ptr).unwrap();
    if entry.visible & KS_PLUGIN_UNTOUCHED_MASK > 0 {
        entry.pid
    } else {
        KS_EMPTY_BIN
    }
}

fn get_task(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> *mut c_char {
    let record = get_record(stream_ptr, entry_ptr).unwrap();

    let dom: String = match record.get_domain().get_type() {
        DomainType::Idle => "idle".to_owned(),
        DomainType::Default => "default".to_owned(),
        not_idle_or_def => format!("d{}", not_idle_or_def.to_id()),
    };

    into_str_ptr(format!("{}/v{}", dom, record.get_domain().get_vcpu()))
}

fn get_event_name(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> *mut c_char {
    let record = get_record(stream_ptr, entry_ptr).unwrap();
    into_str_ptr(format!("{:#010X}", record.get_event().get_code()))
}

fn get_info(stream_ptr: *mut DataStream, entry_ptr: *mut Entry) -> *mut c_char {
    let record = get_record(stream_ptr, entry_ptr).unwrap();
    into_str_ptr(format!("{:?}", record.get_event().get_extra()))
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
    let parser: &Parser = stream.get_interface().get_data_handler().unwrap();

    let first_tsc = parser.get_records().get(0).map(|r| r.get_event().get_tsc());

    let mut offset = 0;
    let rows: Vec<*mut Entry> = parser
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

            entry.pid = match r.get_domain().get_type() {
                DomainType::Idle => 0,
                not_idle => {
                    let task_id = match not_idle {
                        DomainType::Default => not_idle.to_id() as i32,
                        _ => r.get_domain().as_u32() as i32 + 1,
                    };

                    stream.add_task_id(task_id);
                    task_id
                }
            };

            entry.offset = offset;
            offset += 1;

            Box::into_raw(entry)
        })
        .collect();

    unsafe {
        *rows_ptr = Box::into_raw(rows.into_boxed_slice()) as _;
    }

    parser.get_records().len().try_into().unwrap()
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
    let parser = Box::new(Parser::new(stream.get_file_path()).unwrap());

    stream.idle_pid = 0;
    stream.n_cpus = parser.cpu_count().try_into().unwrap();
    stream.n_events = parser.get_records().len().try_into().unwrap();

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
