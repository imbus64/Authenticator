use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub struct ServerState {
    pub db: Mutex<sled::Db>,
}

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub aud: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServerResponse {
    pub message: String,
}

impl ServerResponse {
    pub fn new(message: &str) -> ServerResponse {
        ServerResponse {
            message: message.to_string(),
        }
    }
}
