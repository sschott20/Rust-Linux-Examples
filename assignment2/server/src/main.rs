use opencv::core::{flip, Mat, Vec3b, Vector};
use opencv::prelude::*;

mod utils;
use std::io::{Read, Write};

use std::net::TcpListener;
use tflitec::interpreter::{Interpreter, Options};
use tflitec::model::Model;
use utils::*;

fn main() {
    println!("Server started");
    let options = Options::default();
    let path = format!("resource/lite-model_movenet_singlepose_lightning_tflite_int8_4.tflite");
    let model = Model::new(&path).expect("Load model [FAILED]");
    let interpreter = Interpreter::new(&model, Some(options)).expect("Create interpreter [FAILED]");
    interpreter
        .allocate_tensors()
        .expect("Allocate tensors [FAILED]");

    let listener = TcpListener::bind("10.0.2.15:23451").unwrap();

    for stream in listener.incoming() {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    // let mut buffer: Vec<u8> = vec![0; 40000];
                    // let bytes_read = stream.read_exact(&mut buffer).unwrap();

                    let mut buffer: Vec<u8> = Vec::new();
                    let bytes_read = stream.read_to_end(&mut buffer).unwrap();
                    println!("buffer size: {:?}", bytes_read);

                    let mut frame = Mat::default();
                    opencv::imgcodecs::imdecode_to(
                        &opencv::types::VectorOfu8::from_iter(buffer),
                        -1,
                        &mut frame,
                    )
                    .unwrap();

                    // let mut flipped = Mat::default();
                    // flip(&frame, &mut flipped, 1).expect("flip [FAILED]");

                    // resize the image as a square, size is
                    // let mut resized_img = resize_with_padding(&flipped, [192, 192]);

                    // turn Mat into Vec<u8>
                    let vec_2d: Vec<Vec<Vec3b>> = frame.to_vec_2d().unwrap();
                    let vec_1d: Vec<u8> = vec_2d
                        .iter()
                        .flat_map(|v| v.iter().flat_map(|w| w.as_slice()))
                        .cloned()
                        .collect();

                    // set input (tensor0)
                    interpreter.copy(&vec_1d[..], 0).unwrap();

                    // run interpreter
                    interpreter.invoke().expect("Invoke [FAILED]");

                    // get output
                    let output_tensor = interpreter.output(0).unwrap();
                    // draw_keypoints(&mut flipped, output_tensor.data::<f32>(), 0.25);
                    draw_keypoints(&mut frame, output_tensor.data::<f32>(), 0.25);
                    let mut buffer: Vector<u8> = Vec::new().into();
                    let _ = opencv::imgcodecs::imencode(
                        ".jpg",
                        &frame,
                        &mut buffer,
                        &Vector::new(),
                    );
                    // let _ =
                    //     opencv::imgcodecs::imencode(".jpg", &flipped, &mut buffer, &Vector::new());

                    let buffer: Vec<u8> = buffer.to_vec();
                    stream.write_all(&buffer).unwrap();
                }
                Err(e) => {
                    println!("Error accepting connection: {}", e);
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }
}
