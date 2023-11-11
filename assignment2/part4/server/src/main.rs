#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use opencv::core::{flip, Mat, Vec3b, Vector};
use opencv::prelude::*;

mod utils;
use std::io::{Read, Write};
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

struct Server {
    stream: TcpStream,
}
impl Server {
    fn recieve(&mut self) -> Mat {
        // let mut buffer: Vec<u8> = vec![0; 110646];
        // println!("recieve");
        let mut buffer: Vec<u8> = vec![0; 462848];
        self.stream.read_exact(&mut buffer).unwrap();

        let mut outbuf: Vec<u8> = vec![0; 462848 * 2];

        let _ = yuv422_to_rgb32(&buffer, &mut outbuf);

        let mut frame: Mat = Mat::default();

        let mut red: Vec<u8> = Vec::new();
        let mut green: Vec<u8> = Vec::new();
        let mut blue: Vec<u8> = Vec::new();

        // iterate through outbuf and set frame values
        for (i, v) in outbuf.iter().enumerate() {
            // println!("v: {}", v);
            if i % 4 == 0 {
                blue.push(*v);
            }
            if i % 4 == 1 {
                green.push(*v);
            }
            if i % 4 == 2 {
                red.push(*v);
            }
        }

        let mut framer: Mat = Mat::from_slice(&red).unwrap();
        framer.set_rows(360);
        framer.set_cols(640);

        let mut frameg: Mat = Mat::from_slice(&green).unwrap();
        frameg.set_rows(360);
        frameg.set_cols(640);

        let mut frameb: Mat = Mat::from_slice(&blue).unwrap();
        frameb.set_rows(360);
        frameb.set_cols(640);

        let mut channels: Vector<Mat> = Vector::new();
        channels.push(framer);
        channels.push(frameg);
        channels.push(frameb);

        let _ = opencv::core::merge(&channels, &mut frame).unwrap();

        // println!("frame: {:?}", frame);

        frame
    }
    fn send(&mut self, frame: Mat) {
        let mut buffer: Vector<u8> = Vec::new().into();
        let _ = opencv::imgcodecs::imencode(".bmp", &frame, &mut buffer, &Vector::new());

        let buffer: Vec<u8> = buffer.to_vec();
        println!("buffer send size: {}", buffer.len());
        self.stream.write_all(&buffer).unwrap();
    }
}
struct App<'a> {
    interpreter: Interpreter<'a>,
}
impl App<'_> {
    fn dnn(&mut self, mut f: Mat) -> Mat {
        let mut frame = resize_with_padding(&f, [192, 192]);
        let vec_2d: Vec<Vec<Vec3b>> = frame.to_vec_2d().unwrap();
        let vec_1d: Vec<u8> = vec_2d
            .iter()
            .flat_map(|v| v.iter().flat_map(|w| w.as_slice()))
            .cloned()
            .collect();

        // set input (tensor0)
        self.interpreter.copy(&vec_1d[..], 0).unwrap();

        // run interpreter
        self.interpreter.invoke().expect("Invoke [FAILED]");

        // get output
        let output_tensor = self.interpreter.output(0).unwrap();
        // draw_keypoints(&mut flipped, output_tensor.data::<f32>(), 0.25);

        draw_keypoints(&mut frame, output_tensor.data::<f32>(), 0.25);
        frame
    }
}

fn main() {
    println!("Server started");
    let path = format!("resource/lite-model_movenet_singlepose_lightning_tflite_int8_4.tflite");
    let model = Model::new(&path).expect("Load model [FAILED]");

    let mut app = App {
        interpreter: Interpreter::new(&model, Some(Options::default()))
            .expect("Create interpreter [FAILED]"),
    };

    app.interpreter
        .allocate_tensors()
        .expect("Allocate tensors [FAILED]");

    let listener = TcpListener::bind("10.0.2.15:23451").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut server = Server { stream: stream };

                loop {
                    let mut frame = server.recieve();

                    let frame = app.dnn(frame);

                    server.send(frame);
                }
                // break;
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}
