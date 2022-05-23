use crate::cbind::xen::*;
use xentrace_parser::record::Event;

fn get_trc_gen_info_str<'a>(_event: &Event) -> Option<&'a str> {
    None
}

fn get_trc_sched_info_str<'a>(_event: &Event) -> Option<&'a str> {
    None
}

fn get_trc_dom0op_info_str<'a>(_event: &Event) -> Option<&'a str> {
    None
}

fn get_trc_hvm_info_str<'a>(_event: &Event) -> Option<&'a str> {
    None
}

fn get_trc_mem_info_str<'a>(_event: &Event) -> Option<&'a str> {
    None
}

fn get_trc_pv_info_str<'a>(_event: &Event) -> Option<&'a str> {
    None
}

fn get_trc_shadow_info_str<'a>(_event: &Event) -> Option<&'a str> {
    None
}

fn get_trc_hw_info_str<'a>(_event: &Event) -> Option<&'a str> {
    None
}

fn get_trc_guest_info_str(_event: &Event) -> Option<&str> {
    None
}

pub(crate) fn get_record_info_str(event: &Event) -> String {
    let ecode = event.code;
    let result_str = match ecode.main {
        TRC_GEN => get_trc_gen_info_str(event),
        TRC_SCHED => get_trc_sched_info_str(event),
        TRC_DOM0OP => get_trc_dom0op_info_str(event),
        TRC_HVM => get_trc_hvm_info_str(event),
        TRC_MEM => get_trc_mem_info_str(event),
        TRC_PV => get_trc_pv_info_str(event),
        TRC_SHADOW => get_trc_shadow_info_str(event),
        TRC_HW => get_trc_hw_info_str(event),
        TRC_GUEST => get_trc_guest_info_str(event),
        _ => None,
    };

    match result_str {
        Some(v) => v.to_string(),
        None => event
            .extra
            .iter()
            .filter(|&o| o.is_some())
            .map(|v| format!("0x{:08X}", v.unwrap()))
            .collect::<Vec<_>>()
            .join(", "),
    }
}
