use sqlite;
use sqlite::Connection;

pub fn establish_connection() -> Connection {
  sqlite::open("db.db").unwrap()
}