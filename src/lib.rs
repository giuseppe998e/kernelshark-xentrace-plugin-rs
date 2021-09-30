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

use interface::kshark::{kshark_context, kshark_data_stream, kshark_entry};
use std::os::raw::{c_char, c_int};
use util::string::{ptr_to_str, str_to_ptr};

static KSHARK_SOURCE_TYPE: &str = "kshark_xen";

// KSHARK_INPUT_CHECK @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_check(file: *const c_char, _frmt: *mut *mut c_char) -> bool {
    false
}

// KSHARK_INPUT_FORMAT @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_format() -> *const c_char {
    KSHARK_SOURCE_TYPE.as_ptr() as *const _
}

// KSHARK_INPUT_INITIALIZER @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_initializer(stream: *mut kshark_data_stream) -> c_int {
    0
}

// KSHARK_INPUT_DEINITIALIZER @ libkshark-plugin.h
#[no_mangle]
pub extern "C" fn kshark_input_deinitializer(_stream: *mut kshark_data_stream) {}
