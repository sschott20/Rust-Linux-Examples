use opencv::core::{flip, Vec3b};
use opencv::videoio::*;
use opencv::{highgui::*, prelude::*, videoio};

mod utils;
use std::io::prelude::*;
use std::net::TcpStream;
use tflitec::interpreter::{Interpreter, Options};
use tflitec::model::Model;
use utils::*;

fn main() -> std::io::Result<()> {
    println!("Client started");

    // let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let mut stream = TcpStream::connect("127.0.0.1:54321")?;

    // 0 is the default camera
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();
    videoio::VideoCapture::is_opened(&cam).expect("Open camera [FAILED]");
    cam.set(CAP_PROP_FPS, 30.0)
        .expect("Set camera FPS [FAILED]");

    loop {
        let mut frame = Mat::default();
        cam.read(&mut frame).expect("VideoCapture: read [FAILED]");

        if frame.size().unwrap().width > 0 {
            // flip the image horizontally
            let mut flipped = Mat::default();
            flip(&frame, &mut flipped, 1).expect("flip [FAILED]");
            // resize the image as a square, size is
            let resized_img = resize_with_padding(&flipped, [192, 192]);

            // turn Mat into Vec<u8>
            let vec_2d: Vec<Vec<Vec3b>> = resized_img.to_vec_2d().unwrap();
            let vec_1d: Vec<u8> = vec_2d
                .iter()
                .flat_map(|v| v.iter().flat_map(|w| w.as_slice()))
                .cloned()
                .collect();
            println!("vec_1d.len(): {}", vec_1d.len());
        }
    }

    println!("Client exit");

    Ok(())
}
