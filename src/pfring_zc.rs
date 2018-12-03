use crate::{ __BindgenBitfieldUnit, __IncompleteArrayField, };

use crate::linux::pf_ring::{
    pfring_pkthdr,
    hw_filtering_rule,
};

use crate::libc::{ c_uint, uint8_t, uint16_t, uint32_t, uint64_t, c_uchar, };

pub const PF_RING_ZC_DEVICE_ASYMMETRIC_RSS: u32 = 1;
pub const PF_RING_ZC_DEVICE_FIXED_RSS_Q_0: u32 = 2;
pub const PF_RING_ZC_DEVICE_SW_TIMESTAMP: u32 = 4;
pub const PF_RING_ZC_DEVICE_HW_TIMESTAMP: u32 = 8;
pub const PF_RING_ZC_DEVICE_STRIP_HW_TIMESTAMP: u32 = 16;
pub const PF_RING_ZC_DEVICE_IXIA_TIMESTAMP: u32 = 32;
pub const PF_RING_ZC_DEVICE_NOT_REPROGRAM_RSS: u32 = 64;
pub const PF_RING_ZC_DEVICE_CAPTURE_TX: u32 = 128;
pub const PF_RING_ZC_DEVICE_IPONLY_RSS: u32 = 256;
pub const PF_RING_ZC_DEVICE_NOT_PROMISC: u32 = 512;
pub const PF_RING_ZC_DO_NOT_STRIP_FCS: u32 = 1024;
pub const UNDEFINED_QUEUEID: u32 = 4294967295;
pub const PF_RING_ZC_PKT_FLAGS_GOOD_IP_CS: u32 = 1;
pub const PF_RING_ZC_PKT_FLAGS_BAD_IP_CS: u32 = 2;
pub const PF_RING_ZC_PKT_FLAGS_GOOD_L4_CS: u32 = 4;
pub const PF_RING_ZC_PKT_FLAGS_BAD_L4_CS: u32 = 8;
pub const PF_RING_ZC_PKT_FLAGS_FLOW_OFFLOAD_UPDATE: u32 = 64;
pub const PF_RING_ZC_PKT_FLAGS_FLOW_OFFLOAD_PACKET: u32 = 128;
pub const PF_RING_ZC_PKT_FLAGS_FLOW_OFFLOAD_MARKER: u32 = 256;

pub type pfring_zc_cluster = ::std::os::raw::c_void;
pub type pfring_zc_queue = ::std::os::raw::c_void;
pub type pfring_zc_buffer_pool = ::std::os::raw::c_void;
pub type pfring_zc_worker = ::std::os::raw::c_void;
pub type pfring_zc_multi_queue = ::std::os::raw::c_void;


