use opencv::core::{flip, Mat, Vec3b, Vector};
// use opencv::prelude::*;

mod utils;
use std::io::{Read, Write};
use utils::*;

use std::net::TcpListener;
use std::net::TcpStream;
use std::result;

// use tflitec::interpreter::{Interpreter, Options};
// use tflitec::model::Model;
// use utils::*;

struct Server {
    stream: TcpStream,
}
impl Server {
    fn recieve(&mut self) {
        // let mut buffer: Vec<u8> = vec![0; 110646];
        let mut buffer: Vec<u8> = vec![0; 460800];
        self.stream.read_exact(&mut buffer).unwrap();

        let mut outbuf = [0; 462848 * 2];

        let _ = yuv422_to_rgb32(&buffer, &mut outbuf);

        let mut frame = Mat::from_slice(&outbuf).unwrap();
        let _ = opencv::imgcodecs::imwrite("test.jpg", &frame, &Vector::new());
    }
}
// struct App<'a> {
//     interpreter: Interpreter<'a>,
// }
// impl App<'_> {
//     fn dnn(&mut self, mut frame: Mat) -> Mat {
//         let vec_2d: Vec<Vec<Vec3b>> = frame.to_vec_2d().unwrap();
//         let vec_1d: Vec<u8> = vec_2d
//             .iter()
//             .flat_map(|v| v.iter().flat_map(|w| w.as_slice()))
//             .cloned()
//             .collect();

//         // set input (tensor0)
//         self.interpreter.copy(&vec_1d[..], 0).unwrap();

//         // run interpreter
//         self.interpreter.invoke().expect("Invoke [FAILED]");

//         // get output
//         let output_tensor = self.interpreter.output(0).unwrap();
//         // draw_keypoints(&mut flipped, output_tensor.data::<f32>(), 0.25);

//         draw_keypoints(&mut frame, output_tensor.data::<f32>(), 0.25);
//         frame
//     }
// }

fn main() {
    println!("Server started");
    //     let path = format!("resource/lite-model_movenet_singlepose_lightning_tflite_int8_4.tflite");
    //     let model = Model::new(&path).expect("Load model [FAILED]");

    //     let mut app = App {
    //         interpreter: Interpreter::new(&model, Some(Options::default()))
    //             .expect("Create interpreter [FAILED]"),
    //     };

    //     app.interpreter
    //         .allocate_tensors()
    //         .expect("Allocate tensors [FAILED]");

    let listener = TcpListener::bind("10.0.2.15:23451").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut server = Server { stream: stream };

                loop {
                    let _ = server.recieve();
                    // let mut frame = app.dnn(frame);
                    // server.send(frame);
                }
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}
