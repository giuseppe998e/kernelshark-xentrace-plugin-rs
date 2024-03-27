use std::ptr::null_mut;

use libc::{c_int, c_long, c_short, c_ushort};

/// Kernel Shark entry contains all information from one trace record needed
/// in order to visualize the time-series of trace records. The part of the
/// data which is not directly required for the visualization (latency, record
/// info etc.) is available on-demand via the offset into the trace file.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Entry /* kshark_entry */ {
    /// Pointer to the next (in time) Entry on the same CPU core.
    pub next: *mut Entry,
    /// A bit mask controlling the visibility of the entry. A value of `OxFF`
    /// would mean that the entry is visible everywhere. Use `kshark_filter_masks`
    /// to check the level of visibility/invisibility of the entry.
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
    /// The time of the record in nano seconds. The value is taken from the
    /// timestamps within the trace data file, which are architecture dependent.
    /// The time usually is the timestamp from when the system started.
    pub ts: c_long,
}

impl Entry {
    pub fn from_ptr<'a>(ptr: *mut Self) -> Option<&'a Self> {
        unsafe { ptr.as_ref::<'a>() }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            next: null_mut(),
            visible: 0xFF, // Visible everywhere
            stream_id: Default::default(),
            event_id: Default::default(),
            cpu: Default::default(),
            pid: Default::default(),
            offset: Default::default(),
            ts: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        mem::{align_of, size_of, MaybeUninit},
        ptr::addr_of,
    };

    use super::Entry;

    #[test]
    fn bindgen_layout() {
        const UNINIT: MaybeUninit<Entry> = MaybeUninit::uninit();
        let ptr = UNINIT.as_ptr();

        assert_eq!(
            size_of::<Entry>(),
            40usize,
            concat!("Size of: ", stringify!(Entry))
        );
        assert_eq!(
            align_of::<Entry>(),
            8usize,
            concat!("Alignment of ", stringify!(Entry))
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).next) as usize - ptr as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Entry),
                "::",
                stringify!(next)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).visible) as usize - ptr as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(Entry),
                "::",
                stringify!(visible)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).stream_id) as usize - ptr as usize },
            10usize,
            concat!(
                "Offset of field: ",
                stringify!(Entry),
                "::",
                stringify!(stream_id)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).event_id) as usize - ptr as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(Entry),
                "::",
                stringify!(event_id)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).cpu) as usize - ptr as usize },
            14usize,
            concat!(
                "Offset of field: ",
                stringify!(Entry),
                "::",
                stringify!(cpu)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).pid) as usize - ptr as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(Entry),
                "::",
                stringify!(pid)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).offset) as usize - ptr as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(Entry),
                "::",
                stringify!(offset)
            )
        );
        assert_eq!(
            unsafe { addr_of!((*ptr).ts) as usize - ptr as usize },
            32usize,
            concat!("Offset of field: ", stringify!(Entry), "::", stringify!(ts))
        );
    }
}
