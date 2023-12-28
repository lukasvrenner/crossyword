use std::fs;

mod generate_layout;
use crate::generate_layout::*;

fn main() -> std::io::Result<()> {
    let wordfile = fs::read_to_string("./words.txt")?;

    let all_words: Vec<&str> = wordfile.lines().collect();
    let word_list = get_random_words(all_words); // takes ownership of all_words
    let mut words: Vec<Word> = Vec::new();

    // format words_list
    for item in word_list {
        let split_item: Vec<&str> = item.split('.').collect();
        words.push(Word {
            word: split_item[0],
            clue: split_item[1],
        })
    }
    println!("{:?}", words[0]);
    let puzzle = extract_layout(&words);

    Ok(())
}

