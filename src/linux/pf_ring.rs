use crate::{ __BindgenBitfieldUnit, __IncompleteArrayField, };
use crate::libc::{ c_uint, uint8_t, uint16_t, uint32_t, uint64_t, c_uchar, time_t, timeval, };

use crate::libc::{ in6_addr,  };


pub const RING_MAGIC_VALUE: u32 = 136;
pub const RING_FLOWSLOT_VERSION: u32 = 17;
pub const DEFAULT_BUCKET_LEN: u32 = 128;
pub const MAX_NUM_DEVICES: u32 = 256;
pub const MAX_NUM_RING_SOCKETS: u32 = 256;
pub const DEFAULT_MIN_PKT_QUEUED: u32 = 128;
pub const DEFAULT_POLL_WATERMARK_TIMEOUT: u32 = 0;
pub const FILTERING_SAMPLING_RATIO: u32 = 10;
pub const RING_VERSION: &'static [u8; 6usize] = b"7.3.0\0";
pub const RING_VERSION_NUM: u32 = 459520;
pub const SO_ADD_TO_CLUSTER: u32 = 99;
pub const SO_REMOVE_FROM_CLUSTER: u32 = 100;
pub const SO_SET_STRING: u32 = 101;
pub const SO_ADD_FILTERING_RULE: u32 = 102;
pub const SO_REMOVE_FILTERING_RULE: u32 = 103;
pub const SO_TOGGLE_FILTER_POLICY: u32 = 104;
pub const SO_SET_SAMPLING_RATE: u32 = 105;
pub const SO_ACTIVATE_RING: u32 = 106;
pub const SO_RING_BUCKET_LEN: u32 = 107;
pub const SO_SET_CHANNEL_ID: u32 = 108;
pub const SO_PURGE_IDLE_HASH_RULES: u32 = 109;
pub const SO_SET_APPL_NAME: u32 = 110;
pub const SO_SET_PACKET_DIRECTION: u32 = 111;
pub const SO_SET_MASTER_RING: u32 = 112;
pub const SO_ADD_HW_FILTERING_RULE: u32 = 113;
pub const SO_DEL_HW_FILTERING_RULE: u32 = 114;
pub const SO_DEACTIVATE_RING: u32 = 116;
pub const SO_SET_POLL_WATERMARK: u32 = 117;
pub const SO_SET_VIRTUAL_FILTERING_DEVICE: u32 = 118;
pub const SO_REHASH_RSS_PACKET: u32 = 119;
pub const SO_SET_FILTERING_SAMPLING_RATE: u32 = 120;
pub const SO_SET_POLL_WATERMARK_TIMEOUT: u32 = 121;
pub const SO_SHUTDOWN_RING: u32 = 124;
pub const SO_PURGE_IDLE_RULES: u32 = 125;
pub const SO_SET_SOCKET_MODE: u32 = 126;
pub const SO_USE_SHORT_PKT_HEADER: u32 = 127;
pub const SO_ENABLE_RX_PACKET_BOUNCE: u32 = 131;
pub const SO_SET_APPL_STATS: u32 = 133;
pub const SO_SET_STACK_INJECTION_MODE: u32 = 134;
pub const SO_CREATE_CLUSTER_REFEREE: u32 = 135;
pub const SO_PUBLISH_CLUSTER_OBJECT: u32 = 136;
pub const SO_LOCK_CLUSTER_OBJECT: u32 = 137;
pub const SO_UNLOCK_CLUSTER_OBJECT: u32 = 138;
pub const SO_SET_CUSTOM_BOUND_DEV_NAME: u32 = 139;
pub const SO_SET_IFF_PROMISC: u32 = 140;
pub const SO_SET_VLAN_ID: u32 = 141;
pub const SO_GET_RING_VERSION: u32 = 170;
pub const SO_GET_FILTERING_RULE_STATS: u32 = 171;
pub const SO_GET_HASH_FILTERING_RULE_STATS: u32 = 172;
pub const SO_GET_ZC_DEVICE_INFO: u32 = 173;
pub const SO_GET_NUM_RX_CHANNELS: u32 = 174;
pub const SO_GET_RING_ID: u32 = 175;
pub const SO_GET_BPF_EXTENSIONS: u32 = 176;
pub const SO_GET_BOUND_DEVICE_ADDRESS: u32 = 177;
pub const SO_GET_NUM_QUEUED_PKTS: u32 = 178;
pub const SO_GET_PKT_HEADER_LEN: u32 = 179;
pub const SO_GET_LOOPBACK_TEST: u32 = 180;
pub const SO_GET_BUCKET_LEN: u32 = 181;
pub const SO_GET_DEVICE_TYPE: u32 = 182;
pub const SO_GET_EXTRA_DMA_MEMORY: u32 = 183;
pub const SO_GET_BOUND_DEVICE_IFINDEX: u32 = 184;
pub const SO_GET_DEVICE_IFINDEX: u32 = 185;
pub const SO_GET_APPL_STATS_FILE_NAME: u32 = 186;
pub const SO_GET_LINK_STATUS: u32 = 187;
pub const SO_SELECT_ZC_DEVICE: u32 = 190;


