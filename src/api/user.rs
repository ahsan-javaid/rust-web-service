use crate::types::context::Context;
use crate::types::user::User;
use crate::types::user::UserPayload;
use sqlite;

pub fn get_users(ctx: Context) {
    let connection = sqlite::open("db.db").unwrap();
    let query = "SELECT * FROM users";
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

    let serialized = serde_json::to_string(&users).unwrap();
    ctx.handle_json(serialized);
}

pub fn create_user(ctx: Context) {
    let user: UserPayload = serde_json::from_str(&ctx.body).unwrap();
    let connection = sqlite::open("db.db").unwrap();
    let q = format!(
        "INSERT INTO users (name, email) values ('{}', '{}')",
        &user.name, &user.email
    );
    let _ = connection.execute(q).unwrap();

    let serialized = serde_json::to_string(&user).unwrap();
    ctx.handle_json(serialized);
}

pub fn get_user_by_id(ctx: Context) {
    let connection = sqlite::open("db.db").unwrap();
    let query = format!("SELECT * FROM users where id={}", ctx.param);
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

    let serialized = serde_json::to_string(&users[0]).unwrap();
    ctx.handle_json(serialized);
}
