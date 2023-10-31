use nix::{ioctl_read, ioctl_readwrite};
// use std::mem::size_of;
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

// #define VIDIOC_ENUM_FMT         _IOWR('V',  2, struct v4l2_fmtdesc)
const VIDIOC_ENUM_FMT_MAGIC: u8 = 'V' as u8;
const VIDIOC_ENUM_FMT_TYPE_MODE: u8 = 2;

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
pub struct v4l2_fmtdesc {
    pub index: u32,
    pub r#type: u32,
    pub flags: u32,
    pub description: [u8; 32],
    pub pixelformat: u32,
    pub reserved: [u32; 4],
}

#[repr(C)]
#[derive(Default)]
pub struct v4l2_pix_format {
    pub width: u32,
    pub height: u32,
    pub pixelformat: u32,
    pub field: u32,
    pub bytesperline: u32,
    pub sizeimage: u32,
    pub colorspace: u32,
    pub priv_: u32,
}

#[repr(C)]
pub union v4l2_format_fmt {
    pub pix: std::mem::ManuallyDrop<v4l2_pix_format>,
    pub raw_data: [u8; 200],
}
impl Default for v4l2_format_fmt {
    fn default() -> Self {
        {
            v4l2_format_fmt { raw_data: [0; 200] }
        }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct v4l2_format {
    pub r#type: u32,
    pub fmt: v4l2_format_fmt,
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

    let mut info_input: u32 = Default::default();

    match unsafe { vidioc_g_input(media_fd, &mut info_input) } {
        Ok(_) => {
            println!("get info g_input [OK]");
        }

        Err(e) => {
            println!("get info g_input [FAILED]: {:?}", e);
        }
    }

    // ioctl_readwrite!(
    //     vidioc_g_fmt,
    //     VIDIOC_G_FMT_MAGIC,
    //     VIDIOC_G_FMT_TYPE_MODE,
    //     v4l2_format
    // );

    // let mut info_format: v4l2_format = Default::default();

    // info_format.r#type = 1;
    // match unsafe { vidioc_g_fmt(media_fd, &mut info_format) } {
    //     Ok(_) => {
    //         println!("get info g_fmt [OK]");
    //         println!("type: {:?}", info_format.r#type);
    //         // println!("fmt: {:?}", info_format.fmt);
    //     }

    //     Err(e) => {
    //         println!("get info g_fmt [FAILED]: {:?}", e);
    //     }
    // }

    ioctl_readwrite!(
        vidioc_enum_fmt,
        VIDIOC_ENUM_FMT_MAGIC,
        VIDIOC_ENUM_FMT_TYPE_MODE,
        v4l2_fmtdesc
    );
    let mut info_fmtdesc: v4l2_fmtdesc = Default::default();
    loop {
        println!("index: {:?}", info_fmtdesc.index);
        match unsafe { vidioc_enum_fmt(media_fd, &mut info_fmtdesc) } {
            Ok(_) => {
                println!("get info enum_fmt [OK]");
                println!("index: {:?}", info_fmtdesc.index);
                println!("type: {:?}", info_fmtdesc.r#type);
                println!("flags: {:?}", info_fmtdesc.flags);
                println!(
                    "description: {:?}",
                    str::from_utf8(&info_fmtdesc.description)
                );
                println!("pixelformat: {:?}", info_fmtdesc.pixelformat);
            }

            Err(e) => {
                println!("get info enum_fmt [FAILED]: {:?}", e);
            }
        }
        info_fmtdesc.index = info_fmtdesc.index + 1;
    }

    println!("Client exit [OK]");
}
