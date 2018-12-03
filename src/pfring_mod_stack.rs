use crate::libc;
use crate::pfring::pfring;

extern "C" {
    pub fn pfring_mod_stack_open(ring: *mut pfring) -> libc::c_int;
}