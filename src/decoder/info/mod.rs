use crate::ffi::xen::trace::*;
use xentrace_parser::record::Event;

fn get_gen_info(_event: &Event) -> Option<&str> {
    None
}

fn get_sched_info(_event: &Event) -> Option<&str> {
    None
}

fn get_dom0op_info(_event: &Event) -> Option<&str> {
    None
}

fn get_hvm_info(_event: &Event) -> Option<&str> {
    None
}

fn get_mem_info(_event: &Event) -> Option<&str> {
    None
}

fn get_pv_info(_event: &Event) -> Option<&str> {
    None
}

fn get_shadow_info(_event: &Event) -> Option<&str> {
    None
}

fn get_hw_info(_event: &Event) -> Option<&str> {
    None
}

fn get_guest_info(_event: &Event) -> Option<&str> {
    None
}

pub(crate) fn get_record_info(event: &Event) -> String {
    let ecode = event.code();
    let string_opt = match ecode.main() {
        TRC_GEN => get_gen_info(event),
        TRC_SCHED => get_sched_info(event),
        TRC_DOM0OP => get_dom0op_info(event),
        TRC_HVM => get_hvm_info(event),
        TRC_MEM => get_mem_info(event),
        TRC_PV => get_pv_info(event),
        TRC_SHADOW => get_shadow_info(event),
        TRC_HW => get_hw_info(event),
        TRC_GUEST => get_guest_info(event),
        _ => None,
    };

    match string_opt {
        Some(val) => val.to_string(),
        None => event
            .extra()
            .iter()
            .filter_map(|opt| opt.map(|val| format!("0x{:08X}", val)))
            .collect::<Vec<_>>()
            .join(", "),
    }
}
