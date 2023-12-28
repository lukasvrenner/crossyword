use rand::thread_rng;

const NUM_WORDS: usize = 10;

#[derive(Debug)]
pub struct Word<'a> {
    pub word: &'a str,
    pub clue: &'a str,
}

impl Word<'_> {
    fn calc_position<'a>(
        &self, placed_words: &[PlacedWord<'a>])
        -> Option<PlacedWord<'a>> {
            todo!();
        }
}

pub struct PlacedWord<'a> {
    pub word: &'a str,
    clue: &'a str,
    pub is_verticle: bool,
    pub pos: [i8; 2],
}
// takes ownership because original list is no longer needed
pub fn get_random_words(word_list: Vec<&str>) -> Vec<&str> {
    let mut rng = thread_rng();
    let random_indices = 
        rand::seq::index::sample(&mut rng, word_list.len(), NUM_WORDS);
    let mut random_words: Vec<&str> = Vec::new();

    for index in random_indices {
        random_words.push(word_list[index]);
    }
    random_words
}
pub fn extract_layout<'a>(words: &[Word<'a>])
-> Vec<PlacedWord<'a>> {
    let wrapped_layout = generate_layout(&words);
    match wrapped_layout {
        Some(layout) => layout,
        None => extract_layout(words)
    }
}


/* we return an Option because
 *  we do the same thing regardless
 *  of the type of error*/
fn generate_layout<'a>(word_list: &[Word<'a>])
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
    next_word: &PlacedWord<'_>, placed_words: &[PlacedWord<'_>])
-> bool {
    for placed_word in placed_words {
        // if facing opposite directions
        if next_word.is_verticle ^ placed_word.is_verticle {
            return
                next_word.pos[next_word.is_verticle as usize] -
                placed_word.pos[next_word.is_verticle as usize] > next_word.word.len().try_into().unwrap() &&
                placed_word.pos[placed_word.is_verticle as usize] -
                next_word.pos[placed_word.is_verticle as usize] > next_word.word.len().try_into().unwrap();
        }
        // if facing same direction
        // word.pos[word.is_verticle as usize] < placed_word.pos[word.is_verticle] * 

    }
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    const PLACED_WORDS: &[PlacedWord<'_>] = &[
        PlacedWord {
            word: "cat",
            clue: "an animal of group cat",
            is_verticle: false,
            pos: [0, 0],
        },
        PlacedWord {
            word: "tiger",
            clue: "a wild species of cat",
            is_verticle: true,
            pos: [2, 0],
        },
        PlacedWord {
            word: "ought",
            clue: "should",
            is_verticle: false,
            pos: [0, 2],
        }
    ];

    #[test]
    fn opposite_orientation_overlap() {
        let next_word: &PlacedWord<'_> = 
            &PlacedWord {
                word: "asess",
                clue: "to determine information from",
                is_verticle: true,
                pos: [1, -2],
            };
        assert!(illegal_overlap(next_word, PLACED_WORDS))
    }
}
