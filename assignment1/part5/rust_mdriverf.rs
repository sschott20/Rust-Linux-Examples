// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample.
#![allow(missing_docs)]
#![allow(unused_imports)]
use kernel::prelude::*;
use kernel::sync::smutex::Mutex;

module! {
    type: Mdriverf,
    name: "mdriverf",
    author: "Alex Schott",
    license: "GPL",
}

struct Mdriverf;

static MEM_BUFFER: Mutex<[u8; 512000]> = Mutex::new([0u8; 512000]);

impl kernel::Module for Mdriverf {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("mdriverf (init)\n");

        Ok(Mdriverf {})
    }
}

pub fn read(outbuf: &mut [u8], offset: usize) -> usize {
    // pr_info!("read\n");

    let mut i = 0;
    let mem_guard = MEM_BUFFER.lock();

    while i < outbuf.len() {
        // if offset + i >= 512000 {
        //     return Err(EINVAL);
        // }
        outbuf[i] = mem_guard[offset + i];
        i += 1;
    }
    return i;
}

pub fn write(inbuf: &[u8], offset: usize) -> usize {
    // pr_info!("write\n");
    let mut i = 0;
    let mut mem_guard = MEM_BUFFER.lock();

    while i < inbuf.len() {
        // if offset + i >= 512000 {
        //     return Err(EINVAL);
        // }
        mem_guard[offset + i] = inbuf[i];
        i += 1;
    }

    return i;
}

impl Drop for Mdriverf {
    fn drop(&mut self) {
        

        pr_info!("Rust mdriver (exit)\n");
    }
}
