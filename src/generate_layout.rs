//! A collection of functions and types for creating crossword puzzles
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Debug, Clone, Copy)]
#[wasm_bindgen]
pub enum Orientation {
    Horizontal = 0,
    Vertical = 1,
}

impl std::ops::Not for Orientation {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Orientation::Vertical => Orientation::Horizontal,
            Orientation::Horizontal => Orientation::Vertical,
        }
    }
}

/// initial `Word` type, with no additianl metadata
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct Word {
    pub word: &'static str,
    pub clue: &'static str,
}

impl Word {
    /// calculates positions for `self`
    /// to join `placed_words`
    /// does *not* add `self` to `placed_words`
    fn place(&self, placed_words: &[PlacedWord]) -> Option<PlacedWord> {
        for placed_word in placed_words {
            let new_orientation = !placed_word.orientation;

            for (index, letter) in self.word.char_indices() {
                let dependant_axis_pos = match placed_word.word.find(letter) {
                    Some(position) => {
                        position as isize + placed_word.pos[!new_orientation as usize]
                    }
                    None => continue,
                };

                let independant_axis_pos =
                    placed_word.pos[new_orientation as usize] - index as isize;

                let pos = match new_orientation {
                    Orientation::Vertical => [dependant_axis_pos, independant_axis_pos],
                    Orientation::Horizontal => [independant_axis_pos, dependant_axis_pos],
                };

                let next_word = PlacedWord {
                    word: self.word,
                    clue: self.clue,
                    orientation: new_orientation,
                    pos,
                };

                if !illegal_overlap(&next_word, placed_words) {
                    return Some(next_word);
                }
            }
        }
        if placed_words.is_empty() {
            let next_word = PlacedWord {
                word: self.word,
                clue: self.clue,
                orientation: Orientation::Vertical,
                pos: [0, 0],
            };
            return Some(next_word);
        }
        None
    }
}

/// like `Word`, but with additional metadata
#[cfg_attr(test, derive(PartialEq, Clone, Copy))]
#[derive(Debug)] // only for development purposes -- remove once GUI is created
pub struct PlacedWord {
    pub word: &'static str,
    pub clue: &'static str,
    pub orientation: Orientation,
    pub pos: [isize; 2],
}

impl PlacedWord {
    /// returns `true` if `word` overlaps `self`
    /// they are still considered "overlapping" if the end of one
    /// word touches the other
    /// otherwise, returns `false`
    /// note: only works properly if the words are perpendicular
    fn overlaps(&self, word: &PlacedWord) -> bool {
        let (vertical_word, horizontal_word) = match self.orientation {
            Orientation::Vertical => (self, word),
            Orientation::Horizontal => (word, self),
        };

        vertical_word.pos[0] >= horizontal_word.pos[0]
            && vertical_word.pos[0] - horizontal_word.pos[0] <= horizontal_word.word.len() as isize
            && horizontal_word.pos[1] >= vertical_word.pos[1]
            && horizontal_word.pos[1] - vertical_word.pos[1] <= vertical_word.word.len() as isize
    }

    /// returns the number of words in `placed_words` `self` overlaps with
    fn number_of_overlaps(&self, placed_words: &[PlacedWord]) -> usize {
        let mut overlaps = 0;
        for word in placed_words {
            if self.orientation != word.orientation {
                overlaps += self.overlaps(word) as usize;
            }
        }
        overlaps
    }
}

#[wasm_bindgen]
pub struct OutputWord {
    #[wasm_bindgen(skip)]
    pub word: &'static str,
    #[wasm_bindgen(skip)]
    pub clue: &'static str,
    pub orientation: Orientation,
    pub xpos: usize,
    pub ypos: usize,
}

