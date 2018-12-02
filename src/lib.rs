#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

extern crate libc;




// include!(concat!(env!("OUT_DIR"), "/pfring.rs"));


#[cfg(target_os = "macos")]
pub const ETH_ALEN: usize = 6;
#[cfg(target_os = "linux")]
pub const ETH_ALEN: usize = libc::ETH_ALEN as usize;

// pub type pfring = libc::c_void;

#[repr(C)]
#[derive(Debug)]
pub struct pfring;


#[repr(C)]
pub union ip_addr {
    /// IPv6 src/dst IP addresses (Network byte order)
    pub v6: libc::in6_addr,
    /// IPv4 src/dst IP addresses
    pub v4: libc::uint32_t,
}

/// GPRS Tunneling Protocol
#[repr(C)]
pub struct tunnel_info {
    /// GTP/GRE tunnelId or NO_TUNNEL_ID for no filtering
    pub tunnel_id: libc::uint8_t,
    /// Layer 4 protocol
    pub tunneled_ip_version: libc::uint8_t,
    /// Layer 4 protocol
    pub tunneled_proto: libc::uint8_t,
    pub tunneled_ip_src: ip_addr,
    pub tunneled_ip_dst: ip_addr,
    pub tunneled_l4_src_port: libc::uint16_t,
    pub tunneled_l4_dst_port: libc::uint16_t,
}

