mod rec_info;
pub(crate) use rec_info::get_record_info_str;

mod rec_name;
pub(crate) use rec_name::get_record_name_str;

use xentrace_parser::record::{Domain, DomainType};

pub(crate) fn get_record_task_str(domain: &Domain) -> String {
    match domain.type_ {
        DomainType::Zero => format!("host/v{}", domain.vcpu),
        DomainType::Idle => format!("idle/v{}", domain.vcpu),
        DomainType::Default => "default/v?".to_string(),
        DomainType::Guest(d) => format!("d{}/v{}", d, domain.vcpu),
    }
}
