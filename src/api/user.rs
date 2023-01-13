use crate::types::context::Context;
use crate::types::user::UserPayload;
use crate::types::message::Message;
use crate::models::user::*;

pub fn get_users(ctx: Context) {
    let condition = String::from("");
    let users = User::find_all(condition);
    let serialized = serde_json::to_string(&users).unwrap();
    ctx.handle_json(serialized);
}

pub fn create_user(ctx: Context) {
    let payload: UserPayload = serde_json::from_str(&ctx.body).unwrap();
    let user = User {
        id: 0,
        name: payload.name.clone(),
        email: payload.email.clone()
    };

    User::create(&user);

    let serialized = serde_json::to_string(&payload).unwrap();
    ctx.handle_json(serialized);
}

pub fn get_user_by_id(ctx: Context) {
    let mut user = User::new();

    User::find_by_id(ctx.param, &mut user);
    
    if user.id == 0 {
        let resp = Message {
            msg: String::from("User not found")
        };

        let serialized = serde_json::to_string(&resp).unwrap();
        return ctx.status(404).handle_json(serialized);
    }   
    
    let serialized = serde_json::to_string(&user).unwrap();
    ctx.handle_json(serialized);
}
