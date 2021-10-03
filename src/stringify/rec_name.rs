use crate::cbind::xen::*;
use libc::c_uint;
use xentrace_parser::Event;

fn get_trc_gen_name_str(ecode: c_uint) -> String {
    todo!()
}

fn get_trc_sched_name_str(ecode: c_uint) -> String {
    todo!()
}

fn get_trc_dom0op_name_str(ecode: c_uint) -> String {
    todo!()
}

fn get_trc_hvm_name_str(ecode: c_uint) -> String {
    todo!()
}

fn get_trc_mem_name_str(ecode: c_uint) -> String {
    todo!()
}

fn get_trc_pv_name_str(ecode: c_uint) -> String {
    todo!()
}

fn get_trc_shadow_name_str(ecode: c_uint) -> String {
    todo!()
}

fn get_trc_hw_name_str(ecode: c_uint) -> String {
    todo!()
}

fn get_trc_guest_name_str(ecode: c_uint) -> String {
    todo!()
}

pub(crate) fn get_record_name_str(event: &Event) -> String {
    let ecode: c_uint = event.get_code();
    match ecode & TRC_CLASS {
        // v if v & TRC_GEN == v => get_trc_gen_name_str(ecode),
        // v if v & TRC_SCHED == v => get_trc_sched_name_str(ecode),
        // v if v & TRC_DOM0OP == v => get_trc_dom0op_name_str(ecode),
        // v if v & TRC_HVM == v => get_trc_hvm_name_str(ecode),
        // v if v & TRC_MEM == v => get_trc_mem_name_str(ecode),
        // v if v & TRC_PV == v => get_trc_pv_name_str(ecode),
        // v if v & TRC_SHADOW == v => get_trc_shadow_name_str(ecode),
        // v if v & TRC_HW == v => get_trc_hw_name_str(ecode),
        // v if v & TRC_GUEST == v => get_trc_guest_name_str(ecode),
        v => format!("unknown ({:#010X})", v),
    }
}
