#![allow(dead_code)]

pub(crate) const  VIDIOC_MAGIC: u8 = b'V';

// #define VIDIOC_QUERYCAP   _IOR('V',  0, struct v4l2_capability)
// 1080579584
pub(crate) const  VIDIOC_QUERYCAP : u32 =  1080579584;

// #define VIDIOC_G_INPUT    _IOR('V', 38, int)
// 1074026022
pub(crate) const  VIDIOC_G_INPUT : u32 = 1074026022;

// #define VIDIOC_ENUMINPUT	_IOWR('V', 26, struct v4l2_input)
// 1079006746
pub(crate) const  VIDIOC_ENUMINPUT: u32  = 1079006746;

// #define VIDIOC_G_FMT		_IOWR('V',  4, struct v4l2_format)
// 1087395332
pub(crate) const  VIDIOC_G_FMT : u32 = 1087395332;

// #define VIDIOC_S_FMT		_IOWR('V',  5, struct v4l2_format)
// 1087395333
pub(crate) const  VIDIOC_S_FMT : u32 = 1087395333;

// #define VIDIOC_REQBUFS          _IOWR('V',  8, struct v4l2_requestbuffers)
// 1075074568
pub(crate) const  VIDIOC_REQBUFS : u32 = 1075074568;

// #define VIDIOC_QUERYBUF _IOWR('V', 9, struct v4l2_buffer)
// 1079531017
pub(crate) const  VIDIOC_QUERYBUF: u32  = 1079531017;

// #define VIDIOC_STREAMON		 _IOW('V', 18, int)
// 1074026002
pub(crate) const  VIDIOC_STREAMON: u32  = 1074026002;

// #define VIDIOC_QBUF _IOWR('V', 15, struct v4l2_buffer)
// 1079531023
pub(crate) const  VIDIOC_QBUF: u32  = 1079531023;

// #define VIDIOC_DQBUF _IOWR('V', 17, struct v4l2_buffer)
// 1079531025
pub(crate) const  VIDIOC_DQBUF: u32  = 1079531025;

// [ 2719.221506] rust_client: sizeof v4l2_capability: 104
// [ 2719.221511] rust_client: sizeof v4l2_format: 208
// [ 2719.221512] rust_client: sizeof v4l2_requestbuffers: 20
// [ 2719.221513] rust_client: sizeof v4l2_buffer: 88
// [ 2719.221513] rust_client: sizeof v4l2_input: 80


// pr_info!(
//     "sizeof v4l2_capability: {}\n",
//     core::mem::size_of::<v4l2_capability>()
// );
// pr_info!(
//     "sizeof v4l2_format: {}\n",
//     core::mem::size_of::<v4l2_format>()
// );
// pr_info!(
//     "sizeof v4l2_requestbuffers: {}\n",
//     core::mem::size_of::<v4l2_requestbuffers>()
// );
// pr_info!(
//     "sizeof v4l2_buffer: {}\n",
//     core::mem::size_of::<v4l2_buffer>()
// );
// pr_info!(
//     "sizeof v4l2_input: {}\n",
//     core::mem::size_of::<v4l2_input>()
// );