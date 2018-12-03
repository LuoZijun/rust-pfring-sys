use crate::{ __BindgenBitfieldUnit, __IncompleteArrayField, };
use crate::pfring::{ pfring, pfring_stat, };

use crate::linux::pf_ring::{ pfring_pkthdr, socket_mode, };

use crate::libc::{ c_uint, uint8_t, uint16_t, uint32_t, uint64_t, c_uchar, };


pub const RING_BUF_SIZE: u32 = 8388608;
pub const SYSDIG_RING_LEN: u32 = 16777216;
pub const SYSDIG_DEFAULT_DATA_AVAIL: u32 = 100000;
pub const BUFFER_EMPTY_WAIT_TIME_MS: u32 = 30;

pub const SYSDIG_MAX_NUM_DEVICES: u32 = 64;

pub const SYSDIG_MAX_NAME_LEN: u32 = 32;
pub const SYSDIG_MAX_EVENT_PARAMS: u32 = 16;

pub const SYSDIG_IOCTL_MAGIC: u8 = 115u8;
// #define SYSDIG_IOCTL_DISABLE_CAPTURE       _IO(SYSDIG_IOCTL_MAGIC, 0)
// #define SYSDIG_IOCTL_ENABLE_CAPTURE        _IO(SYSDIG_IOCTL_MAGIC, 1)
// #define SYSDIG_IOCTL_DISABLE_DROPPING_MODE _IO(SYSDIG_IOCTL_MAGIC, 2)
// #define SYSDIG_IOCTL_ENABLE_DROPPING_MODE  _IO(SYSDIG_IOCTL_MAGIC, 3)
// #define SYSDIG_IOCTL_SET_SNAPLEN           _IO(SYSDIG_IOCTL_MAGIC, 4)
// #define SYSDIG_IOCTL_MASK_ZERO_EVENTS      _IO(SYSDIG_IOCTL_MAGIC, 5)
// #define SYSDIG_IOCTL_MASK_SET_EVENT        _IO(SYSDIG_IOCTL_MAGIC, 6)
// #define SYSDIG_IOCTL_MASK_UNSET_EVENT      _IO(SYSDIG_IOCTL_MAGIC, 7)

pub type sysdig_event_type = u32;

