use crate::state::AppState;
use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use std::sync::Mutex;
use std::env;

mod api;
mod state;
mod wordlist;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(Mutex::new(AppState::default()));

    let allowed_origin = env::var("PUBLIC_URL").unwrap_or("http://localhost:8080".to_string());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(allowed_origin.as_str())
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
