use crate::{ __BindgenBitfieldUnit, __IncompleteArrayField, };
use crate::libc::{
    self, c_uint, uint8_t, uint16_t, c_ushort, uint32_t, uint64_t, c_ulong, c_uchar, time_t,
    timeval, timespec, pthread_rwlock_t,
};
use crate::linux::pf_ring::{
    pfring_pkthdr, virtual_filtering_device_info, hash_filtering_rule,
    filtering_rule, cluster_type, socket_mode,
    packet_direction, hw_filtering_rule, 
    FlowSlotInfo, pfring_device_type,
};


#[cfg(target_os = "linux")]
use crate::libc::sockaddr_ll;

// #[cfg(target_os = "macos")]
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// #[doc(hidden)]
// pub struct sockaddr_ll {
//     a: [u8; 8],
// }


pub const MAX_CAPLEN: u32 = 65535;
pub const PAGE_SIZE: u32 = 4096;
pub const DEFAULT_POLL_DURATION: u32 = 500;
pub const POLL_SLEEP_STEP: u32 = 10;
pub const POLL_SLEEP_MIN: u32 = 10;
pub const POLL_SLEEP_MAX: u32 = 1000;
pub const POLL_QUEUE_MIN_LEN: u32 = 500;


// #define pfring_rwlock_t       pthread_rwlock_t       
// #define pfring_rwlock_init    pthread_rwlock_init    
// #define pfring_rwlock_rdlock  pthread_rwlock_rdlock  
// #define pfring_rwlock_wrlock  pthread_rwlock_wrlock  
// #define pfring_rwlock_unlock  pthread_rwlock_unlock  
// #define pfring_rwlock_destroy pthread_rwlock_destroy 
pub type pfring_rwlock_t       = pthread_rwlock_t;
// pub type pfring_rwlock_init    = libc::pthread_rwlock_init;
// pub type pfring_rwlock_rdlock  = libc::pthread_rwlock_rdlock;
// pub type pfring_rwlock_wrlock  = libc::pthread_rwlock_wrlock;
// pub type pfring_rwlock_unlock  = libc::pthread_rwlock_unlock;
// pub type pfring_rwlock_destroy = libc::pthread_rwlock_destroy;


pub type pfringProcesssPacket = ::std::option::Option<
    unsafe extern "C" fn(h: *const pfring_pkthdr, p: *const c_uchar, user_bytes: *const c_uchar),
>;

