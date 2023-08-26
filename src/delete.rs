use crate::{LoginData, ServerResponse, ServerState};
use actix_web::{post, web, HttpResponse, Responder, Result};

#[post("/delete_user")]
pub async fn delete_user(
    data: web::Json<LoginData>,
    state: web::Data<ServerState>,
) -> Result<impl Responder> {
    println!("Deleting user {}", data.username);
    let locked_db = state.db.lock().unwrap();
    let search_match = locked_db.get(&data.username).unwrap();
    match search_match {
        Some(_) => {
            locked_db.remove(&data.username).unwrap();
        }
        None => {
            return Ok(
                HttpResponse::Unauthorized().json(ServerResponse::new("Username does not exist"))
            );
        }
    }

    Ok(HttpResponse::Ok().json(ServerResponse::new("User deleted")))
}
