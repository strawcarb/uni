use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("New connection from {:?}", stream.peer_addr().unwrap());

        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).unwrap();
        println!("Received {} bytes", n);
        stream.write(&buffer[..n]).unwrap();
    }
}
