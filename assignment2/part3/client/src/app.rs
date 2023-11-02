#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use std::{thread, time};

use nix;
use nix::ioctl_read;
use nix::ioctl_readwrite;
use nix::ioctl_write_ptr;
mod bindings;
use bindings::*;
use opencv::core::{flip, Vec3b};
use opencv::videoio::*;
use opencv::{highgui::*, prelude::*, videoio};

use memmap::Mmap;
use memmap::MmapOptions;
mod setup;
use nix::sys::ioctl;
use nix::sys::select;
use nix::sys::select::FdSet;
use setup::*;
mod app;
use app::*;
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
    // #define VIDIOC_QBUF _IOWR('V', 15, struct v4l2_buffer)

}

fn qbuf(media_fd: i32){
    let mut buf: v4l2_buffer = unsafe { std::mem::zeroed() };
    buf.type_ = 1;
    buf.memory = 1;
    buf.index = 0;

    match unsafe { vidioc_qbuf(media_fd, &mut buf) } {
        Ok(_) => {
            println!("qbuf [OK]");
        }
        Err(e) => {
            println!("qbuf [FAILED]: {:?}", e);
        }
    }
}

pub struct App {
    buffer: memmap::MmapMut,
    buf: v4l2_buffer,
    media_fd: i32,
    
}

impl App {
    fn new() -> App {
        let mut file = File::options()
            .write(true)
            .read(true)
            .open("/dev/video2")
            .unwrap();

        media_fd = file.as_raw_fd();
        println!("camera fd = {}", media_fd);

        let mut format: v4l2_format = setup_vidio(media_fd);
        let mut reqbuff: v4l2_requestbuffers = request_buffer(media_fd);
        buf: v4l2_buffer = query_buffer(media_fd);
        let mut stream_on = stream_on(media_fd);

        buffer = unsafe {
            memmap::MmapOptions::new()
                .len(buf.length as usize)
                .map_mut(&file)
                .unwrap()
        };
        App {
            buffer: buffer,
            buf: buf,
            media_fd: media_fd,
        }

        
    }

    fn read(&mut self) {
        qbuf(media_fd);
        
        let mut readfds: FdSet = FdSet::new();
        readfds.insert(self.media_fd);
        let _ = select::select(self.media_fd + 1, &mut readfds, None, None, None);
        println!("select [OK]");

   

        // #define VIDIOC_DQBUF _IOWR('V', 17, struct v4l2_buffer)
        match unsafe { vidioc_dqbuf(self.media_fd, &mut self.buf) } {
            Ok(_) => {
                println!("dqbuf [OK]");
            }
            Err(e) => {
                println!("dqbuf [FAILED]: {:?}", e);
            }
        }

        println!("bytesused: {:?}", buf.bytesused);


 


        // let mut frame = Mat::default();
        // self.cam
        //     .read(&mut frame)
        //     .expect("VideoCapture: read [FAILED]");
        // // resize the image as a square, size is
        // let mut flipped = Mat::default();
        // flip(&frame, &mut flipped, 1).expect("flip [FAILED]");
        // let resized_img = resize_with_padding(&flipped, [192, 192]);
        // resized_img
    }
}