pub const SYSDIG_GENERIC_E: sysdig_event_type = 0;
pub const SYSDIG_GENERIC_X: sysdig_event_type = 1;
pub const SYSDIG_SYSCALL_OPEN_E: sysdig_event_type = 2;
pub const SYSDIG_SYSCALL_OPEN_X: sysdig_event_type = 3;
pub const SYSDIG_SYSCALL_CLOSE_E: sysdig_event_type = 4;
pub const SYSDIG_SYSCALL_CLOSE_X: sysdig_event_type = 5;
pub const SYSDIG_SYSCALL_READ_E: sysdig_event_type = 6;
pub const SYSDIG_SYSCALL_READ_X: sysdig_event_type = 7;
pub const SYSDIG_SYSCALL_WRITE_E: sysdig_event_type = 8;
pub const SYSDIG_SYSCALL_WRITE_X: sysdig_event_type = 9;
pub const SYSDIG_SYSCALL_BRK_1_E: sysdig_event_type = 10;
pub const SYSDIG_SYSCALL_BRK_1_X: sysdig_event_type = 11;
pub const SYSDIG_SYSCALL_EXECVE_8_E: sysdig_event_type = 12;
pub const SYSDIG_SYSCALL_EXECVE_8_X: sysdig_event_type = 13;
pub const SYSDIG_CLONE_11_E: sysdig_event_type = 14;
pub const SYSDIG_CLONE_11_X: sysdig_event_type = 15;
pub const SYSDIG_PROCEXIT_E: sysdig_event_type = 16;
pub const SYSDIG_PROCEXIT_X: sysdig_event_type = 17;
pub const SYSDIG_SOCKET_SOCKET_E: sysdig_event_type = 18;
pub const SYSDIG_SOCKET_SOCKET_X: sysdig_event_type = 19;
pub const SYSDIG_SOCKET_BIND_E: sysdig_event_type = 20;
pub const SYSDIG_SOCKET_BIND_X: sysdig_event_type = 21;
pub const SYSDIG_SOCKET_CONNECT_E: sysdig_event_type = 22;
pub const SYSDIG_SOCKET_CONNECT_X: sysdig_event_type = 23;
pub const SYSDIG_SOCKET_LISTEN_E: sysdig_event_type = 24;
pub const SYSDIG_SOCKET_LISTEN_X: sysdig_event_type = 25;
pub const SYSDIG_SOCKET_ACCEPT_E: sysdig_event_type = 26;
pub const SYSDIG_SOCKET_ACCEPT_X: sysdig_event_type = 27;
pub const SYSDIG_SOCKET_SEND_E: sysdig_event_type = 28;
pub const SYSDIG_SOCKET_SEND_X: sysdig_event_type = 29;
pub const SYSDIG_SOCKET_SENDTO_E: sysdig_event_type = 30;
pub const SYSDIG_SOCKET_SENDTO_X: sysdig_event_type = 31;
pub const SYSDIG_SOCKET_RECV_E: sysdig_event_type = 32;
pub const SYSDIG_SOCKET_RECV_X: sysdig_event_type = 33;
pub const SYSDIG_SOCKET_RECVFROM_E: sysdig_event_type = 34;
pub const SYSDIG_SOCKET_RECVFROM_X: sysdig_event_type = 35;
pub const SYSDIG_SOCKET_SHUTDOWN_E: sysdig_event_type = 36;
pub const SYSDIG_SOCKET_SHUTDOWN_X: sysdig_event_type = 37;
pub const SYSDIG_SOCKET_GETSOCKNAME_E: sysdig_event_type = 38;
pub const SYSDIG_SOCKET_GETSOCKNAME_X: sysdig_event_type = 39;
pub const SYSDIG_SOCKET_GETPEERNAME_E: sysdig_event_type = 40;
pub const SYSDIG_SOCKET_GETPEERNAME_X: sysdig_event_type = 41;
pub const SYSDIG_SOCKET_SOCKETPAIR_E: sysdig_event_type = 42;
pub const SYSDIG_SOCKET_SOCKETPAIR_X: sysdig_event_type = 43;
pub const SYSDIG_SOCKET_SETSOCKOPT_E: sysdig_event_type = 44;
pub const SYSDIG_SOCKET_SETSOCKOPT_X: sysdig_event_type = 45;
pub const SYSDIG_SOCKET_GETSOCKOPT_E: sysdig_event_type = 46;
pub const SYSDIG_SOCKET_GETSOCKOPT_X: sysdig_event_type = 47;
pub const SYSDIG_SOCKET_SENDMSG_E: sysdig_event_type = 48;
pub const SYSDIG_SOCKET_SENDMSG_X: sysdig_event_type = 49;
pub const SYSDIG_SOCKET_SENDMMSG_E: sysdig_event_type = 50;
pub const SYSDIG_SOCKET_SENDMMSG_X: sysdig_event_type = 51;
pub const SYSDIG_SOCKET_RECVMSG_E: sysdig_event_type = 52;
pub const SYSDIG_SOCKET_RECVMSG_X: sysdig_event_type = 53;
pub const SYSDIG_SOCKET_RECVMMSG_E: sysdig_event_type = 54;
pub const SYSDIG_SOCKET_RECVMMSG_X: sysdig_event_type = 55;
pub const SYSDIG_SOCKET_ACCEPT4_E: sysdig_event_type = 56;
pub const SYSDIG_SOCKET_ACCEPT4_X: sysdig_event_type = 57;
pub const SYSDIG_SYSCALL_CREAT_E: sysdig_event_type = 58;
pub const SYSDIG_SYSCALL_CREAT_X: sysdig_event_type = 59;
pub const SYSDIG_SYSCALL_PIPE_E: sysdig_event_type = 60;
pub const SYSDIG_SYSCALL_PIPE_X: sysdig_event_type = 61;
pub const SYSDIG_SYSCALL_EVENTFD_E: sysdig_event_type = 62;
pub const SYSDIG_SYSCALL_EVENTFD_X: sysdig_event_type = 63;
pub const SYSDIG_SYSCALL_FUTEX_E: sysdig_event_type = 64;
pub const SYSDIG_SYSCALL_FUTEX_X: sysdig_event_type = 65;
pub const SYSDIG_SYSCALL_STAT_E: sysdig_event_type = 66;
pub const SYSDIG_SYSCALL_STAT_X: sysdig_event_type = 67;
pub const SYSDIG_SYSCALL_LSTAT_E: sysdig_event_type = 68;
pub const SYSDIG_SYSCALL_LSTAT_X: sysdig_event_type = 69;
pub const SYSDIG_SYSCALL_FSTAT_E: sysdig_event_type = 70;
pub const SYSDIG_SYSCALL_FSTAT_X: sysdig_event_type = 71;
pub const SYSDIG_SYSCALL_STAT64_E: sysdig_event_type = 72;
pub const SYSDIG_SYSCALL_STAT64_X: sysdig_event_type = 73;
pub const SYSDIG_SYSCALL_LSTAT64_E: sysdig_event_type = 74;
pub const SYSDIG_SYSCALL_LSTAT64_X: sysdig_event_type = 75;
pub const SYSDIG_SYSCALL_FSTAT64_E: sysdig_event_type = 76;
pub const SYSDIG_SYSCALL_FSTAT64_X: sysdig_event_type = 77;
pub const SYSDIG_SYSCALL_EPOLLWAIT_E: sysdig_event_type = 78;
pub const SYSDIG_SYSCALL_EPOLLWAIT_X: sysdig_event_type = 79;
pub const SYSDIG_SYSCALL_POLL_E: sysdig_event_type = 80;
pub const SYSDIG_SYSCALL_POLL_X: sysdig_event_type = 81;
pub const SYSDIG_SYSCALL_SELECT_E: sysdig_event_type = 82;
pub const SYSDIG_SYSCALL_SELECT_X: sysdig_event_type = 83;
pub const SYSDIG_SYSCALL_NEWSELECT_E: sysdig_event_type = 84;
pub const SYSDIG_SYSCALL_NEWSELECT_X: sysdig_event_type = 85;
pub const SYSDIG_SYSCALL_LSEEK_E: sysdig_event_type = 86;
pub const SYSDIG_SYSCALL_LSEEK_X: sysdig_event_type = 87;
pub const SYSDIG_SYSCALL_LLSEEK_E: sysdig_event_type = 88;
pub const SYSDIG_SYSCALL_LLSEEK_X: sysdig_event_type = 89;
pub const SYSDIG_SYSCALL_IOCTL_E: sysdig_event_type = 90;
pub const SYSDIG_SYSCALL_IOCTL_X: sysdig_event_type = 91;
pub const SYSDIG_SYSCALL_GETCWD_E: sysdig_event_type = 92;
pub const SYSDIG_SYSCALL_GETCWD_X: sysdig_event_type = 93;
pub const SYSDIG_SYSCALL_CHDIR_E: sysdig_event_type = 94;
pub const SYSDIG_SYSCALL_CHDIR_X: sysdig_event_type = 95;
pub const SYSDIG_SYSCALL_FCHDIR_E: sysdig_event_type = 96;
pub const SYSDIG_SYSCALL_FCHDIR_X: sysdig_event_type = 97;
pub const SYSDIG_SYSCALL_MKDIR_E: sysdig_event_type = 98;
pub const SYSDIG_SYSCALL_MKDIR_X: sysdig_event_type = 99;
pub const SYSDIG_SYSCALL_RMDIR_E: sysdig_event_type = 100;
pub const SYSDIG_SYSCALL_RMDIR_X: sysdig_event_type = 101;
pub const SYSDIG_SYSCALL_OPENAT_E: sysdig_event_type = 102;
pub const SYSDIG_SYSCALL_OPENAT_X: sysdig_event_type = 103;
pub const SYSDIG_SYSCALL_LINK_E: sysdig_event_type = 104;
pub const SYSDIG_SYSCALL_LINK_X: sysdig_event_type = 105;
pub const SYSDIG_SYSCALL_LINKAT_E: sysdig_event_type = 106;
pub const SYSDIG_SYSCALL_LINKAT_X: sysdig_event_type = 107;
pub const SYSDIG_SYSCALL_UNLINK_E: sysdig_event_type = 108;
pub const SYSDIG_SYSCALL_UNLINK_X: sysdig_event_type = 109;
pub const SYSDIG_SYSCALL_UNLINKAT_E: sysdig_event_type = 110;
pub const SYSDIG_SYSCALL_UNLINKAT_X: sysdig_event_type = 111;
pub const SYSDIG_SYSCALL_PREAD_E: sysdig_event_type = 112;
pub const SYSDIG_SYSCALL_PREAD_X: sysdig_event_type = 113;
pub const SYSDIG_SYSCALL_PWRITE_E: sysdig_event_type = 114;
pub const SYSDIG_SYSCALL_PWRITE_X: sysdig_event_type = 115;
pub const SYSDIG_SYSCALL_READV_E: sysdig_event_type = 116;
pub const SYSDIG_SYSCALL_READV_X: sysdig_event_type = 117;
pub const SYSDIG_SYSCALL_WRITEV_E: sysdig_event_type = 118;
pub const SYSDIG_SYSCALL_WRITEV_X: sysdig_event_type = 119;
pub const SYSDIG_SYSCALL_PREADV_E: sysdig_event_type = 120;
pub const SYSDIG_SYSCALL_PREADV_X: sysdig_event_type = 121;
pub const SYSDIG_SYSCALL_PWRITEV_E: sysdig_event_type = 122;
pub const SYSDIG_SYSCALL_PWRITEV_X: sysdig_event_type = 123;
pub const SYSDIG_SYSCALL_DUP_E: sysdig_event_type = 124;
pub const SYSDIG_SYSCALL_DUP_X: sysdig_event_type = 125;
pub const SYSDIG_SYSCALL_SIGNALFD_E: sysdig_event_type = 126;
pub const SYSDIG_SYSCALL_SIGNALFD_X: sysdig_event_type = 127;
pub const SYSDIG_SYSCALL_KILL_E: sysdig_event_type = 128;
pub const SYSDIG_SYSCALL_KILL_X: sysdig_event_type = 129;
pub const SYSDIG_SYSCALL_TKILL_E: sysdig_event_type = 130;
pub const SYSDIG_SYSCALL_TKILL_X: sysdig_event_type = 131;
pub const SYSDIG_SYSCALL_TGKILL_E: sysdig_event_type = 132;
pub const SYSDIG_SYSCALL_TGKILL_X: sysdig_event_type = 133;
pub const SYSDIG_SYSCALL_NANOSLEEP_E: sysdig_event_type = 134;
pub const SYSDIG_SYSCALL_NANOSLEEP_X: sysdig_event_type = 135;
pub const SYSDIG_SYSCALL_TIMERFD_CREATE_E: sysdig_event_type = 136;
pub const SYSDIG_SYSCALL_TIMERFD_CREATE_X: sysdig_event_type = 137;
pub const SYSDIG_SYSCALL_INOTIFY_INIT_E: sysdig_event_type = 138;
pub const SYSDIG_SYSCALL_INOTIFY_INIT_X: sysdig_event_type = 139;
pub const SYSDIG_SYSCALL_GETRLIMIT_E: sysdig_event_type = 140;
pub const SYSDIG_SYSCALL_GETRLIMIT_X: sysdig_event_type = 141;
pub const SYSDIG_SYSCALL_SETRLIMIT_E: sysdig_event_type = 142;
pub const SYSDIG_SYSCALL_SETRLIMIT_X: sysdig_event_type = 143;
pub const SYSDIG_SYSCALL_PRLIMIT_E: sysdig_event_type = 144;
pub const SYSDIG_SYSCALL_PRLIMIT_X: sysdig_event_type = 145;
pub const SYSDIG_SCHEDSWITCH_1_E: sysdig_event_type = 146;
pub const SYSDIG_SCHEDSWITCH_1_X: sysdig_event_type = 147;
pub const SYSDIG_DROP_E: sysdig_event_type = 148;
pub const SYSDIG_DROP_X: sysdig_event_type = 149;
pub const SYSDIG_SYSCALL_FCNTL_E: sysdig_event_type = 150;
pub const SYSDIG_SYSCALL_FCNTL_X: sysdig_event_type = 151;
pub const SYSDIG_SCHEDSWITCH_6_E: sysdig_event_type = 152;
pub const SYSDIG_SCHEDSWITCH_6_X: sysdig_event_type = 153;
pub const SYSDIG_SYSCALL_EXECVE_13_E: sysdig_event_type = 154;
pub const SYSDIG_SYSCALL_EXECVE_13_X: sysdig_event_type = 155;
pub const SYSDIG_CLONE_16_E: sysdig_event_type = 156;
pub const SYSDIG_CLONE_16_X: sysdig_event_type = 157;
pub const SYSDIG_SYSCALL_BRK_4_E: sysdig_event_type = 158;
pub const SYSDIG_SYSCALL_BRK_4_X: sysdig_event_type = 159;
pub const SYSDIG_SYSCALL_MMAP_E: sysdig_event_type = 160;
pub const SYSDIG_SYSCALL_MMAP_X: sysdig_event_type = 161;
pub const SYSDIG_SYSCALL_MMAP2_E: sysdig_event_type = 162;
pub const SYSDIG_SYSCALL_MMAP2_X: sysdig_event_type = 163;
pub const SYSDIG_SYSCALL_MUNMAP_E: sysdig_event_type = 164;
pub const SYSDIG_SYSCALL_MUNMAP_X: sysdig_event_type = 165;
pub const SYSDIG_SYSCALL_SPLICE_E: sysdig_event_type = 166;
pub const SYSDIG_SYSCALL_SPLICE_X: sysdig_event_type = 167;
pub const SYSDIG_EVENT_MAX: sysdig_event_type = 168;


