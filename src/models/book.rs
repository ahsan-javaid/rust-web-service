use crate::config::db::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
}

impl Book {
    pub fn find_all() -> Vec<Book> {
        let connection = establish_connection();

        let query = "SELECT * FROM books";
        let mut books: Vec<Book> = Vec::new();

        connection
            .iterate(query, |pairs| {
                let (_, id) = pairs.get(0).unwrap();
                let (_, title) = pairs.get(1).unwrap();
                let (_, author) = pairs.get(2).unwrap();

                books.push(Book {
                    id: id.unwrap().parse::<u32>().unwrap(),
                    title: String::from(title.unwrap()),
                    author: String::from(author.unwrap()),
                });

                true
            })
            .unwrap();

        books
    }
}
