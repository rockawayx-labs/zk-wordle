#![no_main]

// use risc0_zkvm::{
//     sha::{Digest, DIGEST_WORDS},
//     Receipt,
// };
use methods::WORDLE_ID;
use risc0_zkvm::Receipt;
use wasm_bindgen::prelude::*;

#[no_mangle]
#[wasm_bindgen]
pub fn verify_receipt(receipt_str: String) -> bool {
    let as_bytes = base64::decode(receipt_str).unwrap();
    let receipt = bincode::deserialize::<Receipt>(&as_bytes).unwrap();

    receipt.verify(&WORDLE_ID).is_ok()
}
