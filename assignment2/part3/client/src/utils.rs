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


use rayon::prelude::*;

/// Copies an input buffer of format YUYV422 to the output buffer
/// in the format of RGB24
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

    let r = (y as f32 + 1.140 * (cr as f32)).round();
    let g = (y as f32 - 0.395 * (cb as f32) - 0.581 * (cr as f32)).round();
    let b : f32 = (y as f32 + 2.032 * (cb as f32)).round();

    (clamp(r), clamp(g), clamp(b))
}

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