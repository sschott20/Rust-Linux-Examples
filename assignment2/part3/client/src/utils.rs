#![allow(dead_code)]
#![allow(unused_imports)]
use opencv::{core::*, imgproc::*, prelude::*};

pub fn resize_with_padding(img: &Mat, new_shape: [i32; 2]) -> Mat {
    let img_shape = [img.cols(), img.rows()];
    let width: i32;
    let height: i32;
    if img_shape[0] as f64 / img_shape[1] as f64 > new_shape[0] as f64 / new_shape[1] as f64 {
        width = new_shape[0];
        height = (new_shape[0] as f64 / img_shape[0] as f64 * img_shape[1] as f64) as i32;
    } else {
        width = (new_shape[1] as f64 / img_shape[1] as f64 * img_shape[0] as f64) as i32;
        height = new_shape[1];
    }

    let mut resized = Mat::default();
    resize(
        img,
        &mut resized,
        Size { width, height },
        0.0,
        0.0,
        INTER_LINEAR,
    )
    .expect("resize_with_padding: resize [FAILED]");

    let delta_w = new_shape[0] - width;
    let delta_h = new_shape[1] - height;
    let (top, bottom) = (delta_h / 2, delta_h - delta_h / 2);
    let (left, right) = (delta_w / 2, delta_w - delta_w / 2);

    let mut rslt = Mat::default();
    copy_make_border(
        &resized,
        &mut rslt,
        top,
        bottom,
        left,
        right,
        BORDER_CONSTANT,
        Scalar::new(0.0, 0.0, 0.0, 0.0),
    )
    .expect("resize_with_padding: copy_make_border [FAILED]");
    rslt
}

pub fn draw_keypoints(img: &mut Mat, keypoints: &[f32], threshold: f32) {
    // keypoints: [1, 17, 3]
    let base: f32;
    let pad_x: i32;
    let pad_y: i32;
    if img.rows() > img.cols() {
        base = img.rows() as f32;
        pad_x = (img.rows() - img.cols()) / 2;
        pad_y = 0;
    } else {
        base = img.cols() as f32;
        pad_x = 0;
        pad_y = (img.cols() - img.rows()) / 2;
    }

    for index in 0..17 {
        let y_ratio = keypoints[index * 3];
        let x_ratio = keypoints[index * 3 + 1];
        let confidence = keypoints[index * 3 + 2];
        if confidence > threshold {
            circle(
                img,
                Point {
                    x: (x_ratio * base) as i32 - pad_x,
                    y: (y_ratio * base) as i32 - pad_y,
                },
                0,
                Scalar::new(0.0, 255.0, 0.0, 0.0),
                5,
                LINE_AA,
                0,
            )
            .expect("Draw circle [FAILED]");
        }
    }
}


#![allow(dead_code)]

//! https://www.kernel.org/doc/html/v4.17/media/uapi/v4l/pixfmt-yuyv.html
//!
//! V4L2_PIX_FMT_YUYV — Packed format with ½ horizontal chroma resolution, also known as YUV 4:2:2
//! Description
//!
//! In this format each four bytes is two pixels. Each four bytes is two Y's, a Cb and a Cr. Each Y goes to one of the pixels, and the Cb and Cr belong to both pixels. As you can see, the Cr and Cb components have half the horizontal resolution of the Y component. V4L2_PIX_FMT_YUYV is known in the Windows environment as YUY2.
//!
//! Example 2.19. V4L2_PIX_FMT_YUYV 4 × 4 pixel image
//!
//! Byte Order. Each cell is one byte.
//! start + 0:	Y'00	Cb00	Y'01	Cr00	Y'02	Cb01	Y'03	Cr01
//! start + 8:	Y'10	Cb10	Y'11	Cr10	Y'12	Cb11	Y'13	Cr11
//! start + 16:	Y'20	Cb20	Y'21	Cr20	Y'22	Cb21	Y'23	Cr21
//! start + 24:	Y'30	Cb30	Y'31	Cr30	Y'32	Cb31	Y'33	Cr31
//!
//! Color Sample Location.
//!     0	 	1	 	2	 	3
//! 0	Y	C	Y	 	Y	C	Y
//! 1	Y	C	Y	 	Y	C	Y
//! 2	Y	C	Y	 	Y	C	Y
//! 3	Y	C	Y	 	Y	C	Y