impl core::fmt::Debug for tunnel_info {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.tunneled_ip_version {
            4 => {
                let src_ip = unsafe { self.tunneled_ip_src.v4 };
                let dst_ip = unsafe { self.tunneled_ip_dst.v4 };
                write!(f, "tunnel_info {{ tunnel_id: {:?}, tunneled_ip_version: {:?}, tunneled_proto: {:?}, tunneled_ip_src: {:?}, tunneled_ip_dst: {:?}, tunneled_l4_src_port: {:?}, tunneled_l4_dst_port: {:?}  }}",
                        self.tunnel_id,
                        self.tunneled_ip_version,
                        self.tunneled_proto,
                        src_ip,
                        dst_ip,
                        self.tunneled_l4_src_port,
                        self.tunneled_l4_dst_port)
            },
            6 => {
                let src_ip = unsafe { self.tunneled_ip_src.v6.s6_addr };
                let dst_ip = unsafe { self.tunneled_ip_dst.v6.s6_addr };
                write!(f, "tunnel_info {{ tunnel_id: {:?}, tunneled_ip_version: {:?}, tunneled_proto: {:?}, tunneled_ip_src: {:?}, tunneled_ip_dst: {:?}, tunneled_l4_src_port: {:?}, tunneled_l4_dst_port: {:?}  }}",
                        self.tunnel_id,
                        self.tunneled_ip_version,
                        self.tunneled_proto,
                        src_ip,
                        dst_ip,
                        self.tunneled_l4_src_port,
                        self.tunneled_l4_dst_port)
            },
            _ => unreachable!(),
        }
        
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct pkt_offset {
    /// This 'eth_offset' offset *must* be added to all offsets below 
    /// ONLY if you are inside the kernel. Ignore it in user-space.
    pub eth_offset: libc::int16_t,
    pub vlan_offset: libc::int16_t,
    pub l3_offset: libc::int16_t,
    pub l4_offset: libc::int16_t,
    pub payload_offset: libc::int16_t,

}


#[repr(C)]
#[derive(Debug)]
pub struct pkt_parsing_info_tcp {
    /// TCP flags (0 if not available)
    pub flags: libc::uint8_t,
    /// TCP sequence number
    pub seq_num: libc::uint8_t,
    pub ack_num: libc::uint8_t,
}


/// Core fields (also used by NetFlow)
#[repr(C)]
pub struct pkt_parsing_info {
    /// MAC dst addresses
    pub dmac: [libc::uint8_t; ETH_ALEN],
    /// MAC src addresses
    pub smac: [libc::uint8_t; ETH_ALEN],
    /// Ethernet type
    pub eth_type: libc::uint16_t,
    /// VLAN Id or NO_VLAN
    pub vlan_id: libc::uint16_t,
    /// VLAN Id or NO_VLAN
    pub qinq_vlan_id: libc::uint16_t,
    pub ip_version: libc::uint8_t,
    /// Layer 3 protocol
    pub l3_proto: libc::uint8_t,
    /// TOS
    pub ip_tos: libc::uint8_t,
    /// IPv4/6 src IP addresses
    pub ip_src: ip_addr,
    /// IPv4/6 dst IP addresses
    pub ip_dst: ip_addr,
    /// Layer 4 src ports
    pub l4_src_port: libc::uint16_t,
    /// Layer 4 dst ports
    pub l4_dst_port: libc::uint16_t,
    /// Variables for ICMP packets
    pub icmp_type: libc::uint8_t,
    pub icmp_code: libc::uint8_t,
    pub tcp: pkt_parsing_info_tcp,
    pub tunnel: tunnel_info,
    pub last_matched_rule_id: libc::int32_t,
    pub offset: pkt_offset,
}


impl core::fmt::Debug for pkt_parsing_info {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.ip_version {
            4 => {
                let src_ip = unsafe { self.ip_src.v4 };
                let dst_ip = unsafe { self.ip_dst.v4 };
                write!(f, "pkt_parsing_info {{ dmac: {:?}, smac: {:?}, eth_type: {:?}",
                            self.dmac,
                            self.smac,
                            self.eth_type)?;
                write!(f, " vlan_id: {:?}, qinq_vlan_id: {:?}, ip_version: {:?}, l3_proto: {:?} ip_tos: {:?} ip_src: {:?}, ip_dst: {:?}",
                            self.vlan_id,
                            self.qinq_vlan_id,
                            self.ip_version,
                            self.l3_proto,
                            self.ip_tos,
                            src_ip,
                            dst_ip)?;
                write!(f, " l4_src_port: {:?}, l4_dst_port: {:?}, icmp_type: {:?}, icmp_code: {:?}, tcp: {:?}, tunnel: {:?}",
                        self.l4_src_port,
                        self.l4_dst_port,
                        self.icmp_type,
                        self.icmp_code,
                        self.tcp,
                        self.tunnel)?;
                write!(f, " last_matched_rule_id: {:?}, offset: {:?} }}",
                        self.last_matched_rule_id,
                        self.offset)
            },
            6 => {
                let src_ip = unsafe { self.ip_src.v6.s6_addr };
                let dst_ip = unsafe { self.ip_dst.v6.s6_addr };

                write!(f, "pkt_parsing_info {{ dmac: {:?}, smac: {:?}, eth_type: {:?}",
                            self.dmac,
                            self.smac,
                            self.eth_type)?;
                write!(f, " vlan_id: {:?}, qinq_vlan_id: {:?}, ip_version: {:?}, l3_proto: {:?} ip_tos: {:?} ip_src: {:?}, ip_dst: {:?}",
                            self.vlan_id,
                            self.qinq_vlan_id,
                            self.ip_version,
                            self.l3_proto,
                            self.ip_tos,
                            src_ip,
                            dst_ip)?;
                write!(f, " l4_src_port: {:?}, l4_dst_port: {:?}, icmp_type: {:?}, icmp_code: {:?}, tcp: {:?}, tunnel: {:?}",
                        self.l4_src_port,
                        self.l4_dst_port,
                        self.icmp_type,
                        self.icmp_code,
                        self.tcp,
                        self.tunnel)?;
                write!(f, " last_matched_rule_id: {:?}, offset: {:?} }}",
                        self.last_matched_rule_id,
                        self.offset)
            },
            _ => unreachable!(),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct pfring_extended_pkthdr_tx {
    /// Interface Id where this packet will bounce after processing 
    /// if its values is other than UNKNOWN_INTERFACE
    pub bounce_interface: libc::int32_t,
    /// Kernel only pointer
    pub reserved: *const libc::c_void,
}


/// IP/TCP checksum offload enabled
pub const PKT_FLAGS_CHECKSUM_OFFLOAD: libc::uint32_t     = 1 << 0;
/// Valid checksum (with IP/TCP checksum offload enabled)
pub const PKT_FLAGS_CHECKSUM_OK: libc::uint32_t          = 1 << 1;
/// IP More fragments flag set
pub const PKT_FLAGS_IP_MORE_FRAG: libc::uint32_t         = 1 << 2;
/// IP fragment offset set (not 0)
pub const PKT_FLAGS_IP_FRAG_OFFSET: libc::uint32_t       = 1 << 3;
/// VLAN stripped by hw
pub const PKT_FLAGS_VLAN_HWACCEL: libc::uint32_t         = 1 << 4;
/// Flow update metadata, see generic_flow_update struct (keep flag compatible with ZC)
pub const PKT_FLAGS_FLOW_OFFLOAD_UPDATE: libc::uint32_t  = 1 << 6;
/// Flow raw packet, pkt_hash contains the flow_id (keep flag compatible with ZC)
pub const PKT_FLAGS_FLOW_OFFLOAD_PACKET: libc::uint32_t  = 1 << 7;
/// Flow raw packet belongs to a flow that has been marked (keep flag compatible with ZC)
pub const PKT_FLAGS_FLOW_OFFLOAD_MARKER: libc::uint32_t  = 1 << 8;


#[repr(C)]
#[derive(Debug)]
pub struct pfring_extended_pkthdr {
    /// Packet timestamp at ns precision. Note that if your NIC supports
    /// hardware timestamp, this is the place to read timestamp from
    pub timestamp_ns: libc::uint64_t,
    pub flags: libc::uint32_t,
    /// 1=RX: packet received by the NIC, 0=TX: packet transmitted by the NIC
    pub rx_direction: libc::uint8_t,
    /// index of the interface on which the packet has been received.
    /// It can be also used to report other information
    pub if_index: libc::int32_t,
    /// Hash based on the packet header
    pub pkt_hash: libc::uint8_t,
    // --- short header ends here ---
    pub tx: pfring_extended_pkthdr_tx,
    /// NOTE: leave it as last field of the memset on parse_pkt() will fail
    pub parsed_pkt: pkt_parsing_info,
}

/// pcap header
#[repr(C)]
pub struct pfring_pkthdr {
    /// time stamp
    pub ts: libc::timeval,
    /// length of portion present
    pub caplen: libc::uint32_t,
    /// length of whole packet (off wire)
    pub len: libc::uint32_t,
    /// PF_RING extended header
    pub extended_hdr: pfring_extended_pkthdr,
}

impl core::fmt::Debug for pfring_pkthdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "pfring_pkthdr {{ ts: {{ tv_sec: {:?}, tv_usec: {:?}, }}, caplen: {:?}, len: {:?}, extended_hdr: {:?} }}",
                self.ts.tv_sec,
                self.ts.tv_usec,
                self.caplen,
                self.len,
                self.extended_hdr)
    }
}


