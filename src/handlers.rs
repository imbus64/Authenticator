use crate::{
    db_users::{user_insert, user_remove, user_validate},
    jwt::{token_factory, validate_token},
    types::JwkData,
    AuthResponse, LoginData, ServerResponse, ServerState,
};

use actix_web::{post, web, HttpResponse, Responder};

#[post("/login")]
async fn login(data: web::Json<LoginData>, state: web::Data<ServerState>) -> impl Responder {
    let locked_db = state.db.lock().unwrap(); //.unwrap().insert("k", "v").unwrap();

    if !user_validate(&data.username, &data.password, &locked_db) {
        return HttpResponse::Unauthorized().json(AuthResponse {
            error: Some("Invalid username or password".to_string()),
            token: "".to_string(),
        });
    }

    let token = token_factory(&data.username).unwrap();

    HttpResponse::Ok().json(AuthResponse { error: None, token })
}

#[post("/register")]
pub async fn register(data: web::Json<LoginData>, state: web::Data<ServerState>) -> impl Responder {
    let db = state.db.lock().unwrap();
    user_insert(&data.username, &data.password, &db);

    HttpResponse::Ok().json(ServerResponse::new("User registered"))
}

#[post("/refresh")]
pub async fn refresh(data: web::Json<JwkData>) -> impl Responder {
    match validate_token(&data.token) {
        Ok(claims) => {
            let token = token_factory(&claims.sub).unwrap();
            return HttpResponse::Ok().json(AuthResponse { error: None, token });
        }
        Err(_) => return HttpResponse::Unauthorized().json(ServerResponse::new("Invalid token")),
    }
}

#[post("/remove")]
pub async fn remove(data: web::Json<JwkData>, state: web::Data<ServerState>) -> impl Responder {
    match validate_token(&data.token) {
        Ok(claims) => {
            let db = state.db.lock().unwrap();
            user_remove(&claims.sub, &db);
            return HttpResponse::Ok().json(ServerResponse::new("User removed"));
        }
        Err(_) => return HttpResponse::Unauthorized().json(ServerResponse::new("Invalid token")),
    }
}
