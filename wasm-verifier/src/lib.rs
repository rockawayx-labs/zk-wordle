#![no_main]

use base64::decode;
use bincode::deserialize;
use risc0_zkvm::{serde::from_slice, sha::Digest, Receipt};
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wordle_core::GameState;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type LetterFeedbackType = "Correct" | "Present" | "Miss";

export type VerifyResultType = { 
    success: false; 
    error: string; 
    state: undefined;
} | { 
    success: true; 
    error: ""; 
    state: {
        correct_word_hash: string;
        feedback: LetterFeedbackType[];
    };
}
"#;

#[derive(Serialize)]
pub struct VerifyResult {
    success: bool,
    error: String,
    state: Option<GameState>,
}

pub struct VerifyResultBuilder;
impl VerifyResultBuilder {
    pub fn success(state: GameState) -> Result<String, JsValue> {
        let result = VerifyResult {
            success: true,
            error: "".to_string(),
            state: Some(state),
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
            state: None,
        };
        match serde_json::to_string(&result) {
            Ok(json) => Ok(json),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }
}

#[no_mangle]
#[wasm_bindgen]
pub fn verify_receipt(receipt_str: String) -> Result<String, JsValue> {
    let receipt = match deserialize_receipt(receipt_str) {
        Ok(r) => r,
        Err(e) => return VerifyResultBuilder::failure(e),
    };
    let game_state = match deserialize_state(&receipt) {
        Ok(s) => s,
        Err(e) => return VerifyResultBuilder::failure(e),
    };

    // TODO: move this to input parameter
    let id = Digest::from([
        719113331, 2384567050, 1972360988, 1439713833, 526468864, 546687298, 3259576037, 2517916990,
    ]);

    match receipt.verify(&id) {
        Ok(_) => {
            let result = VerifyResultBuilder::success(game_state);
            println!("Result: {:?}", &result);
            result
        }
        Err(err) => VerifyResultBuilder::failure(err.to_string()),
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
