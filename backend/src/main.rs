use actix_web::{web, post, App, HttpServer, Responder, Result};
use risc0_zkvm::{Receipt, Result as ZkvmResult, Prover, serde::{to_vec, from_slice}};
use serde::{Deserialize, Serialize};
use methods::{GUESS_ID, GUESS_ELF};

#[derive(Deserialize)]
struct GuessInput {
    guess: i32,
}

#[derive(Serialize)]
struct GuessOutput {
    correct: bool,
    receipt: Receipt,
}

#[post("/guess")]
async fn guess(req_body: web::Json<GuessInput>) -> Result<impl Responder> {
    let output = match check_guess_proof(req_body.into_inner()) {
        Ok(output) => output,
        Err(_e) => {
            return Err(actix_web::error::ErrorInternalServerError("Proof failed"))
        }
    };
    Ok(web::Json(output))
}

fn check_guess_proof(input: GuessInput) -> ZkvmResult<GuessOutput> {
    let mut prover = Prover::new(GUESS_ELF, GUESS_ID)?;
    let my_answer = 42;
    
    prover.add_input_u32_slice(&to_vec(&my_answer).unwrap());
    prover.add_input_u32_slice(&to_vec(&input.guess).unwrap());

    let receipt = prover.run()?;
    let product: bool = from_slice(&receipt.journal)?;

    Ok(GuessOutput { correct: product, receipt })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(guess)
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}

