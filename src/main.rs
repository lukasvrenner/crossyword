use std::fs;
use rand::thread_rng;

const NUM_WORDS: usize = 10;

#[derive(Debug, Clone, Copy)]
struct Word<'a> {
    word: &'a str,
    definition: &'a str,
    is_verticle: bool,
    letter_matches: Option<[u8; 2]>
}

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
            definition: split_item[1],
            is_verticle: false,
            letter_matches: None,
        })
    }
    
    // test word to prevent compile-time error
    // remove from final version
    let test_word = Word {
        word: "hello",
        definition: "world",
        is_verticle: false,
        letter_matches: None,
    };
    println!("{}", test_word.word);
    println!("{}", test_word.definition);
    println!("{}", test_word.is_verticle);
    println!("{:?}", test_word.letter_matches);

    println!("{:?}", words);

    Ok(())
}

// takes ownership because original list is no longer needed
fn get_random_words(word_list: Vec<&str>) -> Vec<&str> {
    let mut rng = thread_rng();
    let random_indices = 
        rand::seq::index::sample(&mut rng, word_list.len(), NUM_WORDS);
    let mut random_words: Vec<&str> = Vec::new();

    for index in random_indices {
        random_words.push(word_list[index]);
    }
    random_words
}