pub type sysdig_param_type = u32;
pub const SYSDIG_TYPE_NONE: sysdig_param_type = 0;
pub const SYSDIG_TYPE_INT8: sysdig_param_type = 1;
pub const SYSDIG_TYPE_INT16: sysdig_param_type = 2;
pub const SYSDIG_TYPE_INT32: sysdig_param_type = 3;
pub const SYSDIG_TYPE_INT64: sysdig_param_type = 4;
pub const SYSDIG_TYPE_UINT8: sysdig_param_type = 5;
pub const SYSDIG_TYPE_UINT16: sysdig_param_type = 6;
pub const SYSDIG_TYPE_UINT32: sysdig_param_type = 7;
pub const SYSDIG_TYPE_UINT64: sysdig_param_type = 8;
pub const SYSDIG_TYPE_CHARBUF: sysdig_param_type = 9;
pub const SYSDIG_TYPE_BYTEBUF: sysdig_param_type = 10;
pub const SYSDIG_TYPE_ERRNO: sysdig_param_type = 11;
pub const SYSDIG_TYPE_SOCKADDR: sysdig_param_type = 12;
pub const SYSDIG_TYPE_SOCKTUPLE: sysdig_param_type = 13;
pub const SYSDIG_TYPE_FD: sysdig_param_type = 14;
pub const SYSDIG_TYPE_PID: sysdig_param_type = 15;
pub const SYSDIG_TYPE_FDLIST: sysdig_param_type = 16;
pub const SYSDIG_TYPE_FSPATH: sysdig_param_type = 17;
pub const SYSDIG_TYPE_SYSCALLID: sysdig_param_type = 18;
pub const SYSDIG_TYPE_SIGTYPE: sysdig_param_type = 19;
pub const SYSDIG_TYPE_RELTIME: sysdig_param_type = 20;
pub const SYSDIG_TYPE_ABSTIME: sysdig_param_type = 21;
pub const SYSDIG_TYPE_PORT: sysdig_param_type = 22;
pub const SYSDIG_TYPE_L4PROTO: sysdig_param_type = 23;
pub const SYSDIG_TYPE_SOCKFAMILY: sysdig_param_type = 24;
pub const SYSDIG_TYPE_BOOL: sysdig_param_type = 25;
pub const SYSDIG_TYPE_IPV4ADDR: sysdig_param_type = 26;
pub const SYSDIG_TYPE_DYN: sysdig_param_type = 27;
pub const SYSDIG_TYPE_FLAGS8: sysdig_param_type = 28;
pub const SYSDIG_TYPE_FLAGS16: sysdig_param_type = 29;
pub const SYSDIG_TYPE_FLAGS32: sysdig_param_type = 30;

