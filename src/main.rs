use std::fs;

mod generate_layout;
use crate::generate_layout::*;

fn main() -> std::io::Result<()> {
    let wordfile = fs::read_to_string("./words.txt")?;

    let all_words: Vec<&str> = wordfile.lines().collect();
    let mut formatted_words: Vec<Word> = Vec::new();

    // format words_list
    for item in all_words {
        let split_item: Vec<&str> = item.split('.').collect();
        formatted_words.push(Word {
            word: split_item[0],
            clue: split_item[1],
        })
    }
    let puzzle = new_puzzle(&formatted_words).unwrap();
    println!("{:?}", puzzle);

    Ok(())
}