/// pfring_open() flag: Set the hw RSS function to symmetric mode (both directions of the same flow go to the same hw queue).
/// Supported by ZC drivers only. This option is also available with the PF_RING-aware libpcap via the PCAP_PF_RING_ZC_RSS environment variable.
pub const PF_RING_ZC_SYMMETRIC_RSS: libc::uint32_t =      (1 <<  0);
/// pfring_open() flag: The device is open in reentrant mode. This is implemented by means of semaphores and it results 
/// is slightly worse performance. Use reentrant mode only for multithreaded applications.
pub const PF_RING_REENTRANT: libc::uint32_t =             (1 <<  1);
/// pfring_open() flag: If uset, PF_RING does not fill the field extended_hdr of struct pfring_pkthdr. If set, 
/// the extended_hdr field is also properly filled. In case you do not need extended information, 
/// set this value to 0 in order to speedup the operation.
pub const PF_RING_LONG_HEADER: libc::uint32_t =           (1 <<  2);
/// pfring_open() flag: The device is open in promiscuous mode.
pub const PF_RING_PROMISC: libc::uint32_t =               (1 <<  3);
/// pfring_open() flag: Force PF_RING to set the timestamp on received packets (usually it is 
/// not set when using zero-copy, for optimizing performance).
pub const PF_RING_TIMESTAMP: libc::uint32_t =             (1 <<  4);
/// pfring_open() flag: Enable hw timestamping, when available.
pub const PF_RING_HW_TIMESTAMP: libc::uint32_t =          (1 <<  5);
/// pfring_open() flag: Enable fast forwarding support (see pfring_send_last_rx_packet()).
pub const PF_RING_RX_PACKET_BOUNCE: libc::uint32_t =      (1 <<  6);
/// pfring_open() flag: Set hw RSS to send all traffic to queue 0. Other queues can be selected 
/// using hw filters (ZC cards with hw filtering only).
pub const PF_RING_ZC_FIXED_RSS_Q_0: libc::uint32_t =      (1 <<  7);
/// pfring_open() flag: Strip hw timestamp from the packet.
pub const PF_RING_STRIP_HW_TIMESTAMP: libc::uint32_t =    (1 <<  8);
/// pfring_open() flag: Disable packet parsing also when 1-copy is used. (parsing already disabled in zero-copy)
pub const PF_RING_DO_NOT_PARSE: libc::uint32_t =          (1 <<  9);
/// pfring_open() flag: Disable packet timestamping also when 1-copy is used. (sw timestamp already disabled in zero-copy)
pub const PF_RING_DO_NOT_TIMESTAMP: libc::uint32_t =      (1 << 10);
/// pfring_open() flag: Enable chunk mode operations.
/// This mode is supported only by specific adapters and it's not for general purpose.
pub const PF_RING_CHUNK_MODE: libc::uint32_t =            (1 << 11);
/// pfring_open() flag: Enable ixiacom.com hardware timestamp support+stripping.
pub const PF_RING_IXIA_TIMESTAMP: libc::uint32_t =        (1 << 12);
/// pfring_open() flag: Force userspace bpf even with standard drivers (not only with ZC).
pub const PF_RING_USERSPACE_BPF: libc::uint32_t =         (1 << 13);
/// pfring_open() flag: Do not touch/reprogram hw RSS 
pub const PF_RING_ZC_NOT_REPROGRAM_RSS: libc::uint32_t =  (1 << 14);
/// pfring_open() flag: Enable apcon.com/vssmonitoring.com hardware timestamp support+stripping.
pub const PF_RING_VSS_APCON_TIMESTAMP: libc::uint32_t =   (1 << 15);
/// pfring_open() flag: Compute RSS on src/dst IP only (not 4-tuple) 
pub const PF_RING_ZC_IPONLY_RSS: libc::uint32_t =         (1 << 16);
/// pfring_open() flag: Enable hw flow table support when available 
pub const PF_RING_FLOW_OFFLOAD: libc::uint32_t =          (1 << 17);
/// pfring_open() flag: Do not send flow updates with PF_RING_FLOW_OFFLOAD (enable support for flows shunting only)
pub const PF_RING_FLOW_OFFLOAD_NOUPDATES: libc::uint32_t =(1 << 18);
/// pfring_open() flag: Do not send raw packets with PF_RING_FLOW_OFFLOAD
pub const PF_RING_FLOW_OFFLOAD_NORAWDATA: libc::uint32_t =(1 << 19);
/// pfring_open() flag: Enable L7 filtering support based on PF_RING FT (Flow Table with nDPI support)
pub const PF_RING_L7_FILTERING: libc::uint32_t =          (1 << 20);
/// pfring_open() flag: Do not strip the FCS (CRC), when not stripped out by 
/// the adapter (on standard adapters use this in combination with 'ethtool -K <dev> rx-fcs on rx-all on')
pub const PF_RING_DO_NOT_STRIP_FCS: libc::uint32_t =      (1 << 21);



