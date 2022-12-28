use crate::config::db::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
}

impl Book {
    pub fn find_all(cond: String) -> Vec<Book> {
        let connection = establish_connection();

        let query = format!("SELECT * FROM books {}", cond);
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

    pub fn find_by_id(id: u32, book: &mut Book) {
        let books = Book::find_all(String::from(format!("where id={}", id)));
        // Todo: add len check
        book.id = books[0].id;
        book.title = books[0].title.clone();
        book.author = books[0].author.clone();
    }

    pub fn create(book: &Book) {
        let connection = establish_connection();
        let q = format!(
            "INSERT INTO books (title, author) values ('{}', '{}')",
            &book.title, &book.author
        );
        let _ = connection.execute(q).unwrap();
    }
}
