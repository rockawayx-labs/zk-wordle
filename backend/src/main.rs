use crate::wordlist::words::pick_word;
use actix_cors::Cors;
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder, Result};
use methods::WORDLE_ELF;
use rand::{thread_rng, Rng};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover, Receipt, Result as ZkvmResult,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Mutex;
use wordle_core::GameState;

mod wordlist;

struct AppState {
    salt: [u8; 32],
    word: String,
}

#[derive(Deserialize)]
struct GuessInput {
    guess: String,
}

#[derive(Serialize)]
struct GuessOutput {
    correct: bool,
    receipt: Receipt,
}

#[derive(Serialize)]
struct InitOutput {
    salt: String,
    word: String,
}

#[get("/")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[post("/init")]
async fn init(data: web::Data<Mutex<AppState>>) -> Result<impl Responder> {
    let mut state = data.lock().unwrap();

    state.salt = generate_salt();
    state.word = String::from(pick_word());

    let output = InitOutput {
        salt: hex::encode(state.salt),
        word: state.word.clone(),
    };

    Ok(web::Json(output))
}

#[post("/guess")]
async fn guess(
    req_body: web::Json<GuessInput>,
    data: web::Data<Mutex<AppState>>,
) -> Result<impl Responder> {
    let state = data.lock().unwrap();

    let output = match check_guess_proof(
        req_body.guess.clone(),
        state.word.clone(),
        state.salt.clone(),
    ) {
        Ok(output) => output,
        Err(_e) => return Err(actix_web::error::ErrorInternalServerError("Proof failed")),
    };
    Ok(web::Json(output))
}

fn check_guess_proof(
    guess_word: String,
    correct_word: String,
    salt: [u8; 32],
) -> ZkvmResult<GuessOutput> {
    let mut prover = Prover::new(WORDLE_ELF).expect("failed to construct prover");

    println!("correct_word: {:?}", correct_word);

    let hex_salt = hex::encode(salt);
    println!("hex_salt: {:?}", hex_salt);

    prover.add_input_u32_slice(to_vec(&correct_word).unwrap().as_slice());
    prover.add_input_u32_slice(to_vec(&guess_word).unwrap().as_slice());
    prover.add_input_u32_slice(to_vec(&hex_salt).unwrap().as_slice());

    let receipt = prover.run().unwrap();

    let game_state: GameState = from_slice(&receipt.journal).unwrap();
    let correct = game_state.feedback.game_is_won();

    Ok(GuessOutput { correct, receipt })
}

const ALLOWED_ORIGIN: &str = "http://localhost:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let v = env::var("TEST_VAT").expect("$TEST_VAT is not set");
    println!("v: {:?}", v);

    let data = web::Data::new(Mutex::new(AppState {
        salt: generate_salt(),
        word: String::from(pick_word()),
    }));

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
            .service(guess)
            .service(init)
            .service(status)
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}

fn generate_salt() -> [u8; 32] {
    let mut rng = thread_rng();
    let mut salt = [0u8; 32];
    rng.fill(&mut salt);
    salt
}
