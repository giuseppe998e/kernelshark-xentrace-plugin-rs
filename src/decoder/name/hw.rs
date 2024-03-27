use std::collections::HashMap;

use fxhash::FxBuildHasher;

const HW_PM_NAMES: &[(u16, &str)] = &[
    (0x001, "cpu_freq_change"),
    (0x002, "cpu_idle_entry"),
    (0x003, "cpu_idle_exit"),
];

const HW_IRQ_NAMES: &[(u16, &str)] = &[
    (0x001, "cleanup_move_delayed"),
    (0x002, "cleanup_move"),
    (0x003, "bind_vector"),
    (0x004, "clear_vector"),
    (0x005, "move_vector"),
    (0x006, "assign_vector"),
    (0x007, "bogus_vector"),
    (0x008, "do_irq"),
];

lazy_static::lazy_static! {
    static ref HW_PM_MAP: HashMap<u16, &'static str, FxBuildHasher> = HW_PM_NAMES.iter().copied().collect();
    static ref HW_IRQ_MAP: HashMap<u16, &'static str, FxBuildHasher> = HW_IRQ_NAMES.iter().copied().collect();
}

pub(super) fn get_hw_name(sub: u32, minor: u16) -> Option<&'static str> {
    match sub {
        0x1 => HW_PM_MAP.get(&minor).copied(),
        0x2 => HW_IRQ_MAP.get(&minor).copied(),
        _ => None,
    }
}
