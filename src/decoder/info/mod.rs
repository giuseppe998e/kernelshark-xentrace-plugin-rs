use xentrace_parser::record::Event;

pub(crate) fn get_record_info(event: &Event) -> String {
    event
        .extra()
        .iter()
        .filter_map(|opt| opt.map(|val| format!("0x{val:08X}")))
        .collect::<Vec<_>>()
        .join(", ")
}
