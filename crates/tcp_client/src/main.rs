use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut buffer = [0; 1024];
    stream.write("Hello, world!".as_bytes()).unwrap();
    stream.read(&mut buffer).unwrap();
    println!("Response from server: {}", str::from_utf8(&buffer).unwrap());
}
