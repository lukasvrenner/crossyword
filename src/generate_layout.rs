use rand::thread_rng;

const NUM_WORDS: usize = 10;

#[derive(Debug)]
pub struct Word<'a> {
    pub word: &'a str,
    pub clue: &'a str,
}

impl Word<'_> {
    fn calc_position<'a>(
        &'a self, placed_words: &[PlacedWord<'a>])
        -> Option<PlacedWord<'a>> {
            let mut y_pos: isize = 0;
            let mut x_pos: isize = 0;
            let mut is_verticle: bool = false;
            for placed_word in placed_words {
                is_verticle = !placed_word.is_verticle;
                if placed_word.is_verticle {
                    for (letter_index, letter) in self.word.chars().enumerate() {
                        x_pos = placed_word.pos[0] - letter_index as isize;
                        y_pos = match placed_word.word.find(letter) {
                            Some(index) => index as isize,
                            None => break,
                        }
                    }
                } else {
                    for (letter_index, letter) in self.word.chars().enumerate() {
                        y_pos = placed_word.pos[1] - letter_index as isize;
                        x_pos = match placed_word.word.find(letter) {
                            Some(index) => index as isize,
                            None => break,
                        }
                    }
                }
            }
            let pos = [x_pos, y_pos];
            let new_word = PlacedWord {
                word: self.word,
                clue: self.clue,
                is_verticle,
                pos,
            };
            if illegal_overlap(&new_word, &placed_words) {
                return None
            }
            Some(new_word)
        }
}

pub struct PlacedWord<'a> {
    pub word: &'a str,
    clue: &'a str,
    pub is_verticle: bool,
    pub pos: [isize; 2],
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

// repeats until a sucessful layout is created
pub fn extract_layout<'a>(words: &'a [Word<'a>])
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
fn generate_layout<'a>(word_list: &'a [Word<'a>])
-> Option<Vec<PlacedWord<'a>>> {
    let mut placed_words: Vec<PlacedWord<'a>> = Vec::new();
    for word in word_list {
        let new_word = word.calc_position(&placed_words)?;
        placed_words.push(new_word);
    }
    Some(placed_words)
}

// PROBABLY DOESN'T WORK
fn illegal_overlap(
    next_word: &PlacedWord<'_>, placed_words: &[PlacedWord<'_>])
-> bool {
    let mut illegal = false;
    if next_word.is_verticle {
        for placed_word in placed_words {
            if !placed_word.is_verticle {
                illegal = next_word.pos[0] - placed_word.pos[0] >= 0 &&
                    next_word.pos[0] - placed_word.pos[0]
                    < placed_word.word.len() as isize
                    &&
                    placed_word.pos[1] - next_word.pos[1] >= 0 &&
                    placed_word.pos[1] - next_word.pos[1]
                    < next_word.word.len() as isize
                    &&
                    next_word.word.chars().nth(
                        (placed_word.pos[1] - next_word.pos[1]) as usize
                        )
                    !=
                    placed_word.word.chars().nth(
                        (next_word.pos[0] - placed_word.pos[0]) as usize
                        );
            } else {
                illegal = 
                    next_word.pos[1] - placed_word.pos[1]
                    > next_word.word.len() as isize
                    &&
                    placed_word.pos[1] - next_word.pos[1] 
                    > placed_word.word.len() as isize;
            }
            if illegal {
                break;
            }
        }
    } else {
        for placed_word in placed_words {
            if placed_word.is_verticle {
                illegal = next_word.pos[0] - placed_word.pos[0] >= 0 &&
                    placed_word.pos[0] - next_word.pos[0]
                    < next_word.word.len() as isize
                    &&
                    next_word.pos[1] - placed_word.pos[1] >= 0 &&
                    next_word.pos[1] - placed_word.pos[1]
                    < placed_word.word.len() as isize
                    &&
                    placed_word.word.chars().nth(
                        (next_word.pos[1] - placed_word.pos[1]) as usize
                        )
                    !=
                    next_word.word.chars().nth(
                        (placed_word.pos[0] - next_word.pos[0]) as usize
                        );
            } else {
                illegal = 
                    next_word.pos[0] - placed_word.pos[0]
                    > next_word.word.len() as isize
                    &&
                    placed_word.pos[0] - next_word.pos[0] 
                    > placed_word.word.len() as isize;
            }
            if illegal {
                break;
            }
        }
    }
    illegal
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
/*
 --- --- ---
| c | a | t |
 --- --- ---
        | i |
 --- --- --- --- ---
| o | u | g | h | t |
 --- --- --- --- ---
        | e |
         ---
        | r |
         ---
*/
    #[test]
    fn overlap() {
        let opposite_orientation_illegal: &PlacedWord<'_> = 
            &PlacedWord {
                word: "asess",
                clue: "to determine information from",
                is_verticle: true,
                pos: [1, 0],
            };
/*
 --- --- ---
| c | a | t |
 --- --- ---
    | s | i |
 --- --- --- --- ---
| o | ! | g | h | t |
 --- --- --- --- ---
    | s | e |
     --- ---
    | s | r |
     --- ---
*/
        let opposite_orientation_legal: &PlacedWord<'_> = 
            &PlacedWord {
                word: "alumina",
                clue: "aluminium oxide",
                is_verticle: true,
                pos: [1, 0],
            };
/*
 --- --- ---
| c | a | t |
 --- --- ---
    | l | i |
 --- --- --- --- ---
| o | u | g | h | t |
 --- --- --- --- ---
    | m | e |
     --- ---
    | i | r |
     --- ---
    | n |
     ---
    | a |
     ---
*/
        let same_orientation_illegal: &PlacedWord<'_> = 
            &PlacedWord {
                word: "alumina",
                clue: "aluminium oxide",
                is_verticle: true,
                pos: [1, 0],
            };
        assert!(illegal_overlap(opposite_orientation_illegal, PLACED_WORDS));
        assert!(!illegal_overlap(opposite_orientation_legal, PLACED_WORDS));
    }
}
