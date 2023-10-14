use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("128.0.0.1:54321")?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
}
