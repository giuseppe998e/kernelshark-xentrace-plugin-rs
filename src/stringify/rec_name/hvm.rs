use xentrace_parser::record::EventCode;

fn get_hvm_entryexit_name_str<'a>(ecode: EventCode) -> Option<&'a str> {
    match ecode.get_minor() {
        0x001 => Some("VMENTRY"),
        0x002 | 0x102 => Some("VMEXIT"),
        0x401 => Some("nVMENTRY"),
        0x402 | 0x502 => Some("nVMEXIT"),
        _ => None,
    }
}

fn get_hvm_handler_name_str<'a>(ecode: EventCode) -> Option<&'a str> {
    match ecode.get_minor() {
        0x001 | 0x101 => Some("PF_XEN"),
        0x002 | 0x102 => Some("PF_INJECT"),
        0x003 => Some("INJ_EXC"),
        0x004 => Some("INJ_VIRQ"),
        0x005 => Some("REINJ_VIRQ"),
        0x006 => Some("IO_READ"),
        0x007 => Some("IO_WRITE"),
        0x008 | 0x108 => Some("CR_READ"),
        0x009 | 0x109 => Some("CR_WRITE"),
        0x00A => Some("DR_READ"),
        0x00B => Some("DR_WRITE"),
        0x00C => Some("MSR_READ"),
        0x00D => Some("MSR_WRITE"),
        0x00E => Some("CPUID"),
        0x00F => Some("INTR"),
        0x010 => Some("NMI"),
        0x011 => Some("SMI"),
        0x012 => Some("VMMCALL"),
        0x013 => Some("HLT"),
        0x014 | 0x114 => Some("INVLPG"),
        0x015 => Some("MCE"),
        0x016 => Some("IOPORT_READ"),
        0x216 => Some("IOPORT_WRITE"),
        0x017 => Some("MMIO_READ"),
        0x217 => Some("MMIO_WRITE"),
        0x018 => Some("CLTS"),
        0x019 | 0x119 => Some("LMSW"),
        0x01a => Some("RDTSC"),
        0x020 => Some("INTR_WINDOW"),
        0x021 => Some("NPF"),
        0x023 => Some("TRAP"),
        _ => None,
    }
}

fn get_hvm_emul_name_str<'a>(ecode: EventCode) -> Option<&'a str> {
    match ecode.get_minor() {
        0x001 | 0x005 => Some("hpet"),
        0x003 | 0x007 => Some("rtc"),
        0x002 | 0x006 | 0x009 => Some("pit"),
        0x004 | 0x008 | 0x00a => Some("vlapic"),
        0x00b => Some("vpic_update_int_output"),
        0x00c => Some("vpic"),
        0x00d => Some("__vpic_intack"),
        0x00e => Some("vpic_irq_positive_edge"),
        0x00f => Some("vpic_irq_negative_edge"),
        0x010 => Some("vpic_ack_pending_irq"),
        0x011 => Some("vlapic_accept_pic_intr"),
        _ => None,
    }
}

pub(super) fn get_hvm_name_str<'a>(ecode: EventCode) -> Option<&'a str> {
    match ecode.get_sub() {
        0x1 => get_hvm_entryexit_name_str(ecode),
        0x2 => get_hvm_handler_name_str(ecode),
        0x4 => get_hvm_emul_name_str(ecode),
        _ => None,
    }
}
