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
    match serde_json::from_str::<BookPayload>(&ctx.body) {
        Ok(payload) => {
            let mut book = Book {
                id: 0,
                title: payload.title.clone(),
                author: payload.author.clone()
            };
        
            Book::create(&mut book);
        
            let serialized = serde_json::to_string(&book).unwrap();
            ctx.handle_json(serialized);
        },
        Err(e) => {
            match e.to_string().find("title") {
                Some(_) => {
                    let resp = Message {
                        msg: String::from("title field is required")
                    };
                    let serialized = serde_json::to_string(&resp).unwrap();
                    return ctx.status(400).handle_json(serialized);                },
                None => {}
            }

            match e.to_string().find("author") {
                Some(_) => {
                    let resp = Message {
                        msg: String::from("author field is required")
                    };
                    let serialized = serde_json::to_string(&resp).unwrap();
                    return ctx.status(400).handle_json(serialized);                  },
                None => {}
            }
        }
    }
}

pub fn get_book_by_id(ctx: Context) {
    let mut book = Book::new();

    Book::find_by_id(ctx.param, &mut book);

    if book.id == 0 {
        let resp = Message {
            msg: String::from("Book not found")
        };

        let serialized = serde_json::to_string(&resp).unwrap();
        return ctx.status(404).handle_json(serialized);
    }

    let serialized = serde_json::to_string(&book).unwrap();
    ctx.status(200).handle_json(serialized);
}

pub fn put_book_by_id(ctx: Context) {
    match serde_json::from_str::<BookPayload>(&ctx.body) {
        Ok(payload) => {
            let mut book = Book {
                id: ctx.param,
                title: payload.title.clone(),
                author: payload.title.clone()
            };
        
            Book::update(&mut book);
        
            let serialized = serde_json::to_string(&book).unwrap();
            ctx.handle_json(serialized);
        },
        Err(e) => {
            println!("error {:?}",e);
            match e.to_string().find("title") {
                Some(_) => {
                    let resp = Message {
                        msg: String::from("title field is required")
                    };
                    let serialized = serde_json::to_string(&resp).unwrap();
                    return ctx.status(400).handle_json(serialized);                },
                None => {}
            }

            match e.to_string().find("author") {
                Some(_) => {
                    let resp = Message {
                        msg: String::from("author field is required")
                    };
                    let serialized = serde_json::to_string(&resp).unwrap();
                    return ctx.status(400).handle_json(serialized);                  },
                None => {}
            }
        }
    }

}
