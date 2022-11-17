use crate::types::request::Request;

pub fn get_user(r: Request) {
  let res = String::from("Get user called");
  r.handle_write(res);
}

pub fn create_user(r: Request) {
  let res = String::from("Create user called");
  r.handle_write(res);
}