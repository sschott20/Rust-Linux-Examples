// // #define VIDIOC_ENUM_FMT         _IOWR('V',  2, struct v4l2_fmtdesc)
// ioctl_readwrite!(vidio_enum_fmt, VIDIOC_MAGIC, 2, v4l2_fmtdesc);
// let mut fmtdesc: v4l2_fmtdesc = unsafe { std::mem::zeroed() };
// fmtdesc.type_ = 1;
// loop {
//     match unsafe { vidio_enum_fmt(media_fd, &mut fmtdesc) } {
//         Ok(_) => {
//             println!("get vidio_enum_fmt [OK]");
//             println!("description: {:?}", str::from_utf8(&fmtdesc.description));
//             println!("pixelformat: {:?}", fmtdesc.pixelformat);
//             fmtdesc.index = fmtdesc.index + 1;
//         }
//         Err(e) => {
//             // println!("get vidio_enum_fmt [FAILED]: {:?}", e);
//             break;
//         }
//     }
// }