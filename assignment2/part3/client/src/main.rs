use nix;
use nix::ioctl_read;
use nix::ioctl_readwrite;

mod bindings;
use bindings::*;

mod setup;
use setup::*;

use std::{fs::File, os::unix::prelude::AsRawFd, str};

const VIDIOC_MAGIC: u8 = b'V';

fn main() {
    let file = File::options()
        .write(true)
        .read(true)
        .open("/dev/video0")
        .unwrap();

    let mut media_fd = file.as_raw_fd();
    println!("camera fd = {}", media_fd);

    let mut format: v4l2_format = setup_vidio(media_fd);
    

    println!("Client exit [OK]");
}
