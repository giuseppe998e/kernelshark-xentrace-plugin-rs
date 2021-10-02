pub(crate) mod pointer;
pub(crate) mod string;

use libc::{c_long, c_ulong};
use std::convert::TryInto;

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

    let result_ns = match first_tsc {
        Some(v) => ((tsc - v) << 10) / cpu_qhz,
        None => tsc / cpu_qhz,
    };

    result_ns.try_into().unwrap()
}
