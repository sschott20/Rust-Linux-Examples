use nix::{ioctl_read, sys::ioctl};
use std::mem::size_of;
use std::{fs::File, os::unix::prelude::AsRawFd, str};

const VIDIOC_QUERYCAP_MAGIC: u8 = 'V' as u8;
const VIDIOC_QUERYCAP_TYPE_MODE: u8 = 0;

#[repr(C)]
#[derive(Default)]
pub struct v4l2_capability {
    pub driver: [u8; 16],
    pub card: [u8; 32],
    pub bus_info: [u8; 32],
    pub version: u32,
    pub capabilities: u32,
    pub device_caps: u32,
    pub reserved: [u32; 3],
}

fn main() {
    let mut file = File::options()
        .write(true)
        .read(true)
        .open("/dev/video0")
        .unwrap();

    let media_fd = file.as_raw_fd();
    println!("camera fd = {}", media_fd);

    ioctl_read!(
        vidioc_querycap,
        VIDIOC_QUERYCAP_MAGIC,
        VIDIOC_QUERYCAP_TYPE_MODE,
        v4l2_capability
    );
    let mut info: v4l2_capability = Default::default();

    match unsafe { vidioc_querycap(media_fd, &mut info as *mut v4l2_capability) } {
        Ok(_) => {
            println!("get info [OK]");
            println!("driver: {:?}", str::from_utf8(&info.driver));
            println!("card: {:?}", str::from_utf8(&info.card));
            println!("bus_info: {:?}", str::from_utf8(&info.bus_info));
            println!("version: {:?}", info.version);
            println!("capabilities: {:?}", info.capabilities);
            println!("device_caps: {:?}", info.device_caps);
            
        }
        Err(e) => {
            println!("get info [FAILED]: {:?}", e);
        }
    }

    println!("driver: {:?}", str::from_utf8(&info.driver));
}
