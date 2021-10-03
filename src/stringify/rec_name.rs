use xentrace_parser::Event;

pub(crate) fn get_record_name_str(event: &Event) -> String {
    format!("{:#010X}", event.get_code())
}
