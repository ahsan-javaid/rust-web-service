use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation};
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

fn verify_jwt_token(token: &str, decoding_key: &str) {
  match decode::<serde_json::Value>(
      &token,
      &DecodingKey::from_secret(decoding_key.as_ref()),
      &Validation::default(),
  ) {
      Ok(token_data) => {
          println!("Token is valid: {:?}", token_data);
          // Here, you can access token_data.claims to get the claims of the JWT
      }
      Err(e) => {
          println!("Token is invalid: {:?}", e);
      }
  }
}