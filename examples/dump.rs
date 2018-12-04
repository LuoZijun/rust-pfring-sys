extern crate libc;
extern crate pfring_sys;

use std::mem;
use std::ffi::CStr;
use std::ffi::CString;


pub fn is_pfring_enabled() -> bool {
    if cfg!(target_os = "linux") {
        let output = std::process::Command::new("lsmod")
            .output()
            .expect("failed to execute process: lsmod");
        
        if output.status.success() {
            match std::str::from_utf8(&output.stdout) {
                Ok(res) => {
                    for line in res.lines() {
                        if line.starts_with("pf_ring") {
                            debug!("\n$ lsmod\n{}", line);
                            return true;
                        }
                    }
                    false
                },
                Err(e) => {
                    error!("{:?}", output);
                    panic!("{:?}", e);
                },
            }
        } else {
            error!("{:?}", output);
            panic!("failed to execute process: $ lsmod , exit status: {}", output.status);
        }
    } else {
        false
    }
}

unsafe fn run () {
    let mut if_ptr: *mut pfring_sys::pfring_if = pfring_sys::pfring_findalldevs();
    while if_ptr.is_null() == false {
        println!("netif: {:?}", CStr::from_ptr((*if_ptr).name ));
        if_ptr = (*if_ptr).next;
    }
    
    let mut version = 0u32;
    pfring_sys::pfring_version_noring(&mut version as *mut libc::uint32_t);
    println!("pfring_version_noring: {:?}", version);
    
    let ifname = CString::new("enp3s0").expect("CString::new failed");
    let caplen = 1u32;
    let flags = 0 | pfring_sys::PF_RING_DO_NOT_PARSE ;

    let ring: *mut pfring_sys::__pfring = pfring_sys::pfring_open(ifname.as_ptr(), caplen, flags);
    println!("pfring_open({:?}): {:?}", ifname, ring);
    println!("pfring_enable_ring: {:?}", pfring_sys::pfring_enable_ring(ring) );
    println!("ring is null : {:?}", ring.is_null() );

    let mut version = 0u32;
    pfring_sys::pfring_version(ring, &mut version);
    println!("pfring_version: {:?}", version);

    const BLEN: usize = 1500;
    let buffer_ptr: *mut *mut u8 = [ [0u8; BLEN].as_mut_ptr(); BLEN ].as_mut_ptr();

    let mut hdr: pfring_sys::pfring_pkthdr = mem::zeroed();
    let wait_for_incoming_packet = 1;
    
    println!("pfring_recv ... ", );
    let ret = pfring_sys::pfring::pfring_recv(ring,
                                      buffer_ptr,
                                      BLEN as u32,
                                      &mut hdr,
                                      wait_for_incoming_packet );
    println!("pfring_recv ret: {:?}", ret);
    println!("{:?}", hdr.caplen);
}


fn main () {
    unsafe {
        if !is_pfring_enabled() {
            println!("[ERROR] 内核模块 `pf_ring` 未加载！");
            return ();
        }

        if libc::getuid() != 0 {
            println!("[ERROR]: PermissionDenied");
            return ();
        }
        
        run();
    }
}
