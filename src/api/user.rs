use crate::types::request::Request;
use crate::types::user::User;
use crate::types::user::UserPayload;
use sqlite;

pub fn get_users(r: Request) {
 let connection = sqlite::open("db.db").unwrap();
 let query = "SELECT * FROM users";
 let mut users :Vec<User> = Vec::new();

 connection.iterate(query, |pairs| {
        let (_, id) = pairs.get(0).unwrap();
        let (_, name) = pairs.get(1).unwrap();
        let (_, email) = pairs.get(2).unwrap();

        users.push(User {
          id: id.unwrap().parse::<u32>().unwrap(),
          name: String::from(name.unwrap()),
          email: String::from(email.unwrap())
        });

        true
    })
    .unwrap();

    let serialized = serde_json::to_string(&users).unwrap();
    r.handle_json(serialized);
}

pub fn create_user(r: Request) {
  let user: UserPayload = serde_json::from_str(&r.body).unwrap();
  let connection = sqlite::open("db.db").unwrap();
  let q = format!("INSERT INTO users (name, email) values ('{}', '{}')", &user.name, &user.email);  
  let _ = connection.execute(q).unwrap();

  let serialized = serde_json::to_string(&user).unwrap();
  r.handle_json(serialized);
}

pub fn get_user_by_id(r: Request) {
  let connection = sqlite::open("db.db").unwrap();
  let query = format!("SELECT * FROM users where id={}", r.param);
  let mut users :Vec<User> = Vec::new();
 
  connection.iterate(query, |pairs| {
         let (_, id) = pairs.get(0).unwrap();
         let (_, name) = pairs.get(1).unwrap();
         let (_, email) = pairs.get(2).unwrap();
 
         users.push(User {
           id: id.unwrap().parse::<u32>().unwrap(),
           name: String::from(name.unwrap()),
           email: String::from(email.unwrap())
         });
 
         true
     })
     .unwrap();
 
     let serialized = serde_json::to_string(&users[0]).unwrap();
     r.handle_json(serialized);
}