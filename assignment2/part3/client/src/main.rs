use nix;
use nix::ioctl_read;
use nix::ioctl_readwrite;
use nix::ioctl_write_ptr;
mod bindings;
use bindings::*;

mod setup;
use setup::*;

use std::{fs::File, os::unix::prelude::AsRawFd, str};

const VIDIOC_MAGIC: u8 = b'V';

fn request_buffer(media_fd: i32) -> v4l2_requestbuffers {
    // #define VIDIOC_REQBUFS          _IOWR('V',  8, struct v4l2_requestbuffers)
    ioctl_readwrite!(vidioc_reqbufs, VIDIOC_MAGIC, 8, v4l2_requestbuffers);

    let mut reqbufs: v4l2_requestbuffers = unsafe { std::mem::zeroed() };
    reqbufs.count = 1;

    // V4L2_BUF_TYPE_VIDEO_CAPTURE
    reqbufs.type_ = 1;

    // userptr
    reqbufs.memory = 2;

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
    ioctl_readwrite!(vidioc_querybuf, VIDIOC_MAGIC, 9, v4l2_buffer);
    let mut buf: v4l2_buffer = unsafe { std::mem::zeroed() };
    buf.type_ = 1;
    buf.memory = 2;
    buf.index = 0;
    match unsafe { vidioc_querybuf(media_fd, &mut buf) } {
        Ok(_) => {
            println!("querybuf [OK]");
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
    match unsafe { vidioc_streamon(media_fd, &buf_type) } {
        Ok(_) => {
            println!("streamon [OK]");
        }
        Err(e) => {
            println!("streamon [FAILED]: {:?}", e);
        }
    }
}

fn main() {
    let file = File::options()
        .write(true)
        .read(true)
        .open("/dev/video0")
        .unwrap();

    let mut media_fd = file.as_raw_fd();
    println!("camera fd = {}", media_fd);

    let mut format: v4l2_format = setup_vidio(media_fd);
    let mut reqbuff: v4l2_requestbuffers = request_buffer(media_fd);
    let mut buf: v4l2_buffer = query_buffer(media_fd);
    let mut stream_on = stream_on(media_fd);

    println!("Client exit [OK]");
}
