use actix_web::{web, http, App, HttpServer};
use actix_cors::Cors;
use std::sync::Mutex;
use crate::state::AppState;

mod api;
mod wordlist;
mod state;

const ALLOWED_ORIGIN: &str = "http://localhost:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(Mutex::new(AppState::default()));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(ALLOWED_ORIGIN)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::clone(&data))
            .service(api::status::healthcheck)
            .service(api::game::init)
            .service(api::game::guess)
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}

