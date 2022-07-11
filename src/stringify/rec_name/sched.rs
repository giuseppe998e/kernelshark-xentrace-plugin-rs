use xentrace_parser::record::EventCode;

fn get_sched_min_name_str<'a>(ecode: &EventCode) -> Option<&'a str> {
    match ecode.minor() {
        0x002 => Some("continue_running"),
        0x011 => Some("running_to_runnable"),
        0x021 => Some("running_to_blocked"),
        0x031 => Some("running_to_offline"),
        0x101 => Some("runnable_to_running"),
        0x121 => Some("runnable_to_blocked"),
        0x131 => Some("runnable_to_offline"),
        0x201 => Some("blocked_to_running"),
        0x211 => Some("blocked_to_runnable"),
        0x231 => Some("blocked_to_offline"),
        0x301 => Some("offline_to_running"),
        0x311 => Some("offline_to_runnable"),
        0x321 => Some("offline_to_blocked"),
        _ => None,
    }
}

fn get_sched_class_name_str<'a>(ecode: &EventCode) -> Option<&'a str> {
    match ecode.minor() {
        0x001 => Some("csched:sched_tasklet"),
        0x002 => Some("csched:account_start"),
        0x003 => Some("csched:account_stop"),
        0x004 => Some("csched:stolen_vcpu"),
        0x005 => Some("csched:picked_cpu"),
        0x006 => Some("csched:tickle"),
        0x007 => Some("csched:boost"),
        0x008 => Some("csched:unboost"),
        0x009 => Some("csched:schedule"),
        0x00A => Some("csched:ratelimit"),
        0x00B => Some("csched:steal_check"),

        0x201 => Some("csched2:tick"),
        0x202 => Some("csched2:runq_pos"),
        0x203 => Some("csched2:credit"),
        0x204 => Some("csched2:credit_add"),
        0x205 => Some("csched2:tickle_check"),
        0x206 => Some("csched2:tickle"),
        0x207 => Some("csched2:credit_reset"),
        0x208 => Some("csched2:sched_tasklet"),
        0x209 => Some("csched2:update_load"),
        0x20A => Some("csched2:runq_assign"),
        0x20B => Some("csched2:updt_vcpu_load"),
        0x20C => Some("csched2:updt_runq_load"),
        0x20D => Some("csched2:tickle_new"),
        0x20E => Some("csched2:runq_max_weight"),
        0x20F => Some("csched2:migrrate"),
        0x210 => Some("csched2:load_check"),
        0x211 => Some("csched2:load_balance"),
        0x212 => Some("csched2:pick_cpu"),
        0x213 => Some("csched2:runq_candidate"),
        0x214 => Some("csched2:schedule"),
        0x215 => Some("csched2:ratelimit"),
        0x216 => Some("csched2:runq_cand_chk"),

        0x801 => Some("rtds:tickle"),
        0x802 => Some("rtds:runq_pick"),
        0x803 => Some("rtds:burn_budget"),
        0x804 => Some("rtds:repl_budget"),
        0x805 => Some("rtds:sched_tasklet"),
        0x806 => Some("rtds:schedule"),

        0xA01 => Some("null:pick_cpu"),
        0xA02 => Some("null:assign"),
        0xA03 => Some("null:deassign"),
        0xA04 => Some("null:migrate"),
        0xA05 => Some("null:schedule"),
        0xA06 => Some("null:sched_tasklet"),
        _ => None,
    }
}

fn get_sched_verbose_name_str<'a>(ecode: &EventCode) -> Option<&'a str> {
    match ecode.minor() {
        0x001 => Some("sched_add_domain"),
        0x002 => Some("sched_rem_domain"),
        0x003 => Some("domain_sleep"),
        0x004 => Some("domain_wake"),
        0x005 => Some("do_yield"),
        0x006 => Some("do_block"),
        0x007 => Some("domain_shutdown"),
        0x008 => Some("sched_ctl"),
        0x009 => Some("sched_adjdom"),
        0x00A => Some("__enter_scheduler"),
        0x00B => Some("s_timer_fn"),
        0x00C => Some("t_timer_fn"),
        0x00D => Some("dom_timer_fn"),
        0x00E => Some("switch_infprev"),
        0x00F => Some("switch_infnext"),
        0x010 => Some("domain_shutdown_code"),
        0x011 => Some("switch_infcont"),
        _ => None,
    }
}

pub(super) fn get_sched_name_str<'a>(ecode: &EventCode) -> Option<&'a str> {
    match ecode.sub() {
        0x1 => get_sched_min_name_str(ecode),
        0x2 => get_sched_class_name_str(ecode),
        0x8 => get_sched_verbose_name_str(ecode),
        _ => None,
    }
}
