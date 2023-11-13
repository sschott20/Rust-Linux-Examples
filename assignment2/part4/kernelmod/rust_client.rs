// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample.
#![allow(missing_docs)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::cell::UnsafeCell;
use kernel::io_mem::{IoMem, Resource};

use core::ffi::c_void;
use core::mem::zeroed;
use kernel::bindings;
use kernel::bindings::{filp_open, kernel_sendmsg, vfs_ioctl};
use kernel::bindings::{kvec, msghdr};
use kernel::bindings::{sock_create_kern, sockaddr, sockaddr_in, socket};
use kernel::error::to_result;
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
mod ionum;
use ionum::*;
mod v4l2bindings;
use kernel::sync::Arc;
use kernel::sync::ArcBorrow;
use v4l2bindings::*;

// /home/alex/linux-cs429-fall-2023/rust/bindings/bindings_generated.rs

pub struct Namespace(UnsafeCell<bindings::net>);

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
fn pfn_to_phys(pfn: u64) -> u64 {
    (pfn << 12) as u64
}

struct RustClient {
    _dev: Pin<Box<miscdev::Registration<RustClient>>>,
}

struct Device {
    pfn_list: Mutex<Vec<u64>>,
}

impl kernel::Module for RustClient {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("rust_client init (init)\n");
        let reg = miscdev::Registration::new_pinned(
            fmt!("rust_client"),
            Arc::try_new(Device {
                pfn_list: Mutex::new(Vec::<u64>::new()),
            })
            .unwrap(),
        )?;

        Ok(Self { _dev: reg })
    }
}
#[vtable]
impl Operations for RustClient {
    type Data = Arc<Device>;
    type OpenData = Arc<Device>;

    fn open(ctxt: &Arc<Device>, _file: &File) -> Result<Arc<Device>> {
        pr_info!("RustClient was opened\n");

        Ok(ctxt.clone())
    }
    fn read(
        data: ArcBorrow<'_, Device>,
        _file: &File,
        writer: &mut impl IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("RustClient Read\n");

        let v4 = Ipv4Addr::new(127, 0, 0, 1);
        let addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(v4, 54321));

        let namespace: &'static Namespace =
            unsafe { &*core::ptr::addr_of!(bindings::init_net).cast() };

        let mut socket = core::ptr::null_mut();

        let (pf, addr, addrlen) = match addr {
            SocketAddr::V4(addr) => (
                bindings::PF_INET,
                &addr as *const _ as _,
                core::mem::size_of::<sockaddr_in>(),
            ),
            _ => panic!("ipv6 not supported"),
        };
        to_result(unsafe {
            bindings::sock_create_kern(
                namespace.0.get(),
                pf as _,
                bindings::sock_type_SOCK_STREAM as _,
                bindings::IPPROTO_TCP as _,
                &mut socket,
            )
        })?;

        to_result(unsafe {
            bindings::kernel_connect(socket, addr, addrlen as _, bindings::O_RDWR as _)
        })?;
        let mut buf: v4l2_buffer = unsafe { zeroed() };
        buf.type_ = 1;
        buf.memory = 1;
        buf.index = 0;

        let mut filp = unsafe {
            let c_str = CStr::from_bytes_with_nul(b"/dev/video2\0").unwrap();
            filp_open(c_str.as_ptr() as *const i8, 2, 0)
        };
        let _ = unsafe { vfs_ioctl(filp, VIDIOC_QUERYBUF, &mut buf as *mut _ as u64) };
        pr_info!("Buffer length: {:?}\n", buf.length);
        let _ = unsafe { vfs_ioctl(filp, VIDIOC_DQBUF, &mut buf as *mut _ as u64) };

        // let _ = unsafe { vfs_ioctl(filp, VIDIOC_QUERYCAP, &mut info_capability as *mut _ as u64) };

        let pfn_list = data.pfn_list.lock();
        for pfn in pfn_list.iter() {
            let mut phys_addr = pfn_to_phys(*pfn);
            let mut kern_addr =
                unsafe { bindings::memremap(phys_addr, 2 * 4096, bindings::MEMREMAP_WB as _) }
                    as *mut u8;

            let mut slice = unsafe { core::slice::from_raw_parts_mut(kern_addr, 2 * 4096) };

            let mut msg = bindings::msghdr {
                msg_flags: bindings::MSG_DONTWAIT,
                ..bindings::msghdr::default()
            };
            let mut vec = bindings::kvec {
                iov_base: slice.as_mut_ptr() as _,
                iov_len: 4096,
            };

            let r = unsafe { bindings::kernel_sendmsg(socket, &mut msg, &mut vec, 1, vec.iov_len) };
        }

        let mut buf: v4l2_buffer = unsafe { zeroed() };
        buf.type_ = 1;
        buf.memory = 1;
        buf.index = 0;
        let _ = unsafe { vfs_ioctl(filp, VIDIOC_QBUF, &mut buf as *mut _ as u64) };

        // recieve the processed .bmp image back
        // let mut ret_buf: [u8; 110646] = [0; 110646];

        // let mut msg = bindings::msghdr::default();
        // let mut vec = bindings::kvec {
        //     iov_base: ret_buf.as_mut_ptr().cast(),
        //     iov_len: ret_buf.len(),
        // };
        // let r = unsafe {
        //     bindings::kernel_recvmsg(
        //         socket,
        //         &mut msg,
        //         &mut vec,
        //         1,
        //         vec.iov_len,
        //         bindings::MSG_DONTWAIT as _,
        //     )
        // };
        // write entire ret_buf to writer
        // writer.write_slice(&ret_buf).unwrap();
        Ok(10)
    }
    fn write(
        _data: ArcBorrow<'_, Device>,
        _file: &File,
        reader: &mut impl IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("RustClient Write\n");

        Ok(1)
    }

    // will be used to pass data / addr from user to kernel space
    // seekfrom start means we are sending the physical address of the mmap buffer
    fn seek(data: ArcBorrow<'_, Device>, _file: &File, offset: SeekFrom) -> Result<u64> {
        // pr_info!("Rust Client Seek\n");
        let _len = match offset {
            SeekFrom::Start(pfn) => {
                let mut pfn_list = data.pfn_list.lock();
                pfn_list.try_push(pfn)?;
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
