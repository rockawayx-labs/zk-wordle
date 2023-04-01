#![no_main]

use risc0_zkvm::sha::Digest;
use risc0_zkvm::Receipt;
use wasm_bindgen::prelude::*;

#[no_mangle]
#[wasm_bindgen]
pub fn verify_receipt(receipt_str: String) -> String {
    let as_bytes = match base64::decode(receipt_str) {
        Ok(bytes) => bytes,
        Err(_) => return "Invalid base64".to_string(),
    };
    let receipt = match bincode::deserialize::<Receipt>(&as_bytes) {
        Ok(receipt) => receipt,
        Err(_) => return "Invalid receipt".to_string(),
    };

    // TODO: move this to input parameter
    let id = Digest::from([
        269857112, 1358814123, 3810820780, 2766300945, 3087851643, 3544867356, 319472692,
        4172190676,
    ]);

    match receipt.verify(&id) {
        Ok(_) => "OK".to_string(),
        Err(err) => err.to_string(),
    }
}
