use std::fs;
use rand::thread_rng;

const NUM_WORDS: usize = 10;

#[derive(Debug, Clone, Copy)]
struct Word<'a> {
    word: &'a str,
    definition: &'a str,
    is_verticle: bool,
}

fn main() -> std::io::Result<()> {
    let wordfile = fs::read_to_string("./words.txt")?;

    let all_words: Vec<&str> = wordfile.lines().collect();
    let words_list = get_random_words(all_words);
    let mut words: Vec<Word> = Vec::new();

    for item in words_list {
        let split_item: Vec<&str> = item.split(".").collect();
        let word = split_item[0];
        let definition = split_item[1];
        words.push(Word {
            word,
            definition,
            is_verticle: false,
        })
    }
    
    // test word to prevent compiler error
    // remove from final version
    let test_word = Word {
        word: "hello",
        definition: "world",
        is_verticle: false,
    };
    println!("{}", test_word.word);
    println!("{}", test_word.definition);
    println!("{}", test_word.is_verticle);

    println!("{:?}", words.len());

    Ok(())
}

fn get_random_words(word_list: Vec<&str>) -> Vec<&str> {
    let mut rng = thread_rng();
    let random_indices = rand::seq::index::sample(&mut rng, word_list.len(), NUM_WORDS);
    let mut random_words: Vec<&str> = Vec::new();

    for index in random_indices {
        random_words.push(word_list[index]);
    }
    random_words
}
