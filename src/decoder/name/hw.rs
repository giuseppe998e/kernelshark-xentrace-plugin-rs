use xentrace_parser::record::EventCode;

fn get_hw_pm_name(ecode: &EventCode) -> Option<&str> {
    match ecode.minor() {
        0x001 => Some("cpu_freq_change"),
        0x002 => Some("cpu_idle_entry"),
        0x003 => Some("cpu_idle_exit"),
        _ => None,
    }
}

fn get_hw_irq_name(ecode: &EventCode) -> Option<&str> {
    match ecode.minor() {
        0x001 => Some("cleanup_move_delayed"),
        0x002 => Some("cleanup_move"),
        0x003 => Some("bind_vector"),
        0x004 => Some("clear_vector"),
        0x005 => Some("move_vector"),
        0x006 => Some("assign_vector"),
        0x007 => Some("bogus_vector"),
        0x008 => Some("do_irq"),
        _ => None,
    }
}

pub(super) fn get_hw_name(ecode: &EventCode) -> Option<&str> {
    match ecode.sub() {
        0x1 => get_hw_pm_name(ecode),
        0x2 => get_hw_irq_name(ecode),
        _ => None,
    }
}
