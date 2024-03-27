use std::collections::HashMap;

use fxhash::FxBuildHasher;

const HVM_ENTRYEXIT_NAMES: &[(u16, &str)] = &[
    (0x001, "VMENTRY"),
    (0x002, "VMEXIT"),
    (0x102, "VMEXIT"),
    (0x401, "nVMENTRY"),
    (0x402, "nVMEXIT"),
    (0x502, "nVMEXIT"),
];

const HVM_HANDLER_NAMES: &[(u16, &str)] = &[
    (0x001, "PF_XEN"),
    (0x101, "PF_XEN"),
    (0x002, "PF_INJECT"),
    (0x102, "PF_INJECT"),
    (0x003, "INJ_EXC"),
    (0x004, "INJ_VIRQ"),
    (0x005, "REINJ_VIRQ"),
    (0x006, "IO_READ"),
    (0x007, "IO_WRITE"),
    (0x008, "CR_READ"),
    (0x108, "CR_READ"),
    (0x009, "CR_WRITE"),
    (0x109, "CR_WRITE"),
    (0x00A, "DR_READ"),
    (0x00B, "DR_WRITE"),
    (0x00C, "MSR_READ"),
    (0x00D, "MSR_WRITE"),
    (0x00E, "CPUID"),
    (0x00F, "INTR"),
    (0x010, "NMI"),
    (0x011, "SMI"),
    (0x012, "VMMCALL"),
    (0x013, "HLT"),
    (0x014, "INVLPG"),
    (0x114, "INVLPG"),
    (0x015, "MCE"),
    (0x016, "IOPORT_READ"),
    (0x216, "IOPORT_WRITE"),
    (0x017, "MMIO_READ"),
    (0x217, "MMIO_WRITE"),
    (0x018, "CLTS"),
    (0x019, "LMSW"),
    (0x119, "LMSW"),
    (0x01A, "RDTSC"),
    (0x020, "INTR_WINDOW"),
    (0x021, "NPF"),
    (0x023, "TRAP"),
];

const HVM_EMUL_NAMES: &[(u16, &str)] = &[
    (0x001, "hpet"),
    (0x005, "hpet"),
    (0x003, "rtc"),
    (0x007, "rtc"),
    (0x002, "pit"),
    (0x006, "pit"),
    (0x009, "pit"),
    (0x004, "vlapic"),
    (0x008, "vlapic"),
    (0x00A, "vlapic"),
    (0x00B, "vpic_update_int_output"),
    (0x00C, "vpic"),
    (0x00D, "__vpic_intack"),
    (0x00E, "vpic_irq_positive_edge"),
    (0x00F, "vpic_irq_negative_edge"),
    (0x010, "vpic_ack_pending_irq"),
    (0x011, "vlapic_accept_pic_intr"),
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
