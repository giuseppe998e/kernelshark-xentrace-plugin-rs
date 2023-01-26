/***
 * KernelSheak XenTrace plugin
 * Copyright (C) 2023 Giuseppe Eletto <giuseppe.eletto98@gmail.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301
 * USA
 */
mod decoder;
mod ffi;
mod functions;
mod macros;

use ffi::libkshark::{DataStream, GenericStreamInterface};
use functions::{get_event_id, get_event_name, get_info, get_pid, get_task, load_entries};
use libc::{c_char, c_int};
use std::{fs::File, io::Read, path::Path, ptr::null_mut};
use xentrace_parser::Trace;

const ENOENT: c_int = 2;
const EFAULT: c_int = 14;

const TRC_TRACE_CPU_CHANGE: u32 = 0x0001F003;
static KSHARK_FORMAT_NAME: &str = "xentrace_binary";

// KSHARK_INPUT_INITIALIZER @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_initializer(stream: *mut DataStream) -> c_int {
    let stream = match DataStream::from_ptr_mut(stream) {
        Some(stream) => stream,
        None => return -EFAULT,
    };

    let trace = {
        let path = match stream.get_file_path() {
            Some(path) => path,
            None => return -EFAULT,
        };

        match Trace::try_from(path) {
            Ok(trace) => Box::new(trace),
            Err(err) => return err.raw_os_error().unwrap_or(-ENOENT),
        }
    };

    stream.idle_pid = 0;
    stream.n_cpus = trace.cpu_count().into();
    stream.n_events = trace.record_count().try_into().unwrap_or(c_int::MAX);

    stream.interface = {
        let mut interface = Box::<GenericStreamInterface>::default();

        interface.get_pid = get_pid as _;
        interface.get_event_id = get_event_id as _;
        interface.get_event_name = get_event_name as _;
        interface.get_task = get_task as _;
        interface.get_info = get_info as _;
        //interface.dump_entry = dump_entry as _;
        interface.load_entries = load_entries as _;
        interface.handle = Box::into_raw(trace) as _;

        Box::into_raw(interface)
    };

    0
}

// KSHARK_INPUT_DEINITIALIZER @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_deinitializer(stream: *mut DataStream) {
    if let Some(stream) = DataStream::from_ptr_mut(stream) {
        if let Some(interface) = stream.get_interface_mut() {
            let _ = unsafe { Box::<Trace>::from_raw(interface.handle as _) }; // Drop it
            interface.handle = null_mut();
        }
    }

    // else... Memory leak
}

// KSHARK_INPUT_CHECK @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_check(file: *const c_char, _format: *mut *mut c_char) -> bool {
    if let Some(path) = str_from_ptr!(file).map(Path::new) {
        if let Ok(mut file) = File::open(path) {
            let code = {
                let mut buf = [0u8; 4];
                let _ = file.read_exact(&mut buf);
                u32::from_ne_bytes(buf) & 0x0FFFFFFF
            };

            return code == TRC_TRACE_CPU_CHANGE;
        }
    }

    false
}

// KSHARK_INPUT_FORMAT @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_format() -> *const c_char {
    KSHARK_FORMAT_NAME.as_ptr() as _
}
