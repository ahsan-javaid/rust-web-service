use crate::config::db::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new() -> User {
        User {
            id: 0,
            name: "".to_string(),
            email: "".to_string(),
            password: "****".to_string(),
        }
    }

    pub fn find_all(cond: String) -> Vec<User> {
        let connection = establish_connection();

        let query = format!("SELECT * FROM Users {}", cond);
        let mut users: Vec<User> = Vec::new();

        connection
            .iterate(query, |pairs| {
                let mut user = User::new();

                for p in pairs {
                    match p.0 {
                        "id" => {
                            let id = p.1.unwrap_or("0");
                            user.id = id.parse::<u32>().unwrap_or(0);
                        }
                        "name" => {
                            let name = p.1.unwrap_or("");
                            user.name = name.to_string();
                        }
                        "email" => {
                            let email = p.1.unwrap_or("");
                            user.email = email.to_string();
                        }
                        _ => {}
                    }
                }
                users.push(user);

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

    pub fn create(user: &mut User) {
        let connection = establish_connection();
        
        let q = format!(
            "INSERT INTO Users (name, email) values ('{}', '{}');
             SELECT * from Users where id = (SELECT MAX(id) AS id FROM Users);
            ",
            &user.name, &user.email
        );
        
        connection.iterate(q, |pairs| {
            let id = pairs[0].1.unwrap_or("");
            user.id = id.parse::<u32>().unwrap_or(0);
           
            true
        }).unwrap();
    }
}
