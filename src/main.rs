use std::net::{Shutdown, TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;
mod types;
use crate::types::request::Request;

fn process_url(url: String) {
    let req_parts = url.split(" "); 
    let vec: Vec<&str> = req_parts.collect();
    let req = Request {
        url: String::from(vec[1]),
        req_type: String::from(vec[0]),
    };
    println!("type: {}",req.req_type);
    println!("url: {}", req.url);
}

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            process_url(req_str.to_string());
            println!("Reading stream: {}", req_str);
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    match stream.write(response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }

    // Close connection
    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
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