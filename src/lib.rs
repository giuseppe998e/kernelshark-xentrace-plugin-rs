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
use cbind::kshark::{interface::GenericStreamInterface, stream::DataStream};

mod stringify;
mod util;

mod ifunctions;
use ifunctions::*;

use xentrace_parser::{xentrace_parse, Trace};

use libc::{c_char, c_int, c_uint, c_void};
use std::{convert::TryInto, fs::File, io::Read, path::Path, ptr::null_mut};

static KSHARK_SOURCE_TYPE: &str = "xentrace_bin";

// KSHARK_INPUT_CHECK @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_check(file_ptr: *mut c_char, _frmt: *mut *mut c_char) -> bool {
    if let Ok(fstr) = from_str_ptr!(file_ptr) {
        let path = Path::new(fstr);
        if let Ok(mut file) = File::open(path) {
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
    stream.n_events = trace.record_count().try_into().unwrap_or(c_int::MAX);

    stream.interface = {
        let mut interface = GenericStreamInterface::boxed();

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
    let stream = from_raw_ptr_mut!(stream_ptr).unwrap();
    let interface = stream.get_mut_interface();
    let trace = unsafe { Box::<Trace>::from_raw(interface.handle as _) };

    drop(trace);
    interface.handle = null_mut::<c_void>();
}