#[wasm_bindgen]
impl OutputWord {
    #[wasm_bindgen(getter)]
    pub fn word(&self) -> String {
        self.word.to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn clues(&self) -> String {
        self.clue.to_owned()
    }
}

impl From<PlacedWord> for OutputWord {
    fn from(value: PlacedWord) -> Self {
        Self {
            word: value.word,
            clue: value.clue,
            orientation: value.orientation,
            xpos: value.pos[0] as usize,
            ypos: value.pos[1] as usize,
        }
    }
}

trait GetOverlaps {
    fn total_overlaps(&self) -> usize;
}

pub type PuzzleBorrowed = Vec<PlacedWord>;

impl GetOverlaps for PuzzleBorrowed {
    /// returns the total number of times two words overlap
    fn total_overlaps(&self) -> usize {
        let mut total_overlaps = 0;
        // .filter() offers a minor performance boost
        // because we only have to count half of the words
        for word in self
            .iter()
            .filter(|word| word.orientation == Orientation::Horizontal)
        {
            total_overlaps += word.number_of_overlaps(self);
        }
        total_overlaps
    }
}

fn allign_puzzle(puzzle: &mut [PlacedWord]) {
    let mut left_most = 0;
    let mut up_most = 0;

    for word in &*puzzle {
        let more_left = left_most > word.pos[0];
        let more_up = up_most > word.pos[1];
        left_most = word.pos[0] * more_left as isize + left_most * !more_left as isize;
        up_most = word.pos[1] * more_up as isize + up_most * !more_up as isize;
    }

    for word in puzzle {
        word.pos[0] -= left_most;
        word.pos[1] -= up_most;
    }
}

// consider changing to Puzzle::new()
/// creates a new puzzle given a word list, `word_list`,
/// and a number of words to use, `num_words`
pub fn new_puzzle(word_list: &[Word], num_words: usize) -> Option<Vec<OutputWord>> {
    let mut best_puzzle = None;
    let mut most_ovelaps = 0;

    for _ in 0..50000 {
        let words = get_random_words(&word_list, num_words);
        match generate_layout(words.as_ref()) {
            Some(puzzle) => {
                let overlaps = puzzle.total_overlaps();
                if overlaps > most_ovelaps {
                    most_ovelaps = overlaps;
                    best_puzzle = Some(puzzle);
                }
            }
            None => continue,
        }
    }
    match best_puzzle {
        Some(mut borred_puzzle) => {
            allign_puzzle(&mut borred_puzzle);
            let mut puzzle = Vec::with_capacity(borred_puzzle.len());
            for word in borred_puzzle {
                puzzle.push(OutputWord::from(word));
            }
            Some(puzzle)
        }
        None => None,
    }
}

/// uses `rand` crate to pick `num_words` words
fn get_random_words<'a>(word_list: &'a [Word], num_words: usize) -> Vec<&'a Word> {
    let mut rng = rand::thread_rng();
    let random_indices = rand::seq::index::sample(&mut rng, word_list.len(), num_words);
    let mut random_words = Vec::with_capacity(num_words);

    for index in random_indices {
        random_words.push(&word_list[index]);
    }
    random_words
}

/// attempts to create a crossword puzzle from `words`
fn generate_layout(words: &[&Word]) -> Option<PuzzleBorrowed> {
    let mut placed_words: PuzzleBorrowed = Vec::with_capacity(words.len());

    for word in words {
        placed_words.push(word.place(&placed_words)?);
    }
    Some(placed_words)
}

