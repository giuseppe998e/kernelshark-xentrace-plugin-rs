use libc::c_uint;

// Constants from "include/public/trace.h"
pub(crate) const TRC_CLASS: c_uint = 0x0ffff000;
pub(crate) const TRC_GEN: c_uint = 0x0001f000; // General trace
pub(crate) const TRC_SCHED: c_uint = 0x0002f000; // Xen Scheduler trace
pub(crate) const TRC_DOM0OP: c_uint = 0x0004f000; // Xen DOM0 operation trace
pub(crate) const TRC_HVM: c_uint = 0x0008f000; // Xen HVM trace
pub(crate) const TRC_MEM: c_uint = 0x0010f000; // Xen memory trace
pub(crate) const TRC_PV: c_uint = 0x0020f000; // Xen PV traces
pub(crate) const TRC_SHADOW: c_uint = 0x0040f000; // Xen shadow tracing
pub(crate) const TRC_HW: c_uint = 0x0080f000; // Xen hardware-related traces
pub(crate) const TRC_GUEST: c_uint = 0x0800f000; // Guest-generated traces

pub(crate) const TRC_SUBCLS: c_uint = 0x0000ffff;
