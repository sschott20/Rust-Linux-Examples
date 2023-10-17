#![allow(unused_imports)]

use opencv::core::{flip, Mat, Vec3b, Vector};
use opencv::videoio::*;
use opencv::{highgui::*, imgcodecs, prelude::*, videoio};

mod utils;
use std::{thread, time::Duration};
use tflitec::interpreter::{Interpreter, Options};
use tflitec::model::Model;
use utils::*;

use std::io::prelude::*;
use std::io::{Read, Write};
use std::net::TcpStream;

struct App {
    cam: videoio::VideoCapture,
}

impl App {
    fn init(&mut self) {
        self.cam
            .set(CAP_PROP_FPS, 30.0)
            .expect("Set camera FPS [FAILED]");
        videoio::VideoCapture::is_opened(&self.cam).expect("Open camera [FAILED]");
    }

    fn read(&mut self) -> Mat {
        let mut frame = Mat::default();
        self.cam
            .read(&mut frame)
            .expect("VideoCapture: read [FAILED]");
        // resize the image as a square, size is
        let mut flipped = Mat::default();
        flip(&frame, &mut flipped, 1).expect("flip [FAILED]");
        let resized_img = resize_with_padding(&flipped, [192, 192]);
        resized_img
    }
}

struct Server {}

impl Server {
    fn send(&mut self, frame: Mat) -> Mat {
        let mut stream = TcpStream::connect("127.0.0.1:54321").unwrap();

        let mut buffer: Vector<u8> = Vec::new().into();
        let _ = opencv::imgcodecs::imencode(".jpg", &frame, &mut buffer, &Vector::new());

        let buffer: Vec<u8> = buffer.to_vec();
        stream.write_all(&buffer).unwrap();
        stream.flush().unwrap();
        stream.shutdown(std::net::Shutdown::Write).unwrap();
        // println!("image sent to server");

        // let mut buffer: Vec<u8> = vec![0; 80000];
        let mut buffer: Vec<u8> = Vec::new();
        stream.read_to_end(&mut buffer).unwrap();
        println!("buffer size: {}", buffer.len());

        let mut flipped = Mat::default();

        opencv::imgcodecs::imdecode_to(
            &opencv::types::VectorOfu8::from_iter(buffer),
            -1,
            &mut flipped,
        )
        .unwrap();
        flipped
    }
}
fn main() {
    let mut server = Server {};

    let mut app = App {
        cam: videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap(),
    };

    app.init();

    loop {
        let mut frame = app.read();
        let mut flipped = server.send(frame);
        imshow("MoveNet", &flipped).expect("imshow [ERROR]");
        // print out response

        // keypress check
        let key = wait_key(1).unwrap();
        if key > 0 && key != 255 {
            break;
        }
    }
}
