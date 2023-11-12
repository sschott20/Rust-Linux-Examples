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
use v4l2bindings::*;

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

struct RustClient {
    _dev: Pin<Box<miscdev::Registration<RustClient>>>,
}

impl kernel::Module for RustClient {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("rust_client init (init)\n");

        let reg = miscdev::Registration::new_pinned(fmt!("rust_client"), ())?;

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

        // int kernel_connect(struct socket *sock, struct sockaddr *addr, int addrlen,
        //     int flags);
        to_result(unsafe {
            bindings::kernel_connect(socket, addr, addrlen as _, bindings::O_RDWR as _ )
        })?;

        // let mut sock: socket = unsafe { zeroed() };
        // let mut conn_socket = &mut sock as *mut socket;

        // // let _ = unsafe { sock_create_kern(2, 2, 6, &mut conn_socket) };
        // let _ =
        //     unsafe { sock_create_kern(&mut kernel::bindings::init_net, 2, 2, 0, &mut conn_socket) };

        // pr_info!("sock_create_kern: \n");

        // let mut saddr_in: sockaddr_in = unsafe { zeroed() };
        // saddr_in.sin_family = 2;
        // saddr_in.sin_port = 54321_u16.to_be();

        // // 127.0.0.1 packed into a u32 and then converted to big endian
        // saddr_in.sin_addr.s_addr = 2130706433_u32.to_be();

        // // saddr_in.sin_port = 54321_u16;
        // // saddr_in.sin_addr.s_addr = 2130706433_u32;
        // let mut saddr: sockaddr = unsafe { core::mem::transmute(saddr_in) };
        // pr_info!("saddr:\n");

        // // wtfffffffffffff this is so clipped why is this a thing
        // let connect = unsafe { (*((*conn_socket).ops)).connect.unwrap() };
        // pr_info!("connect start \n");
        // let ret = unsafe {
        //     connect(
        //         conn_socket,
        //         &mut saddr,
        //         core::mem::size_of::<sockaddr>() as i32,
        //         2,
        //     )
        // };
        // pr_info!("connect: {}\n", ret);

        // let mut msg: msghdr = unsafe { zeroed() };
        // let mut vec: kvec = unsafe { zeroed() };
        // let mut reply: [u8; 10] = [69; 10];
        // msg.msg_name = core::ptr::null_mut();
        // msg.msg_namelen = 0;

        // msg.__bindgen_anon_1.msg_control = core::ptr::null_mut();
        // msg.msg_controllen = 0;

        // // MSG_DONTWAIT
        // msg.msg_flags = 0x40;

        // let mut left = reply.len();
        // let mut written = 0;
        // vec.iov_len = left;
        // vec.iov_base = reply.as_ptr() as *mut c_void;

        // let mut len = unsafe { kernel_sendmsg(conn_socket, &mut msg, &mut vec, left, left) };
        // pr_info!("kernel_sendmsg: {}\n", len);

        pr_info!("RustClient finish init\n");
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

        Ok(1)
    }

    // will be used to pass data / addr from user to kernel space
    // seekfrom start means we are sending the physical address of the mmap buffer
    fn seek(_data: (), _file: &File, offset: SeekFrom) -> Result<u64> {
        pr_info!("Rust Client Seek\n");
        let _len = match offset {
            SeekFrom::Start(pfn) => {
                pr_info!("Incoming pfn: {}\n", pfn);
                let kern_addr = pfn_to_virt(pfn);
                pr_info!("Kernel virtual addr: {:x}\n", kern_addr);

                let mut buffer = Vec::new();
                let _ = buffer.try_push(69);
                let buffer_addr = buffer.as_ptr() as usize;
                pr_info!("Buffer virtual addr: {:x}\n", buffer_addr);
                unsafe { pr_info!("Buffer value: {}\n", *(buffer_addr as *const u8)) };

                let byte = unsafe { *(kern_addr as *const u8) };
                pr_info!("First byte at that address: {}\n", byte);
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
