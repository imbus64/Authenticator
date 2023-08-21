use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
// use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Mutex;

struct ServerState {
    db: Mutex<sled::Db>,
}

#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    aud: String,
    iat: usize,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct ServerResponse {
    message: String,
}

impl ServerResponse {
    fn new(message: &str) -> ServerResponse {
        ServerResponse {
            message: message.to_string(),
        }
    }
}

#[get("/users")]
async fn users(state: web::Data<ServerState>) -> impl Responder {
    let mut users = Vec::new();

    for key in state.db.lock().unwrap().iter().keys() {
        users.push(String::from_utf8(key.unwrap().to_vec()).unwrap());
    }

    HttpResponse::Ok().json(users)
}

fn pass_hash(pw: String, salt: String) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(pw);
    hasher.update(salt);
    let hashed = hasher.finalize().to_vec();
    return hashed;
}

#[post("/register")]
async fn register(data: web::Json<LoginData>, state: web::Data<ServerState>) -> impl Responder {
    let locked_db = state.db.lock().unwrap();
    let search_match = locked_db.get(&data.username).unwrap();
    match search_match {
        Some(_) => {
            return HttpResponse::Unauthorized()
                .json(ServerResponse::new("Username already exists"));
        }
        None => {
            locked_db
                .insert(
                    &data.username,
                    pass_hash(data.password.clone(), "salt".to_string()),
                )
                .unwrap();
        }
    }

    HttpResponse::Ok().json(ServerResponse::new("User registered"))
}

#[post("/delete_user")]
async fn delete_user(data: web::Json<LoginData>, state: web::Data<ServerState>) -> impl Responder {
    println!("Deleting user {}", data.username);
    let locked_db = state.db.lock().unwrap();
    let search_match = locked_db.get(&data.username).unwrap();
    match search_match {
        Some(_) => {
            locked_db.remove(&data.username).unwrap();
        }
        None => {
            return HttpResponse::Unauthorized()
                .json(ServerResponse::new("Username does not exist"));
        }
    }

    HttpResponse::Ok().json(ServerResponse::new("User deleted"))
}

#[post("/login")]
async fn login(data: web::Json<LoginData>, state: web::Data<ServerState>) -> impl Responder {
    let locked_db = state.db.lock().unwrap(); //.unwrap().insert("k", "v").unwrap();
    let search_match = locked_db.get(&data.username).unwrap();
    match search_match {
        Some(password) => {
            let password = password.to_vec();
            let hashed = pass_hash(data.password.clone(), "salt".to_string());
            if password != hashed {
                return HttpResponse::Unauthorized().body("Invalid username or password");
            }
        }
        None => {
            return HttpResponse::Unauthorized().body("Invalid username or password");
        }
    }

    println!("Issuing JWT token for {}", data.username);
    let token = encode(
        &Header::default(),
        &Claims {
            sub: data.username.clone(),
            iss: "localhost".to_string(),
            aud: "localhost".to_string(),
            iat: chrono::Utc::now().timestamp() as usize,
            exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize, // 20 hours
        },
        &EncodingKey::from_secret("sss".as_ref()),
    )
    .unwrap();

    HttpResponse::Ok().json(AuthResponse { token })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(ServerState {
        db: Mutex::new(sled::open("my_new_db").unwrap()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(login)
            .service(users)
            .service(register)
            .service(delete_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
