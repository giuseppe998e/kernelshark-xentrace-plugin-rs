use std::collections::HashMap;

use fxhash::FxBuildHasher;

const SCHED_MIN_NAMES: &[(u16, &str)] = &[
    (0x002, "sched:continue_running"),
    (0x011, "sched:running_to_runnable"),
    (0x021, "sched:running_to_blocked"),
    (0x031, "sched:running_to_offline"),
    (0x101, "sched:runnable_to_running"),
    (0x121, "sched:runnable_to_blocked"),
    (0x131, "sched:runnable_to_offline"),
    (0x201, "sched:blocked_to_running"),
    (0x211, "sched:blocked_to_runnable"),
    (0x231, "sched:blocked_to_offline"),
    (0x301, "sched:offline_to_running"),
    (0x311, "sched:offline_to_runnable"),
    (0x321, "sched:offline_to_blocked"),
];

const SCHED_CLASS_NAMES: &[(u16, &str)] = &[
    (0x001, "csched:sched_tasklet"),
    (0x002, "csched:account_start"),
    (0x003, "csched:account_stop"),
    (0x004, "csched:stolen_vcpu"),
    (0x005, "csched:picked_cpu"),
    (0x006, "csched:tickle"),
    (0x007, "csched:boost"),
    (0x008, "csched:unboost"),
    (0x009, "csched:schedule"),
    (0x00A, "csched:ratelimit"),
    (0x00B, "csched:steal_check"),
    //
    (0x201, "csched2:tick"),
    (0x202, "csched2:runq_pos"),
    (0x203, "csched2:credit"),
    (0x204, "csched2:credit_add"),
    (0x205, "csched2:tickle_check"),
    (0x206, "csched2:tickle"),
    (0x207, "csched2:credit_reset"),
    (0x208, "csched2:sched_tasklet"),
    (0x209, "csched2:update_load"),
    (0x20A, "csched2:runq_assign"),
    (0x20B, "csched2:updt_vcpu_load"),
    (0x20C, "csched2:updt_runq_load"),
    (0x20D, "csched2:tickle_new"),
    (0x20E, "csched2:runq_max_weight"),
    (0x20F, "csched2:migrrate"),
    (0x210, "csched2:load_check"),
    (0x211, "csched2:load_balance"),
    (0x212, "csched2:pick_cpu"),
    (0x213, "csched2:runq_candidate"),
    (0x214, "csched2:schedule"),
    (0x215, "csched2:ratelimit"),
    (0x216, "csched2:runq_cand_chk"),
    (0x217, "csched2:???"), // Unknown event name
    //
    (0x801, "sched/rtds:tickle"),
    (0x802, "sched/rtds:runq_pick"),
    (0x803, "sched/rtds:burn_budget"),
    (0x804, "sched/rtds:repl_budget"),
    (0x805, "sched/rtds:sched_tasklet"),
    (0x806, "sched/rtds:schedule"),
    //
    (0xA01, "sched/null:pick_cpu"),
    (0xA02, "sched/null:assign"),
    (0xA03, "sched/null:deassign"),
    (0xA04, "sched/null:migrate"),
    (0xA05, "sched/null:schedule"),
    (0xA06, "sched/null:sched_tasklet"),
];

const SCHED_VERBOSE_NAMES: &[(u16, &str)] = &[
    (0x001, "sched:sched_add_domain"),
    (0x002, "sched:sched_rem_domain"),
    (0x003, "sched:domain_sleep"),
    (0x004, "sched:domain_wake"),
    (0x005, "sched:do_yield"),
    (0x006, "sched:do_block"),
    (0x007, "sched:domain_shutdown"),
    (0x008, "sched:sched_ctl"),
    (0x009, "sched:sched_adjdom"),
    (0x00A, "sched:__enter_scheduler"),
    (0x00B, "sched:s_timer_fn"),
    (0x00C, "sched:t_timer_fn"),
    (0x00D, "sched:dom_timer_fn"),
    (0x00E, "sched:switch_infprev"),
    (0x00F, "sched:switch_infnext"),
    (0x010, "sched:domain_shutdown_code"),
    (0x011, "sched:switch_infcont"),
];

lazy_static::lazy_static! {
    static ref SCHED_MIN_MAP: HashMap<u16, &'static str, FxBuildHasher> = SCHED_MIN_NAMES.iter().copied().collect();
    static ref SCHED_CLASS_MAP: HashMap<u16, &'static str, FxBuildHasher> = SCHED_CLASS_NAMES.iter().copied().collect();
    static ref SCHED_VERBOSE_MAP: HashMap<u16, &'static str, FxBuildHasher> = SCHED_VERBOSE_NAMES.iter().copied().collect();
}

pub(super) fn get_sched_name(sub: u32, minor: u16) -> Option<&'static str> {
    match sub {
        0x1 => SCHED_MIN_MAP.get(&minor).copied(),
        0x2 => SCHED_CLASS_MAP.get(&minor).copied(),
        0x8 => SCHED_VERBOSE_MAP.get(&minor).copied(),
        _ => None,
    }
}
