use crate::cbind::xen::*;
use xentrace_parser::Event;

fn get_trc_gen_name_str(event: &Event) -> Option<String> {
    Some("TRC_GEN".to_owned())
}

fn get_trc_sched_name_str(event: &Event) -> Option<String> {
    Some("TRC_SCHED".to_owned())
}

fn get_trc_dom0op_name_str(event: &Event) -> Option<String> {
    Some("TRC_DOM0OP".to_owned())
}

fn get_trc_hvm_name_str(event: &Event) -> Option<String> {
    Some("TRC_HVM".to_owned())
}

fn get_trc_mem_name_str(event: &Event) -> Option<String> {
    Some("TRC_MEM".to_owned())
}

fn get_trc_pv_name_str(event: &Event) -> Option<String> {
    Some("TRC_PV".to_owned())
}

fn get_trc_shadow_name_str(event: &Event) -> Option<String> {
    Some("TRC_SHADOW".to_owned())
}

fn get_trc_hw_name_str(event: &Event) -> Option<String> {
    Some("TRC_HW".to_owned())
}

fn get_trc_guest_name_str(event: &Event) -> Option<String> {
    Some("TRC_GUEST".to_owned())
}

pub(crate) fn get_record_name_str(event: &Event) -> String {
    let ecode = event.get_code();
    let result_str = match ecode.get_main() {
        TRC_GEN => get_trc_gen_name_str(&event),
        TRC_SCHED => get_trc_sched_name_str(&event),
        TRC_DOM0OP => get_trc_dom0op_name_str(&event),
        TRC_HVM => get_trc_hvm_name_str(&event),
        TRC_MEM => get_trc_mem_name_str(&event),
        TRC_PV => get_trc_pv_name_str(&event),
        TRC_SHADOW => get_trc_shadow_name_str(&event),
        TRC_HW => get_trc_hw_name_str(&event),
        TRC_GUEST => get_trc_guest_name_str(&event),
        _ => None,
    };

    match result_str {
        Some(v) => v,
        None => format!("unknown ({:#010X})", ecode.into_u32()),
    }
}
