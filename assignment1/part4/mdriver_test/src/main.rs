#![allow(unused_imports)]
use std::io::prelude::*;

use std::fs::File;
use std::io::SeekFrom;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    // println!("Hello, world!");
    let path = "/dev/mdriver";

    let mut file = match File::options().read(true).write(true).open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file) => file,
    };

    let bytes_list = [1, 64, 1000, 64000, 512000];

    for bytes in &bytes_list {
        println!("Writing {} bytes to the device file", bytes);
        for _ in 1..10 {
            let _ = file.seek(SeekFrom::Start(0));

            
            let data_to_write = vec![0u8; *bytes];

            let start = std::time::Instant::now();

            let _bytes_written = file
                .write(&data_to_write)
                .expect("Failed to write to the device file");
            let end = std::time::Instant::now();

            let elapsed_seconds = end - start;
            let elapsed_seconds = elapsed_seconds.as_secs_f64();

            println!("{:.06}", elapsed_seconds)
        }

        println!("Reading {} bytes from the device file", bytes);
        for _ in 1..11 {
            let _ = file.seek(SeekFrom::Start(0));

            let mut data_buff = vec![0u8; *bytes];
            let  newf = file.try_clone().unwrap();
            let mut newf = newf.take(*bytes as u64);

            let start = std::time::Instant::now();
            let _bytes_written = newf
                .read_to_end(&mut data_buff)
                .expect("Failed to write to the device file");
            let end = std::time::Instant::now();

            let elapsed_seconds = end - start;
            let elapsed_seconds = elapsed_seconds.as_secs_f64();

            println!("{:.06}", elapsed_seconds)
        }
    }
}
