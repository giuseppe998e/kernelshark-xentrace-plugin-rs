use std::collections::HashMap;

use fxhash::FxBuildHasher;

const HVM_ENTRYEXIT_NAMES: &[(u16, &str)] = &[
    (0x001, "hwm:vmentry"),
    (0x002, "hwm:vmexit"),
    (0x102, "hwm:vmexit64"),
    (0x401, "hwm:nested_vmentry"),
    (0x402, "hwm:nested_vmexit"),
    (0x502, "hwm:nested_vmexit64"),
];

const HVM_HANDLER_NAMES: &[(u16, &str)] = &[
    (0x001, "hwm:pf_xen"),
    (0x101, "hwm:pf_xen64"),
    (0x002, "hwm:pf_inject"),
    (0x102, "hwm:pf_inject64"),
    (0x003, "hwm:inj_exc"),
    (0x004, "hwm:inj_virq"),
    (0x005, "hwm:reinj_virq"),
    (0x006, "hwm:io_read"),
    (0x007, "hwm:io_write"),
    (0x008, "hwm:cr_read"),
    (0x108, "hwm:cr_read64"),
    (0x009, "hwm:cr_write"),
    (0x109, "hwm:cr_write64"),
    (0x00A, "hwm:dr_read"),
    (0x00B, "hwm:dr_write"),
    (0x00C, "hwm:msr_read"),
    (0x00D, "hwm:msr_write"),
    (0x00E, "hwm:cpuid"),
    (0x00F, "hwm:intr"),
    (0x010, "hwm:nmi"),
    (0x011, "hwm:smi"),
    (0x012, "hwm:vmmcall"),
    (0x013, "hwm:hlt"),
    (0x014, "hwm:invlpg"),
    (0x114, "hwm:invlpg64"),
    (0x015, "hwm:mce"),
    (0x016, "hwm:ioport_read"),
    (0x216, "hwm:ioport_write"),
    (0x017, "hwm:iomem_read"),
    (0x217, "hwm:iomem_write"),
    (0x018, "hwm:clts"),
    (0x019, "hwm:lmsw"),
    (0x119, "hwm:lmsw64"),
    (0x01A, "hwm:rdtsc"),
    (0x020, "hwm:intr_window"),
    (0x021, "hwm:npf"),
    (0x022, "hwm:realmode_emulate"),
    (0x023, "hwm:trap"),
    (0x024, "hwm:trap_debug"),
    (0x025, "hwm:vlapic"),
];

const HVM_EMUL_NAMES: &[(u16, &str)] = &[
    (0x001, "hwm/emul:hpet_start_timer"),
    (0x002, "hwm/emul:pit_start_timer"),
    (0x003, "hwm/emul:rtc_start_timer"),
    (0x004, "hwm/emul:lapic_start_timer"),
    (0x005, "hwm/emul:hpet_stop_timer"),
    (0x006, "hwm/emul:pit_stop_timer"),
    (0x007, "hwm/emul:rtc_stop_timer"),
    (0x008, "hwm/emul:lapic_stop_timer"),
    (0x009, "hwm/emul:pit_timer_cb"),
    (0x00A, "hwm/emul:lapic_timer_cb"),
    (0x00B, "hwm/emul:pic_int_output"),
    (0x00C, "hwm/emul:pic_kick"),
    (0x00D, "hwm/emul:pic_intack"),
    (0x00E, "hwm/emul:pic_posedge"),
    (0x00F, "hwm/emul:pic_negedge"),
    (0x010, "hwm/emul:pic_pend_irq_call"),
    (0x011, "hwm/emul:lapic_pic_intr"),
];

lazy_static::lazy_static! {
    static ref HVM_ENTRYEXIT_MAP: HashMap<u16, &'static str, FxBuildHasher> = HVM_ENTRYEXIT_NAMES.iter().copied().collect();
    static ref HVM_HANDLER_MAP: HashMap<u16, &'static str, FxBuildHasher> = HVM_HANDLER_NAMES.iter().copied().collect();
    static ref HVM_EMUL_MAP: HashMap<u16, &'static str, FxBuildHasher> = HVM_EMUL_NAMES.iter().copied().collect();
}

// Function to get the corresponding name based on the event code and category
pub(super) fn get_hvm_name(sub: u32, minor: u16) -> Option<&'static str> {
    match sub {
        0x1 => HVM_ENTRYEXIT_MAP.get(&minor).copied(),
        0x2 => HVM_HANDLER_MAP.get(&minor).copied(),
        0x4 => HVM_EMUL_MAP.get(&minor).copied(),
        _ => None,
    }
}
