#![no_main]
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let number: i32 = env::read();
    let guess: i32 = env::read();

    let product = number == guess;

    env::commit(&product);
}
