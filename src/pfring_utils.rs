use crate::{ __BindgenBitfieldUnit, __IncompleteArrayField, };
use crate::libc::{ self, uint8_t, uint16_t, uint32_t, };


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iphdr {
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

impl iphdr {
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


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tcphdr {
    pub source: uint16_t,
    pub dest: uint16_t,
    pub seq: uint32_t,
    pub ack_seq: uint32_t,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize], u8>,
    pub window: uint16_t,
    pub check: uint16_t,
    pub urg_ptr: uint16_t,
}

impl tcphdr {
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


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct udphdr {
    pub source: uint16_t,
    pub dest: uint16_t,
    pub len: uint16_t,
    pub check: uint16_t,
}
