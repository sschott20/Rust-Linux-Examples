use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("10.0.2.15:8080")?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
}
