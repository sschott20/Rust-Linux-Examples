use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("10.0.2.15:69420").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
