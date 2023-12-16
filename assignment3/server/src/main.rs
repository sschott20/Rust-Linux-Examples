#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use opencv::core::{flip, Mat, Vec3b, Vector};
use opencv::prelude::*;
use rayon::collections::vec_deque;
use rayon::vec;

mod utils;
use std::io::{Read, Write};
use std::process::Output;
use utils::*;

use std::net::TcpListener;
use std::net::TcpStream;
use std::result;

use opencv::prelude::MatTrait;
use std::{fs::File, os::unix::prelude::AsRawFd, str};
use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom},
};
use tflitec::interpreter::{Interpreter, Options};
use tflitec::model::Model;
use utils::*;

fn main() {
    println!("Server started");

    let path: String = format!("resource/upper_model.tflite");
    let upper_model: Model<'_> = Model::new(&path).expect("Load model [FAILED]");

    let path: String = format!("resource/lower_model.tflite");
    let lower_model: Model<'_> = Model::new(&path).expect("Load model [FAILED]");

    let upper_interpreter: Interpreter<'_> =
        Interpreter::new(&upper_model, Some(Options::default())).unwrap();
    let lower_interpreter: Interpreter<'_> =
        Interpreter::new(&lower_model, Some(Options::default())).unwrap();

    upper_interpreter.allocate_tensors().unwrap();
    lower_interpreter.allocate_tensors().unwrap();

    let mut frame = opencv::imgcodecs::imread("resource/pose.jpg", opencv::imgcodecs::IMREAD_COLOR)
        .expect("Can't load image");

    let mut flipped = Mat::default();
    flip(&frame, &mut flipped, 1).expect("error");
    let resized_img = resize_with_padding(&frame, [192, 192]);

    let vec_2d: Vec<Vec<Vec3b>> = resized_img.to_vec_2d().unwrap();
    let vec_1d: Vec<u8> = vec_2d
        .iter()
        .flat_map(|v| v.iter().flat_map(|w| w.as_slice()))
        .cloned()
        .collect();
    // set input (tensor0)
    lower_interpreter.copy(&vec_1d[..], 0).unwrap();
    lower_interpreter.invoke().expect("Invoke [FAILED]");

    let output_tensor = lower_interpreter.output(0).unwrap();

    let vec_1d: Vec<u8> = output_tensor.data::<u8>().to_vec();

    upper_interpreter.copy(&vec_1d, 0).unwrap();
    upper_interpreter.invoke().expect("Invoke [FAILED]");

    let final_output = upper_interpreter.output(0).unwrap();
    draw_keypoints(&mut flipped, final_output.data::<f32>(), 0.25);
    opencv::imgcodecs::imwrite("resource/poseupdate.jpg", &flipped, &Vector::new()).unwrap();
    // println!("final_output: {:?}", final_output.shape());

    let listener = TcpListener::bind("10.0.2.15:23451").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {}
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}
