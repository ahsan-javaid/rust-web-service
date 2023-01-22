use sqlite;
use sqlite::Connection;
use std::env;

pub fn establish_connection() -> Connection {
  let db_name = env::var("DB").unwrap_or("db.db".to_string());
  sqlite::open(db_name).unwrap()
}