#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use std::{thread, time};

use nix;
use nix::ioctl_read;
use nix::ioctl_readwrite;
use nix::ioctl_write_ptr;
use opencv::core::{flip, Vec3b};
use opencv::videoio::*;
use opencv::{highgui::*, prelude::*, videoio};

use memmap::Mmap;
use memmap::MmapOptions;
use nix::sys::ioctl;
use nix::sys::select;
use nix::sys::select::FdSet;

use crate::bindings::*;
use crate::setup::*;

use std::{fs::File, os::unix::prelude::AsRawFd, str};
use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
};

const VIDIOC_MAGIC: u8 = b'V';

ioctl_readwrite!(vidioc_qbuf, VIDIOC_MAGIC, 15, v4l2_buffer);
ioctl_readwrite!(vidioc_dqbuf, VIDIOC_MAGIC, 17, v4l2_buffer);
ioctl_readwrite!(vidioc_reqbufs, VIDIOC_MAGIC, 8, v4l2_requestbuffers);
ioctl_readwrite!(vidioc_querybuf, VIDIOC_MAGIC, 9, v4l2_buffer);

fn request_buffer(media_fd: i32) -> v4l2_requestbuffers {
    // #define VIDIOC_REQBUFS          _IOWR('V',  8, struct v4l2_requestbuffers)

    let mut reqbufs: v4l2_requestbuffers = unsafe { std::mem::zeroed() };
    reqbufs.count = 1;

    // V4L2_BUF_TYPE_VIDEO_CAPTURE
    reqbufs.type_ = 1;

    // mmap
    reqbufs.memory = 1;

    match unsafe { vidioc_reqbufs(media_fd, &mut reqbufs) } {
        Ok(_) => {
            println!("reqbufs [OK]");
        }
        Err(e) => {
            println!("reqbufs [FAILED]: {:?}", e);
        }
    }

    reqbufs
}

fn query_buffer(media_fd: i32) -> v4l2_buffer {
    // #define VIDIOC_QUERYBUF _IOWR('V', 9, struct v4l2_buffer)
    let mut buf: v4l2_buffer = unsafe { std::mem::zeroed() };
    buf.type_ = 1;
    buf.memory = 1;
    buf.index = 0;
    match unsafe { vidioc_querybuf(media_fd, &mut buf) } {
        Ok(_) => {
            println!("querybuf [OK]");
            println!("index: {:?}", buf.index);
            println!("type_: {:?}", buf.type_);
            println!("bytesused: {:?}", buf.bytesused);
            println!("flags: {:?}", buf.flags);
            println!("field: {:?}", buf.field);
            println!("timestamp: {:?}", buf.timestamp);
            println!("timecode: {:?}", buf.timecode);
            println!("sequence: {:?}", buf.sequence);
            println!("memory: {:?}", buf.memory);
            println!("length: {:?}", buf.length);
            println!("reserved2: {:?}", buf.reserved2);
        }
        Err(e) => {
            println!("querybuf [FAILED]: {:?}", e);
        }
    }
    buf
}
fn stream_on(media_fd: i32) {
    // #define VIDIOC_STREAMON		 _IOW('V', 18, int)
    ioctl_write_ptr!(vidioc_streamon, VIDIOC_MAGIC, 18, i32);
    let buf_type = 1;
    match unsafe { vidioc_streamon(media_fd, &1) } {
        Ok(_) => {
            println!("streamon [OK]");
            // println!("")
        }
        Err(e) => {
            println!("streamon [FAILED]: {:?}", e);
        }
    }
}

fn qbuf(media_fd: i32) -> v4l2_buffer {
    let mut buf: v4l2_buffer = unsafe { std::mem::zeroed() };
    buf.type_ = 1;
    buf.memory = 1;
    buf.index = 0;

    // #define VIDIOC_QBUF _IOWR('V', 15, struct v4l2_buffer)

    match unsafe { vidioc_qbuf(media_fd, &mut buf) } {
        Ok(_) => {
            println!("qbuf [OK]");
        }
        Err(e) => {
            println!("qbuf [FAILED]: {:?}", e);
        }
    }

    buf
}

pub struct App {
    pub buffer: memmap::MmapMut,
    pub file: File,
    pub media_fd: i32,
    pub buf: v4l2_buffer,
}

impl App {
    pub fn start_device(&mut self, fd: i32) {
        println!("camera fd = {}", fd);

        let mut format: v4l2_format = setup_vidio(fd);
        let mut reqbuff: v4l2_requestbuffers = request_buffer(fd);
        let mut buf: v4l2_buffer = query_buffer(fd);
        let mut stream_on = stream_on(fd);
        self.buf = qbuf(fd);
    }

    pub fn read(&mut self) {
        let mut readfds: FdSet = FdSet::new();
        readfds.insert(self.media_fd);
        let _ = select::select(self.media_fd + 1, &mut readfds, None, None, None);
        println!("select [OK]");

        self.buf = unsafe { std::mem::zeroed() };
        self.buf.type_ = 1;
        self.buf.memory = 1;
        // #define VIDIOC_DQBUF _IOWR('V', 17, struct v4l2_buffer)
        match unsafe { vidioc_dqbuf(self.media_fd, &mut self.buf) } {
            Ok(_) => {
                println!("dqbuf [OK]");
            }
            Err(e) => {
                println!("dqbuf [FAILED]: {:?}", e);
            }
        }

        println!("bytesused: {:?}", self.buf.bytesused);
    }
}
