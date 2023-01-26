mod info;
mod name;
pub(crate) mod timestamp;

use xentrace_parser::record::{Domain, DomainKind};

pub(crate) use info::get_record_info;
pub(crate) use name::get_record_name;

pub(crate) fn get_record_task(domain: &Domain) -> String {
    match domain.kind() {
        DomainKind::Zero => format!("host/v{}", domain.virtual_cpu()),
        DomainKind::Idle => format!("idle/v{}", domain.virtual_cpu()),
        DomainKind::Default => "default/v?".to_string(),
        DomainKind::Guest(dom) => format!("d{}/v{}", dom, domain.virtual_cpu()),
    }
}
