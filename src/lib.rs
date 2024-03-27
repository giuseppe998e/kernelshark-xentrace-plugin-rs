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
mod interface;
mod util;

use std::{fs, io::Read as _, path::Path, ptr::null_mut};

use libc::{c_char, c_int, EFAULT, ENOENT};
use xentrace_parser::Trace;

use self::{
    ffi::libkshark::{DataStream, GenericStreamInterface},
    interface::{get_event_id, get_event_name, get_info, get_pid, get_task, load_entries},
};

const TRC_TRACE_CPU_CHANGE: u32 = 0x0001F003;
static KSHARK_FORMAT_NAME: &str = "xentrace_binary";

// KSHARK_INPUT_INITIALIZER @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_initializer(stream: *mut DataStream) -> c_int {
    let Some(stream) = DataStream::from_ptr_mut(stream) else {
        return -EFAULT;
    };

    let trace = {
        let Some(bin_path) = stream.get_file_path() else {
            return -EFAULT;
        };

        match Trace::from_file(bin_path) {
            Ok(trace) => Box::new(trace),
            Err(e) => return e.raw_os_error().unwrap_or(-ENOENT),
        }
    };

    stream.idle_pid = 0;
    stream.n_cpus = trace.cpu_count().try_into().unwrap_or(c_int::MAX);
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
        let interface_exists = if let Some(interface) = stream.get_interface_mut() {
            if !interface.handle.is_null() {
                let _ = unsafe { Box::<Trace>::from_raw(interface.handle as _) }; // Drop it
                interface.handle = null_mut();
            }

            true
        } else {
            false
        };

        if interface_exists {
            let _ = unsafe { Box::<GenericStreamInterface>::from_raw(stream.interface as _) }; // Drop it
            stream.interface = null_mut();
        }
    }
}

// KSHARK_INPUT_CHECK @ libkshark-plugin.h
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn kshark_input_check(file: *const c_char, _: *mut *mut c_char) -> bool {
    let Some(path) = str_from_ptr!(file).map(Path::new) else {
        return false;
    };

    let Ok(mut file) = fs::File::open(path) else {
        return false;
    };

    let code = {
        let mut buf = [0u8; 4];
        let _ = file.read_exact(&mut buf);
        u32::from_ne_bytes(buf) & 0x0FFFFFFF
    };

    code == TRC_TRACE_CPU_CHANGE
}

// KSHARK_INPUT_FORMAT @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_format() -> *const c_char {
    KSHARK_FORMAT_NAME.as_ptr() as _
}
