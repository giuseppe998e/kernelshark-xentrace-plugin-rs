use crate::cbind::xen::*;
use xentrace_parser::Event;

fn get_trc_gen_name_str(event: &Event) -> String {
    todo!()
}

fn get_trc_sched_name_str(event: &Event) -> String {
    todo!()
}

fn get_trc_dom0op_name_str(event: &Event) -> String {
    todo!()
}

fn get_trc_hvm_name_str(event: &Event) -> String {
    todo!()
}

fn get_trc_mem_name_str(event: &Event) -> String {
    todo!()
}

fn get_trc_pv_name_str(event: &Event) -> String {
    todo!()
}

fn get_trc_shadow_name_str(event: &Event) -> String {
    todo!()
}

fn get_trc_hw_name_str(event: &Event) -> String {
    todo!()
}

fn get_trc_guest_name_str(event: &Event) -> String {
    todo!()
}

pub(crate) fn get_record_name_str(event: &Event) -> String {
    match event.get_code() & TRC_CLASS {
        // v if v & TRC_GEN == v => get_trc_gen_name_str(&event),
        // v if v & TRC_SCHED == v => get_trc_sched_name_str(&event),
        // v if v & TRC_DOM0OP == v => get_trc_dom0op_name_str(&event),
        // v if v & TRC_HVM == v => get_trc_hvm_name_str(&event),
        // v if v & TRC_MEM == v => get_trc_mem_name_str(&event),
        // v if v & TRC_PV == v => get_trc_pv_name_str(&event),
        // v if v & TRC_SHADOW == v => get_trc_shadow_name_str(&event),
        // v if v & TRC_HW == v => get_trc_hw_name_str(&event),
        // v if v & TRC_GUEST == v => get_trc_guest_name_str(&event),
        v => format!("unknown ({:#010X})", v),
    }
}
