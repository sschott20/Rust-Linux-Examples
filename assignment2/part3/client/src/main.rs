use nix;
use nix::ioctl_read;
use nix::ioctl_readwrite;

// use std::mem::size_of;
use std::{fs::File, os::unix::prelude::AsRawFd, str};

// #define VIDIOC_QUERYCAP          _IOR('V',  0, struct v4l2_capability)
const VIDIOC_QUERYCAP_MAGIC: u8 = b'V';
const VIDIOC_QUERYCAP_TYPE_MODE: u8 = 0;

// #define VIDIOC_G_INPUT           _IOR('V', 38, int)
const VIDIOC_G_INPUT_MAGIC: u8 = b'V';
const VIDIOC_G_INPUT_TYPE_MODE: u8 = 38;

// #define VIDIOC_ENUMINPUT	_IOWR('V', 26, struct v4l2_input)
const VIDIOC_ENUMINPUT_MAGIC: u8 = b'V';
const VIDIOC_ENUMINPUT_TYPE_MODE: u8 = 26;

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
    pub r#type: u32,
    pub audioset: u32,
    pub tuner: u32,
    pub std: u64,
    pub status: u32,
    pub capabilities: u32,
    pub reserved: [u32; 3],
}

fn main() {
    let file = File::options()
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

    match unsafe { vidioc_querycap(media_fd, &mut info_capability) } {
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
        u32
    );

    // ioctl_readwrite!(
    //     vidioc_enuminput,
    //     VIDIOC_ENUMINPUT_MAGIC,
    //     VIDIOC_ENUMINPUT_TYPE_MODE,
    //     v4l2_input
    // );

    ioctl_readwrite!(vidioc_enuminput, b'V', 26, v4l2_input);

    let mut index: u32 = Default::default();

    match unsafe { vidioc_g_input(media_fd, &mut index) } {
        Ok(_) => {
            println!("get info g_input [OK]");

            let mut input: v4l2_input = unsafe { std::mem::zeroed() };
            // input.index = index;

            match unsafe { vidioc_enuminput(media_fd, &mut input) } {
                Ok(_) => {
                    println!("get info enuminput [OK]");
                    println!("current input name: {:?}", str::from_utf8(&input.name));
                }
                Err(e) => {
                    println!("get info enuminput [FAILED]: {:?}", e);
                }
            }
        }

        Err(e) => {
            println!("get info g_input [FAILED]: {:?}", e);
        }
    }

    println!("Client exit [OK]");
}
