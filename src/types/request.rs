use std::net::*;
use std::io::{Write};

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
}