pub type pfring = __pfring;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_card_settings {
    pub max_packet_size: uint32_t,
    pub rx_ring_slots: uint32_t,
    pub tx_ring_slots: uint32_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_stat {
    pub recv: uint64_t,
    pub drop: uint64_t,
    pub shunt: uint64_t,
}
pub const hardware_and_software: filtering_mode = 0;
pub const hardware_only: filtering_mode = 1;
pub const software_only: filtering_mode = 2;
pub type filtering_mode = u32;
pub const FULL_PACKET_SLICING: packet_slicing_level = 0;
pub const L2_SLICING: packet_slicing_level = 2;
pub const L3_SLICING: packet_slicing_level = 3;
pub const L4_SLICING: packet_slicing_level = 4;
pub type packet_slicing_level = u32;
pub const PCAP_CHUNK: pfring_chunk_type = 0;
pub const PCAP_NSEC_CHUNK: pfring_chunk_type = 1;
pub const PCAPNG_NSEC_CHUNK: pfring_chunk_type = 2;
pub const UNKNOWN_CHUNK_TYPE: pfring_chunk_type = 3;
pub type pfring_chunk_type = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_chunk_info {
    pub length: uint32_t,
    pub type_: pfring_chunk_type,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_bpf_program {
    pub bf_len: c_uint,
    pub bf_insns: *mut libc::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_if {
    pub name: *mut libc::c_char,
    pub system_name: *mut libc::c_char,
    pub module: *mut libc::c_char,
    pub sn: *mut libc::c_char,
    pub mac: [libc::c_char; 6usize],
    pub bus_id: pfring_if__bindgen_ty_1,
    pub status: libc::c_int,
    pub license: libc::c_int,
    pub next: *mut pfring_if,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_if__bindgen_ty_1 {
    pub slot: libc::c_int,
    pub bus: libc::c_int,
    pub device: libc::c_int,
    pub function: libc::c_int,
}
pub type pfring_if_t = pfring_if;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pfring {
    pub initialized: uint8_t,
    pub enabled: uint8_t,
    pub long_header: uint8_t,
    pub force_timestamp: uint8_t,
    pub strip_hw_timestamp: uint8_t,
    pub disable_parsing: uint8_t,
    pub disable_timestamp: uint8_t,
    pub ixia_timestamp_enabled: uint8_t,
    pub vss_apcon_timestamp_enabled: uint8_t,
    pub chunk_mode_enabled: uint8_t,
    pub userspace_bpf: uint8_t,
    pub force_userspace_bpf: uint8_t,
    pub rss_mode: uint32_t,
    pub direction: packet_direction,
    pub mode: socket_mode,
    pub userspace_bpf_filter: pfring_bpf_program,
    pub hw_ts: __pfring__bindgen_ty_1,
    pub tx: __pfring__bindgen_ty_2,
    pub zc_device: uint8_t,
    pub priv_data: *mut libc::c_void,
    pub close: ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring)>,
    pub stats: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: *mut pfring_stat) -> libc::c_int,
    >,
    pub recv: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut *mut c_uchar,
            arg3: c_uint,
            arg4: *mut pfring_pkthdr,
            arg5: uint8_t,
        ) -> libc::c_int,
    >,
    pub set_poll_watermark: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: uint16_t) -> libc::c_int,
    >,
    pub set_poll_watermark_timeout: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: uint16_t) -> libc::c_int,
    >,
    pub set_poll_duration: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: c_uint) -> libc::c_int,
    >,
    pub set_tx_watermark: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: uint16_t) -> libc::c_int,
    >,
    pub set_channel_id: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: uint32_t) -> libc::c_int,
    >,
    pub set_channel_mask: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: uint64_t) -> libc::c_int,
    >,
    pub set_application_name: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub set_application_stats: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub get_appl_stats_file_name: ::std::option::Option<
        unsafe extern "C" fn(
            ring: *mut pfring,
            path: *mut libc::c_char,
            path_len: c_uint,
        ) -> *mut libc::c_char,
    >,
    pub set_vlan_id: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: uint16_t) -> libc::c_int,
    >,
    pub bind: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub send: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut libc::c_char,
            arg3: c_uint,
            arg4: uint8_t,
        ) -> libc::c_int,
    >,
    pub send_get_time: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut libc::c_char,
            arg3: c_uint,
            arg4: *mut timespec,
        ) -> libc::c_int,
    >,
    pub get_num_rx_channels:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring) -> uint8_t>,
    pub get_card_settings: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut pfring_card_settings,
        ) -> libc::c_int,
    >,
    pub set_sampling_rate: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: uint32_t) -> libc::c_int,
    >,
    pub set_filtering_sampling_rate: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: uint32_t) -> libc::c_int,
    >,
    pub set_packet_slicing: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: packet_slicing_level,
            arg3: uint32_t,
        ) -> libc::c_int,
    >,
    pub get_selectable_fd:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring) -> libc::c_int>,
    pub set_direction: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: packet_direction) -> libc::c_int,
    >,
    pub set_socket_mode: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: socket_mode) -> libc::c_int,
    >,
    pub set_cluster: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: c_uint,
            arg3: cluster_type,
        ) -> libc::c_int,
    >,
    pub remove_from_cluster:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring) -> libc::c_int>,
    pub set_master_id: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: uint32_t) -> libc::c_int,
    >,
    pub set_master: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: *mut pfring) -> libc::c_int,
    >,
    pub get_ring_id: ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring) -> uint32_t>,
    pub get_num_queued_pkts:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring) -> uint32_t>,
    pub get_hash_filtering_rule_stats: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut hash_filtering_rule,
            arg3: *mut libc::c_char,
            arg4: *mut c_uint,
        ) -> libc::c_int,
    >,
    pub handle_hash_filtering_rule: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut hash_filtering_rule,
            arg3: c_uchar,
        ) -> libc::c_int,
    >,
    pub purge_idle_hash_rules: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: uint16_t) -> libc::c_int,
    >,
    pub add_filtering_rule: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: *mut filtering_rule) -> libc::c_int,
    >,
    pub remove_filtering_rule: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: uint16_t) -> libc::c_int,
    >,
    pub purge_idle_rules: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: uint16_t) -> libc::c_int,
    >,
    pub get_filtering_rule_stats: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: uint16_t,
            arg3: *mut libc::c_char,
            arg4: *mut c_uint,
        ) -> libc::c_int,
    >,
    pub toggle_filtering_policy: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: uint8_t) -> libc::c_int,
    >,
    pub enable_rss_rehash:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring) -> libc::c_int>,
    pub poll: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: c_uint) -> libc::c_int,
    >,
    pub is_pkt_available:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring) -> libc::c_int>,
    pub next_pkt_time: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: *mut timespec) -> libc::c_int,
    >,
    pub next_pkt_raw_timestamp: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, ts: *mut uint64_t) -> libc::c_int,
    >,
    pub version: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: *mut uint32_t) -> libc::c_int,
    >,
    pub get_bound_device_address: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: *mut c_uchar) -> libc::c_int,
    >,
    pub get_bound_device_ifindex: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub get_device_ifindex: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut libc::c_char,
            arg3: *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub get_slot_header_len:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring) -> uint16_t>,
    pub set_virtual_device: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut virtual_filtering_device_info,
        ) -> libc::c_int,
    >,
    pub add_hw_rule: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut hw_filtering_rule,
        ) -> libc::c_int,
    >,
    pub remove_hw_rule: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: uint16_t) -> libc::c_int,
    >,
    pub loopback_test: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut libc::c_char,
            arg3: c_uint,
            arg4: c_uint,
        ) -> libc::c_int,
    >,
    pub enable_ring:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring) -> libc::c_int>,
    pub disable_ring:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring) -> libc::c_int>,
    pub shutdown: ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring)>,
    pub set_bpf_filter: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub remove_bpf_filter:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring) -> libc::c_int>,
    pub get_device_clock: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: *mut timespec) -> libc::c_int,
    >,
    pub set_device_clock: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: *mut timespec) -> libc::c_int,
    >,
    pub adjust_device_clock: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut timespec,
            arg3: i8,
        ) -> libc::c_int,
    >,
    pub sync_indexes_with_kernel: ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring)>,
    pub send_last_rx_packet: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: libc::c_int,
        ) -> libc::c_int,
    >,
    pub flush_tx_packets: ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring)>,
    pub register_zerocopy_tx_ring: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut pfring, arg2: *mut pfring) -> libc::c_int,
    >,
    pub recv_chunk: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut *mut libc::c_void,
            arg3: *mut pfring_chunk_info,
            arg4: uint8_t,
        ) -> libc::c_int,
    >,
    pub set_bound_dev_name: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub get_metadata: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut pfring,
            arg2: *mut *mut c_uchar,
            arg3: *mut uint32_t,
        ) -> libc::c_int,
    >,
    pub get_interface_speed:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring) -> uint32_t>,
    pub rdi: __pfring__bindgen_ty_3,
    pub ft_mode: filtering_mode,
    pub ft_device_type: pfring_device_type,
    pub buffer: *mut libc::c_char,
    pub slots: *mut libc::c_char,
    pub device_name: *mut libc::c_char,
    pub caplen: uint32_t,
    pub slot_header_len: uint16_t,
    pub mtu: uint16_t,
    pub sampling_rate: uint32_t,
    pub sampling_counter: uint32_t,
    pub slicing_level: packet_slicing_level,
    pub slicing_additional_bytes: uint32_t,
    pub is_shutting_down: uint8_t,
    pub socket_default_accept_policy: uint8_t,
    pub fd: libc::c_int,
    pub device_id: libc::c_int,
    pub slots_info: *mut FlowSlotInfo,
    pub poll_sleep: uint32_t,
    pub poll_duration: uint16_t,
    pub promisc: uint8_t,
    pub ft_enabled: uint8_t,
    pub reentrant: uint8_t,
    pub break_recv_loop: uint8_t,
    pub num_poll_calls: c_ulong,
    pub rx_lock: pthread_rwlock_t,
    pub tx_lock: pthread_rwlock_t,
    pub flags: uint32_t,
    pub ft: *mut libc::c_void,
    pub sock_tx: sockaddr_ll,
    pub reflector_socket: *mut pfring,
    pub one_copy_rx_pfring: *mut pfring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pfring__bindgen_ty_1 {
    pub force_timestamp: uint8_t,
    pub is_silicom_hw_timestamp_card: uint8_t,
    pub enable_hw_timestamp: uint8_t,
    pub last_hw_timestamp_head_offset: uint8_t,
    pub last_hw_timestamp: timespec,
}

