use crate::state::AppState;
use actix_web::{post, web, Responder, Result};
use ethers::providers::{Http, Provider};
use ethers::{prelude::*, types::U256};
use methods::{WORDLE_ELF, WORDLE_ID};
use risc0_zkvm::sha::{Impl, Sha256};
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Prover, Result as ZkvmResult,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::{Arc, Mutex};
use wordle_core::GameState;

// Add client type
type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

// Generates a type-safe interface for the Wordle smart contract
abigen!(
    WordleContract,
    r"[
    function setCommitment(bytes32 commitment)
    ]"
);

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

    let hex_salt = hex::encode(state.salt);

    set_commitment_in_contract(&state.word, &hex_salt).await?;

    let output = InitOutput {
        salt: hex_salt.clone(),
        word: state.word.clone(),
    };

    Ok(web::Json(output))
}

#[post("/guess")]
pub async fn guess(
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

#[post("/check")]
pub async fn check(req_body: web::Json<CheckInput>) -> Result<impl Responder> {
    let as_bytes = match base64::decode(&req_body.receipt) {
        Ok(bytes) => bytes,
        Err(_) => return Err(actix_web::error::ErrorInternalServerError("Invalid base64")),
    };
    let receipt = match bincode::deserialize::<risc0_zkvm::Receipt>(&as_bytes) {
        Ok(receipt) => receipt,
        Err(_) => {
            return Err(actix_web::error::ErrorInternalServerError(
                "Invalid receipt",
            ))
        }
    };

    let output = match receipt.verify(&WORDLE_ID) {
        Ok(_) => CheckOutput { correct: true },
        Err(_e) => CheckOutput { correct: false },
    };

    Ok(web::Json(output))
}

fn check_guess_proof(
    guess_word: String,
    correct_word: String,
    salt: [u8; 32],
) -> ZkvmResult<GuessOutput> {
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
        receipt: base64::encode(bincode::serialize(&receipt).unwrap()),
    })
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
    let contract_addr = env::var("CONTRACT_ADDRESS").expect("$CONTRACT_ADDRESS is not set");
    let contract = WordleContract::new(
        contract_addr.parse::<Address>()?.clone(),
        Arc::new(client.clone()),
    );

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
