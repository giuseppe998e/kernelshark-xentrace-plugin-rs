mod hvm;
mod hw;
mod sched;

use std::collections::HashMap;

use fxhash::FxBuildHasher;
use xentrace_parser::record::Event;

use self::{hvm::get_hvm_name, hw::get_hw_name, sched::get_sched_name};
use crate::ffi::xen::trace::*;

const DOM0OP_NAMES: &[(u16, &str)] = &[
    (0x001, "domain_create"),
    (0x002, "domain_destroy"),
    // prevent fmt
];

const GEN_NAMES: &[(u16, &str)] = &[
    (0x001, "lost_records"),
    (0x002, "wrap_buffer"),
    (0x004, "trace_irq"),
];

const MEM_NAMES: &[(u16, &str)] = &[
    (0x001, "page_grant_map"),
    (0x002, "page_grant_unmap"),
    (0x003, "page_grant_transfer"),
];

const PV_NAMES: &[(u16, &str)] = &[
    (0x003, "trap"),
    (0x004, "page_fault"),
    (0x005, "forced_invalid_op"),
    (0x006, "emulate_privop"),
    (0x007, "emulate_4G"),
    (0x008, "math_state_restore"),
    (0x009, "paging_fixup"),
    (0x00A, "gdt_ldt_mapping_fault"),
    (0x00B, "ptwr_emulation"),
    (0x00C, "ptwr_emulation_pae"),
    (0x00D, "hypercall"),
    (0x00E, "hypercall"),
    (0x00F, "hypercall"),
];

const SHADOW_NAMES: &[(u16, &str)] = &[
    (0x001, "shadow_not_shadow"),
    (0x002, "shadow_fast_propagate"),
    (0x003, "shadow_fast_mmio"),
    (0x004, "shadow_false_fast_path"),
    (0x005, "shadow_mmio"),
    (0x006, "shadow_fixup"),
    (0x007, "shadow_domf_dying"),
    (0x008, "shadow_emulate"),
    (0x009, "shadow_emulate_unshadow_user"),
    (0x00A, "shadow_emulate_unshadow_evtinj"),
    (0x00B, "shadow_emulate_unshadow_unhandled"),
    (0x00C, "shadow_emulate_wrmap_bf"),
    (0x00D, "shadow_emulate_prealloc_unpin"),
    (0x00E, "shadow_emulate_resync_full"),
    (0x00F, "shadow_emulate_resync_only"),
];

lazy_static::lazy_static! {
    static ref DOM0OP_MAP: HashMap<u16, &'static str, FxBuildHasher> = DOM0OP_NAMES.iter().copied().collect();
    static ref GEN_MAP: HashMap<u16, &'static str, FxBuildHasher> = GEN_NAMES.iter().copied().collect();
    static ref MEM_MAP: HashMap<u16, &'static str, FxBuildHasher> = MEM_NAMES.iter().copied().collect();
    static ref PV_MAP: HashMap<u16, &'static str, FxBuildHasher> = PV_NAMES.iter().copied().collect();
    static ref SHADOW_MAP: HashMap<u16, &'static str, FxBuildHasher> = SHADOW_NAMES.iter().copied().collect();
}

pub(crate) fn get_record_name(event: &Event) -> String {
    let ecode = event.code();

    let main = ecode.main();
    let sub = ecode.sub();
    let minor = ecode.minor() as u16;

    let string_opt = match main {
        TRC_HVM => get_hvm_name(sub, minor),
        TRC_HW => get_hw_name(sub, minor),
        TRC_SCHED => get_sched_name(sub, minor),

        TRC_DOM0OP => DOM0OP_MAP.get(&minor).copied(),
        TRC_GEN => GEN_MAP.get(&minor).copied(),
        TRC_MEM => MEM_MAP.get(&minor).copied(),
        TRC_PV => PV_MAP.get(&(minor & 0x00F)).copied(),
        TRC_SHADOW => SHADOW_MAP.get(&(minor & 0x00F)).copied(),

        TRC_GUEST => Some("TRC_GUEST"),
        _ => None,
    };

    match string_opt {
        Some(val) => val.to_string(),
        None => format!("unknown (0x{:08X})", u32::from(ecode)),
    }
}
