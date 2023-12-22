use std::io;
use std::fs;
use rand::thread_rng;

const NUM_WORDS: usize = 10;

#[derive(Debug, Clone, Copy)]
struct Word<'a> {
    word: &'a str,
    definition: &'a str,
}
fn main() -> io::Result<()> {
    let wordfile = fs::read_to_string("./words.txt")?;
    let mut word_list: Vec<Word> = Vec::new();
    for item in wordfile.lines() {
        let split_item: Vec<&str> = item.split(".").collect();
        let word = split_item[0];
        let definition = split_item[1];
        word_list.push(Word {
            word,
            definition,
        })
    }
    
    // test word to prevent compiler error
    // remove from final version
    let test_word = Word {
        word: "hello",
        definition: "world",
    };
    println!("{}", test_word.word);
    println!("{}", test_word.definition);

    println!("{:?}", word_list[0]);
    println!("{:?}", get_random_words(word_list));
    Ok(())
}

fn get_random_words(word_list: Vec<Word>) -> Vec<Word<'_>> {
    let mut rng = thread_rng();
    let random_indices = rand::seq::index::sample(&mut rng, word_list.len(), NUM_WORDS);
    let mut random_words: Vec<Word> = Vec::new();

    for index in random_indices {
        random_words.push(word_list[index]);
    }
    random_words

}
