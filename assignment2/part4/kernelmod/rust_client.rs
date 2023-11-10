// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample.
#![allow(missing_docs)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::mem::zeroed;
use kernel::bindings::{filp_open, vfs_ioctl};
use kernel::file::SeekFrom;
use kernel::file::{File, Operations};
use kernel::io_buffer::IoBufferReader;
use kernel::io_buffer::IoBufferWriter;
use kernel::net::TcpStream;
use kernel::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use kernel::prelude::Vec;
use kernel::prelude::*;
use kernel::sync::smutex::Mutex;
use kernel::{miscdev, Module};
mod bindings;
use bindings::*;
const VIDIOC_MAGIC: u8 = b'V';

const VIDIOC_QUERYCAP: u32 = 1080579584;
// pub type __u8 = core::ffi::cchar;
// pub type __u32 = core::ffi::c_uint;

// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct v4l2_pix_format {
//     pub width: __u32,
//     pub height: __u32,
//     pub pixelformat: __u32,
//     pub field: __u32,
//     pub bytesperline: __u32,
//     pub sizeimage: __u32,
//     pub colorspace: __u32,
//     pub priv_: __u32,
//     pub flags: __u32,
//     pub __bindgen_anon_1: v4l2_pix_format__bindgen_ty_1,
//     pub quantization: __u32,
//     pub xfer_func: __u32,
// }

// #[repr(C)]
// #[derive(Copy, Clone)]
// pub union v4l2_format__bindgen_ty_1 {
//     pub pix: v4l2_pix_format,
//     pub pix_mp: v4l2_pix_format_mplane,
//     pub win: v4l2_window,
//     pub vbi: v4l2_vbi_format,
//     pub sliced: v4l2_sliced_vbi_format,
//     pub sdr: v4l2_sdr_format,
//     pub meta: v4l2_meta_format,
//     pub raw_data: [__u8; 200usize],
// }

// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct v4l2_format {
//     pub type_: __u32,
//     pub fmt: v4l2_format__bindgen_ty_1,
// }

// #[repr(C)]
// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub struct v4l2_capability {
//     pub driver: [__u8; 16usize],
//     pub card: [__u8; 32usize],
//     pub bus_info: [__u8; 32usize],
//     pub version: __u32,
//     pub capabilities: __u32,
//     pub device_caps: __u32,
//     pub reserved: [__u32; 3usize],
// }

// #define VIDIOC_S_FMT		_IOWR('V',  5, struct v4l2_format)

module! {
    type: RustClient,
    name: "rust_client",
    author: "Alex Schott",
    license: "GPL",
}

const PAGE_OFFSET: u64 = 0xffff_8000_0000_0000; // example for x86_64, typically high half

fn pfn_to_virt(pfn: u64) -> u64 {
    ((pfn << 12) + PAGE_OFFSET) as u64
}

struct RustClient {
    _dev: Pin<Box<miscdev::Registration<RustClient>>>,
}

impl kernel::Module for RustClient {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("rust_client init (init)\n");
        let reg = miscdev::Registration::new_pinned(fmt!("rust_client"), ())?;

        pr_info!(
            "sizeof v4l2_capability: {}\n",
            core::mem::size_of::<v4l2_capability>()
        );
        pr_info!("sizeof : {}\n", core::mem::size_of::<v4l2_format>());
        Ok(RustClient { _dev: reg })
    }
}
#[vtable]
impl Operations for RustClient {
    fn open(_context: &(), _file: &File) -> Result {
        pr_info!("RustClient was opened\n");

        // let stream = TcpStream::connect("127.0.0.1:54321").unwrap();
        // ioctl_read!(vidioc_querycap, VIDIOC_MAGIC, 0, v4l2_capability);
        // let _ = unsafe { vfs_ioctl(file, VIDIOC_MAGIC as u32, 0) };

        Ok(())
    }
    fn read(
        _data: (),
        _file: &File,
        writer: &mut impl IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("RustClient Read\n");
        // let mut file = unsafe {
        //     let c_str = CStr::from_bytes_with_nul(b"/dev/video2\0").unwrap();
        //     filp_open(c_str.as_ptr() as *const i8, 0, 0)
        // };
        // // let mut stream = TcpStream::connect()
        // let addr = Ipv4Addr::new(127, 0, 0, 1);
        // let sok: SocketAddr = SocketAddr::V4(SocketAddrV4::new(addr, 54321));
        // let namespace = kernel::net::init_ns();
        // let lo_cstr = CStr::from_bytes_with_nul(b"lo\0").unwrap();
        // namespace.dev_get_by_name(lo_cstr).unwrap();
        // let listener = kernel::net::TcpListener::try_new(&namespace, &sok).unwrap();
        // let mut stream = listener.accept(true).unwrap();

        Ok(10)
    }
    fn write(
        _data: (),
        _file: &File,
        reader: &mut impl IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("RustClient Write\n");

        let mut info_capability: v4l2_capability = unsafe { zeroed() };

        let mut filp = unsafe {
            let c_str = CStr::from_bytes_with_nul(b"/dev/video0\0").unwrap();
            filp_open(c_str.as_ptr() as *const i8, 2, 0)
        };
        let _ = unsafe { vfs_ioctl(filp, VIDIOC_QUERYCAP, &mut info_capability as *mut _ as u64) };
        pr_info!(
            "driver: {:?}\n",
            core::str::from_utf8(&info_capability.driver)
        );

        Ok(10)
    }

    // will be used to pass data / addr from user to kernel space
    // seekfrom start means we are sending the physical address of the mmap buffer
    fn seek(_data: (), _file: &File, offset: SeekFrom) -> Result<u64> {
        pr_info!("Rust Client Seek\n");
        let _len = match offset {
            SeekFrom::Start(pfn) => {
                // pr_info!("Incoming pfn: {}\n", pfn);
                // let kern_addr = pfn_to_virt(pfn);
                // pr_info!("Kernel virtual addr: {:x}\n", kern_addr);
                // // let mut buffer = vec![1, 2, 3];
                // let mut buffer = Vec::new();
                // let _ = buffer.try_push(69);
                // let buffer_addr = buffer.as_ptr() as usize;
                // pr_info!("Buffer virtual addr: {:x}\n", buffer_addr);
                // unsafe { pr_info!("Buffer value: {}\n", *(buffer_addr as *const u8)) };
                // let byte = unsafe { *(kern_addr as *const u8) };
                // pr_info!("First byte at that address: {}\n", byte);
                // pr_info!("Kernel virtual addr: {:x}\n", kern_addr);
            }
            _ => {
                return Err(EINVAL);
            }
        };

        Ok(10)
    }
}
impl Drop for RustClient {
    fn drop(&mut self) {
        pr_info!("RustClient (exit)\n");
    }
}
