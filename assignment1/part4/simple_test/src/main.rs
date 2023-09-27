#![allow(unused_imports)]
use std::io::prelude::*;

use std::fs::File;
use std::io::SeekFrom;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    println!("Hello, world!");
    let path = "/dev/mdriver";

    let mut f = match File::options().read(true).write(true).open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(f) => f,
    };
    f.seek(SeekFrom::Start(0)).unwrap();
    let val: i64 = 420;
    let val_bytes = val.to_le_bytes();
    f.write(&val_bytes).unwrap();

    f.seek(SeekFrom::Start(0)).unwrap();
    let mut buffer = [0; 8];
    f.read(&mut buffer).unwrap();
    let value: i64 = i64::from_le_bytes(buffer);
    println!("value is {}", value);

    f.seek(SeekFrom::Start(0)).unwrap();
    let val: i64 = 12345;
    let val_bytes = val.to_le_bytes();
    f.write(&val_bytes).unwrap();

    f.seek(SeekFrom::Start(0)).unwrap();
    let mut buffer = [0; 8];
    f.read(&mut buffer).unwrap();
    let value: i64 = i64::from_le_bytes(buffer);
    println!("value is {}", value);

    println!("Simple Test Over");
}