/// returns `true` if `next_word` has any "illegal" overlaps in `placed_words`
///
/// an overlap is considered "illegal" if the letters at the place of overlap
/// are not the same
fn illegal_overlap(next_word: &PlacedWord, placed_words: &[PlacedWord]) -> bool {
    let mut illegal = false;
    for placed_word in placed_words {
        if placed_word.orientation != next_word.orientation {
            let (vertical_word, horizontal_word) = match next_word.orientation {
                Orientation::Vertical => (next_word, placed_word),
                Orientation::Horizontal => (placed_word, next_word),
            };

            illegal = horizontal_word.overlaps(vertical_word)
                &&
                // if overlapped characters are different
                vertical_word.word.chars().nth(
                    (horizontal_word.pos[1] - vertical_word.pos[1]) as usize
                    )
                !=
                horizontal_word.word.chars().nth(
                    (vertical_word.pos[0] - horizontal_word.pos[0]) as usize
                    );
        } else {
            // any same-direction overlap is illegal
            let is_vertical = next_word.orientation as usize;
            let is_horizontal = !next_word.orientation as usize;

            illegal = (next_word.pos[is_vertical] - placed_word.pos[is_vertical]
                < next_word.word.len() as isize
                || placed_word.pos[is_vertical] - next_word.pos[is_vertical]
                    < placed_word.word.len() as isize)
                && placed_word.pos[is_horizontal] == next_word.pos[is_horizontal];
        }

        if illegal {
            break;
        }
    }

    illegal
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLACED_WORDS: [PlacedWord; 4] = [
        PlacedWord {
            word: "cat",
            clue: "an animal of group cat",
            orientation: Orientation::Horizontal,
            pos: [0, 0],
        },
        PlacedWord {
            word: "tiger",
            clue: "a wild species of cat",
            orientation: Orientation::Vertical,
            pos: [2, 0],
        },
        PlacedWord {
            word: "ought",
            clue: "should",
            orientation: Orientation::Horizontal,
            pos: [0, 2],
        },
        PlacedWord {
            word: "batter",
            clue: "hit repeatedly",
            orientation: Orientation::Vertical,
            pos: [4, 0],
        },
    ];
    /*
     --- --- ---     ---
    | c | a | t |   | b |
     --- --- ---     ---
            | i |   | a |
     --- --- --- --- ---
    | o | u | g | h | t |
     --- --- --- --- ---
            | e |   | t |
             ---     ---
            | r |   | e |
             ---     ---
                    | r |
                     ---
    */

    #[test]
    fn word_overlaps_other_word() {
        assert!(PLACED_WORDS[1].overlaps(&PLACED_WORDS[2]));
        assert!(!PLACED_WORDS[0].overlaps(&PLACED_WORDS[3]));
        assert!(!PLACED_WORDS[0].overlaps(&PLACED_WORDS[2]));
    }

    #[test]
    fn count_individual_word_overlaps() {
        assert_eq!(&PLACED_WORDS[0].number_of_overlaps(&PLACED_WORDS), &1);
        assert_eq!(&PLACED_WORDS[1].number_of_overlaps(&PLACED_WORDS), &2);
    }

    #[test]
    fn count_total_overlaps() {
        let words_as_vec = &PLACED_WORDS.to_vec();
        assert_eq!(words_as_vec.total_overlaps(), 3);
    }

    #[test]
    fn illegal() {
        let vert_opposite_orientation_illegal: PlacedWord = PlacedWord {
            word: "assess",
            clue: "to determine information from",
            orientation: Orientation::Vertical,
            pos: [1, 0],
        };

        assert!(illegal_overlap(
            &vert_opposite_orientation_illegal,
            &PLACED_WORDS
        ));

        let vert_opposite_orientation_legal: PlacedWord = PlacedWord {
            word: "alumina",
            clue: "aluminium oxide",
            orientation: Orientation::Vertical,
            pos: [1, 0],
        };

        assert!(!illegal_overlap(
            &vert_opposite_orientation_legal,
            &PLACED_WORDS
        ));

        let hori_opposite_orientation_illegal: PlacedWord = PlacedWord {
            word: "bitter",
            clue: "having a sharp, pungent taste or smell",
            orientation: Orientation::Vertical,
            pos: [1, 0],
        };

        assert!(illegal_overlap(
            &hori_opposite_orientation_illegal,
            &PLACED_WORDS
        ));

        let off_by_one_illegal: PlacedWord = PlacedWord {
            word: "bit",
            clue: "small amount",
            orientation: Orientation::Horizontal,
            pos: [1, 1],
        };

        assert!(illegal_overlap(&off_by_one_illegal, &PLACED_WORDS));

        let hori_same_orientation_illegal: PlacedWord = PlacedWord {
            word: "its",
            clue: "posessive case of it",
            orientation: Orientation::Horizontal,
            pos: [3, 2],
        };

        assert!(illegal_overlap(
            &hori_same_orientation_illegal,
            &PLACED_WORDS
        ));
    }

    #[test]
    fn calc_position() {
        let vertical: Word = Word {
            word: "crouch",
            clue: "kneel",
        };

        let vertical_placed: PlacedWord = PlacedWord {
            word: "crouch",
            clue: "kneel",
            orientation: Orientation::Vertical,
            pos: [0, 0],
        };
        assert_eq!(vertical.place(&PLACED_WORDS), Some(vertical_placed));

        let no_possible_pos: Word = Word {
            word: "snaps",
            clue: "breaks",
        };
        assert_eq!(no_possible_pos.place(&PLACED_WORDS), None);

        let horizontal: Word = Word {
            word: "better",
            clue: "superior",
        };
        let horizontal_placed: PlacedWord = PlacedWord {
            word: "better",
            clue: "superior",
            orientation: Orientation::Horizontal,
            pos: [1, 3],
        };
        assert_eq!(horizontal.place(&PLACED_WORDS), Some(horizontal_placed));
    }
}
