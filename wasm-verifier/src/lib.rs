#![no_main]

// use risc0_zkvm::{
//     sha::{Digest, DIGEST_WORDS},
//     Receipt,
// };
use wasm_bindgen::prelude::*;

#[no_mangle]
#[wasm_bindgen]
pub fn is_even(n: i32) -> bool {
    // TODO: use a real receipt and image_id
    // let receipt = Receipt::new(&[], &[]);
    // let image_id = Digest::from([0; DIGEST_WORDS]);
    // receipt.verify(&image_id).unwrap();
    n % 2 == 0
}
