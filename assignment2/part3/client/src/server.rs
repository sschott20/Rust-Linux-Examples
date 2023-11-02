#![allow(unused_imports)]

use opencv::core::{flip, Mat, Vec3b, Vector};
use opencv::{highgui::*, imgcodecs, prelude::*, videoio};

use crate::utils::*;
use std::io::prelude::*;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::{thread, time::Duration};

pub struct Server {
    pub stream: TcpStream,
}

impl Server {
    pub fn send(&mut self, img_buffer: &[u8]) -> Mat {
        // let mut stream = TcpStream::connect("127.0.0.1:54321").unwrap();
        // let mut buffer: Vector<u8> = Vec::new().into();

        // let _ = opencv::imgcodecs::imencode(".bmp", &frame, &mut buffer, &Vector::new());

        // let buffer: Vec<u8> = buffer.to_vec();
        // self.stream.rewind().unwrap();

        self.stream.write_all(&img_buffer).unwrap();
        println!("buffer send size: {}", img_buffer.len());

        // let mut buffer: Vec<u8> = vec![0; 110646];
        // self.stream.read_exact(&mut buffer).unwrap();
        // // println!("buffer recieve size: {}", buffer.len());

        let mut flipped = Mat::default();
        flipped
        // opencv::imgcodecs::imdecode_to(
        //     &opencv::types::VectorOfu8::from_iter(buffer),
        //     -1,
        //     &mut flipped,
        // )
        // .unwrap();
        // let flipped = resize_with_padding(&flipped, [196 * 2, 196 * 2]);
        // flipped
    }
}
