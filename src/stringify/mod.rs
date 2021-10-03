mod rec_info;
mod rec_name;

pub(crate) use rec_info::get_record_info_str;
pub(crate) use rec_name::get_record_name_str;

use xentrace_parser::{DomainType, Record};

pub(crate) fn get_record_task_str(record: &Record) -> String {
    let dom = record.get_domain();
    if dom.get_type() != DomainType::Default {
        let dom_str: String = match dom.get_type() {
            DomainType::Idle => "idle".to_owned(),
            not_idle => format!("d{}", not_idle.to_id()),
        };

        format!("{}/v{}", dom_str, dom.get_vcpu())
    } else {
        "default/v?".to_owned()
    }
}
