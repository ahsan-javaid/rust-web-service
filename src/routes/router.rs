use crate::api::user::get_users;
use crate::api::user::*;
use crate::api::book::create_book;
use crate::api::book::*;
use crate::types::context::Context;

pub fn router_handler(r: Context) {
    r.log();
    match r.method.as_str() {
        "GET" => get(r),
        "PUT" => put(r),
        "POST" => post(r),
        _ => println!("Ain't special"),
    }
}

fn get(mut ctx: Context) {
    match ctx.url.as_str() {
        "/" => ctx.handle_write("Server is listening".to_string()),
        "/users" => get_users(ctx),
        "/books" => get_books(ctx),
        _ => {
            // Parameter handling
            let urls_parts: Vec<&str> = ctx.url.as_str().split("/").collect();
            ctx.param = urls_parts[2].parse::<u32>().unwrap();
            match urls_parts[1] {
                "users" => get_user_by_id(ctx),
                "books" => get_book_by_id(ctx),
                _ => {}
            }
        }
    }
}

fn put(mut ctx: Context) {
    // Parameter handling
    let urls_parts: Vec<&str> = ctx.url.as_str().split("/").collect();
    ctx.param = urls_parts[2].parse::<u32>().unwrap();

    match urls_parts[1] {
        "users" => put_user_by_id(ctx),
        "books" => put_book_by_id(ctx),
        _ => {}
    }
}

fn post(ctx: Context) {
    match ctx.url.as_str() {
        "/users" => create_user(ctx),
        "/books" => create_book(ctx),
        _ => println!("Ain't special"),
    }
}
