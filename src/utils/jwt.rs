use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::models::user::*;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
   id: u32,
   email: String
}

pub fn create_jwt(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
  let my_claims = Claims {
    id: user.id,
    email: user.email.clone()
  };

   encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))
}