use crate::types::request::Request;

pub fn get_book(r: Request) {
  let res = String::from("Get book called");
  r.handle_write(res);
}

pub fn create_book(r: Request) {
  let res = String::from("Create book called");
  r.handle_write(res);
}