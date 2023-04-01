// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]

use risc0_zkvm::guest::{env, sha};
use wordle_core::{GameState, LetterFeedback, WordFeedback, WORD_LENGTH};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let word: String = env::read();
    let guess: String = env::read();

    if word.chars().count() != WORD_LENGTH {
        panic!("Word length must be {}", WORD_LENGTH);
    }
    if guess.chars().count() != WORD_LENGTH {
        panic!("Guess length must be {}", WORD_LENGTH);
    }

    let correct_word_hash = sha::digest(&word.as_bytes()).to_owned();
    env::commit(&correct_word_hash);

    let mut score: WordFeedback = WordFeedback::default();
    for i in 0..WORD_LENGTH {
        score[i] = if word.as_bytes()[i] == guess.as_bytes()[i] {
            LetterFeedback::LetterCorrect
        } else if word.as_bytes().contains(&guess.as_bytes()[i]) {
            LetterFeedback::LetterPresent
        } else {
            LetterFeedback::LetterMiss
        };
    }

    let game_state = GameState {
        correct_word_hash,
        word_feedback: score,
    };
    env::commit(&game_state);
}
