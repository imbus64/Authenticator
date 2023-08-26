// #![allow(unused_imports)]
use actix_web::dev::ConnectionInfo;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

mod delete;
mod login;
mod register;
mod types;
mod users;
use delete::delete_user;
use login::login as login_path;
use register::register as register_path;
use types::*;
use users::pass_hash;
use users::users as users_path;

#[get("/")]
async fn ip_echo(conn: ConnectionInfo) -> impl Responder {
    let ip = conn.host().to_string();
    HttpResponse::Ok().body(ip)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let sled_conf = sled::Config::new()
        .path("user_db")
        .mode(sled::Mode::LowSpace);

    let state = web::Data::new(ServerState {
        // db: Mutex::new(sled::open("my_new_db").unwrap()),
        db: Mutex::new(sled_conf.open().unwrap()),
    });

    use users::argon_hash;
    println!("Hashed password: {}", argon_hash("password"));

    println!("Server running at http://localhost:8080/");
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(login_path)
            .service(users_path)
            .service(register_path)
            .service(delete_user)
            .service(ip_echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
