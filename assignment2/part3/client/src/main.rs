use nix;
use nix::ioctl_read;
use nix::ioctl_readwrite;

mod bindings;
use bindings::*;

use std::{fs::File, os::unix::prelude::AsRawFd, str};

const VIDIOC_MAGIC: u8 = b'V';

fn main() {
    let file = File::options()
        .write(true)
        .read(true)
        .open("/dev/video0")
        .unwrap();

    let media_fd = file.as_raw_fd();
    println!("camera fd = {}", media_fd);

    // #define VIDIOC_QUERYCAP          _IOR('V',  0, struct v4l2_capability)
    ioctl_read!(vidioc_querycap, VIDIOC_MAGIC, 0, v4l2_capability);

    let mut info_capability: v4l2_capability = unsafe { std::mem::zeroed() };

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

    // #define VIDIOC_G_INPUT           _IOR('V', 38, int)
    ioctl_read!(vidioc_g_input, VIDIOC_MAGIC, 38, u32);

    // #define VIDIOC_ENUMINPUT	_IOWR('V', 26, struct v4l2_input)
    ioctl_readwrite!(vidioc_enuminput, VIDIOC_MAGIC, 26, v4l2_input);

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

    // #define VIDIOC_G_FMT		_IOWR('V',  4, struct v4l2_format)
    ioctl_readwrite!(vidio_g_fmt, VIDIOC_MAGIC, 4, v4l2_format);
    let mut format: v4l2_format = unsafe { std::mem::zeroed() };
    format.type_ = 1;

    match unsafe { vidio_g_fmt(media_fd, &mut format) } {
        Ok(_) => {
            println!("get vidio_g_fmt [OK]");
            println!("Image format:");
            println!("width: {:?}", unsafe { format.fmt.pix.width });
            println!("height: {:?}", unsafe { format.fmt.pix.height });
            println!("pixelformat: {:?}", unsafe { format.fmt.pix.pixelformat });
            println!("field: {:?}", unsafe { format.fmt.pix.field });
            println!("bytesperline: {:?}", unsafe { format.fmt.pix.bytesperline });
            println!("sizeimage: {:?}", unsafe { format.fmt.pix.sizeimage });
            println!("colorspace: {:?}", unsafe { format.fmt.pix.colorspace });
        }
        Err(e) => {
            println!("get vidio_g_fmt [FAILED]: {:?}", e);
        }
    }
    // #define VIDIOC_ENUM_FMT         _IOWR('V',  2, struct v4l2_fmtdesc)
    ioctl_readwrite!(vidio_enum_fmt, VIDIOC_MAGIC, 2, v4l2_fmtdesc);
    let mut fmtdesc: v4l2_fmtdesc = unsafe { std::mem::zeroed() };
    fmtdesc.type_ = 1;
    match unsafe { vidio_enum_fmt(media_fd, &mut fmtdesc) } {
        Ok(_) => {
            println!("get vidio_enum_fmt [OK]");
            println!("description: {:?}", str::from_utf8(&fmtdesc.description));
            println!("pixelformat: {:?}", fmtdesc.pixelformat);
        }
        Err(e) => {
            println!("get vidio_enum_fmt [FAILED]: {:?}", e);
        }
    }

    println!("Client exit [OK]");
}
