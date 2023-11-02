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
use opencv::core::{flip, Vec3b};
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
        i = i + 1;
        if i > 5 {
            break;
        }
        client.read();
        // let m: [u8] = client.buffer[0..client.buf.bytesused as usize];
        let name = format!("output{}.yuv", i);

        // let mut output: File = OpenOptions::new()
        //     .write(true)
        //     .read(true)
        //     .create(true)
        //     .open(name)
        //     .unwrap();
        let _ = s.send(&client.buffer);

        client.qbuf();
        // output
        //     .write(&client.buffer[0..client.buf.bytesused as usize])
        //     .unwrap();
    }
}
