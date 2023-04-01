#![no_main]

use methods::WORDLE_ID;
use risc0_zkvm::sha::Digest;
use risc0_zkvm::Receipt;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[no_mangle]
#[wasm_bindgen]
pub fn verify_receipt_with(receipt_str: String) -> String {
    let as_bytes = match base64::decode(receipt_str) {
        Ok(bytes) => bytes,
        Err(_) => return "Invalid base64".to_string(),
    };
    let receipt = match bincode::deserialize::<Receipt>(&as_bytes) {
        Ok(receipt) => receipt,
        Err(_) => return "Invalid receipt".to_string(),
    };

    console::log_1(&format!("Receipt: {:?}", &receipt).into());

    let id = Digest::from(WORDLE_ID);

    console::log_1(&format!("ID: {:?}", &id).into());
    console::log_1(&format!("Worlde ID: {:?}", &WORDLE_ID).into());

    match receipt.verify(&id) {
        Ok(_) => "OK".to_string(),
        Err(err) => err.to_string(),
    }
}
