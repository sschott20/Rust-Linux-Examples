use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("10.0.2.15:12345").unwrap();

    for stream in listener.incoming() {
        println!("Connection established!");

        let _stream = stream.unwrap();
    }
}
