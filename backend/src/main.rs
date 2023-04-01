use crate::wordlist::words::pick_word;
use actix_cors::Cors;
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder, Result};
use ethers::providers::{Http, Provider};
use ethers::{prelude::*, types::U256};
use methods::WORDLE_ELF;
use rand::thread_rng;
use rand::Rng;
use risc0_zkvm::sha::{Impl, Sha256};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover, Receipt, Result as ZkvmResult,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::{Arc, Mutex};
use wordle_core::GameState;

mod wordlist;

// Add client type
type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

// Generates a type-safe interface for the Wordle smart contract
abigen!(
    WordleContract,
    r"[
    function setCommitment(bytes32 commitment)
    ]"
);

const CONTRACT_ADDRESS: &str = "0xd216FC36d49A07629619d5F6eE81F0F950EA62A9";

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

    let hex_salt = hex::encode(state.salt);

    // Set the commitment first
    set_commitment_in_contract(&state.word, &hex_salt).await?;

    // If ok update contract state
    let output = InitOutput {
        salt: hex_salt.clone(),
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

async fn set_commitment_in_contract(
    word: &String,
    hex_salt: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nSetting new commitment...");

    // 1. Get the commitment value
    let salted_word = format!("{}{}", word, hex_salt);
    let salted_word_hash = *Impl::hash_bytes(&salted_word.as_bytes());
    let commitment_bytes: [u8; 32] = salted_word_hash.try_into().unwrap();

    println!("salted_word_hash: {}", salted_word_hash);
    println!("commitment_bytes: {:?}", commitment_bytes);

    // 2. Create contract instance
    let client = get_client().await.unwrap();
    let contract_addr = CONTRACT_ADDRESS.parse::<Address>()?;
    let contract = WordleContract::new(contract_addr.clone(), Arc::new(client.clone()));

    // 3. Send transaction that updates commitment
    let tx = contract
        .set_commitment(commitment_bytes)
        .gas(U256::from(50000)) // Gas
        .gas_price(U256::from(10_000_000_000u128)) // 10 Gwei - set experimentally. 1 Gwei is too little
        .send()
        .await?
        .await?;

    println!("\nTransaction Receipt: {}", serde_json::to_string(&tx)?);

    Ok(())
}

async fn get_client() -> Result<Client, Box<dyn std::error::Error>> {
    // Use a private key to create a wallet
    // Do not include the private key in plain text in any production code
    // This is just for demonstration purposes
    // Do not include '0x' at the start of the private key
    let owner_private_key = env::var("OWNER_PRIVATE_KEY").expect("$OWNER_PRIVATE_KEY is not set");
    let mumbai_api_key =
        env::var("ALCHEMY_MUMBAI_API_KEY").expect("$ALCHEMY_MUMBAI_API_KEY is not set");

    let provider = Provider::<Http>::try_from(mumbai_api_key)?;

    let wallet: LocalWallet = owner_private_key
        .parse::<LocalWallet>()?
        .with_chain_id(Chain::PolygonMumbai);

    // Wrap the provider and wallet together to create a signer client
    let client = SignerMiddleware::new(provider.clone(), wallet.clone());
    Ok(client)
}