pub const PF_RING_ERROR_GENERIC: i32 = -1;
pub const PF_RING_ERROR_INVALID_ARGUMENT: i32 = -2;
pub const PF_RING_ERROR_NO_PKT_AVAILABLE: i32 = -3;
pub const PF_RING_ERROR_NO_TX_SLOT_AVAILABLE: i32 = -4;
pub const PF_RING_ERROR_WRONG_CONFIGURATION: i32 = -5;
pub const PF_RING_ERROR_END_OF_DEMO_MODE: i32 = -6;
pub const PF_RING_ERROR_NOT_SUPPORTED: i32 = -7;
pub const PF_RING_ERROR_INVALID_LIB_VERSION: i32 = -8;
pub const PF_RING_ERROR_UNKNOWN_ADAPTER: i32 = -9;
pub const PF_RING_ERROR_NOT_ENOUGH_MEMORY: i32 = -10;
pub const PF_RING_ERROR_INVALID_STATUS: i32 = -11;
pub const PF_RING_ERROR_RING_NOT_ENABLED: i32 = -12;
pub const REFLECTOR_NAME_LEN: u32 = 8;
pub const NETDEV_PRE_UP: u32 = 13;


#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pkt_offset {
    pub eth_offset: i16,
    pub vlan_offset: i16,
    pub l3_offset: i16,
    pub l4_offset: i16,
    pub payload_offset: i16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ip_addr {
    pub v6: in6_addr,
    pub v4: uint32_t,
    _bindgen_union_align: [u32; 4usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct eth_vlan_hdr {
    pub h_vlan_id: uint16_t,
    pub h_proto: uint16_t,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct kcompact_ipv6_hdr {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
    pub payload_len: uint16_t,
    pub nexthdr: uint8_t,
    pub hop_limit: uint8_t,
    pub saddr: in6_addr,
    pub daddr: in6_addr,
}
impl kcompact_ipv6_hdr {
    #[inline]
    pub fn flow_lbl(&self) -> uint32_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 24u8) as u32) }
    }
    #[inline]
    pub fn set_flow_lbl(&mut self, val: uint32_t) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 24u8, val as u64)
        }
    }
    #[inline]
    pub fn priority(&self) -> uint32_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_priority(&mut self, val: uint32_t) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn version(&self) -> uint32_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(28usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_version(&mut self, val: uint32_t) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(28usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        flow_lbl: uint32_t,
        priority: uint32_t,
        version: uint32_t,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u32> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u32> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 24u8, {
            let flow_lbl: u32 = unsafe { ::std::mem::transmute(flow_lbl) };
            flow_lbl as u64
        });
        __bindgen_bitfield_unit.set(24usize, 4u8, {
            let priority: u32 = unsafe { ::std::mem::transmute(priority) };
            priority as u64
        });
        __bindgen_bitfield_unit.set(28usize, 4u8, {
            let version: u32 = unsafe { ::std::mem::transmute(version) };
            version as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct kcompact_ipv6_opt_hdr {
    pub nexthdr: uint8_t,
    pub hdrlen: uint8_t,
    pub padding: [uint8_t; 6usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct gre_header {
    pub flags_and_version: uint16_t,
    pub proto: uint16_t,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct gtp_v1_hdr {
    pub flags: uint8_t,
    pub message_type: uint8_t,
    pub payload_len: uint16_t,
    pub teid: uint32_t,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct gtp_v1_opt_hdr {
    pub seq_num: uint16_t,
    pub npdu_num: uint8_t,
    pub next_ext_hdr: uint8_t,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct gtp_v1_ext_hdr {
    pub len: uint8_t,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct tunnel_info {
    pub tunnel_id: uint32_t,
    pub tunneled_ip_version: uint8_t,
    pub tunneled_proto: uint8_t,
    pub tunneled_ip_src: ip_addr,
    pub tunneled_ip_dst: ip_addr,
    pub tunneled_l4_src_port: uint16_t,
    pub tunneled_l4_dst_port: uint16_t,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct mobile_ip_hdr {
    pub message_type: uint8_t,
    pub next_header: uint8_t,
    pub reserved: uint16_t,
}
pub const long_pkt_header: pkt_header_len = 0;
pub const short_pkt_header: pkt_header_len = 1;
pub type pkt_header_len = u32;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct pkt_parsing_info {
    pub dmac: [uint8_t; 6usize],
    pub smac: [uint8_t; 6usize],
    pub eth_type: uint16_t,
    pub vlan_id: uint16_t,
    pub qinq_vlan_id: uint16_t,
    pub ip_version: uint8_t,
    pub l3_proto: uint8_t,
    pub ip_tos: uint8_t,
    pub ip_src: ip_addr,
    pub ip_dst: ip_addr,
    pub l4_src_port: uint16_t,
    pub l4_dst_port: uint16_t,
    pub icmp_type: uint8_t,
    pub icmp_code: uint8_t,
    pub tcp: pkt_parsing_info__bindgen_ty_1,
    pub tunnel: tunnel_info,
    pub last_matched_rule_id: i32,
    pub offset: pkt_offset,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pkt_parsing_info__bindgen_ty_1 {
    pub flags: uint8_t,
    pub seq_num: uint32_t,
    pub ack_num: uint32_t,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct pfring_extended_pkthdr {
    pub timestamp_ns: uint64_t,
    pub flags: uint32_t,
    pub rx_direction: uint8_t,
    pub if_index: i32,
    pub pkt_hash: uint32_t,
    pub tx: pfring_extended_pkthdr__bindgen_ty_1,
    pub parsed_pkt: pkt_parsing_info,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_extended_pkthdr__bindgen_ty_1 {
    pub bounce_interface: i32,
    pub reserved: *mut sk_buff,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct pfring_pkthdr {
    pub ts: timeval,
    pub caplen: uint32_t,
    pub len: uint32_t,
    pub extended_hdr: pfring_extended_pkthdr,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct filtering_rule_core_fields {
    pub if_index: i32,
    pub smac: [uint8_t; 6usize],
    pub dmac: [uint8_t; 6usize],
    pub vlan_id: uint16_t,
    pub eth_type: uint16_t,
    pub proto: uint8_t,
    pub shost: ip_addr,
    pub dhost: ip_addr,
    pub shost_mask: ip_addr,
    pub dhost_mask: ip_addr,
    pub sport_low: uint16_t,
    pub sport_high: uint16_t,
    pub dport_low: uint16_t,
    pub dport_high: uint16_t,
    pub tcp: filtering_rule_core_fields__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct filtering_rule_core_fields__bindgen_ty_1 {
    pub flags: uint8_t,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct filtering_rule_extended_fields {
    pub optional_fields: uint16_t,
    pub tunnel: filtering_rule_extended_fields__bindgen_ty_1,
    pub payload_pattern: [::std::os::raw::c_char; 32usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filtering_rule_extended_fields__bindgen_ty_1 {
    pub tunnel_id: uint32_t,
    pub shost: ip_addr,
    pub dhost: ip_addr,
    pub shost_mask: ip_addr,
    pub dhost_mask: ip_addr,
}
pub const forward_packet_and_stop_rule_evaluation: rule_action_behaviour = 0;
pub const dont_forward_packet_and_stop_rule_evaluation: rule_action_behaviour = 1;
pub const execute_action_and_continue_rule_evaluation: rule_action_behaviour = 2;
pub const execute_action_and_stop_rule_evaluation: rule_action_behaviour = 3;
pub const forward_packet_add_rule_and_stop_rule_evaluation: rule_action_behaviour = 4;
pub const reflect_packet_and_stop_rule_evaluation: rule_action_behaviour = 5;
pub const reflect_packet_and_continue_rule_evaluation: rule_action_behaviour = 6;
pub const bounce_packet_and_stop_rule_evaluation: rule_action_behaviour = 7;
pub const bounce_packet_and_continue_rule_evaluation: rule_action_behaviour = 8;
pub type rule_action_behaviour = u32;
pub const pkt_detail_flow: pkt_detail_mode = 0;
pub const pkt_detail_aggregation: pkt_detail_mode = 1;
pub type pkt_detail_mode = u32;
pub const rx_and_tx_direction: packet_direction = 0;
pub const rx_only_direction: packet_direction = 1;
pub const tx_only_direction: packet_direction = 2;
pub type packet_direction = u32;
pub const send_and_recv_mode: socket_mode = 0;
pub const send_only_mode: socket_mode = 1;
pub const recv_only_mode: socket_mode = 2;
pub type socket_mode = u32;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct filtering_internals {
    pub jiffies_last_match: ::std::os::raw::c_ulong,
    pub reflector_dev: *mut net_device,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct filtering_rule {
    pub rule_id: uint16_t,
    pub rule_action: rule_action_behaviour,
    pub balance_id: uint8_t,
    pub balance_pool: uint8_t,
    pub locked: uint8_t,
    pub bidirectional: uint8_t,
    pub core_fields: filtering_rule_core_fields,
    pub extended_fields: filtering_rule_extended_fields,
    pub reflector_device_name: [::std::os::raw::c_char; 8usize],
    pub internals: filtering_internals,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct intel_82599_five_tuple_filter_hw_rule {
    pub proto: uint8_t,
    pub s_addr: uint32_t,
    pub d_addr: uint32_t,
    pub s_port: uint16_t,
    pub d_port: uint16_t,
    pub queue_id: uint16_t,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct intel_82599_perfect_filter_hw_rule {
    pub vlan_id: uint16_t,
    pub proto: uint8_t,
    pub s_addr: uint32_t,
    pub d_addr: uint32_t,
    pub s_port: uint16_t,
    pub d_port: uint16_t,
    pub queue_id: uint16_t,
}
pub const drop_rule: silicom_redirector_rule_type = 0;
pub const redirect_rule: silicom_redirector_rule_type = 1;
pub const mirror_rule: silicom_redirector_rule_type = 2;
pub type silicom_redirector_rule_type = u32;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct silicom_redirector_hw_rule {
    pub rule_type: silicom_redirector_rule_type,
    pub rule_port: uint8_t,
    pub rule_target_port: uint8_t,
    pub vlan_id_low: uint16_t,
    pub vlan_id_high: uint16_t,
    pub l3_proto: uint8_t,
    pub src_addr: ip_addr,
    pub dst_addr: ip_addr,
    pub src_mask: uint32_t,
    pub dst_mask: uint32_t,
    pub src_port_low: uint16_t,
    pub src_port_high: uint16_t,
    pub dst_port_low: uint16_t,
    pub dst_port_high: uint16_t,
}
pub const accolade_drop: accolade_rule_action_type = 0;
pub const accolade_pass: accolade_rule_action_type = 1;
pub type accolade_rule_action_type = u32;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct accolade_hw_rule {
    pub action: accolade_rule_action_type,
    pub port_mask: uint32_t,
    pub ip_version: uint8_t,
    pub protocol: uint8_t,
    pub vlan_id: uint16_t,
    pub mpls_label: uint32_t,
    pub src_addr: ip_addr,
    pub dst_addr: ip_addr,
    pub src_addr_bits: uint32_t,
    pub dst_addr_bits: uint32_t,
    pub src_port_low: uint16_t,
    pub src_port_high: uint16_t,
    pub dst_port_low: uint16_t,
    pub dst_port_high: uint16_t,
    pub l4_port_not: uint8_t,
}
pub const flow_drop_rule: generic_flow_rule_action_type = 0;
pub const flow_mark_rule: generic_flow_rule_action_type = 1;
pub type generic_flow_rule_action_type = u32;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct generic_flow_id_hw_rule {
    pub action: generic_flow_rule_action_type,
    pub flow_id: uint32_t,
    pub thread: uint32_t,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct generic_flow_tuple_hw_rule {
    pub action: generic_flow_rule_action_type,
    pub src_ip: ip_addr,
    pub dst_ip: ip_addr,
    pub src_port: uint16_t,
    pub dst_port: uint16_t,
    pub ip_version: uint8_t,
    pub protocol: uint8_t,
    pub interface: uint8_t,
}
pub const intel_82599_five_tuple_rule: hw_filtering_rule_type = 0;
pub const intel_82599_perfect_filter_rule: hw_filtering_rule_type = 1;
pub const silicom_redirector_rule: hw_filtering_rule_type = 2;
pub const generic_flow_id_rule: hw_filtering_rule_type = 3;
pub const generic_flow_tuple_rule: hw_filtering_rule_type = 4;
pub const accolade_rule: hw_filtering_rule_type = 5;
pub const accolade_default: hw_filtering_rule_type = 6;
pub type hw_filtering_rule_type = u32;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hw_filtering_rule {
    pub rule_family_type: hw_filtering_rule_type,
    pub rule_id: uint16_t,
    pub rule_family: hw_filtering_rule__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hw_filtering_rule__bindgen_ty_1 {
    pub five_tuple_rule: intel_82599_five_tuple_filter_hw_rule,
    pub perfect_rule: intel_82599_perfect_filter_hw_rule,
    pub redirector_rule: silicom_redirector_hw_rule,
    pub flow_id_rule: generic_flow_id_hw_rule,
    pub flow_tuple_rule: generic_flow_tuple_hw_rule,
    pub accolade_rule: accolade_hw_rule,
    _bindgen_union_align: [u8; 65usize],
}
pub const add_hw_rule: hw_filtering_rule_command = 0;
pub const remove_hw_rule: hw_filtering_rule_command = 1;
pub type hw_filtering_rule_command = u32;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_timespec {
    pub tv_sec: uint32_t,
    pub tv_nsec: uint32_t,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct generic_flow_update {
    pub flow_id: uint32_t,
    pub ip_version: uint8_t,
    pub l4_protocol: uint8_t,
    pub tos: uint8_t,
    pub tcp_flags: uint8_t,
    pub src_ip: ip_addr,
    pub dst_ip: ip_addr,
    pub src_port: uint16_t,
    pub dst_port: uint16_t,
    pub fwd_packets: uint32_t,
    pub fwd_bytes: uint32_t,
    pub rev_packets: uint32_t,
    pub rev_bytes: uint32_t,
    pub fwd_ts_first: pfring_timespec,
    pub fwd_ts_last: pfring_timespec,
    pub rev_ts_first: pfring_timespec,
    pub rev_ts_last: pfring_timespec,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct generic_flow_feedback {
    pub action: generic_flow_rule_action_type,
    pub flow_id: uint32_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pf_ring_socket {
    _unused: [u8; 0],
}
extern "C" {
    #[link_name = "\u{1}pfr"]
    pub static mut pfr: *mut pf_ring_socket;
}

pub type five_tuple_rule_handler = ::std::option::Option<
    unsafe extern "C" fn(
        pfr: *mut pf_ring_socket,
        rule: *mut hw_filtering_rule,
        request: hw_filtering_rule_command,
    ) -> ::std::os::raw::c_int,
>;
pub type perfect_filter_hw_rule_handler = ::std::option::Option<
    unsafe extern "C" fn(
        pfr: *mut pf_ring_socket,
        rule: *mut hw_filtering_rule,
        request: hw_filtering_rule_command,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct hw_filtering_device_handler {
    pub five_tuple_handler: five_tuple_rule_handler,
    pub perfect_filter_handler: perfect_filter_hw_rule_handler,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct hash_filtering_rule {
    pub rule_id: uint16_t,
    pub vlan_id: uint16_t,
    pub ip_version: uint8_t,
    pub proto: uint8_t,
    pub host_peer_a: ip_addr,
    pub host_peer_b: ip_addr,
    pub port_peer_a: uint16_t,
    pub port_peer_b: uint16_t,
    pub rule_action: rule_action_behaviour,
    pub reflector_device_name: [::std::os::raw::c_char; 8usize],
    pub internals: filtering_internals,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct hash_filtering_rule_stats {
    pub match_: uint64_t,
    pub filtered: uint64_t,
    pub match_forward: uint64_t,
    pub inactivity: uint32_t,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct _sw_filtering_hash_bucket {
    pub rule: hash_filtering_rule,
    pub match_: uint64_t,
    pub filtered: uint64_t,
    pub match_forward: uint64_t,
    pub next: *mut _sw_filtering_hash_bucket,
}
pub type sw_filtering_hash_bucket = _sw_filtering_hash_bucket;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct flowSlotInfo {
    pub version: uint16_t,
    pub sample_rate: uint16_t,
    pub min_num_slots: uint32_t,
    pub slot_len: uint32_t,
    pub data_len: uint32_t,
    pub tot_mem: uint64_t,
    pub insert_off: uint64_t,
    pub kernel_remove_off: uint64_t,
    pub tot_pkts: uint64_t,
    pub tot_lost: uint64_t,
    pub tot_insert: uint64_t,
    pub kernel_tot_read: uint64_t,
    pub tot_fwd_ok: uint64_t,
    pub tot_fwd_notok: uint64_t,
    pub good_pkt_sent: uint64_t,
    pub pkt_send_error: uint64_t,
    pub padding: [::std::os::raw::c_char; 24usize],
    pub k_padding: [::std::os::raw::c_char; 3968usize],
    pub tot_read: uint64_t,
    pub remove_off: uint64_t,
    pub u_padding: [::std::os::raw::c_char; 4080usize],
}
pub type FlowSlotInfo = flowSlotInfo;
pub type zc_dev_wait_packet = ::std::option::Option<
    unsafe extern "C" fn(
        adapter: *mut ::std::os::raw::c_void,
        mode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type zc_dev_notify = ::std::option::Option<
    unsafe extern "C" fn(
        rx_adapter_ptr: *mut ::std::os::raw::c_void,
        tx_adapter_ptr: *mut ::std::os::raw::c_void,
        device_in_use: uint8_t,
    ),
>;
pub const add_device_mapping: zc_dev_operation = 0;
pub const remove_device_mapping: zc_dev_operation = 1;
pub type zc_dev_operation = u32;
pub const intel_e1000e: zc_dev_model = 0;
pub const intel_igb: zc_dev_model = 1;
pub const intel_ixgbe: zc_dev_model = 2;
pub const intel_ixgbe_82598: zc_dev_model = 3;
pub const intel_ixgbe_82599: zc_dev_model = 4;
pub const intel_igb_82580: zc_dev_model = 5;
pub const intel_e1000: zc_dev_model = 6;
pub const intel_ixgbe_82599_ts: zc_dev_model = 7;
pub const intel_i40e: zc_dev_model = 8;
pub const intel_fm10k: zc_dev_model = 9;
pub const intel_ixgbe_vf: zc_dev_model = 10;
pub type zc_dev_model = u32;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct mem_ring_info {
    pub packet_memory_num_slots: uint32_t,
    pub packet_memory_slot_len: uint32_t,
    pub descr_packet_memory_tot_len: uint32_t,
    pub registers_index: uint16_t,
    pub stats_index: uint16_t,
    pub vector: uint32_t,
    pub num_queues: uint32_t,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct zc_memory_info {
    pub rx: mem_ring_info,
    pub tx: mem_ring_info,
    pub phys_card_memory_len: uint32_t,
    pub device_model: zc_dev_model,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct zc_dev_info {
    pub mem_info: zc_memory_info,
    pub channel_id: uint16_t,
    pub rx_descr_packet_memory: *mut ::std::os::raw::c_void,
    pub tx_descr_packet_memory: *mut ::std::os::raw::c_void,
    pub phys_card_memory: *mut ::std::os::raw::c_char,
    pub dev: *mut net_device,
    pub hwdev: *mut device,
    pub device_address: [c_uchar; 6usize],
    pub packet_waitqueue: *mut ::std::os::raw::c_void,
    pub interrupt_received: *mut uint8_t,
    pub in_use: uint8_t,
    pub rx_adapter_ptr: *mut ::std::os::raw::c_void,
    pub tx_adapter_ptr: *mut ::std::os::raw::c_void,
    pub wait_packet_function_ptr: zc_dev_wait_packet,
    pub usage_notification: zc_dev_notify,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct zc_dev_mapping {
    pub operation: zc_dev_operation,
    pub device_name: [::std::os::raw::c_char; 16usize],
    pub channel_id: i32,
}
pub const cluster_per_flow: cluster_type = 0;
pub const cluster_round_robin: cluster_type = 1;
pub const cluster_per_flow_2_tuple: cluster_type = 2;
pub const cluster_per_flow_4_tuple: cluster_type = 3;
pub const cluster_per_flow_5_tuple: cluster_type = 4;
pub const cluster_per_flow_tcp_5_tuple: cluster_type = 5;
pub const cluster_per_inner_flow: cluster_type = 6;
pub const cluster_per_inner_flow_2_tuple: cluster_type = 7;
pub const cluster_per_inner_flow_4_tuple: cluster_type = 8;
pub const cluster_per_inner_flow_5_tuple: cluster_type = 9;
pub const cluster_per_inner_flow_tcp_5_tuple: cluster_type = 10;
pub const cluster_per_flow_ip_5_tuple: cluster_type = 11;
pub const cluster_per_inner_flow_ip_5_tuple: cluster_type = 12;
pub type cluster_type = u32;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct add_to_cluster {
    pub clusterId: c_uint,
    pub the_type: cluster_type,
}
pub const standard_nic_family: pfring_device_type = 0;
pub const intel_82599_family: pfring_device_type = 1;
pub type pfring_device_type = u32;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct virtual_filtering_device_info {
    pub device_name: [::std::os::raw::c_char; 16usize],
    pub device_type: pfring_device_type,
    pub proc_entry: *mut proc_dir_entry,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct create_cluster_referee_info {
    pub cluster_id: uint32_t,
    pub recovered: uint32_t,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct public_cluster_object_info {
    pub cluster_id: uint32_t,
    pub object_type: uint32_t,
    pub object_id: uint32_t,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct lock_cluster_object_info {
    pub cluster_id: uint32_t,
    pub object_type: uint32_t,
    pub object_id: uint32_t,
    pub lock_mask: uint32_t,
    pub reserved: uint32_t,
}
pub const cluster_slave: cluster_client_type = 0;
pub const cluster_master: cluster_client_type = 1;
pub type cluster_client_type = u32;


// Kernel
//
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sk_buff {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct net_device {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct device {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct proc_dir_entry {
    pub _address: u8,
}