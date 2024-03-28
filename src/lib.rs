mod generate_layout;
use crate::generate_layout::{new_puzzle, PlacedWord};

use wasm_bindgen::prelude::*;
mod words;
use words::WORDS;

#[wasm_bindgen]
pub fn create_puzzle() -> Option<Vec<PlacedWord>> {
    // let words_file = "./words.txt";
    //
    // // note: display error to window once GUI is created
    // let unformatted_words =
    //     fs::read_to_string(words_file).unwrap_or_else(|err| {
    //         eprintln!("could not read {}: {}", words_file, err);
    //         std::process::exit(1);
    //     });
    //
    // // note: display error to window once GUI is created
    // let formatted_words =
    //     parse_words(&unformatted_words).unwrap_or_else(|| {
    //         eprintln!("could not parse {}", words_file);
    //         std::process::exit(1);
    //     });
    //
    new_puzzle(WORDS.to_vec(), 10)
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn create_puzzle() {
        super::create_puzzle();
    }
}
