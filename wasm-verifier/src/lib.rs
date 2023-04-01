#![no_main]

use base64::decode;
use bincode::deserialize;
use risc0_zkvm::{serde::from_slice, sha::Digest, Receipt};
use serde::Serialize;
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;
// use web_sys::console;
use wordle_core::GameState;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export enum LetterFeedbackType {
    Correct = "Correct",
    Present = "Present",
    Miss = "Miss",
}
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
pub fn verify_receipt(
    receipt_str: String,
    image_id: String,
    word_commitment: String,
) -> Result<String, JsValue> {
    // console::log_1(&format!("IDDDDD").into());
    // console::log_1(&JsValue::from_str(&format!("verify_receipt",)));

    let receipt = match deserialize_receipt(receipt_str) {
        Ok(r) => r,
        Err(e) => return VerifyResultBuilder::failure(e),
    };
    let game_state = match deserialize_state(&receipt) {
        Ok(s) => s,
        Err(e) => return VerifyResultBuilder::failure(e),
    };

    // check that the image id and commitment to the guessed word match
    // compare_commitment(&receipt, &word_commitment)?;

    // TODO: move this to input parameter
    let id = Digest::from([
        483212511, 1898642769, 1783376007, 3825807163, 723133285, 543264778, 1560362080, 1202528983,
    ]);
    // let id_from_hex = Digest::from(&image_id);
    // let id_from_hex = Digest::from(&image_id).into();

    // let salted_word_hash = *Impl::hash_bytes(&salted_word.as_bytes());
    // let img_id_clone = image_id.clone();
    // let img_id_bytes = img_id_clone.as_bytes();
    // let image_id_bytes: [u8; 32] = img_id_bytes.try_into().unwrap();
    // let digest_contract = Digest::from(image_id_bytes);

    match receipt.verify(&id) {
        // match receipt.verify(&digest_contract) {
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

fn compare_commitment(receipt: &Receipt, contract_commitment: &String) -> Result<bool, String> {
    // console::log_1(&JsValue::from_str(&format!(
    //     "Contract commitment: {}",
    //     contract_commitment
    // )));

    let game_state: GameState = from_slice(&receipt.journal).unwrap();
    let receipt_commitment = game_state.correct_word_hash.clone().to_string();

    // console::log_1(&JsValue::from_str(&format!(
    //     "Receipt commitment: {}",
    //     receipt_commitment
    // )));

    match receipt_commitment.cmp(contract_commitment) {
        Ordering::Equal => Ok(true),
        _ => Err(String::from(
            "Contract commitment does not match receipt commitment",
        )),
    }
}
