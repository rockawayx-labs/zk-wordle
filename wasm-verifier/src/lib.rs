#![no_main]

use base64::decode;
use bincode::deserialize;
use risc0_zkvm::{serde::from_slice, sha::Digest, Receipt};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wordle_core::{GameState, WordFeedback};

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type LetterFeedbackType = "Correct" | "Present" | "Miss";

export type VerifyResultType = { 
    success: false; 
    error: string; 
    feedback: undefined;
} | { 
    success: true; 
    error: ""; 
    feedback: LetterFeedbackType[];
}
"#;

#[derive(Serialize)]
pub struct VerifyResult {
    success: bool,
    error: String,
    feedback: Option<WordFeedback>,
}

pub struct VerifyResultBuilder;
impl VerifyResultBuilder {
    pub fn success(state: GameState) -> Result<String, JsValue> {
        let result = VerifyResult {
            success: true,
            error: "".to_string(),
            feedback: Some(state.feedback),
        };

        match serde_json::to_string(&result) {
            Ok(json) => Ok(json),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    pub fn failure(error: String) -> Result<String, JsValue> {
        let result = VerifyResult {
            success: false,
            error,
            feedback: None,
        };
        match serde_json::to_string(&result) {
            Ok(json) => Ok(json),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }
}

#[no_mangle]
#[wasm_bindgen]
pub fn verify_receipt(
    receipt_str: String,
    contract_image_id: String,
    contract_word_commitment: String,
) -> Result<String, JsValue> {
    let receipt = match deserialize_receipt(receipt_str) {
        Ok(r) => r,
        Err(e) => return VerifyResultBuilder::failure(e),
    };
    let game_state = match deserialize_state(&receipt) {
        Ok(s) => s,
        Err(e) => return VerifyResultBuilder::failure(e),
    };

    if game_state.correct_word_hash.to_string() != contract_word_commitment {
        return VerifyResultBuilder::failure(format!(
            "Word commitment mismatch: {contract_word_commitment} != {}",
            game_state.correct_word_hash
        ));
    }

    // TODO: move this to input parameter
    // let id = Digest::from([
    //     719113331, 2384567050, 1972360988, 1439713833, 526468864, 546687298,
    // 3259576037, 2517916990, ]);

    let id_vec = match hex::decode(contract_image_id) {
        Ok(vec) => vec,
        Err(e) => {
            return VerifyResultBuilder::failure(format!("Error decoding contract image id: {e}"))
        }
    };
    let id_bytes: [u8; 32] = match id_vec.try_into() {
        Ok(bytes) => bytes,
        Err(e) => {
            return VerifyResultBuilder::failure(format!(
                "Error converting contract image id to bytes: {:?}",
                e
            ))
        }
    };
    let id = Digest::from(id_bytes);

    match receipt.verify(&id) {
        Ok(_) => {
            let result = VerifyResultBuilder::success(game_state);
            println!("Result: {:?}", &result);
            result
        }
        Err(err) => VerifyResultBuilder::failure(format!("Error: {err}, trying to verify id {id}")),
    }
}

fn deserialize_receipt(receipt_str: String) -> Result<Receipt, String> {
    match decode(receipt_str) {
        Ok(as_bytes) => match deserialize::<Receipt>(&as_bytes) {
            Ok(receipt) => Ok(receipt),
            Err(e) => Err(format!("Bincode error: {e}")),
        },
        Err(e) => Err(format!("Decode error: {e}")),
    }
}

fn deserialize_state(receipt: &Receipt) -> Result<GameState, String> {
    match from_slice(&receipt.journal) {
        Ok(state) => Ok(state),
        Err(e) => Err(format!("Serde error: {e}")),
    }
}
