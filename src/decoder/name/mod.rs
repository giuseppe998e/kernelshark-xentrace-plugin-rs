mod hvm;
mod hw;
mod sched;

use std::collections::HashMap;

use fxhash::FxBuildHasher;
use xentrace_parser::record::Event;

use self::{hvm::get_hvm_name, hw::get_hw_name, sched::get_sched_name};
use crate::ffi::xen::trace::*;

const DOM0OP_NAMES: &[(u16, &str)] = &[
    (0x001, "dom0:create"),
    (0x002, "dom0:destroy"),
    // prevent fmt
];

const GEN_NAMES: &[(u16, &str)] = &[
    (0x001, "gen:lost_records"),
    (0x002, "gen:wrap_buffer"),
    (0x003, "gen:cpu_change"),
];

const MEM_NAMES: &[(u16, &str)] = &[
    (0x001, "mem:page_grant_map"),
    (0x002, "mem:page_grant_unmap"),
    (0x003, "mem:page_grant_transfer"),
    (0x004, "mem:set_p2m_entry"),
    (0x005, "mem:decrease_reservation"),
    (0x010, "mem:pod_populate"),
    (0x011, "mem:pod_zero_reclaim"),
    (0x012, "mem:pod_superpage_splinter"),
];

const PV_NAMES: &[(u16, &str)] = &[
    (0x001, "pv:hypercall"),
    (0x003, "pv:trap"),
    (0x004, "pv:page_fault"),
    (0x005, "pv:forced_invalid_op"),
    (0x006, "pv:emulate_privop"),
    (0x007, "pv:emulate_4gb"),
    (0x008, "pv:math_state_restore"),
    (0x009, "pv:paging_fixup"),
    (0x00A, "pv:gdt_ldt_mapping_fault"),
    (0x00B, "pv:ptwr_emulation"),
    (0x00C, "pv:ptwr_emulation_pae"),
    (0x00D, "pv:hypercall_v2"),
    (0x00E, "pv:hypercall_subcall"),
];

const SHADOW_NAMES: &[(u16, &str)] = &[
    (0x001, "shadow:not_shadow"),
    (0x002, "shadow:fast_propagate"),
    (0x003, "shadow:fast_mmio"),
    (0x004, "shadow:false_fast_path"),
    (0x005, "shadow:mmio"),
    (0x006, "shadow:fixup"),
    (0x007, "shadow:domf_dying"),
    (0x008, "shadow:emulate"),
    (0x009, "shadow:emulate_unshadow_user"),
    (0x00A, "shadow:emulate_unshadow_evtinj"),
    (0x00B, "shadow:emulate_unshadow_unhandled"),
    (0x00C, "shadow:wrmap_bf"),
    (0x00D, "shadow:prealloc_unpin"),
    (0x00E, "shadow:resync_full"),
    (0x00F, "shadow:resync_only"),
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
