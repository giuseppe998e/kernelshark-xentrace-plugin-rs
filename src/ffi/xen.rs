pub mod trace {
    // FFI derived from https://github.com/xen-project/xen/blob/f5d56f4b253072264efc0fece698a91779e362f5/xen/include/public/trace.h
    use libc::c_uint;

    // Constants
    pub const TRC_GEN: c_uint = 0x0001; // General trace
    pub const TRC_SCHED: c_uint = 0x0002; // Xen Scheduler trace
    pub const TRC_DOM0OP: c_uint = 0x0004; // Xen DOM0 operation trace
    pub const TRC_HVM: c_uint = 0x0008; // Xen HVM trace
    pub const TRC_MEM: c_uint = 0x0010; // Xen memory trace
    pub const TRC_PV: c_uint = 0x0020; // Xen PV traces
    pub const TRC_SHADOW: c_uint = 0x0040; // Xen shadow tracing
    pub const TRC_HW: c_uint = 0x0080; // Xen hardware-related traces
    pub const TRC_GUEST: c_uint = 0x0800; // Guest-generated traces
}
