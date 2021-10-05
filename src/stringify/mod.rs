mod rec_info;
mod rec_name;

pub(crate) use rec_info::get_record_info_str;
pub(crate) use rec_name::get_record_name_str;

use xentrace_parser::{Domain, DomainType};

pub(crate) fn get_record_task_str(domain: &Domain) -> String {
    if domain.get_type() == DomainType::Default {
        return "default/v?".to_owned();
    }

    let dom_str: String = match domain.get_type() {
        DomainType::Idle => "idle".to_owned(),
        not_idle => format!("d{}", not_idle.into_id()),
    };

    format!("{}/v{}", dom_str, domain.get_vcpu())
}
