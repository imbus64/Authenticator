use actix_web::{web, App, HttpServer};
mod config;
mod db_users;
mod handlers;
mod jwt;
mod types;

use handlers::*;
use log::*;
use types::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    info!("Starting server");

    let state = ServerState::new();
    let state = web::Data::new(state);

    // dump_users(&state.db.lock().unwrap());

    debug!("Server running at http://localhost:8080/");
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(login)
            .service(register)
            .service(refresh)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
