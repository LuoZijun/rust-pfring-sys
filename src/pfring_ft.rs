use crate::{ __BindgenBitfieldUnit, __IncompleteArrayField, };

use crate::libc::{ self, c_uint, uint8_t, uint16_t, uint32_t, uint64_t, c_uchar, time_t, timeval, };


pub type pfring_ft_table = libc::c_void;
pub type pfring_ft_list = libc::c_void;
pub type pfring_ft_flow = libc::c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ndpi_detection_module_struct {
    _unused: [u8; 0],
}

pub const PFRING_FT_ACTION_DEFAULT: pfring_ft_action = 0;
pub const PFRING_FT_ACTION_FORWARD: pfring_ft_action = 1;
pub const PFRING_FT_ACTION_DISCARD: pfring_ft_action = 2;

#[doc = " enums"]
pub type pfring_ft_action = u32;
#[doc = "< Source to destination"]
pub const s2d_direction: pfring_ft_direction = 0;
#[doc = "< Destination to source"]
pub const d2s_direction: pfring_ft_direction = 1;
pub const NUM_DIRECTIONS: pfring_ft_direction = 2;

pub type pfring_ft_direction = u32;
#[doc = " packet header structs"]
pub type pfring_ft_in4_addr = uint32_t;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct pfring_ft_in6_addr {
    pub u6_addr: pfring_ft_in6_addr__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pfring_ft_in6_addr__bindgen_ty_1 {
    pub u6_addr8: [uint8_t; 16usize],
    pub u6_addr16: [uint16_t; 8usize],
    pub u6_addr32: [uint32_t; 4usize],
    _bindgen_union_align: [u32; 4usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_ft_iphdr {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub tos: uint8_t,
    pub tot_len: uint16_t,
    pub id: uint16_t,
    pub frag_off: uint16_t,
    pub ttl: uint8_t,
    pub protocol: uint8_t,
    pub check: uint16_t,
    pub saddr: uint32_t,
    pub daddr: uint32_t,
}
impl pfring_ft_iphdr {
    #[inline]
    pub fn ihl(&self) -> uint8_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_ihl(&mut self, val: uint8_t) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn version(&self) -> uint8_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_version(&mut self, val: uint8_t) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        ihl: uint8_t,
        version: uint8_t,
    ) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let ihl: u8 = unsafe { ::std::mem::transmute(ihl) };
            ihl as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let version: u8 = unsafe { ::std::mem::transmute(version) };
            version as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct pfring_ft_ipv6hdr {
    pub ip6_un1_flow: uint32_t,
    pub ip6_un1_plen: uint16_t,
    pub ip6_un1_nxt: uint8_t,
    pub ip6_un1_hlim: uint8_t,
    pub ip6_src: pfring_ft_in6_addr,
    pub ip6_dst: pfring_ft_in6_addr,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_ft_tcphdr {
    pub source: uint16_t,
    pub dest: uint16_t,
    pub seq: uint32_t,
    pub ack_seq: uint32_t,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize], u8>,
    pub window: uint16_t,
    pub check: uint16_t,
    pub urg_ptr: uint16_t,
}
impl pfring_ft_tcphdr {
    #[inline]
    pub fn res1(&self) -> uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u16) }
    }
    #[inline]
    pub fn set_res1(&mut self, val: uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn doff(&self) -> uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u16) }
    }
    #[inline]
    pub fn set_doff(&mut self, val: uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn fin(&self) -> uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_fin(&mut self, val: uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn syn(&self) -> uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_syn(&mut self, val: uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn rst(&self) -> uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_rst(&mut self, val: uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn psh(&self) -> uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_psh(&mut self, val: uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn ack(&self) -> uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_ack(&mut self, val: uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn urg(&self) -> uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_urg(&mut self, val: uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn ece(&self) -> uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_ece(&mut self, val: uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn cwr(&self) -> uint16_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(15usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set_cwr(&mut self, val: uint16_t) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        res1: uint16_t,
        doff: uint16_t,
        fin: uint16_t,
        syn: uint16_t,
        rst: uint16_t,
        psh: uint16_t,
        ack: uint16_t,
        urg: uint16_t,
        ece: uint16_t,
        cwr: uint16_t,
    ) -> __BindgenBitfieldUnit<[u8; 2usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let res1: u16 = unsafe { ::std::mem::transmute(res1) };
            res1 as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let doff: u16 = unsafe { ::std::mem::transmute(doff) };
            doff as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let fin: u16 = unsafe { ::std::mem::transmute(fin) };
            fin as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let syn: u16 = unsafe { ::std::mem::transmute(syn) };
            syn as u64
        });
        __bindgen_bitfield_unit.set(10usize, 1u8, {
            let rst: u16 = unsafe { ::std::mem::transmute(rst) };
            rst as u64
        });
        __bindgen_bitfield_unit.set(11usize, 1u8, {
            let psh: u16 = unsafe { ::std::mem::transmute(psh) };
            psh as u64
        });
        __bindgen_bitfield_unit.set(12usize, 1u8, {
            let ack: u16 = unsafe { ::std::mem::transmute(ack) };
            ack as u64
        });
        __bindgen_bitfield_unit.set(13usize, 1u8, {
            let urg: u16 = unsafe { ::std::mem::transmute(urg) };
            urg as u64
        });
        __bindgen_bitfield_unit.set(14usize, 1u8, {
            let ece: u16 = unsafe { ::std::mem::transmute(ece) };
            ece as u64
        });
        __bindgen_bitfield_unit.set(15usize, 1u8, {
            let cwr: u16 = unsafe { ::std::mem::transmute(cwr) };
            cwr as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_ft_udphdr {
    pub source: uint16_t,
    pub dest: uint16_t,
    pub len: uint16_t,
    pub check: uint16_t,
}
#[doc = " packet metadata structs"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfring_ft_pcap_pkthdr {
    #[doc = "< time stamp"]
    pub ts: timeval,
    #[doc = "< length of captured portion"]
    pub caplen: uint32_t,
    #[doc = "< length original packet (off wire)"]
    pub len: uint32_t,
}

impl core::fmt::Debug for pfring_ft_pcap_pkthdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "pfring_ft_pcap_pkthdr {{ ts: timeval {{ tv_sec: {:?}, tv_usec: {:?} }}, caplen: {:?}, len: {:?} }}",
            self.ts.tv_sec, self.ts.tv_usec,
            self.caplen,
            self.len)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_ft_ext_pkthdr {
    #[doc = "< packet hash"]
    pub hash: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfring_ft_packet_metadata {
    pub hdr: *mut pfring_ft_pcap_pkthdr,
    pub ext_hdr: *mut pfring_ft_ext_pkthdr,
    pub direction: pfring_ft_direction,
    pub vlan_id: uint16_t,
    pub ip_version: uint8_t,
    pub l4_proto: uint8_t,
    pub payload_len: uint16_t,
    pub reserved: uint16_t,
    pub l3: pfring_ft_packet_metadata__bindgen_ty_1,
    pub l4: pfring_ft_packet_metadata__bindgen_ty_2,
    pub payload: *const c_uchar,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pfring_ft_packet_metadata__bindgen_ty_1 {
    pub ip4: *mut pfring_ft_iphdr,
    pub ip6: *mut pfring_ft_ipv6hdr,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pfring_ft_packet_metadata__bindgen_ty_2 {
    pub tcp: *mut pfring_ft_tcphdr,
    pub udp: *mut pfring_ft_udphdr,
    _bindgen_union_align: u64,
}
#[doc = " flow metadata structs"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union pfring_ft_ip_address {
    pub v4: pfring_ft_in4_addr,
    pub v6: pfring_ft_in6_addr,
    _bindgen_union_align: [u32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_ft_ndpi_protocol {
    #[doc = "< e.g. HTTP"]
    pub master_protocol: uint16_t,
    #[doc = "< e.g. FaceBook"]
    pub app_protocol: uint16_t,
    pub category: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfring_ft_flow_key {
    #[doc = "< Source IP address"]
    pub saddr: pfring_ft_ip_address,
    #[doc = "< Destination IP address"]
    pub daddr: pfring_ft_ip_address,
    #[doc = "< IP version"]
    pub ip_version: uint8_t,
    #[doc = "< L4 protocol"]
    pub protocol: uint8_t,
    #[doc = "< Source port"]
    pub sport: uint16_t,
    #[doc = "< Destination port"]
    pub dport: uint16_t,
    #[doc = "< VLAN ID"]
    pub vlan_id: uint16_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_ft_flow_value {
    #[doc = "< Metadata per flow direction"]
    pub direction: [pfring_ft_flow_value__bindgen_ty_1; 2usize],
    #[doc = "< nDPI protocol"]
    pub l7_protocol: pfring_ft_ndpi_protocol,
    #[doc = "< User metadata"]
    pub user: *mut libc::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfring_ft_flow_value__bindgen_ty_1 {
    #[doc = "< Number of packets per direction"]
    pub pkts: uint64_t,
    #[doc = "< Number of bytes per direction"]
    pub bytes: uint64_t,
    #[doc = "< Time of first packet seen per direction"]
    pub first: timeval,
    #[doc = "< Time of last packet seen per direction"]
    pub last: timeval,
    #[doc = "< TCP flags per direction"]
    pub tcp_flags: uint8_t,
}

impl core::fmt::Debug for pfring_ft_flow_value__bindgen_ty_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "pfring_ft_flow_value__bindgen_ty_1 {{ pkts: {:?}, bytes: {:?}",
            self.pkts,
            self.bytes)?;
        write!(f, ", first: {{ tv_sec: {:?}, tv_usec: {:?} }}",
            self.first.tv_sec,
            self.first.tv_usec)?;
        write!(f, ", last: {{ tv_sec: {:?}, tv_usec: {:?} }}",
            self.last.tv_sec,
            self.last.tv_usec)?;
        write!(f, ", tcp_flags: {:?} }}", self.tcp_flags)
    }
}


#[doc = " stats struct"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_ft_stats {
    #[doc = "< Number of currently active flows"]
    pub active_flows: uint64_t,
    #[doc = "< Number of total flows"]
    pub flows: uint64_t,
    #[doc = "< Flow creation errors due to no room left in the flow table"]
    pub err_no_room: uint64_t,
    #[doc = "< Flow creation errors due to memory allocation failures"]
    pub err_no_mem: uint64_t,
    #[doc = "< Number of packets not processed because L3 header was missing"]
    pub disc_no_ip: uint64_t,
    #[doc = "< Maximum collition list depth during flow lookup"]
    pub max_lookup_depth: uint64_t,
}
#[doc = " Callbacks prototypes"]
pub type pfring_ft_export_list_func = ::std::option::Option<
    unsafe extern "C" fn(flows_list: *mut pfring_ft_list, user: *mut libc::c_void),
>;
pub type pfring_ft_export_flow_func = ::std::option::Option<
    unsafe extern "C" fn(flow: *mut pfring_ft_flow, user: *mut libc::c_void),
>;
pub type pfring_ft_flow_packet_func = ::std::option::Option<
    unsafe extern "C" fn(
        data: *const c_uchar,
        metadata: *mut pfring_ft_packet_metadata,
        flow: *mut pfring_ft_flow,
        user: *mut libc::c_void,
    ),
>;
extern "C" {
    #[doc = " Create a new flow table."]
    #[doc = " @param flags Flags to enable selected flow table features."]
    #[doc = " @param max_flows Maximum number of concurrent flows the table should be able to handle (use 0 if not sure to use default settings)."]
    #[doc = " @param flow_idle_timeout Maximum flow idle time (seconds) before expiration (use 0 if not sure to use default: 30s)."]
    #[doc = " @param flow_lifetime_timeout Maximum flow duration (seconds) before expiration (use 0 if not sure to use default: 2m)."]
    #[doc = " @return The flow table on success, NULL on failure."]
    pub fn pfring_ft_create_table(
        flags: uint32_t,
        max_flows: uint32_t,
        flow_idle_timeout: uint32_t,
        flow_lifetime_timeout: uint32_t,
    ) -> *mut pfring_ft_table;
}
extern "C" {
    #[doc = " Destroy a flow table."]
    #[doc = " @param table The flow table handle."]
    pub fn pfring_ft_destroy_table(table: *mut pfring_ft_table);
}
extern "C" {
    #[doc = " Set the function to be called when a new flow has been created."]
    #[doc = " @param table The flow table handle."]
    #[doc = " @param callback The callback."]
    #[doc = " @param user The user data provided to the callback."]
    pub fn pfring_ft_set_new_flow_callback(
        table: *mut pfring_ft_table,
        callback: pfring_ft_export_flow_func,
        user: *mut libc::c_void,
    );
}
extern "C" {
    #[doc = " Set the function to be called when a packet and its flow have been processed, for each packet."]
    #[doc = " @param table The flow table handle."]
    #[doc = " @param callback The callback."]
    #[doc = " @param user The user data provided to the callback."]
    pub fn pfring_ft_set_flow_packet_callback(
        table: *mut pfring_ft_table,
        callback: pfring_ft_flow_packet_func,
        user: *mut libc::c_void,
    );
}
extern "C" {
    #[doc = " Set the function to be called when a packet and its flow have been processed and the l7 protocol has been just detected."]
    #[doc = " @param table The flow table handle."]
    #[doc = " @param callback The callback."]
    #[doc = " @param user The user data provided to the callback."]
    pub fn pfring_ft_set_l7_detected_callback(
        table: *mut pfring_ft_table,
        callback: pfring_ft_flow_packet_func,
        user: *mut libc::c_void,
    );
}
extern "C" {
    #[doc = " Set the function to be called when a flow expires and needs to be exported."]
    #[doc = " The callback should release the flow calling pfring_ft_flow_free(flow)."]
    #[doc = " @param table The flow table handle."]
    #[doc = " @param callback The callback."]
    #[doc = " @param user The user data provided to the callback."]
    pub fn pfring_ft_set_flow_export_callback(
        table: *mut pfring_ft_table,
        callback: pfring_ft_export_flow_func,
        user: *mut libc::c_void,
    );
}
extern "C" {
    #[doc = " Set the function to be called when a some flow expires and need to be exported."]
    #[doc = " This can be used as an optimised alternative to pfring_ft_set_flow_export_callback()."]
    #[doc = " The callback should release all flows in the list calling pfring_ft_flow_free(flow) for each flow."]
    #[doc = " It is possible to iterate all the flows in the list using pfring_ft_list_get_next()."]
    #[doc = " @param table The flow table handle."]
    #[doc = " @param callback The callback."]
    #[doc = " @param user The user data provided to the callback."]
    pub fn pfring_ft_set_flow_list_export_callback(
        table: *mut pfring_ft_table,
        callback: pfring_ft_export_list_func,
        user: *mut libc::c_void,
    );
}
extern "C" {
    #[doc = " Provide a raw packet to the flow table for processing. Usually the main"]
    #[doc = " capture loop provides all the packets to the hash table calling this function."]
    #[doc = " @param table The flow table handle."]
    #[doc = " @param packet The raw packet."]
    #[doc = " @param header The packet metadata (including length and timestamp)."]
    #[doc = " @param ext_header Additional packet metadata not available in the pcap header (including hash)."]
    #[doc = " @return The action for the packet, in case filtering rules have been specified."]
    pub fn pfring_ft_process(
        table: *mut pfring_ft_table,
        packet: *const c_uchar,
        header: *const pfring_ft_pcap_pkthdr,
        ext_header: *const pfring_ft_ext_pkthdr,
    ) -> pfring_ft_action;
}
extern "C" {
    #[doc = " This should be called when there is no packet to be processed and the"]
    #[doc = " main loop is idle, for running housekeeping activities in the flow table."]
    #[doc = " @param table The flow table handle."]
    #[doc = " @param epoch The current epoch (sec)."]
    #[doc = " @return 1 if there is more work to do, 0 if the caller can sleep a bit."]
    pub fn pfring_ft_housekeeping(
        table: *mut pfring_ft_table,
        epoch: uint32_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Flush all flows (usually called on program termination, before destroying the flow table)."]
    #[doc = " @param table The flow table handle."]
    pub fn pfring_ft_flush(table: *mut pfring_ft_table);
}
extern "C" {
    #[doc = " Pop the next from a flow list."]
    #[doc = " @param list The flow list."]
    #[doc = " @return The flow if the list is not empty, NULL otherwise."]
    pub fn pfring_ft_list_get_next(list: *mut pfring_ft_list) -> *mut pfring_ft_flow;
}
extern "C" {
    #[doc = " Get the flow key."]
    #[doc = " @param flow The flow handle."]
    #[doc = " @return The flow key."]
    pub fn pfring_ft_flow_get_key(flow: *mut pfring_ft_flow) -> *mut pfring_ft_flow_key;
}
extern "C" {
    #[doc = " Get the flow value."]
    #[doc = " @param flow The flow handle."]
    #[doc = " @return The flow value."]
    pub fn pfring_ft_flow_get_value(flow: *mut pfring_ft_flow) -> *mut pfring_ft_flow_value;
}
extern "C" {
    #[doc = " Set the flow action, to be returned by pfring_ft_process() for all packets for this flow."]
    #[doc = " @param flow The flow handle."]
    #[doc = " @param action The action."]
    pub fn pfring_ft_flow_set_action(flow: *mut pfring_ft_flow, action: pfring_ft_action);
}
extern "C" {
    #[doc = " Get the computed/actual flow action, the same returned by pfring_ft_process() for this flow."]
    #[doc = " @param flow The flow handle."]
    #[doc = " @return The action."]
    pub fn pfring_ft_flow_get_action(flow: *mut pfring_ft_flow) -> pfring_ft_action;
}
extern "C" {
    #[doc = " Release a flow."]
    #[doc = " @param flow The flow handle."]
    pub fn pfring_ft_flow_free(flow: *mut pfring_ft_flow);
}
extern "C" {
    #[doc = " Set the default action for detected L7 protocols with no filtering rule."]
    #[doc = " This can be used to \'drop all\' traffic, exception made for specific protocols"]
    #[doc = " setting the default to PFRING_FT_ACTION_DISCARD and filter actions to PFRING_FT_ACTION_FORWARD"]
    #[doc = " Default: PFRING_FT_ACTION_DEFAULT"]
    #[doc = " @param table The flow table handle."]
    #[doc = " @param protocol_name The nDPI protocol name."]
    #[doc = " @param action The action returned by pfring_ft_process() for all packets matching the protocol."]
    pub fn pfring_ft_set_default_action(table: *mut pfring_ft_table, action: pfring_ft_action);
}
extern "C" {
    #[doc = " Load filtering/shunting rules from a configuration file."]
    #[doc = " Please refer to the documentation for the file format."]
    #[doc = " @param table The flow table handle."]
    #[doc = " @param path The configuration file path."]
    #[doc = " @return 0 on success, a negative number on failures."]
    pub fn pfring_ft_load_configuration(
        table: *mut pfring_ft_table,
        path: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Set a shunt rule for a L7 protocol."]
    #[doc = " @param table The flow table handle."]
    #[doc = " @param protocol_name The nDPI protocol name."]
    #[doc = " @param packets The number of packets before shunting the flow returning a discard action from pfring_ft_process()."]
    pub fn pfring_ft_set_shunt_protocol_by_name(
        table: *mut pfring_ft_table,
        protocol_name: *const libc::c_char,
        packets: uint8_t,
    );
}
extern "C" {
    #[doc = " Set a filtering rule for a L7 protocol."]
    #[doc = " @param table The flow table handle."]
    #[doc = " @param protocol_name The nDPI protocol name."]
    #[doc = " @param action The action returned by pfring_ft_process() for all packets matching the protocol."]
    pub fn pfring_ft_set_filter_protocol_by_name(
        table: *mut pfring_ft_table,
        protocol_name: *const libc::c_char,
        action: pfring_ft_action,
    );
}
extern "C" {
    #[doc = " Return the L7 protocol name providing the nDPI protocol ID."]
    #[doc = " @param table The flow table handle."]
    #[doc = " @param protocol The nDPI protocol ID."]
    #[doc = " @param buffer The output buffer."]
    #[doc = " @param buffer_len The output buffer length."]
    #[doc = " @return The buffer."]
    pub fn pfring_ft_l7_protocol_name(
        table: *mut pfring_ft_table,
        protocol: *mut pfring_ft_ndpi_protocol,
        buffer: *mut libc::c_char,
        buffer_len: libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C" {
    #[doc = " Set the nDPI handle. This is meant to be used for custom nDPI settings only,"]
    #[doc = " as FT already creates a nDPI instance internally when using PFRING_FT_TABLE_FLAGS_DPI."]
    #[doc = " FT takes care of releasing the nDPI instance on pfring_ft_destroy_table."]
    #[doc = " @param table The flow table handle."]
    #[doc = " @return 0 on success, a negative number on failures."]
    pub fn pfring_ft_set_ndpi_handle(
        table: *mut pfring_ft_table,
        ndpi: *mut ndpi_detection_module_struct,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Load custom nDPI protocols from a configuration file."]
    #[doc = " Please refer to the nDPI documentation for the file format."]
    #[doc = " Example: https://github.com/ntop/nDPI/blob/dev/example/protos.txt"]
    #[doc = " @param table The flow table handle."]
    #[doc = " @param path The configuration file path."]
    #[doc = " @return 0 on success, a negative number on failures."]
    pub fn pfring_ft_load_ndpi_protocols(
        table: *mut pfring_ft_table,
        path: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Load nDPI categories (defined by hostname) from a configuration file."]
    #[doc = " Please refer to the nDPI documentation for the file format."]
    #[doc = " Example: https://github.com/ntop/nDPI/blob/dev/example/mining_hosts.txt"]
    #[doc = " @param table The flow table handle."]
    #[doc = " @param path The configuration file path."]
    #[doc = " @return 0 on success, a negative number on failures."]
    pub fn pfring_ft_load_ndpi_categories(
        table: *mut pfring_ft_table,
        path: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Check if nDPI is available."]
    #[doc = " #return 1 if nDPI is available, 0 otherwise."]
    pub fn pfring_ft_is_ndpi_available() -> libc::c_int;
}
extern "C" {
    #[doc = " Get flow processing statistics."]
    #[doc = " @param table The flow table handle."]
    #[doc = " @return The stats struct."]
    pub fn pfring_ft_get_stats(table: *mut pfring_ft_table) -> *mut pfring_ft_stats;
}
extern "C" {
    #[doc = " Get the PF_RING FT version."]
    #[doc = " @param version A buffer (32 bytes long) where version is returned. (out)"]
    pub fn pfring_ft_version(version: *mut libc::c_char);
}
extern "C" {
    #[doc = " Get license info."]
    #[doc = " @param system_id A buffer (32 bytes long) where system id  is returned. (out)"]
    #[doc = " @param license_expiration A pointer to a time_t where license expiration is returned. (out)"]
    #[doc = " @param maintenance_expiration A pointer to a time_t where maintenance expiration is returned. (out)"]
    #[doc = " @return 1 if a valid license is installed, 0 otherwise."]
    pub fn pfring_ft_license(
        system_id: *mut libc::c_char,
        license_expiration: *mut time_t,
        maintenance_expiration: *mut time_t,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = " Enable debug mode"]
    pub fn pfring_ft_debug();
}



