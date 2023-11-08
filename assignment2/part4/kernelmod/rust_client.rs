// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample.
#![allow(missing_docs)]
#![allow(unused_imports)]
use kernel::bindings::{filp_open, vfs_ioctl};
use kernel::file::{File, Operations};
use kernel::prelude::*;
use kernel::sync::smutex::Mutex;
use kernel::{miscdev, Module};
// use kernel::str::CStr;

module! {
    type: RustClient,
    name: "rust_client",
    author: "Alex Schott",
    license: "GPL",
}

struct RustClient {
    _dev: Pin<Box<miscdev::Registration<RustClient>>>,
}

impl kernel::Module for RustClient {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("rust_client init (init)\n");
        let reg = miscdev::Registration::new_pinned(fmt!("rust_client"), ())?;
        Ok(RustClient { _dev: reg })
    }
}
#[vtable]
impl Operations for RustClient {
    fn open(_context: &(), _file: &File) -> Result {
        pr_info!("RustClient was opened\n");
        unsafe {
            let c_str = CStr::from_bytes_with_nul(b"/dev/video2\0").unwrap();
            let _file = filp_open(c_str.as_ptr() as *const i8, 0, 0);
        }

        Ok(())
    }
}
impl Drop for RustClient {
    fn drop(&mut self) {
        pr_info!("RustClient (exit)\n");
    }
}
