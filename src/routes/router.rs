use crate::types::request::Request;
use crate::api::user::get_user;
use crate::api::user::create_user;

use crate::api::book::get_book;
use crate::api::book::create_book;

pub fn router_handler(r: Request) {
  r.log();
  match r.req_type.as_str() {
    "GET" => get(r),
    "POST" => post(r),
    _ => println!("Ain't special"),
  }
}

fn get(r: Request) {
  match r.url.as_str() {
    "/user" => get_user(r),
    "/book" => get_book(r),
    _ => println!("Ain't special"),
  }
}

fn post(r: Request) {
  match r.url.as_str() {
    "/user" => create_user(r),
    "/book" => create_book(r),
    _ => println!("Ain't special"),
  }
}