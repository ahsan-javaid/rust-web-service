use std::net::{TcpStream, TcpListener};
use std::io::{Read};
use std::thread;
mod types;
use crate::types::request::Request;
mod routes;
mod api;
use crate::routes::router::router_handler;

fn process_url(url: String, stream: TcpStream) {
    let req_parts = url.split(" "); 
    let vec: Vec<&str> = req_parts.collect();
    let req = Request {
        url: String::from(vec[1]),
        req_type: String::from(vec[0]),
        socket: stream
    };
    router_handler(req);
}

fn handle_read(mut stream: TcpStream) {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            process_url(req_str.to_string(), stream);
            println!("Reading stream: {}", req_str);
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(stream);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening for connections on port {}", 8080);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}