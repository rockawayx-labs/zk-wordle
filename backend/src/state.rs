use crate::wordlist::words::pick_word;
use rand::{thread_rng, Rng};

pub struct AppState {
    pub salt: [u8; 32],
    pub word: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            salt: generate_salt(),
            word: String::from(pick_word()),
        }
    }
}

fn generate_salt() -> [u8; 32] {
    let mut rng = thread_rng();
    let mut salt = [0u8; 32];
    rng.fill(&mut salt);
    salt
}
