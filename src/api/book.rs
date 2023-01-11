use crate::types::context::Context;
use crate::types::message::Message;
use crate::types::book::BookPayload;
use crate::models::book::*;

pub fn get_books(ctx: Context) {
    let condition = String::from("");
    let books = Book::find_all(condition);
    let serialized = serde_json::to_string(&books).unwrap();
    ctx.handle_json(serialized);
}

pub fn create_book(ctx: Context) {
    let payload: BookPayload = serde_json::from_str(&ctx.body).unwrap();
    let book = Book {
        id: 0,
        title: payload.title.clone(),
        author: payload.author.clone()
    };

    Book::create(&book);

    let serialized = serde_json::to_string(&payload).unwrap();
    ctx.handle_json(serialized);
}

pub fn get_book_by_id(ctx: Context) {
    let mut book = Book::new();

    Book::find_by_id(ctx.param, &mut book);

    if book.id == 0 {
        let resp = Message {
            msg: String::from("Book not found")
        };

        let serialized = serde_json::to_string(&resp).unwrap();
        return ctx.status(200).handle_json(serialized);
    }

    let serialized = serde_json::to_string(&book).unwrap();
    ctx.status(200).handle_json(serialized);
}
