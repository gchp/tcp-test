#![feature(collections)]

use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;

use std::io::Read;
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    let mut data = Vec::new();
    stream.read_to_end(&mut data);

    let mut return_data = Vec::new();

    let raw = String::from("SERVER GOT: ").into_bytes();
    return_data.push_all(&*raw);
    return_data.push_all(&*data);

    stream.write(&*return_data);
}

fn main() {
    println!("Listening for TCP connections on port 9090");
    let listener = TcpListener::bind("127.0.0.1:9090").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {}
        }
    }
}