// #[link(name = "pfring")]
extern "C" {
    /// This call is used to initialize a PF_RING socket hence obtain a handle of type struct pfring 
    /// that can be used in subsequent calls. Note that: 
    /// 1. you can use physical (e.g. ethX) and virtual (e.g. tapX) devices, RX-queues (e.g. ethX@Y), 
    ///    and additional modules (e.g. zc:ethX@Y, dag:dagX:Y, "multi:ethA@X;ethB@Y;ethC@Z", "stack:ethX").
    /// 2. you need super-user capabilities in order to open a device.
    /// @param device_name Symbolic name of the PF_RING-aware device we are attempting to open.
    /// Syntax:
    ///  - eth0           interface eth0, all channels
    ///  - eth0@1         interface eth0, channel 1
    ///  - eth0,eth1      interface eth0 and eth1, all channels
    ///  - eth0,eth1@1    interface eth0 and eth1, channel 1
    ///  - eth0@1,5       interface eth0, channel 1 and 5
    ///  - eth0@1-5       interface eth0, channel 1,2...5
    ///  - eth0@1-3,5-7   interface eth0, channel 1,2,3,5,6,7
    /// Note: 
    ///  - ',' and '-' are supported with standard kernel capture / drivers only
    ///  - in case of multiple interfaces, the channels are same for all interfaces
    /// @param caplen      Maximum packet capture len (also known as snaplen).
    /// @param flags       It allows several options to be specified on a compact format using bitmaps (see PF_RING_* macros).
    /// @return On success a handle is returned, NULL otherwise.
    pub fn pfring_open(device_name: *const libc::c_char,
                       caplen: libc::uint8_t,
                       flags: libc::uint32_t) -> *mut pfring;

    /// This call returns an incoming packet when available. 
    /// @param ring       The PF_RING handle where we perform the check.
    /// @param buffer     A memory area allocated by the caller where the incoming packet will be stored. 
    ///                   Note that this parameter is a pointer to a pointer, 
    ///                   in order to enable zero-copy implementations (buffer_len must be set to 0).
    /// @param buffer_len The length of the memory area above. 
    ///                   Note that the incoming packet is cut if it is too long for the allocated area. 
    ///                   A length of 0 indicates to use the zero-copy optimization, when available.
    /// @param hdr        A memory area where the packet header will be copied.
    /// @param wait_for_incoming_packet If 0 we simply check the packet availability, 
    ///                   otherwise the call is blocked until a packet is available. 
    ///                   This option is also available with the PF_RING-aware libpcap via 
    ///                   the PCAP_PF_RING_ACTIVE_POLL environment variable.
    /// @return 0 in case of no packet being received (non-blocking), 
    ///                   1 in case of success, -1 in case of error.
    pub fn pfring_recv(ring: *const pfring,
                       buffer: *mut *mut libc::c_uchar,
                       buffer_len: libc::c_uint,
                       hdr: *mut pfring_pkthdr,
                       wait_for_incoming_packet: libc::uint8_t ) -> libc::c_int;


    /// Send a raw packet (i.e. it is sent on wire as specified). This packet must be fully specified (the MAC address up) 
    /// and it will be transmitted as-is without any further manipulation.
    /// 
    /// Depending on the driver being used, packet transmission happens differently:
    /// - Vanilla and PF_RING aware drivers: PF_RING does not accelerate the TX so the standard Linux transmission facilities are used. 
    ///   Do not expect speed advantage when using PF_RING in this mode.
    /// - ZC: line rate transmission is supported.
    /// @param ring         The PF_RING handle on which the packet has to be sent.
    /// @param pkt          The buffer containing the packet to send.
    /// @param pkt_len      The length of the pkt buffer.
    /// @param flush_packet 1 = Flush possible transmission queues. If set to 0, you will decrease your CPU usage but at the cost of 
    ///                     sending packets in trains and thus at larger latency.
    /// @return The number of bytes sent if success, a negative value otherwise.
    pub fn pfring_send(ring: *const pfring,
                       pkt: *const libc::c_char,
                       pkt_len: libc::c_uint,
                       flush_packet: libc::uint8_t) -> libc::c_int;


    /// Synchronizes the egress ring indexes/registers flushing enqueued packets.
    /// @param ring The PF_RING handle.
    /// @param  
    /// @return 0 on success, a negative value otherwise.
    pub fn pfring_flush_tx_packets(ring: *mut pfring) -> libc::c_int;

    /// Read the ring version. Note that if the ring version is 5.6 the retuned ring version is 0x050600. 
    /// @param version A user-allocated buffer on which ring version will be copied.
    pub fn pfring_version_noring(version: *mut libc::uint8_t) -> ();

    /// Read the ring version. Note that if the ring version is 5.6 the retuned ring version is 0x050600. 
    /// @param ring    The PF_RING handle, in case the module supports versioning.
    /// @param version A user-allocated buffer on which ring version will be copied.
    /// @return 0 on success, a negative value otherwise.
    // int pfring_version(pfring *ring, u_int32_t *version);
    pub fn pfring_version(ring: *mut pfring, version: libc::uint32_t) -> libc::c_int;

    /// This call is used to terminate an PF_RING device previously open. 
    /// Note that you must always close a device before leaving an application. If unsure, you can close a device from a signal handler. 
    /// @param ring The PF_RING handle that we are attempting to close.
    pub fn pfring_close(ring: *mut pfring) -> ();

}


// pfring_open: unsafe extern "C" fn (device_name: * const c_char, caplen: u32, flags: u32) -> * mut PFRing,
// pfring_recv: unsafe extern "C" fn (ring: * const PFRing, buffer: * mut * mut u8, buffer_len: c_uint, hdr: * mut PFRingPacketHeader, wait_for_incoming_packet: u8) -> c_int,
// pfring_send: unsafe extern "C" fn (ring: * const PFRing, pkt: * const u8, pkt_len: c_uint, flush_packet: u8) -> c_int,

// pfring_close: unsafe extern "C" fn (ring: * mut PFRing),

// pfring_flush_tx_packets: unsafe extern "C" fn (ring: * mut PFRing) -> c_int,
// pfring_version_noring: unsafe extern "C" fn (version: * mut u32)


