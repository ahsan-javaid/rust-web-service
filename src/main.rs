use std::net::{TcpListener, TcpStream};
use std::thread;
use sqlite;

mod types;
mod config;
mod routes;
mod models;
mod api;
mod utils;

use crate::types::context::Context;
use crate::routes::router::router_handler;
use crate::config::env::*;

fn handle_client(stream: TcpStream) {
    let request = Context::from(stream);
    router_handler(request);
}

fn main() {
    let config: Config = load_env();
    let address = format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind(address).unwrap(); // Todo: get rid of unwrap
    println!("Listening for connections on port {}", config.port);

    // DB PART
    let connection = sqlite::open(config.db).unwrap();

    let query = "
        CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, email TEXT, password TEXT);
        CREATE TABLE IF NOT EXISTS books (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, author TEXT);
    ";

    match connection.execute(query) {
        Ok(_) => {
            println!("Migrations executed");
        },
        Err(e) => {
            panic!("DB error: {:?}", e);
        }
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    }
}
