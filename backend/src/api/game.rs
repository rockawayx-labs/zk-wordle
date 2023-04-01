use actix_web::{post, web, Responder, Result};
use risc0_zkvm::{Prover, Result as ZkvmResult, serde::{from_slice, to_vec}};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use wordle_core::GameState;
use methods::{WORDLE_ELF, WORDLE_ID};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct GuessInput {
    guess: String,
}

#[derive(Deserialize)]
pub struct CheckInput {
    receipt: String,
}

#[derive(Serialize)]
pub struct CheckOutput {
    correct: bool,
}

#[derive(Serialize)]
struct GuessOutput {
    correct: bool,
    receipt: String,
}

#[derive(Serialize)]
struct InitOutput {
    salt: String,
    word: String,
}

#[post("/init")]
pub async fn init(data: web::Data<Mutex<AppState>>) -> Result<impl Responder> {
    let mut state = data.lock().unwrap();
    let default_state = AppState::default();

    state.salt = default_state.salt;
    state.word = default_state.word;

    let output = InitOutput {
        salt: hex::encode(state.salt),
        word: state.word.clone(),
    };

    Ok(web::Json(output))
}

#[post("/guess")]
pub async fn guess(req_body: web::Json<GuessInput>, data: web::Data<Mutex<AppState>>) -> Result<impl Responder> {
    let state = data.lock().unwrap();

    let output = match check_guess_proof(req_body.guess.clone(), state.word.clone(), state.salt.clone()) {
        Ok(output) => output,
        Err(_e) => {
            return Err(actix_web::error::ErrorInternalServerError("Proof failed"))
        }
    };
    Ok(web::Json(output))
}

#[post("/check")]
pub async fn check(req_body: web::Json<CheckInput>) -> Result<impl Responder> {
    let as_bytes = match base64::decode(&req_body.receipt) {
        Ok(bytes) => bytes,
        Err(_) => return Err(actix_web::error::ErrorInternalServerError("Invalid base64")),
    };
    let receipt = match bincode::deserialize::<risc0_zkvm::Receipt>(&as_bytes) {
        Ok(receipt) => receipt,
        Err(_) => return Err(actix_web::error::ErrorInternalServerError("Invalid receipt")),
    };

    let output = match receipt.verify(&WORDLE_ID) {
        Ok(_) => CheckOutput { correct: true },
        Err(_e) => CheckOutput { correct: false },
    };

    Ok(web::Json(output))
}

fn check_guess_proof(guess_word: String, correct_word: String, salt: [u8; 32]) -> ZkvmResult<GuessOutput> {
    let mut prover = Prover::new(WORDLE_ELF).expect("failed to construct prover");

    println!("correct_word: {:?}", &correct_word);

    let hex_salt = hex::encode(salt);
    println!("hex_salt: {:?}", &hex_salt);

    prover.add_input_u32_slice(to_vec(&correct_word).unwrap().as_slice());
    prover.add_input_u32_slice(to_vec(&guess_word).unwrap().as_slice());
    prover.add_input_u32_slice(to_vec(&hex_salt).unwrap().as_slice());

    let receipt = prover.run().unwrap();

    let game_state: GameState = from_slice(&receipt.journal).unwrap();
    let correct = game_state.feedback.game_is_won();

    Ok(GuessOutput { 
        correct,
        receipt: base64::encode(bincode::serialize(&receipt).unwrap()) 
    })
}
