use libc::{c_int, c_long, c_short, c_ushort};
use std::ptr::null;

/// Kernel Shark entry contains all information from one trace record needed
/// in order to  visualize the time-series of trace records. The part of the
/// data which is not directly required for the visualization (latency, record
/// info etc.) is available on-demand via the offset into the trace file.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Entry /* kshark_entry */ {
    /// Pointer to the next (in time) kshark_entry on the same CPU core.
    pub next: *const Entry,
    /// A bit mask controlling the visibility of the entry. A value of OxFF
    /// would mean that the entry is visible everywhere. Use
    /// kshark_filter_masks to check the level of visibility/invisibility
    /// of the entry.
    pub visible: c_ushort,
    /// Data stream identifier.
    pub stream_id: c_short,
    /// Unique Id of the trace event type.
    pub event_id: c_short,
    /// The CPU core of the record.
    pub cpu: c_short,
    /// The PID of the task the record was generated.
    pub pid: c_int,
    /// The offset into the trace file, used to find the record.
    pub offset: c_long,
    /// The time of the record in nano seconds. The value is taken from
    /// the timestamps within the trace data file, which are architecture
    /// dependent. The time usually is the timestamp from when the system
    /// started.
    pub ts: c_long,
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            next: null::<Entry>(),
            ..Default::default()
        }
    }
}
