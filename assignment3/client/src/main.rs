#![allow(unused_imports)]

use opencv::core::{flip, Mat, Vec3b, Vector};
use opencv::videoio::*;
use opencv::{highgui::*, imgcodecs, prelude::*, videoio};

mod utils;
use std::vec;
use std::{thread, time::Duration};
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

struct Server {
    stream: TcpStream,
}

impl Server {
    fn send(&mut self, vec_1d: Vec<u8>) -> Mat {
        // let mut stream = TcpStream::connect("127.0.0.1:54321").unwrap();
        let mut buffer: Vector<u8> = Vec::new().into();

        // let _ = opencv::imgcodecs::imencode(".bmp", &frame, &mut buffer, &Vector::new());

        // let buffer: Vec<u8> = buffer.to_vec();
        // self.stream.rewind().unwrap();
        self.stream.write_all(&vec_1d).unwrap();
        // println!("buffer send size: {}", buffer.len());

        let mut buffer: Vec<u8> = vec![0; 204];
        self.stream.read_exact(&mut buffer).unwrap();
        // println!("buffer recieve size: {}", buffer.len());

        let mut flipped = Mat::default();

        opencv::imgcodecs::imdecode_to(
            &opencv::types::VectorOfu8::from_iter(buffer),
            -1,
            &mut flipped,
        )
        .unwrap();
        let flipped = resize_with_padding(&flipped, [196 * 2, 196 * 2]);
        flipped
    }
}
fn main() {
    let mut server = Server {
        stream: TcpStream::connect("127.0.0.1:54321").unwrap(),
    };

    let mut app = App {
        cam: videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap(),
    };

    let path: String = format!("resource/lower_model.tflite");
    let lower_model: Model<'_> = Model::new(&path).expect("Load model [FAILED]");
    let lower_interpreter: Interpreter<'_> =
        Interpreter::new(&lower_model, Some(Options::default())).unwrap();
    lower_interpreter.allocate_tensors().unwrap();

    app.init();

    loop {
        let mut frame = app.read();

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
        let mut vec_1d: Vec<u8> = server.send(vec_1d);
        let f32_vec: Vec<f32> = vec_1d.iter().map(|&x| x as f32).collect();
        draw_keypoints(&mut flipped, vec_1d, 0.25);
        imshow("MoveNet", &flipped).expect("imshow [ERROR]");

        let key = wait_key(1).unwrap();
        if key > 0 && key != 255 {
            break;
        }
    }
}
