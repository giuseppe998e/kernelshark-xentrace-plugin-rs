mod hvm;
mod hw;
mod sched;

use self::{hvm::get_hvm_name, hw::get_hw_name, sched::get_sched_name};
use crate::ffi::xen::trace::*;
use xentrace_parser::record::{Event, EventCode};

fn get_gen_name(ecode: &EventCode) -> Option<&str> {
    match ecode.minor() {
        0x001 => Some("lost_records"),
        0x002 => Some("wrap_buffer"),
        // 0x003 => unreachable!(), /* cpu_change */
        0x004 => Some("trace_irq"),
        _ => None,
    }
}

fn get_dom0op_name(ecode: &EventCode) -> Option<&str> {
    match ecode.minor() {
        0x001 => Some("domain_create"),
        0x002 => Some("domain_destroy"),
        _ => None,
    }
}

fn get_mem_name(ecode: &EventCode) -> Option<&str> {
    match ecode.minor() {
        0x001 => Some("page_grant_map"),
        0x002 => Some("page_grant_unmap"),
        0x003 => Some("page_grant_transfer"),
        _ => None,
    }
}

fn get_pv_name(ecode: &EventCode) -> Option<&str> {
    match ecode.minor() & 0x00F {
        0x003 => Some("trap"),
        0x004 => Some("page_fault"),
        0x005 => Some("forced_invalid_op"),
        0x006 => Some("emulate_privop"),
        0x007 => Some("emulate_4G"),
        0x008 => Some("math_state_restore"),
        0x009 => Some("paging_fixup"),
        0x00A => Some("gdt_ldt_mapping_fault"),
        0x00B => Some("ptwr_emulation"),
        0x00C => Some("ptwr_emulation_pae"),
        0x001 | 0x00D | 0x00E => Some("hypercall"),
        _ => None,
    }
}

fn get_shadow_name(ecode: &EventCode) -> Option<&str> {
    match ecode.minor() & 0x00F {
        0x001 => Some("shadow_not_shadow"),
        0x002 => Some("shadow_fast_propagate"),
        0x003 => Some("shadow_fast_mmio"),
        0x004 => Some("shadow_false_fast_path"),
        0x005 => Some("shadow_mmio"),
        0x006 => Some("shadow_fixup"),
        0x007 => Some("shadow_domf_dying"),
        0x008 => Some("shadow_emulate"),
        0x009 => Some("shadow_emulate_unshadow_user"),
        0x00A => Some("shadow_emulate_unshadow_evtinj"),
        0x00B => Some("shadow_emulate_unshadow_unhandled"),
        0x00C => Some("shadow_emulate_wrmap_bf"),
        0x00D => Some("shadow_emulate_prealloc_unpin"),
        0x00E => Some("shadow_emulate_resync_full"),
        0x00F => Some("shadow_emulate_resync_only"),
        _ => None,
    }
}

pub(crate) fn get_record_name(event: &Event) -> String {
    let ecode = event.code();
    let string_opt = match ecode.main() {
        TRC_GEN => get_gen_name(&ecode),
        TRC_SCHED => get_sched_name(&ecode),
        TRC_DOM0OP => get_dom0op_name(&ecode),
        TRC_HVM => get_hvm_name(&ecode),
        TRC_MEM => get_mem_name(&ecode),
        TRC_PV => get_pv_name(&ecode),
        TRC_SHADOW => get_shadow_name(&ecode),
        TRC_HW => get_hw_name(&ecode),
        TRC_GUEST => Some("TRC_GUEST"),
        _ => None,
    };

    match string_opt {
        Some(val) => val.to_string(),
        None => format!("unknown (0x{:08X})", u32::from(ecode)),
    }
}
