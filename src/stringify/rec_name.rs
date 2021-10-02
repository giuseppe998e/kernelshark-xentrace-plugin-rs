use xentrace_parser::Record;

pub(crate) fn get_record_name_str(record: &Record) -> String {
    format!("{:#010X}", record.get_event().get_code())
}
