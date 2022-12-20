use std::io::Read;
use std::io::Write;
use std::net::*;
use std::process;
use std::thread;

pub struct Context {
    pub url: String,
    pub body: String,
    pub param: u32,
    pub method: String, // GET, PUT, POST, DELETE
    pub socket: TcpStream,
}

impl Context {
    pub fn handle_write(mut self, res: String) {
        let result = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>{}</body></html>\r\n", res);
        match self.socket.write(result.as_bytes()) {
            Ok(_) => println!("Response sent"),
            Err(e) => println!("Failed sending response: {}", e),
        }
        // Close connection
        self.socket
            .shutdown(Shutdown::Both)
            .expect("shutdown call failed");
    }

    pub fn handle_json(mut self, res: String) {
        let result = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}\r\n", res.len(),res);
        match self.socket.write_all(result.as_bytes()) {
            Ok(_) => println!("Response sent"),
            Err(e) => println!("Failed sending response: {}", e),
        }
        match self.socket.flush() {
            Ok(_) => println!("Response sent"),
            Err(e) => println!("Failed sending response: {}", e),
        }
        // Close connection
        self.socket
            .shutdown(Shutdown::Both)
            .expect("shutdown call failed");
    }

    pub fn from(mut stream: TcpStream) -> Context {
        let mut buf = [0u8; 4096];
        match stream.read(&mut buf) {
            Ok(_) => {
                let req_str = String::from_utf8_lossy(&buf);
                let req_parts = req_str.split(" ");
                let vec: Vec<&str> = req_parts.collect();
                let body: String;

                // Parsing request body
                if vec[0] == "POST" {
                    let start = req_str.find("{").unwrap();
                    let end = req_str.find("}").unwrap();
                    body = String::from(&req_str[start..end + 1]);
                } else {
                    body = String::from("")
                }

                Context {
                    url: String::from(vec[1]),
                    body: body,
                    param: 0, // Default param
                    method: String::from(vec[0]),
                    socket: stream,
                }
            }
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    pub fn log(&self) {
        println!(
            "pid: {:?} tid:{:?} Request:{} {}",
            process::id(),
            thread::current().id(),
            self.method,
            self.url
        );
    }
}