// use core_simd::f32x4;
use std::simd::f32x4;
use rayon::prelude::*;

/// Copies an input buffer of format YUYV422 to the output buffer
/// in the format of RGB24
#[inline]
pub fn yuv422_to_rgb24(in_buf: &[u8], out_buf: &mut [u8]) {
    debug_assert!(out_buf.len() as f32 == in_buf.len() as f32 * 1.5);

    in_buf
        .par_chunks_exact(4) // FIXME: use par_array_chunks() when stabalized (https://github.com/rayon-rs/rayon/pull/789)
        .zip(out_buf.par_chunks_exact_mut(6))
        .for_each(|(ch, out)| {
            let y1 = ch[0];
            let y2 = ch[2];
            let cb = ch[1];
            let cr = ch[3];

            let (r, g, b) = ycbcr_to_rgb(y1, cb, cr);

            out[0] = r;
            out[1] = g;
            out[2] = b;

            let (r, g, b) = ycbcr_to_rgb(y2, cb, cr);

            out[3] = r;
            out[4] = g;
            out[5] = b;
        });
}

#[inline]
pub fn yuv422_to_rgb32(in_buf: &[u8], out_buf: &mut [u8]) {
    debug_assert!(out_buf.len() == in_buf.len() * 2);

    in_buf
        .par_chunks_exact(4) // FIXME: use par_array_chunks() when stabalized (https://github.com/rayon-rs/rayon/pull/789)
        .zip(out_buf.par_chunks_exact_mut(8))
        .for_each(|(ch, out)| {
            let y1 = ch[0];
            let y2 = ch[2];
            let cb = ch[1];
            let cr = ch[3];

            let (r, g, b) = ycbcr_to_rgb(y1, cb, cr);

            out[0] = b;
            out[1] = g;
            out[2] = r;
            // out[3] = 0;

            let (r, g, b) = ycbcr_to_rgb(y2, cb, cr);

            out[4] = b;
            out[5] = g;
            out[6] = r;
            // out[7] = 0;
        });
}

// COLOR CONVERSION: https://stackoverflow.com/questions/28079010/rgb-to-ycbcr-using-simd-vectors-lose-some-data

#[inline]
fn ycbcr_to_rgb(y: u8, cb: u8, cr: u8) -> (u8, u8, u8) {
    let ycbcr = f32x4::from_array([y as f32, cb as f32 - 128.0f32, cr as f32 - 128.0f32, 0.0]);

    // rec 709: https://mymusing.co/bt-709-yuv-to-rgb-conversion-color/
    let r = (ycbcr * f32x4::from_array([1.0, 0.00000, 1.5748, 0.0])).horizontal_sum();
    let g = (ycbcr * f32x4::from_array([1.0, -0.187324, -0.468124, 0.0])).horizontal_sum();
    let b = (ycbcr * f32x4::from_array([1.0, 1.8556, 0.00000, 0.0])).horizontal_sum();

    (clamp(r), clamp(g), clamp(b))
}

// fn rgb_to_ycbcr((r, g, b): (u8, u8, u8)) -> (u8, u8, u8) {
//     let rgb = F32x4(r as f32, g as f32, b as f32, 1.0);
//     let y = sum(mul(&rgb, F32x4(0.299000, 0.587000, 0.114000, 0.0)));
//     let cb = sum(mul(&rgb, F32x4(-0.168736, -0.331264, 0.500000, 128.0)));
//     let cr = sum(mul(&rgb, F32x4(0.500000, -0.418688, -0.081312, 128.0)));

//     (clamp(y), clamp(cb), clamp(cr))
// }

#[inline]
fn clamp(val: f32) -> u8 {
    if val < 0.0 {
        0
    } else if val > 255.0 {
        255
    } else {
        val.round() as u8
    }
}