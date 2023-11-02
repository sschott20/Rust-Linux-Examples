#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use std::{thread, time};
mod utils;

use nix;
use nix::ioctl_read;
use nix::ioctl_readwrite;
use nix::ioctl_write_ptr;
mod bindings;
use bindings::*;
use opencv::core::{flip, Mat, Vec3b};
use opencv::videoio::*;
use opencv::{highgui::*, prelude::*, videoio};
use std::net::TcpStream;

use memmap::Mmap;
use memmap::MmapOptions;
mod server;
mod setup;
use nix::sys::ioctl;
use nix::sys::select;
use nix::sys::select::FdSet;
use server::*;
use setup::*;
mod app;
use app::*;
use std::{fs::File, os::unix::prelude::AsRawFd, str};
use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
};
use utils::*;

fn main() {
    let mut f = File::options()
        .write(true)
        .read(true)
        .open("/dev/video2")
        .unwrap();

    let mut fd = f.as_raw_fd();
    let mut ftmp = File::options()
        .write(true)
        .read(true)
        .open("capture.c")
        .unwrap();

    let mut client: App = App {
        buffer: unsafe { memmap::MmapOptions::new().len(1).map_mut(&ftmp).unwrap() },
        file: f,
        buf: unsafe { std::mem::zeroed() },
        media_fd: fd,
    };
    client.start_device(3);

    client.buffer = unsafe {
        memmap::MmapOptions::new()
            .len(client.buf.length as usize)
            .map_mut(&client.file)
            .unwrap()
    };

    let mut s = Server {
        stream: TcpStream::connect("127.0.0.1:54321").expect("failed to connect"),
    };
    let mut i = 0;
    loop {
        if i > 0 {
            break;
        }
        i = i + 1;

        client.read();
        let mut tmp = File::options()
            .write(true)
            .read(true)
            .create(true)
            .open("tmp.yuv")
            .unwrap();

        let mut inbuf = [0; 462848];
        let mut outbuf = [0; 462848];
        inbuf = (&client.buffer[0..client.buf.bytesused as usize]);
        let converted = yuv422_to_rgb32(&inbuf, &mut outbuf);

        // tmp.write_all(&client.buffer).unwrap();
        // let mut mat: Mat = Mat::default();
        // let mut b: Vec<u8> = vec![0; 462848];
        // b.copy_from_slice(&client.buffer);
        // println!("first 100 of b: {:?}", &b[0..100]);
        // let _ =
        //     opencv::imgcodecs::imdecode_to(&opencv::types::VectorOfu8::from_iter(b), -1, &mut mat);

        // opencv::highgui::imshow("test", &mut mat).expect("imshow [error]");
        // let key = wait_key(10).unwrap();
        // if key > 0 && key != 255 {
        //     break;
        // }
        // let _ = s.send(&client.buffer);

        client.qbuf();
    }
}
