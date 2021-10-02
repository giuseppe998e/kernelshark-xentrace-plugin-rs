use xentrace_parser::Record;

pub(crate) fn get_record_info_str(record: &Record) -> String {
    format!("{:?}", record.get_event().get_extra())
}
