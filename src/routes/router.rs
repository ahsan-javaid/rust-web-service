use crate::types::request::Request;
use crate::api::user::get_user;

pub fn router_handler(r: Request) {
  match r.req_type.as_str() {
    "GET" => get(r),
    _ => println!("Ain't special"),
  }
}

fn get(r: Request) {
  match r.url.as_str() {
    "/user" => get_user(),
    _ => println!("Ain't special"),
  }
}