use actix_web::{post, web, HttpResponse, Responder};

use crate::{pass_hash, LoginData, ServerResponse, ServerState};

#[post("/register")]
pub async fn register(data: web::Json<LoginData>, state: web::Data<ServerState>) -> impl Responder {
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
            locked_db.flush().unwrap();
        }
    }

    HttpResponse::Ok().json(ServerResponse::new("User registered"))
}
