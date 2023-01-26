use libc::{c_double, c_long, c_ulong};
use std::env;

const ENV_VAR_CPU_HZ: &str = "XENTRACE_CPUHZ";
const DEFAULT_CPU_HZ: c_double = 2_400_000_000.0;

const GHZ: c_double = 1_000_000_000.0;
const MHZ: c_double = 1_000_000.0;
const KHZ: c_double = 1_000.0;

pub(crate) fn get_env_cpu_freq() -> c_double {
    let cpu_freq = env::var(ENV_VAR_CPU_HZ)
        .ok()
        .and_then(|mut freq_str| {
            let last_char_idx = freq_str.len() - 1;
            let multiplier = match &freq_str[last_char_idx..] {
                "G" => {
                    let _ = freq_str.pop();
                    GHZ
                }
                "M" => {
                    let _ = freq_str.pop();
                    MHZ
                }
                "K" => {
                    let _ = freq_str.pop();
                    KHZ
                }
                _ => 1.0,
            };

            freq_str
                .parse::<c_double>()
                .map(|base_hz| base_hz * multiplier)
                .ok()
        })
        .unwrap_or(DEFAULT_CPU_HZ);

    (cpu_freq * 1024.0/* << 10 */) / GHZ
}

pub(crate) fn tsc_to_ns(tsc: c_ulong, cpu_qhz: c_double, first_tsc: Option<c_ulong>) -> c_long {
    let tsc = first_tsc.map(|val| (tsc - val) << 10).unwrap_or(tsc);
    let ns = (tsc as c_double / cpu_qhz) as c_ulong;
    ns.try_into().unwrap()
}
