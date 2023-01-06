use crate::config::db::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new() -> User {
        User {
            id: 0,
            name: "".to_string(),
            email: "".to_string(),
        }
    }

    pub fn find_all(cond: String) -> Vec<User> {
        let connection = establish_connection();

        let query = format!("SELECT * FROM Users {}", cond);
        let mut users: Vec<User> = Vec::new();

        connection
            .iterate(query, |pairs| {
                let (_, id) = pairs.get(0).unwrap();
                let (_, name) = pairs.get(1).unwrap();
                let (_, email) = pairs.get(2).unwrap();

                users.push(User {
                    id: id.unwrap().parse::<u32>().unwrap(),
                    name: String::from(name.unwrap()),
                    email: String::from(email.unwrap()),
                });

                true
            })
            .unwrap();

        users
    }

    pub fn find_by_id(id: u32, user: &mut User) {
        let users = User::find_all(String::from(format!("where id={}", id)));
       
        for b in users.iter() {
          user.id = b.id;
          user.name = b.name.clone();
          user.email = b.email.clone();
        }
    }

    pub fn create(user: &User) {
        let connection = establish_connection();
        let q = format!(
            "INSERT INTO Users (name, email) values ('{}', '{}')",
            &user.name, &user.email
        );
        let _ = connection.execute(q).unwrap();
    }
}
