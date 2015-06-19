use std::io::prelude::*;
use std::io;
use std::net::TcpStream;
use std::net::Shutdown;

use std::env::args;

fn send_message(stream: &mut TcpStream, msg: String) {
    let _ = stream.write(&*msg.into_bytes());
    stream.shutdown(Shutdown::Write);
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9090").unwrap();

    let mut d =args().last().unwrap();

    send_message(&mut stream, d);

    let mut response = String::new();
    let data = stream.read_to_string(&mut response);
    stream.shutdown(Shutdown::Both);
    println!("{}", response);
}