#[doc = "< RX only mode."]
pub const rx_only: pfring_zc_queue_mode = 0;
#[doc = "< TX only mode."]
pub const tx_only: pfring_zc_queue_mode = 1;
#[doc = " List of possible queue modes."]
pub type pfring_zc_queue_mode = u32;
#[doc = " Queue stats structure."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_zc_stat {
    pub recv: uint64_t,
    pub sent: uint64_t,
    pub drop: uint64_t,
}
#[doc = " Struct for nsec time (similar to struct timespec)."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_zc_timespec {
    pub tv_sec: uint32_t,
    pub tv_nsec: uint32_t,
}
#[doc = " Buffer handle."]
#[repr(C)]
#[derive(Debug)]
pub struct pfring_zc_pkt_buff {
    #[doc = "< Packet length."]
    pub len: uint16_t,
    #[doc = "< Packet flags."]
    pub flags: uint16_t,
    #[doc = "< Packet hash."]
    pub hash: uint32_t,
    #[doc = "< Packet timestamp (nsec)"]
    pub ts: pfring_zc_timespec,
    #[doc = "< Start of user metadata, if any."]
    pub user: __IncompleteArrayField<c_uchar>,
}
#[doc = " Buffer handle."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pfring_zc_queue_info {
    #[doc = "< Max packet length."]
    pub buffer_len: uint32_t,
    #[doc = "< User metadata length."]
    pub metadata_len: uint32_t,
}
extern "C" {
    #[doc = " Return the pointer to the actual packet data."]
    #[doc = " @param pkt_handle The buffer handle."]
    #[doc = " @param queue      Any queue from the cluster (e.g. the queue from which the packet is arrived or destined)."]
    #[doc = " @return           The pointer on success, NULL otherwise."]
    pub fn pfring_zc_pkt_buff_data(
        pkt_handle: *mut pfring_zc_pkt_buff,
        queue: *mut pfring_zc_queue,
    ) -> *mut c_uchar;
}
extern "C" {
    #[doc = " Remove data from the start of a buffer."]
    #[doc = " @param pkt_handle The buffer handle."]
    #[doc = " @param queue Any queue from the cluster."]
    #[doc = " @param len The number of bytes to remove."]
    #[doc = " @return The pointer to the start of the buffer on success, NULL otherwise."]
    pub fn pfring_zc_pkt_buff_pull(
        pkt_handle: *mut pfring_zc_pkt_buff,
        queue: *mut pfring_zc_queue,
        len: uint16_t,
    ) -> *mut c_uchar;
}
extern "C" {
    #[doc = " Add data to the start of a buffer."]
    #[doc = " @param pkt_handle The buffer handle."]
    #[doc = " @param queue Any queue from the cluster."]
    #[doc = " @param len The number of bytes to add."]
    #[doc = " @return The pointer to the start of the buffer on success, NULL otherwise."]
    pub fn pfring_zc_pkt_buff_push(
        pkt_handle: *mut pfring_zc_pkt_buff,
        queue: *mut pfring_zc_queue,
        len: uint16_t,
    ) -> *mut c_uchar;
}
extern "C" {
    #[doc = " Create a new cluster."]
    #[doc = " @param cluster_id           The unique cluster identifier."]
    #[doc = " @param buffer_len           The size of each buffer: it must be at least as large as the MTU + L2 header (it will be rounded up to cache line) and not bigger than the page size."]
    #[doc = " @param metadata_len         The size of each buffer metadata."]
    #[doc = " @param tot_num_buffers      The total number of buffers to reserve for queues/devices/extra allocations."]
    #[doc = " @param numa_node_id         The NUMA node id for cpu/memory binding."]
    #[doc = " @param hugepages_mountpoint The HugeTLB mountpoint (NULL for auto-detection) for memory allocation."]
    #[doc = " @return                     The cluster handle on success, NULL otherwise (errno is set appropriately)."]
    pub fn pfring_zc_create_cluster(
        cluster_id: uint32_t,
        buffer_len: uint32_t,
        metadata_len: uint32_t,
        tot_num_buffers: uint32_t,
        numa_node_id: i32,
        hugepages_mountpoint: *const ::std::os::raw::c_char,
    ) -> *mut pfring_zc_cluster;
}
extern "C" {
    #[doc = " Destroy a cluster."]
    #[doc = " @param cluster The cluster handle."]
    pub fn pfring_zc_destroy_cluster(cluster: *mut pfring_zc_cluster);
}
extern "C" {
    #[doc = " Open a network device."]
    #[doc = " @param cluster     The cluster handle."]
    #[doc = " @param device_name The device name."]
    #[doc = " @param queue_mode  The direction, RX or TX."]
    #[doc = " @param flags       Optional flags."]
    #[doc = " @return            The queue handle on success, NULL otherwise (errno is set appropriately)."]
    pub fn pfring_zc_open_device(
        cluster: *mut pfring_zc_cluster,
        device_name: *const ::std::os::raw::c_char,
        queue_mode: pfring_zc_queue_mode,
        flags: uint32_t,
    ) -> *mut pfring_zc_queue;
}
extern "C" {
    #[doc = " Create a SPSC queue."]
    #[doc = " @param cluster   The cluster handle."]
    #[doc = " @param queue_len The queue length."]
    #[doc = " @return          The queue handle on success, NULL otherwise (errno is set appropriately)."]
    pub fn pfring_zc_create_queue(
        cluster: *mut pfring_zc_cluster,
        queue_len: uint32_t,
    ) -> *mut pfring_zc_queue;
}
extern "C" {
    #[doc = " Read the next packet from the queue."]
    #[doc = " @param queue                    The queue handle."]
    #[doc = " @param pkt_handle               The pointer to the buffer handle for the received buffer. The buffer handle must have been allocated earlier with get_packet_handle()/get_packet_handle_from_pool()."]
    #[doc = " @param wait_for_incoming_packet The flag indicating whether this call is blocking or not."]
    #[doc = " @return                         1 on success, 0 on empty queue (non-blocking only), a negative value otherwise."]
    pub fn pfring_zc_recv_pkt(
        queue: *mut pfring_zc_queue,
        pkt_handle: *mut *mut pfring_zc_pkt_buff,
        wait_for_incoming_packet: uint8_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Read a burst of packets from the queue."]
    #[doc = " @param queue                    The queue handle."]
    #[doc = " @param pkt_handles              The array with the buffer handles for the received buffers. The buffer handles must have been allocated earlier with get_packet_handle()/get_packet_handle_from_pool()."]
    #[doc = " @param max_num_packets          The maximum number of packets to read from the queue."]
    #[doc = " @param wait_for_incoming_packet The flag indicating whether this call is blocking or not."]
    #[doc = " @return                         The number of received packets on success, 0 on empty queue (non-blocking only), a negative value otherwise."]
    pub fn pfring_zc_recv_pkt_burst(
        queue: *mut pfring_zc_queue,
        pkt_handles: *mut *mut pfring_zc_pkt_buff,
        max_num_packets: uint32_t,
        wait_for_incoming_packet: uint8_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Check if the queue is empty (rx only for devices)."]
    #[doc = " @param queue The queue handle."]
    #[doc = " @return      1 on empty queue, 0 otherwise."]
    pub fn pfring_zc_queue_is_empty(queue: *mut pfring_zc_queue) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Break the receive loop in case of blocking pfring_zc_recv_pkt()/pfring_zc_recv_pkt_burst()."]
    #[doc = " @param queue The queue handle."]
    pub fn pfring_zc_queue_breakloop(queue: *mut pfring_zc_queue);
}
extern "C" {
    #[doc = " Insert a packet into the queue."]
    #[doc = " @param queue        The queue handle."]
    #[doc = " @param pkt_handle   The pointer to the buffer handle to send. Once a packet has been sent, the buffer handle can be reused or if not longer necessary it must be freed by calling pfring_zc_release_packet_handle()."]
    #[doc = " @param flush_packet The flag indicating whether this call should flush the enqueued packet, and older packets if any."]
    #[doc = " @return             The packet length on success, a negative value otherwise."]
    pub fn pfring_zc_send_pkt(
        queue: *mut pfring_zc_queue,
        pkt_handle: *mut *mut pfring_zc_pkt_buff,
        flush_packet: uint8_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Send a burst of packets to the queue."]
    #[doc = " @param queue        The queue handle."]
    #[doc = " @param pkt_handles  The array with the buffer handles for the buffers to send."]
    #[doc = " @param num_packets  The number of packets to send to the queue."]
    #[doc = " @param flush_packet The flag indicating whether this call should flush the enqueued packets, and older packets if any."]
    #[doc = " @return             The number of packets successfully sent, a negative value in case of error."]
    pub fn pfring_zc_send_pkt_burst(
        queue: *mut pfring_zc_queue,
        pkt_handles: *mut *mut pfring_zc_pkt_buff,
        num_packets: uint32_t,
        flush_packets: uint8_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Check if the queue is full (tx only for devices)."]
    #[doc = " @param queue The queue handle."]
    #[doc = " @return      1 on full queue, 0 otherwise."]
    pub fn pfring_zc_queue_is_full(queue: *mut pfring_zc_queue) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Sync/flush a queue."]
    #[doc = " @param queue     The queue handle."]
    #[doc = " @param direction The direction to sync/flush, RX or TX."]
    pub fn pfring_zc_sync_queue(queue: *mut pfring_zc_queue, direction: pfring_zc_queue_mode);
}
extern "C" {
    #[doc = " Set a BPF filter."]
    #[doc = " @param queue  The queue handle."]
    #[doc = " @param filter The BPF filter."]
    #[doc = " @return       0 on success, a negative value otherwise."]
    pub fn pfring_zc_set_bpf_filter(
        queue: *mut pfring_zc_queue,
        filter: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Remove the BPF filter."]
    #[doc = " @param queue  The queue handle."]
    #[doc = " @return       0 on success, a negative value otherwise."]
    pub fn pfring_zc_remove_bpf_filter(queue: *mut pfring_zc_queue) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Add an hw filtering rule to the network device, when the queue is bound to a supported card."]
    #[doc = " @param queue The queue handle."]
    #[doc = " @param rule  The filtering rule."]
    #[doc = " @return      0 on success, a negative value otherwise."]
    pub fn pfring_zc_add_hw_rule(
        queue: *mut pfring_zc_queue,
        rule: *mut hw_filtering_rule,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Remove an hw filtering rule from the network device."]
    #[doc = " @param queue    The queue handle."]
    #[doc = " @param rule_id  The filtering rule identifier."]
    #[doc = " @return         0 on success, a negative value otherwise."]
    pub fn pfring_zc_remove_hw_rule(
        queue: *mut pfring_zc_queue,
        rule_id: uint16_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Change the hw RSS indirection table (RETA) for Intel igb/ixgbe-based cards."]
    #[doc = " @param queue       The queue handle."]
    #[doc = " @param indir_table The indirection table (128 cells), with the destination queue for each hash value input."]
    pub fn pfring_zc_set_rxfh_indir(queue: *mut pfring_zc_queue, indir_table: *mut uint8_t);
}
extern "C" {
    #[doc = " Get the cluster id."]
    #[doc = " @param cluster The cluster handle."]
    #[doc = " @return        The cluster id."]
    pub fn pfring_zc_get_cluster_id(cluster: *mut pfring_zc_cluster) -> uint32_t;
}
extern "C" {
    #[doc = " Read the queue id. If the actual queue is a device, it is possible to convert the ID to the device index using QUEUEID_TO_IFINDEX(id)"]
    #[doc = " @param queue The queue handle."]
    #[doc = " @return      The queue id."]
    pub fn pfring_zc_get_queue_id(queue: *mut pfring_zc_queue) -> uint32_t;
}
extern "C" {
    #[doc = " Read queue settings, including queue len, buffers len, metadata len."]
    #[doc = " @param queue The queue handle."]
    #[doc = " @param info  The queue settings (out)."]
    pub fn pfring_zc_get_queue_settings(
        queue: *mut pfring_zc_queue,
        info: *mut pfring_zc_queue_info,
    );
}
extern "C" {
    #[doc = " Read queue speed."]
    #[doc = " @param queue The queue handle."]
    #[doc = " @return      The queue speed in Mbit/s, 0 if unknown."]
    pub fn pfring_zc_get_queue_speed(queue: *mut pfring_zc_queue) -> uint32_t;
}
extern "C" {
    #[doc = " Read the queue stats."]
    #[doc = " @param queue The queue handle."]
    #[doc = " @param stats The stats structure."]
    #[doc = " @return      0 on success, a negative value otherwise."]
    pub fn pfring_zc_stats(
        queue: *mut pfring_zc_queue,
        stats: *mut pfring_zc_stat,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Allocate a buffer from global resources."]
    #[doc = " @param cluster The cluster handle."]
    #[doc = " @return        The buffer handle on success, NULL otherwise."]
    pub fn pfring_zc_get_packet_handle(cluster: *mut pfring_zc_cluster) -> *mut pfring_zc_pkt_buff;
}
extern "C" {
    #[doc = " Release a buffer to global resources."]
    #[doc = " @param cluster    The cluster handle."]
    #[doc = " @param pkt_handle The buffer handle."]
    pub fn pfring_zc_release_packet_handle(
        cluster: *mut pfring_zc_cluster,
        pkt_handle: *mut pfring_zc_pkt_buff,
    );
}
extern "C" {
    #[doc = " Create a multi-queue object to send the same packet to multiple queues."]
    #[doc = " Constraints: when using fan-out with multiqueue (i.e. calling pfring_zc_send_pkt_multi() with multiple bits set in queues_mask)"]
    #[doc = " it is not possible to have multiple multiqueue sharing the same consumers (expect metadata corruptions in this case)."]
    #[doc = " Note: this call will disable standard send on the queues (only pfring_zc_send_pkt_multi() is allowed)."]
    #[doc = " @param queues     The array with the queues to bind to the multi-queue object."]
    #[doc = " @param num_queues The number of egress queues."]
    #[doc = " @return           The multi-queue handle on success, NULL otherwise (errno is set appropriately)."]
    pub fn pfring_zc_create_multi_queue(
        queues: *mut *mut pfring_zc_queue,
        num_queues: uint32_t,
    ) -> *mut pfring_zc_multi_queue;
}
extern "C" {
    #[doc = " Send a packet to multiple queues bound to a multi-queue object."]
    #[doc = " @param multi_queue  The multi-queue handle."]
    #[doc = " @param pkt_handle   The pointer to the buffer handle to send. Once a packet has been sent, the buffer handle can be reused or if not longer necessary it must be freed by calling pfring_zc_release_packet_handle()."]
    #[doc = " @param queues_mask  The mask with the egress queues where the buffer should be inserted. The LSB indicates the first queue in the multi-queue array."]
    #[doc = " @param flush_packet The flag indicating whether this call should flush the enqueued packet, and older packets if any."]
    #[doc = " @return             The number of packet copies enqueued."]
    pub fn pfring_zc_send_pkt_multi(
        multi_queue: *mut pfring_zc_multi_queue,
        pkt_handle: *mut *mut pfring_zc_pkt_buff,
        queues_mask: uint64_t,
        flush_packet: uint8_t,
    ) -> ::std::os::raw::c_int;
}
#[doc = "< Round-Robin policy."]
pub const round_robin_policy: pfring_zc_recv_policy = 0;
#[doc = "< Round-Robin policy using bursts."]
pub const round_robin_bursts_policy: pfring_zc_recv_policy = 1;
#[doc = " List of possible policies when receiving packets from multiple queues."]
pub type pfring_zc_recv_policy = u32;
#[doc = " The distribution function prototype."]
#[doc = " @param pkt_handle The received buffer handle."]
#[doc = " @param in_queue   The ingress queues handle from which the packet arrived."]
#[doc = " @param user       The pointer to the user data."]
#[doc = " @return           The egress queue index (or a negative value to drop the packet) in case of balancing, the egress queues bit-mask in case of fan-out."]
pub type pfring_zc_distribution_func = ::std::option::Option<
    unsafe extern "C" fn(
        pkt_handle: *mut pfring_zc_pkt_buff,
        in_queue: *mut pfring_zc_queue,
        user: *mut ::std::os::raw::c_void,
    ) -> i64,
>;
#[doc = " The idle callback prototype."]
pub type pfring_zc_idle_callback = ::std::option::Option<unsafe extern "C" fn()>;
extern "C" {
    #[doc = " Run a balancer worker."]
    #[doc = " @param in_queues        The ingress queues handles array."]
    #[doc = " @param out_queues       The egress queues handles array."]
    #[doc = " @param num_in_queues    The number of ingress queues."]
    #[doc = " @param num_out_queues   The number of egress queues."]
    #[doc = " @param working_set_pool The pool handle for working set buffers allocation. The worker uses 8 buffers in burst mode, 1 otherwise."]
    #[doc = " @param recv_policy      The receive policy."]
    #[doc = " @param callback         The function called when there is no incoming packet."]
    #[doc = " @param func             The distribution function, or NULL for the defualt IP-based distribution function."]
    #[doc = " @param user_data        The user data passed to distribution function."]
    #[doc = " @param active_wait      The flag indicating whether the worker should use active or passive wait for incoming packets."]
    #[doc = " @param core_id_affinity The core affinity for the worker thread."]
    #[doc = " @return                 The worker handle on success, NULL otherwise (errno is set appropriately)."]
    pub fn pfring_zc_run_balancer(
        in_queues: *mut *mut pfring_zc_queue,
        out_queues: *mut *mut pfring_zc_queue,
        num_in_queues: uint32_t,
        num_out_queues: uint32_t,
        working_set_pool: *mut pfring_zc_buffer_pool,
        recv_policy: pfring_zc_recv_policy,
        callback: pfring_zc_idle_callback,
        func: pfring_zc_distribution_func,
        user_data: *mut ::std::os::raw::c_void,
        active_wait: uint32_t,
        core_id_affinity: i32,
    ) -> *mut pfring_zc_worker;
}
extern "C" {
    #[doc = " Run a fan-out worker."]
    #[doc = " @param in_queues        The ingress queues handles array."]
    #[doc = " @param out_multi_queues The egress multi-queue handle."]
    #[doc = " @param num_in_queues    The number of ingress queues."]
    #[doc = " @param working_set_pool The pool handle for working set buffers allocation. The worker uses 8 buffers in burst mode, 1 otherwise."]
    #[doc = " @param recv_policy      The receive policy."]
    #[doc = " @param callback         The function called when there is no incoming packet."]
    #[doc = " @param func             The distribution function, or NULL to send all the packets to all the egress queues."]
    #[doc = " @param user_data        The user data passed to distribution function."]
    #[doc = " @param active_wait      The flag indicating whether the worker should use active or passive wait for incoming packets."]
    #[doc = " @param core_id_affinity The core affinity for the worker thread."]
    #[doc = " @return                 The worker handle on success, NULL otherwise (errno is set appropriately)."]
    pub fn pfring_zc_run_fanout(
        in_queues: *mut *mut pfring_zc_queue,
        out_multi_queue: *mut pfring_zc_multi_queue,
        num_in_queues: uint32_t,
        working_set_pool: *mut pfring_zc_buffer_pool,
        recv_policy: pfring_zc_recv_policy,
        callback: pfring_zc_idle_callback,
        func: pfring_zc_distribution_func,
        user_data: *mut ::std::os::raw::c_void,
        active_wait: uint32_t,
        core_id_affinity: i32,
    ) -> *mut pfring_zc_worker;
}
extern "C" {
    #[doc = " Run a fifo worker. (experimental)"]
    #[doc = " @param in_queues        The ingress queues handles array."]
    #[doc = " @param out_queue        The egress queue handle, or NULL for processing packets directly using the provided func."]
    #[doc = " @param num_in_queues    The number of ingress queues."]
    #[doc = " @param working_set_pool The pool handle for working set buffers allocation. The worker uses num_in_queues * 32 buffers."]
    #[doc = " @param callback         The function called when there is no incoming packet."]
    #[doc = " @param func             The packet processing function."]
    #[doc = " @param user_data        The user data passed to func."]
    #[doc = " @param active_wait      The flag indicating whether the worker should use active or passive wait for incoming packets."]
    #[doc = " @param core_id_affinity_sorter The core affinity for the sorter thread."]
    #[doc = " @param core_id_affinity_timer  The core affinity for the timer thread."]
    #[doc = " @return                 The worker handle on success, NULL otherwise (errno is set appropriately)."]
    pub fn pfring_zc_run_fifo(
        in_queues: *mut *mut pfring_zc_queue,
        out_queue: *mut pfring_zc_queue,
        num_in_queues: uint32_t,
        working_set_pool: *mut pfring_zc_buffer_pool,
        callback: pfring_zc_idle_callback,
        func: pfring_zc_distribution_func,
        user_data: *mut ::std::os::raw::c_void,
        active_wait: uint32_t,
        core_id_affinity_sorter: i32,
        core_id_affinity_timer: i32,
    ) -> *mut pfring_zc_worker;
}
extern "C" {
    #[doc = " Kill the worker."]
    #[doc = " @param worker The worker handle."]
    pub fn pfring_zc_kill_worker(worker: *mut pfring_zc_worker);
}
extern "C" {
    #[doc = " Create a buffer pool to reserve a subset of the global resources."]
    #[doc = " @param cluster  The cluster handle."]
    #[doc = " @param pool_len The number of buffers to reserve for the pool."]
    #[doc = " @return         The pool handle on success, NULL otherwise (errno is set appropriately)."]
    pub fn pfring_zc_create_buffer_pool(
        cluster: *mut pfring_zc_cluster,
        pool_len: uint32_t,
    ) -> *mut pfring_zc_buffer_pool;
}
extern "C" {
    #[doc = " Allocate a buffer from a pool resource."]
    #[doc = " @param pool The pool handle."]
    #[doc = " @return     The buffer handle on success, NULL otherwise."]
    pub fn pfring_zc_get_packet_handle_from_pool(
        pool: *mut pfring_zc_buffer_pool,
    ) -> *mut pfring_zc_pkt_buff;
}
extern "C" {
    #[doc = " Release a buffer to a pool."]
    #[doc = " @param pool       The pool handle."]
    #[doc = " @param pkt_handle The buffer handle."]
    pub fn pfring_zc_release_packet_handle_to_pool(
        pool: *mut pfring_zc_buffer_pool,
        pkt_handle: *mut pfring_zc_pkt_buff,
    );
}
extern "C" {
    #[doc = " Initialise the inter-process support on a slave."]
    #[doc = " @param hugepages_mountpoint The HugeTLB mountpoint (NULL for auto-detection) for the shared memory."]
    pub fn pfring_zc_ipc_init(hugepages_mountpoint: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " Attach to a pool created by a cluster in another process."]
    #[doc = " @param cluster_id The cluster identifier."]
    #[doc = " @param pool_id    The pool identifier."]
    #[doc = " @return           The pool handle on success, NULL otherwise (errno is set appropriately)."]
    pub fn pfring_zc_ipc_attach_buffer_pool(
        cluster_id: uint32_t,
        pool_id: uint32_t,
    ) -> *mut pfring_zc_buffer_pool;
}
extern "C" {
    #[doc = " Detach a pool."]
    #[doc = " @param pool The pool handle."]
    pub fn pfring_zc_ipc_detach_buffer_pool(pool: *mut pfring_zc_buffer_pool);
}
extern "C" {
    #[doc = " Attach to a queue created by a cluster on another process."]
    #[doc = " @param cluster_id The cluster identifier."]
    #[doc = " @param queue_id   The queue identifier."]
    #[doc = " @param queue_mode The direction to open, RX or TX."]
    #[doc = " @return           The queue handle on success, NULL otherwise (errno is set appropriately)."]
    pub fn pfring_zc_ipc_attach_queue(
        cluster_id: uint32_t,
        queue_id: uint32_t,
        queue_mode: pfring_zc_queue_mode,
    ) -> *mut pfring_zc_queue;
}
extern "C" {
    #[doc = " Detach a queue."]
    #[doc = " @param queue The queue handle."]
    pub fn pfring_zc_ipc_detach_queue(queue: *mut pfring_zc_queue);
}
extern "C" {
    #[doc = " (Host) Initialise the KVM support for a VM."]
    #[doc = " @param cluster                The cluster handle."]
    #[doc = " @param vm_monitor_socket_path The monitor socket of the VM to initialise."]
    #[doc = " @return                       0 on success, a negative value otherwise."]
    pub fn pfring_zc_vm_register(
        cluster: *mut pfring_zc_cluster,
        vm_monitor_socket_path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " (Host) Enable the KVM support for all the VMs registered with pfring_zc_vm_register()."]
    #[doc = " @param cluster The cluster handle."]
    #[doc = " @return        0 on success, a negative value otherwise."]
    pub fn pfring_zc_vm_backend_enable(cluster: *mut pfring_zc_cluster) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " (Guest) Initialise the inter-VM support on a slave."]
    #[doc = " @param uio_device The UIO device path for the shared memory."]
    pub fn pfring_zc_vm_guest_init(uio_device: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " Computes an IP-based packet hash."]
    #[doc = " Hash input: <src ip, dst ip>"]
    #[doc = " @param pkt_handle The pointer to the buffer handle."]
    #[doc = " @param queue      The queue from which the packet is arrived or destined."]
    #[doc = " @return           The packet hash."]
    pub fn pfring_zc_builtin_ip_hash(
        pkt_handle: *mut pfring_zc_pkt_buff,
        queue: *mut pfring_zc_queue,
    ) -> uint32_t;
}
extern "C" {
    #[doc = " Computes a 5-tuple packet hash."]
    #[doc = " Hash input: <src ip, dst ip, src port, dst port, protocol>"]
    #[doc = " @param pkt_handle The pointer to the buffer handle."]
    #[doc = " @param queue      The queue from which the packet is arrived or destined."]
    #[doc = " @return           The packet hash."]
    pub fn pfring_zc_builtin_5tuple_hash(
        pkt_handle: *mut pfring_zc_pkt_buff,
        queue: *mut pfring_zc_queue,
    ) -> uint32_t;
}
extern "C" {
    #[doc = " Computes a GTP-C Seq-based packet hash and a GTP-U Inner-IP/Port-based packet hash, Outer-IP/Port-based packet hash otherwise."]
    #[doc = " Hash input:"]
    #[doc = " - <GTP-C Seq> for GTP-C packets"]
    #[doc = " - <inner src ip, inner dst ip, inner src port, inner dst port> for GTP-U packets"]
    #[doc = " - <src ip, dst ip, src port, dst port> for non GTP packets"]
    #[doc = " @param pkt_handle The pointer to the buffer handle."]
    #[doc = " @param queue      The queue from which the packet is arrived or destined."]
    #[doc = " @return           The packet hash."]
    pub fn pfring_zc_builtin_gtp_hash(
        pkt_handle: *mut pfring_zc_pkt_buff,
        queue: *mut pfring_zc_queue,
    ) -> uint32_t;
}
extern "C" {
    #[doc = " Computes a Inner-IP-based packet hash on GRE packets, Outer-IP/Port-based packet hash otherwise."]
    #[doc = " Hash input:"]
    #[doc = " - <inner src ip, inner dst ip> for GRE packets"]
    #[doc = " - <src ip, dst ip, src port, dst port> for non GRE packets"]
    #[doc = " @param pkt_handle The pointer to the buffer handle."]
    #[doc = " @param queue      The queue from which the packet is arrived or destined."]
    #[doc = " @return           The packet hash."]
    pub fn pfring_zc_builtin_gre_hash(
        pkt_handle: *mut pfring_zc_pkt_buff,
        queue: *mut pfring_zc_queue,
    ) -> uint32_t;
}
extern "C" {
    #[doc = " Write custom stats under /proc/net/pf_ring/stats/<cluster file>"]
    #[doc = " @param cluster The cluster handle."]
    #[doc = " @param stats   The stats string to write."]
    #[doc = " @return        0 on success, a negative value otherwise."]
    pub fn pfring_zc_set_proc_stats(
        cluster: *mut pfring_zc_cluster,
        stats: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Write application name under /proc/net/pf_ring/<socket>"]
    #[doc = " @param name  The application name."]
    #[doc = " @return      0 on success, a negative value otherwise."]
    pub fn pfring_zc_set_app_name(
        cluster: *mut pfring_zc_cluster,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Write custom device stats under /proc/net/pf_ring/stats/<device file>"]
    #[doc = " @param queue The queue handle for the device."]
    #[doc = " @param stats The stats string to write."]
    #[doc = " @return      0 on success, a negative value otherwise."]
    pub fn pfring_zc_set_device_proc_stats(
        queue: *mut pfring_zc_queue,
        stats: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Write application name under /proc/net/pf_ring/<socket>"]
    #[doc = " @param queue The queue handle for the device."]
    #[doc = " @param name  The application name."]
    #[doc = " @return      0 on success, a negative value otherwise."]
    pub fn pfring_zc_set_device_app_name(
        queue: *mut pfring_zc_queue,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Return the ZC version"]
    #[doc = " @return The PF_RING ZC version."]
    pub fn pfring_zc_version() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[doc = " Check if ZC is running in demo mode (using adapters in zero-copy mode without a valid license)"]
    #[doc = " @return 1 if ZC is running with no demo limit, 0 otherwise."]
    pub fn pfring_zc_check_license() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Check if the license for a ZC device is valid and returns the license expiration epoch."]
    #[doc = " @param queue            The queue handle for the device."]
    #[doc = " @param expiration_epoch The variable (ptr) that will contain the expiration epoch as return value."]
    #[doc = " @return 1 if the license is valid, and set the expiration epoch accordingly, 0 otherwise."]
    pub fn pfring_zc_check_device_license(
        queue: *mut pfring_zc_queue,
        expiration_epoch: *mut uint32_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Check if the license for a ZC device is valid and returns the license expiration epoch."]
    #[doc = " @param device_name      The interface name."]
    #[doc = " @param expiration_epoch The variable (ptr) that will contain the expiration epoch as return value."]
    #[doc = " @return 1 if the license is valid, and set the expiration epoch accordingly, 0 otherwise."]
    pub fn pfring_zc_check_device_license_by_name(
        device_name: *mut ::std::os::raw::c_char,
        expiration_epoch: *mut uint32_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Return the NUMA node bound to the selected core"]
    #[doc = " @param core_id The core id"]
    #[doc = " @return        node id on success, -1 otherwise."]
    pub fn pfring_zc_numa_get_cpu_node(core_id: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Set the NUMA affinity to the selected NUMA node (for memory allocation)"]
    #[doc = " @param node_id The NUMA node id"]
    #[doc = " @return        0 on success, -1 otherwise."]
    pub fn pfring_zc_numa_set_numa_affinity(
        node_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Enable debug mode"]
    pub fn pfring_zc_debug();
}
