use crate::{pass_hash, AuthResponse, Claims, LoginData, ServerState};
use actix_web::{post, web, HttpResponse, Responder};
use jsonwebtoken::{encode, EncodingKey, Header};

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
            exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
        },
        &EncodingKey::from_secret("sss".as_ref()),
    )
    .unwrap();

    HttpResponse::Ok().json(AuthResponse { token })
}
