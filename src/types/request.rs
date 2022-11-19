use std::net::*;
use std::io::{Write};
use std::io::{Read};

pub struct Request {
  pub url: String,
  pub req_type: String, // GET, PUT, POST, DELETE
  pub socket: TcpStream
}

impl Request {
  pub fn handle_write(mut self, res: String) {
    let result = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>{}</body></html>\r\n", res);
    match self.socket.write(result.as_bytes()) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
    // Close connection
    self.socket.shutdown(Shutdown::Both).expect("shutdown call failed");
  }

  pub fn from(mut stream: TcpStream) -> Request {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            let req_parts = req_str.split(" "); 
            let vec: Vec<&str> = req_parts.collect();
            Request {
                url: String::from(vec[1]),
                req_type: String::from(vec[0]),
                socket: stream
            }
            },
        Err(e) => panic!("Error: {:?}", e),
    }
  }
}
