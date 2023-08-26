use crate::ServerState;
use actix_web::{get, web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sha2::{Digest, Sha256};

#[get("/users")]
async fn users(state: web::Data<ServerState>) -> impl Responder {
    let mut users = Vec::new();

    for key in state.db.lock().unwrap().iter().keys() {
        users.push(String::from_utf8(key.unwrap().to_vec()).unwrap());
    }

    HttpResponse::Ok().json(users)
}

// Returns a PHC string
pub fn argon_hash(pw: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(pw.as_bytes(), &salt).unwrap();
    return hash.to_string();
}

pub fn pass_hash(pw: String, salt: String) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(pw);
    hasher.update(salt);
    let hashed = hasher.finalize().to_vec();
    return hashed;
}
