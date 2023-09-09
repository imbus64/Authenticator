use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// This holds a sync mutex, which doesent allow for concurrent reads and writes
// Sled is thread safe, but not concurrent safe (i think)
// This needs a better solution
pub struct ServerState {
    pub db: Mutex<sled::Db>,
}

impl ServerState {
    pub fn new() -> ServerState {
        let sled_conf = sled::Config::new()
            .path("user_db")
            .mode(sled::Mode::LowSpace);

        ServerState {
            db: Mutex::new(sled_conf.open().unwrap()),
        }
    }
}

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct JwkData {
    pub token: String,
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
    pub error: Option<String>,
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
