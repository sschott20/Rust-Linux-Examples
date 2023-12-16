use opencv::core::{flip, Mat, Vec3b, Vector};
use opencv::videoio::*;
use opencv::{highgui::*, imgcodecs, prelude::*, videoio};

mod utils;
use std::{thread, time::Duration};
use utils::*;

use std::io::prelude::*;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    
    let stream = TcpStream::connect("127.0.0.1:54321").unwrap(),
     
    // loop {
       
    // }
}
