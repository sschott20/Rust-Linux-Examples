#![allow(unused_imports)]

use opencv::core::{flip, Mat, Vec3b, Vector};
use opencv::videoio::*;
use opencv::{highgui::*, imgcodecs, prelude::*, videoio};

mod utils;
use tflitec::interpreter::{Interpreter, Options};
use tflitec::model::Model;
use utils::*;

use std::io::prelude::*;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    // load model and create interpreter
    let mut stream = TcpStream::connect("127.0.0.1:54321").expect("Connection failed");


    // open camera
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap(); // 0 is the default camera
    videoio::VideoCapture::is_opened(&cam).expect("Open camera [FAILED]");
    cam.set(CAP_PROP_FPS, 30.0)
        .expect("Set camera FPS [FAILED]");

    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame).expect("VideoCapture: read [FAILED]");

        if frame.size().unwrap().width > 0 {
            let mut buffer: Vector<u8> = Vec::new().into();
            buffer.clear();
            let _ = opencv::imgcodecs::imencode(".jpg", &frame, &mut buffer, &Vector::new());

            // convert buffer into Vec<u8>
            let buffer: Vec<u8> = buffer.to_vec();
            let mut stream = TcpStream::connect("127.0.0.1:54321").unwrap();
            stream.write_all(&buffer).unwrap();

            // imshow("MoveNet", &flipped).expect("imshow [ERROR]");
        }
        // keypress check
        let key = wait_key(1).unwrap();
        if key > 0 && key != 255 {
            break;
        }
    }
}
