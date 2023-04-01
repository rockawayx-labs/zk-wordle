use crate::state::AppState;
use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use std::sync::Mutex;

mod api;
mod state;
mod wordlist;

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
            .service(api::game::image)
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}
