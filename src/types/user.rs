use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  pub id: u32,
  pub name: String,
  pub email: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserPayload {
  pub name: String,
  pub email: String,
}