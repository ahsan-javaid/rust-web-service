use crate::types::context::Context;
use crate::models::book::*;

pub fn get_books(ctx: Context) {
    let books = Book::find_all();
    let serialized = serde_json::to_string(&books).unwrap();
    ctx.handle_json(serialized);
}

pub fn create_book(ctx: Context) {
    let res = String::from("Create book called");
    ctx.handle_write(res);
}
