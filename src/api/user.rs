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
    match serde_json::from_str::<UserPayload>(&ctx.body) {
        Ok(payload) => {
            let mut user = User {
                id: 0,
                name: payload.name.clone(),
                email: payload.email.clone(),
                password: payload.password.clone()
            };
        
            User::create(&mut user);
        
            let serialized = serde_json::to_string(&user).unwrap();
            ctx.handle_json(serialized);
        },
        Err(e) => {
            match e.to_string().find("name") {
                Some(_) => {
                    let resp = Message {
                        msg: String::from("name field is required")
                    };
                    let serialized = serde_json::to_string(&resp).unwrap();
                    return ctx.status(400).handle_json(serialized);                },
                None => {}
            }

            match e.to_string().find("email") {
                Some(_) => {
                    let resp = Message {
                        msg: String::from("email field is required")
                    };
                    let serialized = serde_json::to_string(&resp).unwrap();
                    return ctx.status(400).handle_json(serialized);                  },
                None => {}
            }

            match e.to_string().find("password") {
                Some(_) => {
                    let resp = Message {
                        msg: String::from("password field is required")
                    };
                    let serialized = serde_json::to_string(&resp).unwrap();
                    return ctx.status(400).handle_json(serialized);                  },
                None => {}
            }
        }
    }
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

pub fn put_user_by_id(ctx: Context) {
    match serde_json::from_str::<UserPayload>(&ctx.body) {
        Ok(payload) => {
            let mut user = User {
                id: ctx.param,
                name: payload.name.clone(),
                email: payload.email.clone(),
                password: payload.password.clone()
            };
        
            User::update(&mut user);
        
            let serialized = serde_json::to_string(&user).unwrap();
            ctx.handle_json(serialized);
        },
        Err(e) => {
            match e.to_string().find("name") {
                Some(_) => {
                    let resp = Message {
                        msg: String::from("name field is required")
                    };
                    let serialized = serde_json::to_string(&resp).unwrap();
                    return ctx.status(400).handle_json(serialized);                },
                None => {}
            }

            match e.to_string().find("email") {
                Some(_) => {
                    let resp = Message {
                        msg: String::from("email field is required")
                    };
                    let serialized = serde_json::to_string(&resp).unwrap();
                    return ctx.status(400).handle_json(serialized);                  },
                None => {}
            }

            match e.to_string().find("password") {
                Some(_) => {
                    let resp = Message {
                        msg: String::from("password field is required")
                    };
                    let serialized = serde_json::to_string(&resp).unwrap();
                    return ctx.status(400).handle_json(serialized);                  },
                None => {}
            }
        }
    }
}
