mod generate_layout;
use crate::generate_layout::{new_puzzle, OutputWord};

use wasm_bindgen::prelude::*;
mod words;
use words::WORDS;

#[wasm_bindgen]
pub fn create_puzzle() -> Option<Vec<OutputWord>> {
    new_puzzle(WORDS.as_ref(), 10)
}
