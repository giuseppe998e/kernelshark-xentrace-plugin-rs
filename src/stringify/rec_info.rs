use xentrace_parser::Event;

pub(crate) fn get_record_info_str(event: &Event) -> String {
    format!("{:?}", event.get_extra())
}
