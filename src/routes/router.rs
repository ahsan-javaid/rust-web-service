use crate::types::context::Context;
use crate::api::user::get_users;
use crate::api::user::*;

use crate::api::book::get_book;
use crate::api::book::create_book;

pub fn router_handler(r: Context) {
  r.log();
  match r.method.as_str() {
    "GET" => get(r),
    "POST" => post(r),
    _ => println!("Ain't special"),
  }
}

fn get(mut ctx: Context) {
  match ctx.url.as_str() {
    "/users" => get_users(ctx),
    "/book" => get_book(ctx),
    _ => {
      // Parameter handling
      let urls_parts: Vec<&str> = ctx.url.as_str().split("/").collect();
      ctx.param = urls_parts[2].parse::<u32>().unwrap();
      match urls_parts[1] {
        "users" => get_user_by_id(ctx),
        "books" => {},
        _ => {}
      }
    },
  }
}

fn post(ctx: Context) {
  match ctx.url.as_str() {
    "/users" => create_user(ctx),
    "/book" => create_book(ctx),
    _ => println!("Ain't special"),
  }
}