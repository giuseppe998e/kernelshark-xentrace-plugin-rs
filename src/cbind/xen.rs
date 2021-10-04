use libc::c_ushort;

// Constants from "include/public/trace.h"
pub(crate) const TRC_GEN: c_ushort = 0x0001; // General trace
pub(crate) const TRC_SCHED: c_ushort = 0x0002; // Xen Scheduler trace
pub(crate) const TRC_DOM0OP: c_ushort = 0x0004; // Xen DOM0 operation trace
pub(crate) const TRC_HVM: c_ushort = 0x0008; // Xen HVM trace
pub(crate) const TRC_MEM: c_ushort = 0x0010; // Xen memory trace
pub(crate) const TRC_PV: c_ushort = 0x0020; // Xen PV traces
pub(crate) const TRC_SHADOW: c_ushort = 0x0040; // Xen shadow tracing
pub(crate) const TRC_HW: c_ushort = 0x0080; // Xen hardware-related traces
pub(crate) const TRC_GUEST: c_ushort = 0x0800; // Guest-generated traces
