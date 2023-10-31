use nix::{ioctl_read, ioctl_readwrite, sys::ioctl};
use std::mem::size_of;
use std::{fs::File, os::unix::prelude::AsRawFd, str};

// #define VIDIOC_QUERYCAP          _IOR('V',  0, struct v4l2_capability)
const VIDIOC_QUERYCAP_MAGIC: u8 = 'V' as u8;
const VIDIOC_QUERYCAP_TYPE_MODE: u8 = 0;

// #define VIDIOC_G_INPUT           _IOR('V', 38, int)
const VIDIOC_G_INPUT_MAGIC: u8 = 'V' as u8;
const VIDIOC_G_INPUT_TYPE_MODE: u8 = 38;

// #define VIDIOC_G_FMT            _IOWR('V',  4, struct v4l2_format)
const VIDIOC_G_FMT_MAGIC: u8 = 'V' as u8;
const VIDIOC_G_FMT_TYPE_MODE: u8 = 4;

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
// #[repr(C)]
// pub union V4L2FormatUnion {
//     raw_data: [u8; 200],
// }

#[repr(C)]
pub struct v4l2_format {
    pub r#type: u32,
    pub fmt: [u8; 200],
}
impl Default for v4l2_format {
    fn default() -> Self {
        v4l2_format {
            r#type: 0,
            fmt: [0; 200],
        }
    }
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
        u32
    );

    let mut info_input: u32 = Default::default();

    match unsafe { vidioc_g_input(media_fd, &mut info_input as *mut u32) } {
        Ok(_) => {
            println!("get info g_input [OK]");
        }

        Err(e) => {
            println!("get info g_input [FAILED]: {:?}", e);
        }
    }

    ioctl_readwrite!(
        vidioc_g_fmt,
        VIDIOC_G_FMT_MAGIC,
        VIDIOC_G_FMT_TYPE_MODE,
        v4l2_format
    );

    let mut info_format: v4l2_format = Default::default();

    info_format.r#type = 1;
    match unsafe { vidioc_g_fmt(media_fd, &mut info_format as *mut v4l2_format) } {
        Ok(_) => {
            println!("get info g_fmt [OK]");
            println!("type: {:?}", info_format.r#type);
            // println!("fmt: {:?}", info_format.fmt);
        }

        Err(e) => {
            println!("get info g_fmt [FAILED]: {:?}", e);
        }
    }

    println!("Client exit [OK]");
}
