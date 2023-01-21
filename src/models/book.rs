use crate::config::db::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
}

impl Book {

    pub fn new() -> Book {
        Book {
            id: 0,
            title: "".to_string(),
            author: "".to_string()
        }
    }

    pub fn find_all(cond: String) -> Vec<Book> {
        let connection = establish_connection();

        let query = format!("SELECT * FROM books {}", cond);
        let mut books: Vec<Book> = Vec::new();

        connection
            .iterate(query, |pairs| {
                
                let mut book = Book::new();

                for p in pairs {
                    match p.0 {
                        "id" => {
                            let id = p.1.unwrap_or("0");
                            book.id = id.parse::<u32>().unwrap_or(0);
                        },
                        "title" => {
                            let title = p.1.unwrap_or("");
                            book.title = title.to_string();
                        },
                        "author" => {
                            let author = p.1.unwrap_or("");
                            book.author = author.to_string();
                        },
                        _ => {}
                    }
                }

                books.push(book);

                true
            })
            .unwrap();

        books
    }

    pub fn find_by_id(id: u32, book: &mut Book) {
        let books = Book::find_all(String::from(format!("where id={}", id)));
        
        for b in books.iter() {
            book.id = b.id;
            book.title = b.title.clone();
            book.author = b.author.clone();
        }
    }

    pub fn create(book: &mut Book) {
        let connection = establish_connection();
        
        let q = format!(
            "INSERT INTO books (title, author) values ('{}', '{}');
             SELECT * from books where id = (SELECT MAX(id) AS id FROM books);
            ",
            &book.title, &book.author
        );
        
        connection.iterate(q, |pairs| {
            let id = pairs[0].1.unwrap_or("");
            book.id = id.parse::<u32>().unwrap_or(0);
           
            true
        }).unwrap();
    }
}