pub type sysdig_print_format = u32;
pub const SYSDIG_PRINT_FORMAT_NA: sysdig_print_format = 0;
pub const SYSDIG_PRINT_FORMAT_DEC: sysdig_print_format = 1;
pub const SYSDIG_PRINT_FORMAT_HEX: sysdig_print_format = 2;
pub const SYSDIG_PRINT_FORMAT_10_PADDED_DEC: sysdig_print_format = 3;

pub type sysdig_syscall_mode = u32;
pub const SYSDIG_ENTER: sysdig_syscall_mode = 0;
pub const SYSDIG_EXIT: sysdig_syscall_mode = 1;



#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sysdig_param_info {
    #[doc = "< Paramter name, e.g. \'size\'."]
    pub name: [::std::os::raw::c_char; 32usize],
    #[doc = "< Paramter type, e.g. \'u_int16\', \'string\'..."]
    pub type_: sysdig_param_type,
    #[doc = "< If this is a numeric parameter, this flag specifies if it should be rendered as decimal or hex."]
    pub fmt: sysdig_print_format,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sysdig_event_info {
    #[doc = "< Event mode (enter or exit)."]
    pub mode: sysdig_syscall_mode,
    #[doc = "< Name."]
    pub name: [::std::os::raw::c_char; 32usize],
    #[doc = "< Number of parameter in the params array."]
    pub nparams: uint32_t,
    #[doc = "< parameters descriptions."]
    pub params: [sysdig_param_info; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sysdig_ring_info {
    pub head: uint32_t,
    pub tail: uint32_t,
    pub n_evts: uint64_t,
    pub n_drops_buffer: uint64_t,
    pub n_drops_pf: uint64_t,
    pub n_preemptions: uint64_t,
    pub n_context_switches: uint64_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_sysdig_device {
    pub fd: ::std::os::raw::c_int,
    pub ring_mmap: *mut ::std::os::raw::c_char,
    pub ring_info: *mut sysdig_ring_info,
    pub last_evt_read_len: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfring_sysdig {
    pub num_devices: uint8_t,
    pub bytes_watermark: uint32_t,
    pub devices: [pfring_sysdig_device; 64usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct sysdig_event_header {
    pub ts: uint64_t,
    pub thread_id: uint64_t,
    pub event_len: uint32_t,
    pub event_type: uint16_t,
}


extern "C" {
    pub fn pfring_mod_sysdig_open(ring: *mut pfring) -> ::std::os::raw::c_int;

    pub fn pfring_mod_sysdig_close(ring: *mut pfring);

    pub fn pfring_mod_sysdig_stats(
        ring: *mut pfring,
        stats: *mut pfring_stat,
    ) -> ::std::os::raw::c_int;

    pub fn pfring_mod_sysdig_recv(
        ring: *mut pfring,
        buffer: *mut *mut c_uchar,
        buffer_len: c_uint,
        hdr: *mut pfring_pkthdr,
        wait_for_incoming_packet: uint8_t,
    ) -> ::std::os::raw::c_int;

    pub fn pfring_mod_sysdig_poll(ring: *mut pfring, wait_duration: c_uint)
        -> ::std::os::raw::c_int;

    pub fn pfring_mod_sysdig_enable_ring(ring: *mut pfring) -> ::std::os::raw::c_int;

    pub fn pfring_mod_sysdig_set_socket_mode(
        ring: *mut pfring,
        mode: socket_mode,
    ) -> ::std::os::raw::c_int;

    pub fn pfring_mod_sysdig_set_poll_watermark(
        ring: *mut pfring,
        watermark: uint16_t,
    ) -> ::std::os::raw::c_int;

    pub fn pfring_mod_sysdig_get_bound_device_ifindex(
        ring: *mut pfring,
        if_index: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn pfring_mod_sysdig_set_bpf_filter(
        ring: *mut pfring,
        filter_buffer: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn pfring_mod_sysdig_remove_bpf_filter(ring: *mut pfring) -> ::std::os::raw::c_int;

    pub fn sysdig_event2name(event_type: sysdig_event_type) -> *mut ::std::os::raw::c_char;

    pub fn sysdig_event2info(event_type: sysdig_event_type) -> *const sysdig_event_info;
}