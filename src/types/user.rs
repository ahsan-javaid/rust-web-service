use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPayload {
    pub name: String,
    pub email: String,
}
