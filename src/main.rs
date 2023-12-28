use std::fs;
use rand::thread_rng;

const NUM_WORDS: usize = 10;

#[derive(Debug)]
struct Word<'a> {
    word: &'a str,
    definition: &'a str,
}

impl Word<'_> {
    fn calc_position<'a>(
        &self, placed_words: &Vec<PlacedWord<'a>>)
        -> Option<PlacedWord<'a>> {
        todo!();
    }
}

struct PlacedWord<'a> {
    word: &'a str,
    definition: &'a str,
    is_verticle: bool,
    pos: [usize; 2],
    letter_matches: Option<[usize; 2]>
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
        })
    }
    // let puzzle = match generate_layout(words) {
    //     Some(layout) => layout,
    //     None => generate_layout(words)
    // }
    println!("{:?}", words[0]);
    let puzzle = extract_layout(words);

    Ok(())
}

fn extract_layout<'a>(words: Vec<Word<'a>>)
    -> Vec<PlacedWord<'a>> {
    let wrapped_layout = generate_layout(&words);
    match wrapped_layout {
        Some(layout) => layout,
        None => extract_layout(words)
    }
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

/* we return an Option because
*  we do the same thing regardless
*  of the type of error*/
fn generate_layout<'a>(word_list: &Vec<Word<'a>>)
    -> Option<Vec<PlacedWord<'a>>> {
    let mut placed_words: Vec<PlacedWord<'_>> = Vec::new();
    for word in word_list {
        let new_word = word.calc_position(&placed_words)?;
        if !illegal_overlap(&new_word, &placed_words) {
            placed_words.push(new_word);
        } else {
            return None;
        }
    }
   Some(placed_words)
}

// PROBABLY DOESN'T WORK
fn illegal_overlap(
    next_word: &PlacedWord<'_>, placed_words: &Vec<PlacedWord<'_>>)
    -> bool {
    for placed_word in placed_words {
        // if facing opposite directions
        if next_word.is_verticle ^ placed_word.is_verticle {
            return
                next_word.pos[next_word.is_verticle as usize] -
                placed_word.pos[next_word.is_verticle as usize] > next_word.word.len() &&
                placed_word.pos[placed_word.is_verticle as usize] -
                next_word.pos[placed_word.is_verticle as usize] > next_word.word.len();
        }
        // if facing same direction
        // word.pos[word.is_verticle as usize] < placed_word.pos[word.is_verticle] * 
            
    }
    todo!();
}
