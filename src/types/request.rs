use std::net::*;
use std::io::{Write};
use std::io::{Read};
use std::thread;
use std::process;

pub struct Request {
  pub url: String,
  pub body: String,
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

  pub fn handle_json(mut self, res: String) {
    println!("let me check {}", res);
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
     self.socket.shutdown(Shutdown::Both).expect("shutdown call failed");
   }

  pub fn from(mut stream: TcpStream) -> Request {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            let req_parts = req_str.split(" "); 
            let vec: Vec<&str> = req_parts.collect();
            let body: String;
            if vec[0] == "POST" {
              let start = req_str.find("{").unwrap();
              let end = req_str.find("}").unwrap();
              body = String::from(&req_str[start..end+1]);
            } else {
              body = String::from("")
            }
            Request {
                url: String::from(vec[1]),
                body: body,
                req_type: String::from(vec[0]),
                socket: stream
            }
            },
        Err(e) => panic!("Error: {:?}", e),
    }
  }
 
  pub fn log(&self) {
    println!("pid: {:?} tid:{:?} Request:{} {}", process::id() ,thread::current().id() ,self.req_type ,self.url);
  }
}
