use std::net::{TcpStream, TcpListener};
use std::thread;
mod types;
use crate::types::request::Request;
mod routes;
mod config;
use crate::config::env::*;
mod api;
use crate::routes::router::router_handler;
use sqlite;
use sqlite::Connection;

fn handle_client(stream: TcpStream) {
    let request = Request::from(stream);
    router_handler(request);
}

fn main() {
    let config: Config = load_env();
    let address = format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind(address).unwrap(); // Todo: get rid of unwrap
    println!("Listening for connections on port {}", config.port);

    // DB PART
    let connection = sqlite::open("db.db").unwrap();

    let query = "
        CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, email TEXT);
        INSERT INTO users VALUES (1, 'Alice', 'alice@gmail.com');
        INSERT INTO users VALUES (2, 'Bob', 'bob@gmail.com');
    ";
    connection.execute(query).unwrap();

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