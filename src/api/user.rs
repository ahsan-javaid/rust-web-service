use crate::types::request::Request;
use crate::types::user::User;
use sqlite;

pub fn get_users(r: Request) {
 let connection = sqlite::open("db.db").unwrap();
 let query = "SELECT * FROM users";

 connection
     .iterate(query, |pairs| {
         for &(name, value) in pairs.iter() {
             println!("{} = {}", name, value.unwrap());
         }
         true
     })
     .unwrap();

  r.handle_write("done".to_string());
}

pub fn create_user(r: Request) {
  let res = String::from("Create user called");
  r.handle_write(res);
}