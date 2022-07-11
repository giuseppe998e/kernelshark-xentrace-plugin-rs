mod rec_info;
pub(crate) use rec_info::get_record_info_str;

mod rec_name;
pub(crate) use rec_name::get_record_name_str;

use xentrace_parser::record::{Domain, DomainKind};

pub(crate) fn get_record_task_str(domain: &Domain) -> String {
    match domain.kind() {
        DomainKind::Zero => format!("host/v{}", domain.virt_cpu()),
        DomainKind::Idle => format!("idle/v{}", domain.virt_cpu()),
        DomainKind::Default => "default/v?".to_string(),
        DomainKind::Guest(d) => format!("d{}/v{}", d, domain.virt_cpu()),
    }
}
