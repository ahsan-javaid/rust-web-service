use sqlite;
use sqlite::Connection;
use std::env;

pub fn establish_connection() -> Connection {
  let db_name = env::var("DB").unwrap_or("db.db".to_string());
  println!("ss :{}",db_name);
  sqlite::open(db_name).unwrap()
}