use crate::types::request::Request;
use crate::api::user::get_users;
use crate::api::user::*;

use crate::api::book::get_book;
use crate::api::book::create_book;

pub fn router_handler(r: Request) {
  r.log();
  match r.method.as_str() {
    "GET" => get(r),
    "POST" => post(r),
    _ => println!("Ain't special"),
  }
}

fn get(mut r: Request) {
  match r.url.as_str() {
    "/users" => get_users(r),
    "/book" => get_book(r),
    _ => {
      // Parameter handling
      let urls_parts: Vec<&str> = r.url.as_str().split("/").collect();
      r.param = urls_parts[2].parse::<u32>().unwrap();
      match urls_parts[1] {
        "users" => get_user_by_id(r),
        "books" => {},
        _ => {}
      }
    },
  }
}

fn post(r: Request) {
  match r.url.as_str() {
    "/users" => create_user(r),
    "/book" => create_book(r),
    _ => println!("Ain't special"),
  }
}