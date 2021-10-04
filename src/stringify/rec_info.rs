use crate::cbind::xen::*;
use xentrace_parser::Event;

fn get_trc_gen_info_str(event: &Event) -> Option<String> {
    None
}

fn get_trc_sched_info_str(event: &Event) -> Option<String> {
    None
}

fn get_trc_dom0op_info_str(event: &Event) -> Option<String> {
    None
}

fn get_trc_hvm_info_str(event: &Event) -> Option<String> {
    None
}

fn get_trc_mem_info_str(event: &Event) -> Option<String> {
    None
}

fn get_trc_pv_info_str(event: &Event) -> Option<String> {
    None
}

fn get_trc_shadow_info_str(event: &Event) -> Option<String> {
    None
}

fn get_trc_hw_info_str(event: &Event) -> Option<String> {
    None
}

fn get_trc_guest_info_str(event: &Event) -> Option<String> {
    None
}

pub(crate) fn get_record_info_str(event: &Event) -> String {
    let einfo = match event.get_code() & TRC_ALL {
        v if v & TRC_GEN == v => get_trc_gen_info_str(&event),
        v if v & TRC_SCHED == v => get_trc_sched_info_str(&event),
        v if v & TRC_DOM0OP == v => get_trc_dom0op_info_str(&event),
        v if v & TRC_HVM == v => get_trc_hvm_info_str(&event),
        v if v & TRC_MEM == v => get_trc_mem_info_str(&event),
        v if v & TRC_PV == v => get_trc_pv_info_str(&event),
        v if v & TRC_SHADOW == v => get_trc_shadow_info_str(&event),
        v if v & TRC_HW == v => get_trc_hw_info_str(&event),
        v if v & TRC_GUEST == v => get_trc_guest_info_str(&event),
        _ => None,
    };

    match einfo {
        Some(v) => v,
        None => event
            .get_extra()
            .iter()
            .map(|v| format!("{:#010X}", v))
            .collect::<Vec<String>>()
            .join(", "),
    }
}
