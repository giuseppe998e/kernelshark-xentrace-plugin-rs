use std::env;

use libc::{c_double, c_long, c_ulong};

const ENV_VAR_CPU_HZ: &str = "XENTRACE_CPUHZ";
const DEFAULT_CPU_HZ: c_double = 2_400_000_000.0;

const GHZ: c_double = 1_000_000_000.0;
const MHZ: c_double = 1_000_000.0;
const KHZ: c_double = 1_000.0;

pub(crate) fn get_env_cpu_freq() -> c_double {
    let cpu_freq = env::var(ENV_VAR_CPU_HZ)
        .ok()
        .and_then(|mut env_freq| {
            let multiplier = match env_freq.chars().last()? {
                'G' => {
                    let _ = env_freq.pop();
                    GHZ
                }
                'M' => {
                    let _ = env_freq.pop();
                    MHZ
                }
                'K' => {
                    let _ = env_freq.pop();
                    KHZ
                }
                '0'..='9' => 1.0,
                _ => return None,
            };

            env_freq
                .parse::<c_double>()
                .map(|base_hz| base_hz * multiplier)
                .ok()
        })
        .unwrap_or(DEFAULT_CPU_HZ);

    cpu_freq * 1024.0 / GHZ
}

pub(crate) fn tsc_to_ns(tsc: c_ulong, cpu_qhz: c_double, first_tsc: Option<c_ulong>) -> c_long {
    let tsc = first_tsc.map(|first| (tsc - first) << 10).unwrap_or(tsc);
    (tsc as c_double / cpu_qhz) as c_long
}
