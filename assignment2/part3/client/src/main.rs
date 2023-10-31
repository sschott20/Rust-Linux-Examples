use nix::{ioctl_read, sys::ioctl};
use std::mem::size_of;
use std::{fs::File, os::unix::prelude::AsRawFd, str};

// #define VIDIOC_QUERYCAP          _IOR('V',  0, struct v4l2_capability)
const VIDIOC_QUERYCAP_MAGIC: u8 = 'V' as u8;
const VIDIOC_QUERYCAP_TYPE_MODE: u8 = 0;

// #define VIDIOC_G_INPUT           _IOR('V', 38, int)
const VIDIOC_G_INPUT_MAGIC: u8 = 'V' as u8;
const VIDIOC_G_INPUT_TYPE_MODE: u8 = 38;

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

#[repr(C)]
#[derive(Default)]
pub struct v4l2_input {
    pub index: u32,
    pub name: [u8; 32],
    pub type_: u32,
    pub audioset: u32,
    pub tuner: u32,
    pub std: u32,
    pub status: u32,
    pub capabilities: u32,
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
    let mut info_capability: v4l2_capability = Default::default();

    match unsafe { vidioc_querycap(media_fd, &mut info_capability as *mut v4l2_capability) } {
        Ok(_) => {
            println!("get info querycap [OK]");
            println!("driver: {:?}", str::from_utf8(&info_capability.driver));
            println!("card: {:?}", str::from_utf8(&info_capability.card));
            println!("bus_info: {:?}", str::from_utf8(&info_capability.bus_info));
            println!("version: {:?}", info_capability.version);
            println!("capabilities: {:?}", info_capability.capabilities);
            println!("device_caps: {:?}", info_capability.device_caps);
        }
        Err(e) => {
            println!("get info querycap [FAILED]: {:?}", e);
        }
    }

    ioctl_read!(
        vidioc_g_input,
        VIDIOC_G_INPUT_MAGIC,
        VIDIOC_G_INPUT_TYPE_MODE,
        v4l2_input
    );

    let mut info_input: v4l2_input = Default::default();

    match unsafe { vidioc_g_input(media_fd, &mut info_input as *mut v4l2_input) } {
        Ok(_) => {
            println!("get info g_input [OK]");

            println!("index: {:?}", info_input.index);
        }
        Err(e) => {
            println!("get info g_input [FAILED]: {:?}", e);
        }
    }

    println!("Client exit [OK]");
}
