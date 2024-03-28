use std::collections::HashMap;

use fxhash::FxBuildHasher;

const HW_PM_NAMES: &[(u16, &str)] = &[
    (0x001, "hw/pm:freq_change"),
    (0x002, "hw/pm:idle_entry"),
    (0x003, "hw/pm:idle_exit"),
];

const HW_IRQ_NAMES: &[(u16, &str)] = &[
    (0x001, "hw/irq:move_cleanup_delay"),
    (0x002, "hw/irq:move_cleanup"),
    (0x003, "hw/irq:bind_vector"),
    (0x004, "hw/irq:clear_vector"),
    (0x005, "hw/irq:move_finish"),
    (0x006, "hw/irq:assign_vector"),
    (0x007, "hw/irq:unmapped_vector"),
    (0x008, "hw/irq:handled"),
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