impl core::fmt::Debug for __pfring__bindgen_ty_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "__pfring__bindgen_ty_1: {{ force_timestamp: {:?}",
            self.force_timestamp)?;
        write!(f, ", is_silicom_hw_timestamp_card: {:?}",
            self.is_silicom_hw_timestamp_card)?;
        write!(f, ", enable_hw_timestamp: {:?}",
            self.enable_hw_timestamp)?;
        write!(f, ", last_hw_timestamp_head_offset: {:?}",
            self.last_hw_timestamp_head_offset)?;
        write!(f, ", last_hw_timestamp: timespec {{ tv_sec: {:?}, tv_nsec: {:?} }} }}",
            self.last_hw_timestamp.tv_sec, self.last_hw_timestamp.tv_nsec)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pfring__bindgen_ty_2 {
    pub enabled_rx_packet_send: uint8_t,
    pub last_received_hdr: *mut pfring_pkthdr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pfring__bindgen_ty_3 {
    pub device_id: i8,
    pub port_id: i8,
}

pub const PF_RING_ZC_SYMMETRIC_RSS: u32 = 1;
pub const PF_RING_REENTRANT: u32 = 2;
pub const PF_RING_LONG_HEADER: u32 = 4;
pub const PF_RING_PROMISC: u32 = 8;
pub const PF_RING_TIMESTAMP: u32 = 16;
pub const PF_RING_HW_TIMESTAMP: u32 = 32;
pub const PF_RING_RX_PACKET_BOUNCE: u32 = 64;
pub const PF_RING_ZC_FIXED_RSS_Q_0: u32 = 128;
pub const PF_RING_STRIP_HW_TIMESTAMP: u32 = 256;
pub const PF_RING_DO_NOT_PARSE: u32 = 512;
pub const PF_RING_DO_NOT_TIMESTAMP: u32 = 1024;
pub const PF_RING_CHUNK_MODE: u32 = 2048;
pub const PF_RING_IXIA_TIMESTAMP: u32 = 4096;
pub const PF_RING_USERSPACE_BPF: u32 = 8192;
pub const PF_RING_ZC_NOT_REPROGRAM_RSS: u32 = 16384;
pub const PF_RING_VSS_APCON_TIMESTAMP: u32 = 32768;
pub const PF_RING_ZC_IPONLY_RSS: u32 = 65536;
pub const PF_RING_FLOW_OFFLOAD: u32 = 131072;
pub const PF_RING_FLOW_OFFLOAD_NOUPDATES: u32 = 262144;
pub const PF_RING_FLOW_OFFLOAD_NORAWDATA: u32 = 524288;
pub const PF_RING_L7_FILTERING: u32 = 1048576;
pub const PF_RING_DO_NOT_STRIP_FCS: u32 = 2097152;
pub const PF_RING_DNA_SYMMETRIC_RSS: u32 = 1;
pub const PF_RING_DNA_FIXED_RSS_Q_0: u32 = 128;




extern "C" {
    #[doc = " This call is used to initialize a PF_RING socket hence obtain a handle of type struct pfring"]
    #[doc = " that can be used in subsequent calls. Note that:"]
    #[doc = " 1. you can use physical (e.g. ethX) and virtual (e.g. tapX) devices, RX-queues (e.g. ethX@Y),"]
    #[doc = "    and additional modules (e.g. zc:ethX@Y, dag:dagX:Y, \"multi:ethA@X;ethB@Y;ethC@Z\", \"stack:ethX\")."]
    #[doc = " 2. you need super-user capabilities in order to open a device."]
    #[doc = " @param device_name Symbolic name of the PF_RING-aware device we are attempting to open."]
    #[doc = " Syntax:"]
    #[doc = "  - eth0           interface eth0, all channels"]
    #[doc = "  - eth0@1         interface eth0, channel 1"]
    #[doc = "  - eth0,eth1      interface eth0 and eth1, all channels"]
    #[doc = "  - eth0,eth1@1    interface eth0 and eth1, channel 1"]
    #[doc = "  - eth0@1,5       interface eth0, channel 1 and 5"]
    #[doc = "  - eth0@1-5       interface eth0, channel 1,2...5"]
    #[doc = "  - eth0@1-3,5-7   interface eth0, channel 1,2,3,5,6,7"]
    #[doc = " Note:"]
    #[doc = "  - \',\' and \'-\' are supported with standard kernel capture / drivers only"]
    #[doc = "  - in case of multiple interfaces, the channels are same for all interfaces"]
    #[doc = " @param caplen      Maximum packet capture len (also known as snaplen)."]
    #[doc = " @param flags       It allows several options to be specified on a compact format using bitmaps (see PF_RING_* macros)."]
    #[doc = " @return On success a handle is returned, NULL otherwise."]
    pub fn pfring_open(
        device_name: *const libc::c_char,
        caplen: uint32_t,
        flags: uint32_t,
    ) -> *mut pfring;
}
extern "C" {
    #[doc = " This call is similar to pfring_open() with the exception that in case of a multi RX-queue NIC,"]
    #[doc = " instead of opening a single ring for the whole device, several individual rings are open (one per RX-queue)."]
    #[doc = " @param device_name Symbolic name of the PF_RING-aware device we are attempting to open (e.g. eth0)."]
    #[doc = "                    No queue name hash to be specified, but just the main device name."]
    #[doc = " @param caplen      Maximum packet capture len (also known as snaplen)."]
    #[doc = " @param flags       See pfring_open() for details."]
    #[doc = " @param ring        A pointer to an array of rings that will contain the opened ring pointers."]
    #[doc = " @return The last index of the ring array that contain a valid ring pointer."]
    pub fn pfring_open_multichannel(
        device_name: *const libc::c_char,
        caplen: uint32_t,
        flags: uint32_t,
        ring: *mut *mut pfring,
    ) -> uint8_t;
}
extern "C" {
    #[doc = " Shutdown a socket."]
    #[doc = " @param ring The PF_RING handle."]
    pub fn pfring_shutdown(ring: *mut pfring);
}
extern "C" {
    #[doc = " Set the scheduler priority for the current thread."]
    #[doc = " @param cpu_percentage The priority."]
    pub fn pfring_config(cpu_percentage: c_ushort);
}
extern "C" {
    #[doc = " Process ingress packets until pfring_breakloop() is called, or an error occurs."]
    #[doc = " @param ring            The PF_RING handle."]
    #[doc = " @param looper          The user callback for packet processing."]
    #[doc = " @param user_bytes      The user ptr passed to the callback."]
    #[doc = " @param wait_for_packet If 0 active wait is used to check the packet availability."]
    #[doc = " @return 0 on success (pfring_breakloop()), a negative value otherwise."]
    pub fn pfring_loop(
        ring: *mut pfring,
        looper: pfringProcesssPacket,
        user_bytes: *const c_uchar,
        wait_for_packet: uint8_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Break a receive loop (pfring_loop() or blocking pfring_recv())."]
    #[doc = " @param ring The PF_RING handle."]
    pub fn pfring_breakloop(arg1: *mut pfring);
}
extern "C" {
    #[doc = " This call is used to terminate an PF_RING device previously open."]
    #[doc = " Note that you must always close a device before leaving an application. If unsure, you can close a device from a signal handler."]
    #[doc = " @param ring The PF_RING handle that we are attempting to close."]
    pub fn pfring_close(ring: *mut pfring);
}
extern "C" {
    #[doc = " Read ring statistics (packets received and dropped)."]
    #[doc = " @param ring  The PF_RING handle."]
    #[doc = " @param stats A user-allocated buffer on which stats (number of received and dropped packets) will be stored."]
    #[doc = " @return 0 on uccess, a negative value otherwise."]
    pub fn pfring_stats(ring: *mut pfring, stats: *mut pfring_stat) -> libc::c_int;
}
extern "C" {
    #[doc = " This call returns an incoming packet when available."]
    #[doc = " @param ring       The PF_RING handle where we perform the check."]
    #[doc = " @param buffer     A memory area allocated by the caller where the incoming packet will be stored."]
    #[doc = "                   Note that this parameter is a pointer to a pointer, in order to enable zero-copy implementations (buffer_len must be set to 0)."]
    #[doc = " @param buffer_len The length of the memory area above."]
    #[doc = "                   Note that the incoming packet is cut if it is too long for the allocated area."]
    #[doc = "                   A length of 0 indicates to use the zero-copy optimization, when available."]
    #[doc = " @param hdr        A memory area where the packet header will be copied."]
    #[doc = " @param wait_for_incoming_packet If 0 we simply check the packet availability, otherwise the call is blocked until a packet is available."]
    #[doc = "                   This option is also available with the PF_RING-aware libpcap via the PCAP_PF_RING_ACTIVE_POLL environment variable."]
    #[doc = " @return 0 in case of no packet being received (non-blocking), 1 in case of success, -1 in case of error."]
    pub fn pfring_recv(
        ring: *mut pfring,
        buffer: *mut *mut c_uchar,
        buffer_len: c_uint,
        hdr: *mut pfring_pkthdr,
        wait_for_incoming_packet: uint8_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Same of pfring_recv(), with additional parameters to force packet parsing."]
    #[doc = " @param ring"]
    #[doc = " @param buffer"]
    #[doc = " @param buffer_len"]
    #[doc = " @param hdr"]
    #[doc = " @param wait_for_incoming_packet"]
    #[doc = " @param level         The header level where to stop parsing."]
    #[doc = " @param add_timestamp Add the timestamp."]
    #[doc = " @param add_hash      Compute an IP-based bidirectional hash."]
    #[doc = " @return 0 in case of no packet being received (non-blocking), 1 in case of success, -1 in case of error."]
    pub fn pfring_recv_parsed(
        ring: *mut pfring,
        buffer: *mut *mut c_uchar,
        buffer_len: c_uint,
        hdr: *mut pfring_pkthdr,
        wait_for_incoming_packet: uint8_t,
        level: uint8_t,
        add_timestamp: uint8_t,
        add_hash: uint8_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Get metadata for the last captured packet, if any. This is usually used with ZC SPSC queues for reading packet metadata."]
    #[doc = " @param ring"]
    #[doc = " @param metadata Ptr to a variable that will contain the packet metadata (out)."]
    #[doc = " @return 0 if this is supported by the actual module and metadata is found, a negative error value otherwise."]
    pub fn pfring_get_metadata(
        ring: *mut pfring,
        metadata: *mut *mut c_uchar,
        metadata_len: *mut uint32_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Whenever a user-space application has to wait until incoming packets arrive, it can instruct PF_RING not to return from poll() call"]
    #[doc = " unless at least \u{201c}watermark\u{201d} packets have been returned. A low watermark value such as 1, reduces the latency of poll() but likely"]
    #[doc = " increases the number of poll() calls. A high watermark (it cannot exceed 50% of the ring size, otherwise the PF_RING kernel module"]
    #[doc = " will top its value) instead reduces the number of poll() calls but slightly increases the packet latency."]
    #[doc = " The default value for the watermark (i.e. if user-space applications do not manipulate this value via this call) is 128."]
    #[doc = " @param ring      The PF_RING handle to enable."]
    #[doc = " @param watermark The packet poll watermark."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_poll_watermark(
        ring: *mut pfring,
        watermark: uint16_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Flush ring\'s queue if timeout passed."]
    #[doc = " This helps to avoid situation where packets are waiting in the rings\'s queue too long (e.g. low-traffic network)."]
    #[doc = " The default value for the timeout is 0, which disables the flushing."]
    #[doc = " @param ring                   The PF_RING handle."]
    #[doc = " @param poll_watermark_timeout Milliseconds to flush ring\'s queue even if watermark packets hasn\'t reached yet."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_poll_watermark_timeout(
        ring: *mut pfring,
        poll_watermark_timeout: uint16_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Set the poll timeout when passive wait is used."]
    #[doc = " @param ring     The PF_RING handle to enable."]
    #[doc = " @param duration The poll timeout in msec."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_poll_duration(ring: *mut pfring, duration: c_uint) -> libc::c_int;
}
extern "C" {
    #[doc = " Set the number of packets that have to be enqueued in the egress queue before being sent on the wire."]
    #[doc = " @param ring      The PF_RING handle to enable."]
    #[doc = " @param watermark The tx watermark."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_tx_watermark(
        ring: *mut pfring,
        watermark: uint16_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Set a specified filtering rule into the NIC. Note that no PF_RING filter is added, but only a NIC filter."]
    #[doc = ""]
    #[doc = " Some multi-queue modern network adapters feature \"packet steering\" capabilities. Using them it is possible to"]
    #[doc = " instruct the hardware NIC to assign selected packets to a specific RX queue. If the specified queue has an Id"]
    #[doc = " that exceeds the maximum queueId, such packet is discarded thus acting as a hardware firewall filter."]
    #[doc = " Note: kernel packet filtering is not supported by ZC."]
    #[doc = " @param ring The PF_RING handle on which the rule will be added."]
    #[doc = " @param rule The filtering rule to be set in the NIC as defined in the last chapter of this document."]
    #[doc = "             All rule parameters should be defined, and if set to zero they do not participate to filtering."]
    #[doc = " @return 0 on success, a negative value otherwise (e.g. the rule to be added has wrong format or if the NIC to"]
    #[doc = "         which this ring is bound does not support hardware filters)."]
    pub fn pfring_add_hw_rule(
        ring: *mut pfring,
        rule: *mut hw_filtering_rule,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Remove the specified filtering rule from the NIC."]
    #[doc = " @param ring The PF_RING handle on which the rule will be removed."]
    #[doc = " @param rule The filtering rule to be removed from the NIC."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_remove_hw_rule(ring: *mut pfring, rule_id: uint16_t) -> libc::c_int;
}
extern "C" {
    #[doc = " Set the device channel id to be used."]
    #[doc = " @param ring       The PF_RING handle."]
    #[doc = " @param channel_id The channel id."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_channel_id(ring: *mut pfring, channel_id: uint32_t)
        -> libc::c_int;
}
extern "C" {
    #[doc = " Set the channel mask to be used for packet capture."]
    #[doc = " @param ring         The PF_RING handle."]
    #[doc = " @param channel_mask The channel mask."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_channel_mask(
        ring: *mut pfring,
        channel_mask: uint64_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Tell PF_RING the name of the application (usually `argv[0]`) that uses this ring. This information is used to identify the application"]
    #[doc = " when accessing the files present in the PF_RING /proc filesystem."]
    #[doc = " This is also available with the PF_RING-aware libpcap via the PCAP_PF_RING_APPNAME environment variable."]
    #[doc = " Example:"]
    #[doc = " $ cat /proc/net/pf_ring/16614-eth0.0 | grep Name"]
    #[doc = " Appl. Name     : pfcount"]
    #[doc = " @param ring The PF_RING handle to enable."]
    #[doc = " @param name The name of the application using this ring."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_application_name(
        ring: *mut pfring,
        name: *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Set custom application statistics."]
    #[doc = " @param ring The PF_RING handle."]
    #[doc = " @param stats The application stats."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_application_stats(
        ring: *mut pfring,
        stats: *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Return the filename where the application statistics can be read."]
    #[doc = " @param ring     The PF_RING handle."]
    #[doc = " @param path     A user-allocated buffer on which the stats filename will be stored."]
    #[doc = " @param path_len The path len."]
    #[doc = " @return The path if success, NULL otherwise."]
    pub fn pfring_get_appl_stats_file_name(
        ring: *mut pfring,
        path: *mut libc::c_char,
        path_len: c_uint,
    ) -> *mut libc::c_char;
}
extern "C" {
    #[doc = " Set the VLAN Id of the packets that will be copied to this ring (RX only)"]
    #[doc = " @param ring       The PF_RING handle."]
    #[doc = " @param vlan_id    The vlan id to filter or 0 to accept only untagged VLAN packets"]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_vlan_id(ring: *mut pfring, vlan_id: uint16_t) -> libc::c_int;
}
extern "C" {
    #[doc = " Bind a socket to a device."]
    #[doc = " @param ring        The PF_RING handle."]
    #[doc = " @param device_name The device name."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_bind(
        ring: *mut pfring,
        device_name: *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Send a raw packet (i.e. it is sent on wire as specified). This packet must be fully specified (the MAC address up)"]
    #[doc = " and it will be transmitted as-is without any further manipulation."]
    #[doc = ""]
    #[doc = " Depending on the driver being used, packet transmission happens differently:"]
    #[doc = " - Vanilla and PF_RING aware drivers: PF_RING does not accelerate the TX so the standard Linux transmission facilities are used."]
    #[doc = "   Do not expect speed advantage when using PF_RING in this mode."]
    #[doc = " - ZC: line rate transmission is supported."]
    #[doc = " @param ring         The PF_RING handle on which the packet has to be sent."]
    #[doc = " @param pkt          The buffer containing the packet to send."]
    #[doc = " @param pkt_len      The length of the pkt buffer."]
    #[doc = " @param flush_packet 1 = Flush possible transmission queues. If set to 0, you will decrease your CPU usage but at the cost of"]
    #[doc = "                     sending packets in trains and thus at larger latency."]
    #[doc = " @return The number of bytes sent if success, a negative value otherwise."]
    pub fn pfring_send(
        ring: *mut pfring,
        pkt: *mut libc::c_char,
        pkt_len: c_uint,
        flush_packet: uint8_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Same as pfring_send(), but this function allows to send a raw packet returning the exact time (ns) it has been sent on the wire."]
    #[doc = " Note that this is available when the adapter supports tx hardware timestamping only and might affect performance."]
    #[doc = " @param ring"]
    #[doc = " @param pkt"]
    #[doc = " @param pkt_len"]
    #[doc = " @param ts      The struct where the tx timestamp will be stored."]
    #[doc = " @return The number of bytes sent if success, a negative value otherwise."]
    pub fn pfring_send_get_time(
        ring: *mut pfring,
        pkt: *mut libc::c_char,
        pkt_len: c_uint,
        ts: *mut timespec,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Returns the number of RX channels (also known as RX queues) of the ethernet interface to which this ring is bound."]
    #[doc = " @param ring The PF_RING handle to query."]
    #[doc = " @return The number of RX channels, or 1 (default) in case this in information is unknown."]
    pub fn pfring_get_num_rx_channels(ring: *mut pfring) -> uint8_t;
}
extern "C" {
    #[doc = " Implement packet sampling directly into the kernel. Note that this solution is much more efficient than implementing it in user-space."]
    #[doc = " Sampled packets are only those that pass all filters (if any)."]
    #[doc = " @param ring The PF_RING handle on which sampling is applied."]
    #[doc = " @param rate The sampling rate. Rate of X means that 1 packet out of X is forwarded. This means that a sampling rate of 1 disables sampling."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_sampling_rate(ring: *mut pfring, rate: uint32_t) -> libc::c_int;
}
extern "C" {
    #[doc = " Implement packet sampling during filtering directly into the kernel. Note that this solution is much more efficient than implementing it in user-space."]
    #[doc = " Sampled packets during filtering are only those that already have been filtered out (if any)."]
    #[doc = " @param ring The PF_RING handle on which filtering sampling is applied."]
    #[doc = " @param rate The filtering sampling rate. Rate of X means 1 packet every X packets is forwarded. Rate of 0 disables sampling."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_filtering_sampling_rate(
        ring: *mut pfring,
        rate: uint32_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Set packet slicing level."]
    #[doc = " @param ring             The PF_RING handle on which slicing is applied, when supported."]
    #[doc = " @param level            The slicing level (disabled, L2, L3, L4, ..)"]
    #[doc = " @param additional_bytes Bytes to capture in addition to the selected layer."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_packet_slicing(
        ring: *mut pfring,
        level: packet_slicing_level,
        additional_bytes: uint32_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Returns the file descriptor associated to the specified ring."]
    #[doc = " This number can be used in function calls such as poll() and select() for passively waiting for incoming packets."]
    #[doc = " @param ring The PF_RING handle to query."]
    #[doc = " @return A number that can be used as reference to this ring, in function calls that require a selectable file descriptor."]
    pub fn pfring_get_selectable_fd(ring: *mut pfring) -> libc::c_int;
}
extern "C" {
    #[doc = " Tell PF_RING to consider only those packets matching the specified direction. If the application does not call this function,"]
    #[doc = " all the packets (regardless of the direction, either RX or TX) are returned."]
    #[doc = " @param ring      The PF_RING handle to enable."]
    #[doc = " @param direction The packet direction (RX, TX or both RX and TX)."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_direction(
        ring: *mut pfring,
        direction: packet_direction,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Tell PF_RING if the application needs to send and/or receive packets to/from the socket."]
    #[doc = " @param ring The PF_RING handle to enable."]
    #[doc = " @param mode The socket mode (send, receive or both send and receive)."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_socket_mode(ring: *mut pfring, mode: socket_mode) -> libc::c_int;
}
extern "C" {
    #[doc = " This call allows a ring to be added to a cluster that can spawn across address spaces."]
    #[doc = " On a nuthsell when two or more sockets are clustered they share incoming packets that are balanced on a per-flow manner."]
    #[doc = " This technique is useful for exploiting multicore systems of for sharing packets in the same address space across multiple threads."]
    #[doc = " Clustering is also available with the PF_RING-aware libpcap via the PCAP_PF_RING_CLUSTER_ID environment variable (Round-Robin by default,"]
    #[doc = " per-flow via the PCAP_PF_RING_USE_CLUSTER_PER_FLOW environment variable)."]
    #[doc = " @param ring The  PF_RING handle to be cluster."]
    #[doc = " @param clusterId A numeric identifier of the cluster to which the ring will be bound."]
    #[doc = " @param the_type  The cluster type (2-tuple, 4-tuple, 5-tuple, tcp only 5-tuple, 6-tuple flow or Round-Robin)."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_cluster(
        ring: *mut pfring,
        clusterId: c_uint,
        the_type: cluster_type,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " This call allows a ring to be removed from a previous joined cluster."]
    #[doc = " @param ring      The PF_RING handle to be cluster."]
    #[doc = " @param clusterId A numeric identifier of the cluster to which the ring will be bound."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_remove_from_cluster(ring: *mut pfring) -> libc::c_int;
}
extern "C" {
    #[doc = " Set the master ring using the id (vanilla PF_RING only)"]
    #[doc = " @param ring   The PF_RING handle."]
    #[doc = " @param master The master socket id."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_master_id(ring: *mut pfring, master_id: uint32_t) -> libc::c_int;
}
extern "C" {
    #[doc = " Set the master ring using the PF_RING handle (vanilla PF_RING only)."]
    #[doc = " @param ring   The PF_RING handle."]
    #[doc = " @param master The master PF_RING handle."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_master(ring: *mut pfring, master: *mut pfring) -> libc::c_int;
}
extern "C" {
    #[doc = " Return the ring id."]
    #[doc = " @param ring The PF_RING handle."]
    #[doc = " @return The ring id."]
    pub fn pfring_get_ring_id(ring: *mut pfring) -> uint32_t;
}
extern "C" {
    #[doc = " Return an estimation of the enqueued packets."]
    #[doc = " @param ring The PF_RING handle."]
    #[doc = " @param"]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_get_num_queued_pkts(ring: *mut pfring) -> uint32_t;
}
extern "C" {
    #[doc = " Add or remove a hash filtering rule."]
    #[doc = " All rule parameters should be defined in the filtering rule (no wildcards)."]
    #[doc = " @param ring        The PF_RING handle from which stats will be read."]
    #[doc = " @param rule_to_add The rule that will be added/removed as defined in the last chapter of this document."]
    #[doc = "                    All rule parameters should be defined in the filtering rule (no wildcards)."]
    #[doc = " @param add_rule    If set to a positive value the rule is added, if zero the rule is removed."]
    #[doc = " @return 0 on success, a negative value otherwise (e.g. the rule to be removed does not exist)."]
    pub fn pfring_handle_hash_filtering_rule(
        ring: *mut pfring,
        rule_to_add: *mut hash_filtering_rule,
        add_rule: c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    /// Add a wildcard filtering rule to an existing ring. Each rule will have a unique rule Id across the ring (i.e. two rings can have rules with the same id).
    /// 
    /// PF_RING allows filtering packets in two ways: precise (a.k.a. hash filtering) or wildcard filtering. 
    /// Precise filtering is used when it is necessary to track a precise 6-tuple connection <vlan Id, protocol, source IP, source port, destination IP, destination port>. 
    /// Wildcard filtering is used instead whenever a filter can have wildcards on some of its fields (e.g. match all UDP packets regardless of their destination). 
    /// If some field is set to zero it will not participate in filter calculation.
    /// 
    /// Note about packet reflection: packet reflection is the ability to bridge packets in kernel without sending them to userspace and back. 
    /// You can specify packet reflection inside the filtering rules.
    /// 
    /// typedef struct {
    /// 
    /// ...
    ///
    /// char reflector_device_name`[`REFLECTOR_NAME_LEN`]`;
    ///
    /// ...
    /// 
    /// } filtering_rule;
    /// 
    /// In the reflector_device_name you need to specify a device name (e.g. eth0) on which packets matching the filter will be reflected. 
    /// Make sure NOT to specify as reflection device the same device name on which you capture packets, as otherwise you will create a packet loop.
    /// 
    /// @param ring        The PF_RING handle on which the rule will be added.
    /// @param rule_to_add The rule to add as defined in the last chapter of this document.
    /// @return 0 on success, a negative value otherwise.
    pub fn pfring_add_filtering_rule(
        ring: *mut pfring,
        rule_to_add: *mut filtering_rule,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Remove a previously added filtering rule."]
    #[doc = " @param ring    The PF_RING handle on which the rule will be removed."]
    #[doc = " @param rule_id The id of a previously added rule that will be removed."]
    #[doc = " @return 0 on success, a negative value otherwise (e.g. the rule does not exist)."]
    pub fn pfring_remove_filtering_rule(
        ring: *mut pfring,
        rule_id: uint16_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Remove hash filtering rules inactive for the specified number of seconds."]
    #[doc = " @param ring           The PF_RING handle on which the rules will be removed."]
    #[doc = " @param inactivity_sec The inactivity threshold."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_purge_idle_hash_rules(
        ring: *mut pfring,
        inactivity_sec: uint16_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Remove filtering rules inactive for the specified number of seconds."]
    #[doc = " @param ring           The PF_RING handle on which the rules will be removed."]
    #[doc = " @param inactivity_sec The inactivity threshold."]
    #[doc = " @return 0 on success, a negative value otherwise"]
    pub fn pfring_purge_idle_rules(
        ring: *mut pfring,
        inactivity_sec: uint16_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Read statistics of a hash filtering rule."]
    #[doc = " @param ring      The PF_RING handle on which the rule will be added/removed."]
    #[doc = " @param rule      The rule for which stats are read. This needs to be the same rule that has been previously added."]
    #[doc = " @param stats     A buffer allocated by the user that will contain the rule statistics."]
    #[doc = "                  Please make sure that the buffer is large enough to contain the statistics (hash_filtering_rule_stats struct)."]
    #[doc = " @param stats_len The size (in bytes) of the stats buffer."]
    #[doc = " @return 0 on success, a negative value otherwise (e.g. the rule to be removed does not exist)."]
    pub fn pfring_get_hash_filtering_rule_stats(
        ring: *mut pfring,
        rule: *mut hash_filtering_rule,
        stats: *mut libc::c_char,
        stats_len: *mut c_uint,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Read statistics of a hash filtering rule."]
    #[doc = " @param ring      The PF_RING handle from which stats will be read."]
    #[doc = " @param rule_id   The rule id that identifies the rule for which stats are read."]
    #[doc = " @param stats     A buffer allocated by the user that will contain the rule statistics."]
    #[doc = "                  Please make sure that the buffer is large enough to contain the statistics."]
    #[doc = "                  Such buffer will contain number of received and dropped packets."]
    #[doc = " @param stats_len The size (in bytes) of the stats buffer."]
    #[doc = " @return 0 on success, a negative value otherwise (e.g. the rule does not exist)."]
    pub fn pfring_get_filtering_rule_stats(
        ring: *mut pfring,
        rule_id: uint16_t,
        stats: *mut libc::c_char,
        stats_len: *mut c_uint,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Set the default filtering policy. This means that if no rule is matching the incoming packet the default policy will decide"]
    #[doc = " if the packet is forwarded to user space or dropped. Note that filtering rules are limited to a ring, so each ring can have"]
    #[doc = " a different set of rules and default policy."]
    #[doc = " @param ring The PF_RING handle on which the rule will be added/removed."]
    #[doc = " @param rules_default_accept_policy If set to a positive value the default policy is accept (i.e. forward packets to user space), drop otherwise."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_toggle_filtering_policy(
        ring: *mut pfring,
        rules_default_accept_policy: uint8_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Tells PF_RING to rehash incoming packets using a bi-directional hash function."]
    #[doc = " This is also available with the PF_RING-aware libpcap via the PCAP_PF_RING_RSS_REHASH environment variable."]
    #[doc = " @param ring The PF_RING handle to query."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_enable_rss_rehash(ring: *mut pfring) -> libc::c_int;
}
extern "C" {
    #[doc = " Performs passive wait on a PF_RING socket, similar to the standard poll(), taking care of data structures synchronization."]
    #[doc = " @param ring          The PF_RING socket to poll."]
    #[doc = " @param wait_duration The poll timeout in msec."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_poll(ring: *mut pfring, wait_duration: c_uint) -> libc::c_int;
}
extern "C" {
    #[doc = " Check if a packet is available."]
    #[doc = " @param ring The PF_RING handle."]
    #[doc = " @return 1 if a packet is available, 0 if there is no packet available, a negative number in case of error."]
    pub fn pfring_is_pkt_available(ring: *mut pfring) -> libc::c_int;
}
extern "C" {
    #[doc = " This call returns the arrival time of the next incoming packet, when available."]
    #[doc = " @param ring The PF_RING handle where we perform the check."]
    #[doc = " @param ts   The struct where the time will be stored."]
    #[doc = " @return 0 in case of success, a negative number in case of error."]
    pub fn pfring_next_pkt_time(ring: *mut pfring, ts: *mut timespec) -> libc::c_int;
}
extern "C" {
    #[doc = " This call returns the raw timestamp of the next incoming packet, when available. This is available with adapters supporting rx hardware timestamping only."]
    #[doc = " @param ring         The PF_RING handle where we perform the check."]
    #[doc = " @param timestamp_ns Where the timestamp will be stored."]
    #[doc = " @return 0 in case of success, a negative number in case of error."]
    pub fn pfring_next_pkt_raw_timestamp(
        ring: *mut pfring,
        timestamp_ns: *mut uint64_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Read the ring version. Note that if the ring version is 5.6 the retuned ring version is 0x050600."]
    #[doc = " @param version A user-allocated buffer on which ring version will be copied."]
    pub fn pfring_version_noring(version: *mut uint32_t);
}
extern "C" {
    #[doc = " Read the ring version. Note that if the ring version is 5.6 the retuned ring version is 0x050600."]
    #[doc = " @param ring    The PF_RING handle, in case the module supports versioning."]
    #[doc = " @param version A user-allocated buffer on which ring version will be copied."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_version(ring: *mut pfring, version: *mut uint32_t) -> libc::c_int;
}
extern "C" {
    #[doc = " Set a reflector device to send all incoming packets. This open a new socket and packets are automatically sent using pfring_send()."]
    #[doc = " @param ring        The PF_RING handle."]
    #[doc = " @param device_name The device name."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_reflector_device(
        ring: *mut pfring,
        device_name: *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Returns the MAC address of the device bound to the socket."]
    #[doc = " @param ring        The PF_RING handle to query."]
    #[doc = " @param mac_address The memory area where the MAC address will be copied."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_get_bound_device_address(
        ring: *mut pfring,
        mac_address: *mut c_uchar,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Return the size of the PF_RING packet header (vanilla PF_RING only)."]
    #[doc = " @param ring The PF_RING handle."]
    #[doc = " @return The size of the packet header."]
    pub fn pfring_get_slot_header_len(ring: *mut pfring) -> uint16_t;
}
extern "C" {
    #[doc = " Returns the interface index of the device bound to the socket."]
    #[doc = " @param ring     The PF_RING handle to query."]
    #[doc = " @param if_index The memory area where the interface index will be copied"]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_get_bound_device_ifindex(
        ring: *mut pfring,
        if_index: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Return the interface index of the provided device."]
    #[doc = " @param ring        The PF_RING handle."]
    #[doc = " @param device_name The device name."]
    #[doc = " @param if_index    The memory area for storing the interface index."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_get_device_ifindex(
        ring: *mut pfring,
        device_name: *mut libc::c_char,
        if_index: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Set a filtering device."]
    #[doc = " @param ring The PF_RING handle."]
    #[doc = " @param info The filtering device info."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_virtual_device(
        ring: *mut pfring,
        info: *mut virtual_filtering_device_info,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " This call processes packets until pfring_breakloop() is called or an error occurs."]
    #[doc = " @param ring            The PF_RING handle."]
    #[doc = " @param looper          A callback to be called for each received packet. The parameters passed to this routine are:"]
    #[doc = "                        a pointer to a struct pfring_pkthdr, a pointer to the packet memory, and a pointer to user_bytes."]
    #[doc = " @param user_bytes      A pointer to user\u{2019}s data which is passed to the callback."]
    #[doc = " @param wait_for_packet If 0 active wait is used to check the packet availability."]
    #[doc = " @return A non-negative number if pfring_breakloop() is called. A negative number in case of error."]
    pub fn pfring_loopback_test(
        ring: *mut pfring,
        buffer: *mut libc::c_char,
        buffer_len: c_uint,
        test_len: c_uint,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " When a ring is created, it is not enabled (i.e. incoming packets are dropped) until the above function is called."]
    #[doc = " @param ring The PF_RING handle to enable."]
    #[doc = " @return 0 on success, a negative value otherwise (e.g. the ring cannot be enabled)."]
    pub fn pfring_enable_ring(ring: *mut pfring) -> libc::c_int;
}
extern "C" {
    #[doc = " Disable a ring."]
    #[doc = " @param ring The PF_RING handle to disable."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_disable_ring(ring: *mut pfring) -> libc::c_int;
}
extern "C" {
    #[doc = " In order to set BPF filters through the PF_RING API it\u{2019}s necessary to enable (this is the default) BPF support"]
    #[doc = " at compile time and link PF_RING-enabled applications against the -lpcap library (it is possible to disable the"]
    #[doc = " BPF support with \"cd userland/lib/; ./configure --disable-bpf; make\" to avoid linking libpcap)."]
    #[doc = " @param ring          The PF_RING handle on which the filter will be set."]
    #[doc = " @param filter_buffer The filter to set."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_bpf_filter(
        ring: *mut pfring,
        filter_buffer: *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Remove the BPF filter."]
    #[doc = " @param ring The PF_RING handle."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_remove_bpf_filter(ring: *mut pfring) -> libc::c_int;
}
extern "C" {
    #[doc = " Sets the filtering mode (software only, hardware only, both software and hardware) in order to implicitly"]
    #[doc = " add/remove hardware rules by means of the same API functionality used for software (wildcard and hash) rules."]
    #[doc = " @param ring The PF_RING handle on which the rule will be removed."]
    #[doc = " @param mode The filtering mode."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_filtering_mode(
        ring: *mut pfring,
        mode: filtering_mode,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Reads the time from the device hardware clock, when the adapter supports hardware timestamping."]
    #[doc = " @param ring The PF_RING handle."]
    #[doc = " @param ts   The struct where time will be stored."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_get_device_clock(ring: *mut pfring, ts: *mut timespec) -> libc::c_int;
}
extern "C" {
    #[doc = " Sets the time in the device hardware clock, when the adapter supports hardware timestamping."]
    #[doc = " @param ring The PF_RING handle."]
    #[doc = " @param ts   The time to be set."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_device_clock(ring: *mut pfring, ts: *mut timespec) -> libc::c_int;
}
extern "C" {
    #[doc = " Adjust the time in the device hardware clock with an offset, when the adapter supports hardware timestamping."]
    #[doc = " @param ring   The PF_RING handle."]
    #[doc = " @param offset The time offset."]
    #[doc = " @param sign   The offset sign."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_adjust_device_clock(
        ring: *mut pfring,
        offset: *mut timespec,
        sign: i8,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Synchronizes the ingress ring indexes/registers with the kernel."]
    #[doc = " @param ring The PF_RING handle."]
    pub fn pfring_sync_indexes_with_kernel(ring: *mut pfring);
}
extern "C" {
    #[doc = " Send the last received packet to the specified device. This is an optimization working with standard PF_RING only."]
    #[doc = " @param ring            The PF_RING handle on which the packet has been received."]
    #[doc = " @param tx_interface_id The egress interface index."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_send_last_rx_packet(
        ring: *mut pfring,
        tx_interface_id: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Return the link status."]
    #[doc = " @param ring The PF_RING handle."]
    #[doc = " @return 1 if link is up, 0 otherwise."]
    pub fn pfring_get_link_status(ring: *mut pfring) -> libc::c_int;
}
extern "C" {
    #[doc = " Synchronizes the egress ring indexes/registers flushing enqueued packets."]
    #[doc = " @param ring The PF_RING handle."]
    #[doc = " @param"]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_flush_tx_packets(ring: *mut pfring) -> libc::c_int;
}
extern "C" {
    #[doc = " Add a string to search in the packet payload (used for filtering)."]
    #[doc = " @param ring             The PF_RING handle."]
    #[doc = " @param string_to_search The string to search."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_search_payload(
        ring: *mut pfring,
        string_to_search: *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Parse a packet."]
    #[doc = " It expects that the hdr memory is either zeroed or contains valid values for the current packet, in order to avoid  parsing twice the same packet headers."]
    #[doc = " This is implemented by controlling the l3_offset and l4_offset fields, indicating that respectively the L2 and L3 layers have been parsed when other than zero."]
    #[doc = " @param pkt           The packet buffer."]
    #[doc = " @param hdr           The header to be filled."]
    #[doc = " @param level         The header level where to stop parsing."]
    #[doc = " @param add_timestamp Add the timestamp."]
    #[doc = " @param add_hash      Compute an IP-based bidirectional hash."]
    #[doc = " @return A non-negative number indicating the topmost header level on success,  a negative value otherwise."]
    pub fn pfring_parse_pkt(
        pkt: *mut c_uchar,
        hdr: *mut pfring_pkthdr,
        level: uint8_t,
        add_timestamp: uint8_t,
        add_hash: uint8_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Set the promiscuous mode flag to a device."]
    #[doc = " @param device      The device name."]
    #[doc = " @param set_promisc The promisc flag."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_if_promisc(
        device: *const libc::c_char,
        set_promisc: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Set the promiscuous mode to bound device."]
    #[doc = " @param ring        The PF_RING handle."]
    #[doc = " @param set_promisc The promisc flag."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_promisc(
        ring: *mut pfring,
        set_promisc: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Format a number."]
    #[doc = " @param val          The value."]
    #[doc = " @param buf          The destination buffer."]
    #[doc = " @param buf_len      The destination buffer length."]
    #[doc = " @param add_decimals A flag indicating whether to add decimals."]
    #[doc = " @return The produced string."]
    pub fn pfring_format_numbers(
        val: f64,
        buf: *mut libc::c_char,
        buf_len: c_uint,
        add_decimals: uint8_t,
    ) -> *mut libc::c_char;
}
extern "C" {
    #[doc = " Enables rx and tx hardware timestamping, when the adapter supports it."]
    #[doc = " @param ring        The PF_RING handle."]
    #[doc = " @param device_name The name of the device where timestamping will be enabled."]
    #[doc = " @param enable_rx   Flag to enable rx timestamping."]
    #[doc = " @param enable_tx   Flag to enable tx timestamping."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_enable_hw_timestamp(
        ring: *mut pfring,
        device_name: *mut libc::c_char,
        enable_rx: uint8_t,
        enable_tx: uint8_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Return the size of the MTU."]
    #[doc = " @param ring The PF_RING handle."]
    #[doc = " @return The MTU size on success, a negative value otherwise."]
    pub fn pfring_get_mtu_size(ring: *mut pfring) -> libc::c_int;
}
extern "C" {
    #[doc = " Return NIC settings: max packet length, num rx/tx slots (ZC only)."]
    #[doc = " @param ring     The PF_RING handle."]
    #[doc = " @param settings The card settings (output)."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_get_card_settings(
        ring: *mut pfring,
        settings: *mut pfring_card_settings,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Print a packet (the header with parsing info must be provided)."]
    #[doc = " @param buff     The destination buffer."]
    #[doc = " @param buff_len The destination buffer length."]
    #[doc = " @param p        The packet."]
    #[doc = " @param h        The header."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_print_parsed_pkt(
        buff: *mut libc::c_char,
        buff_len: c_uint,
        p: *const c_uchar,
        h: *const pfring_pkthdr,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Print a packet."]
    #[doc = " @param buff     The destination buffer."]
    #[doc = " @param buff_len The destination buffer length."]
    #[doc = " @param p        The packet."]
    #[doc = " @param caplen   The packet length."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_print_pkt(
        buff: *mut libc::c_char,
        buff_len: c_uint,
        p: *const c_uchar,
        len: c_uint,
        caplen: c_uint,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Receive a packet chunk, if enabled via pfring_open() flag."]
    #[doc = " @param ring                      The PF_RING handle."]
    #[doc = " @param chunk                     A buffer that will point to the received chunk. Note that the chunk format is adapter specific."]
    #[doc = " @param chunk_info                Informations about the chunk content and length."]
    #[doc = " @param wait_for_incoming_chunk   If 0 active wait is used to check the packet availability."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_recv_chunk(
        ring: *mut pfring,
        chunk: *mut *mut libc::c_void,
        chunk_info: *mut pfring_chunk_info,
        wait_for_incoming_chunk: uint8_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Set a custom device name to which the socket is bound. This function should be called for devices that are not visible via ifconfig"]
    #[doc = " @param ring            The PF_RING handle."]
    #[doc = " @param custom_dev_name The custom device name to be used for this socket."]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_set_bound_dev_name(
        ring: *mut pfring,
        custom_dev_name: *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Reads a IXIA-formatted timestamp from an incoming packet and puts it into the timestamp variable."]
    #[doc = " @param buffer            Incoming packet buffer."]
    #[doc = " @param buffer_len        Incoming packet buffer length."]
    #[doc = " @param ts                If found the hardware timestamp will be placed here"]
    #[doc = " @return The length of the IXIA timestamp (hence 0 means that the timestamp has not been found)."]
    pub fn pfring_read_ixia_hw_timestamp(
        buffer: *mut c_uchar,
        buffer_len: uint32_t,
        ts: *mut timespec,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Strip a IXIA-formatted timestamp from an incoming packet. If the timestamp is found, the"]
    #[doc = " hdr parameter (caplen and len fields) are decreased by the size of the timestamp."]
    #[doc = " @param buffer            Incoming packet buffer."]
    #[doc = " @param hdr               This is an in/out parameter: it is used to read the original packet len, and it is updated (size decreased) if the hw timestamp is found"]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_handle_ixia_hw_timestamp(buffer: *mut c_uchar, hdr: *mut pfring_pkthdr);
}
extern "C" {
    #[doc = " Reads a VSS/APCON-formatted timestamp from an incoming packet and puts it into the timestamp variable."]
    #[doc = " @param buffer            Incoming packet buffer."]
    #[doc = " @param buffer_len        Incoming packet buffer length."]
    #[doc = " @param ts                If found the hardware timestamp will be placed here"]
    #[doc = " @return The length of the VSS/APCON timestamp"]
    pub fn pfring_read_vss_apcon_hw_timestamp(
        buffer: *mut c_uchar,
        buffer_len: uint32_t,
        ts: *mut timespec,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Strip an VSS/APCON-formatted timestamp from an incoming packet. If the timestamp is found, the"]
    #[doc = " hdr parameter (caplen and len fields) are decreased by the size of the timestamp."]
    #[doc = " @param buffer            Incoming packet buffer."]
    #[doc = " @param hdr               This is an in/out parameter: it is used to read the original packet len, and it is updated (size decreased) if the hw timestamp is found"]
    #[doc = " @return 0 on success, a negative value otherwise."]
    pub fn pfring_handle_vss_apcon_hw_timestamp(buffer: *mut c_uchar, hdr: *mut pfring_pkthdr);
}
extern "C" {
    #[doc = " Get interface speed."]
    #[doc = " @param ring"]
    #[doc = " @return 0 if interface speed is unknown, the interface speed (Mbit/s) otherwise."]
    pub fn pfring_get_interface_speed(ring: *mut pfring) -> uint32_t;
}
extern "C" {
    #[doc = " List all interfaces."]
    #[doc = " @return The interface list."]
    pub fn pfring_findalldevs() -> *mut pfring_if_t;
}
extern "C" {
    #[doc = " Free an interface list returned by pfring_findalldevs()."]
    #[doc = " @param list The interface list."]
    pub fn pfring_freealldevs(list: *mut pfring_if_t);
}


extern "C" {
    pub fn pfring_parse_bpf_filter(
        filter_buffer: *mut libc::c_char,
        caplen: c_uint,
        filter: *mut pfring_bpf_program,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pfring_free_bpf_filter(filter: *mut pfring_bpf_program);
}
extern "C" {
    pub fn pfring_bpf_filter(
        bpf_insn: *mut libc::c_void,
        buffer: *mut c_uchar,
        caplen: uint32_t,
        len: uint32_t,
    ) -> uint32_t;
}
extern "C" {
    pub fn gmt_to_local(t: time_t) -> i32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_module_info {
    pub name: *mut libc::c_char,
    pub open:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut pfring) -> libc::c_int>,
    pub findalldevs: ::std::option::Option<unsafe extern "C" fn() -> *mut pfring_if_t>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct thirdparty_func {
    pub name: *const libc::c_char,
    pub ptr: ::std::option::Option<unsafe extern "C" fn()>,
}
extern "C" {
    pub fn pfring_thirdparty_lib_init(
        thirdparty_lib_name: *const libc::c_char,
        thirdparty_function_ptr: *mut thirdparty_func,
    );
}

