use std::net::{TcpStream, TcpListener};
use std::thread;
mod types;
use crate::types::request::Request;
mod routes;
mod api;
use crate::routes::router::router_handler;

fn handle_client(stream: TcpStream) {
    let request = Request::from(stream);
    router_handler(request);
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
                panic!("Error: {:?}", e);
            }
        }
    }
}