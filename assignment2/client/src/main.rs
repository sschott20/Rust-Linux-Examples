use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    println!("Client started");

    // let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let mut stream = TcpStream::connect("127.0.0.1:54321")?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    println!("Client finished");

    Ok(())
}
