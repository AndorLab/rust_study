extern crate frank_jwt;
extern crate dotenv_codegen;
use frank_jwt::{ encode, Algorithm };
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginBaseInfo {
  pub username: String,
  pub password: String,
}

pub fn generate_jwt(payload: &Value) -> String {
  let header = json!({
    "alg": "HS256",
    "typ": "JWT"
  });
  let secret = dotenv!("salt");
  encode(header, &secret.to_string(), &payload, Algorithm::HS256).unwrap()
}