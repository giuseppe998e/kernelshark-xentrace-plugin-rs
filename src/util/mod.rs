pub(crate) mod pointer;
pub(crate) mod string;

use self::pointer::from_raw_ptr;
use crate::cbind::kshark::{entry::Entry, stream::DataStream};
use libc::{c_long, c_ulong};
use std::convert::TryInto;
use xentrace_parser::{Parser, Record};

const DEFAULT_CPU_QHZ: c_ulong = 2_400_000_000;

pub(crate) fn tsc_to_ns(
    tsc: c_ulong,
    first_tsc: Option<c_ulong>,
    cpu_freq: Option<c_ulong>,
) -> c_long {
    let cpu_qhz = {
        let cpu_hz = cpu_freq.unwrap_or(DEFAULT_CPU_QHZ);
        (cpu_hz << 10) / 1_000_000_000
    };

    let tsc = match first_tsc {
        Some(v) => (tsc - v) << 10,
        None => tsc,
    };

    (tsc / cpu_qhz).try_into().unwrap()
}

pub(crate) fn get_record<'a>(
    stream_ptr: *mut DataStream,
    entry_ptr: *mut Entry,
) -> Option<&'a Record> {
    let entry = from_raw_ptr(entry_ptr)?;
    let parser: &Parser = {
        let stream = from_raw_ptr(stream_ptr).unwrap();
        let interface = stream.get_interface();
        interface.get_data_handler()?
    };

    parser.get_records().get(entry.offset as usize)
}
