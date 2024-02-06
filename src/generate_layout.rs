//! A collection of functions and types for creating crossword puzzles

/// initial `Word` type, with no additianl metadata
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct Word<'a> {
    pub word: &'a str,
    pub clue: &'a str,
}

/// like `Word`, but with additional metadata
#[cfg_attr(test, derive(PartialEq, Clone, Copy))]
#[derive(Debug)] // only for development purposes -- remove once GUI is created
pub struct PlacedWord<'a> {
    pub word: &'a str,
    pub clue: &'a str,
    pub is_vertical: bool,
    pub pos: [isize; 2],
}

pub type Puzzle<'a> = Vec<PlacedWord<'a>>;

impl Word<'_> {
    /// calculates positions for `self`
    /// to join `placed_words`
    /// does *not* add `self` to `placed_words`
    fn place<'a>(
        &'a self,
        placed_words: &[PlacedWord<'a>],
    ) -> Option<PlacedWord<'a>> {
        for placed_word in placed_words {
            let is_vertical = !placed_word.is_vertical;

            for (index, letter) in self.word.char_indices() {
                let dependant_axis_pos = match placed_word.word.find(letter) {
                    Some(position) => {
                        position as isize
                            + placed_word.pos[!is_vertical as usize]
                    }
                    None => continue,
                };

                let independant_axis_pos =
                    placed_word.pos[is_vertical as usize] - index as isize;

                let pos: [isize; 2] = if is_vertical {
                    [dependant_axis_pos, independant_axis_pos]
                } else {
                    [independant_axis_pos, dependant_axis_pos]
                };

                let next_word = PlacedWord {
                    word: self.word,
                    clue: self.clue,
                    is_vertical,
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
                is_vertical: false,
                pos: [0, 0],
            };
            return Some(next_word);
        }
        None
    }
}

impl PlacedWord<'_> {
    /// returns `true` if `word` overlaps `self`
    /// otherwise, returns `false`
    /// note: only works properly if the words are perpendicular
    fn overlaps(&self, word: &PlacedWord) -> bool {
        let (vertical_word, horizontal_word) = if self.is_vertical {
            (self, word)
        } else {
            (word, self)
        };

        vertical_word.pos[0] >= horizontal_word.pos[0]
            && vertical_word.pos[0] - horizontal_word.pos[0]
                < horizontal_word.word.len() as isize
            && horizontal_word.pos[1] >= vertical_word.pos[1]
            && horizontal_word.pos[1] - vertical_word.pos[1]
                < vertical_word.word.len() as isize
    }

    /// returns the number of words in `placed_words` `self` overlaps with
    fn number_of_overlaps(&self, placed_words: &[PlacedWord]) -> u8 {
        let mut overlaps = 0u8;
        for word in placed_words {
            if self.is_vertical ^ word.is_vertical {
                overlaps += self.overlaps(word) as u8;
            }
        }
        overlaps
    }
}

trait GetOverlaps {
    fn total_overlaps(&self) -> u8;
}

trait Shift {
    fn shift(self) -> Self;
}

impl GetOverlaps for Puzzle<'_> {
    /// returns the total number of times two words overlap
    fn total_overlaps(&self) -> u8 {
        let mut total_overlaps = 0;
        // filter based on orientation because
        // there is an equal number of vertical and
        // horizontal overlaps
        for word in self.iter().filter(|word| word.is_vertical) {
            total_overlaps += word.number_of_overlaps(self);
        }
        total_overlaps
    }
}

impl Shift for Puzzle<'_> {
    /// shifts puzzle so that all coordinates are >= 0
    fn shift(mut self) -> Self {
        let mut left_most = 0isize;
        let mut up_most = 0isize;

        for word in &self {
            let more_left = left_most > word.pos[0];
            let more_up = up_most > word.pos[1];
            left_most = word.pos[0] * more_left as isize
                + left_most * !more_left as isize;
            up_most =
                word.pos[1] * more_up as isize + up_most * !more_up as isize;
        }

        for word in &mut self {
            word.pos[0] -= left_most;
            word.pos[1] -= up_most;
        }
        self
    }
}

pub fn parse_words(all_words: &str) -> Option<Vec<Word>> {
    let mut formatted_words = Vec::<Word>::new();

    for word in all_words.lines() {
        let mut split_word = word.split('.');
        formatted_words.push(Word {
            word: split_word.next()?,
            clue: split_word.next()?,
        })
    }
    Some(formatted_words)
}

pub fn new_puzzle<'a>(
    word_list: &'a [Word],
    num_words: usize,
) -> Option<Puzzle<'a>> {
    let mut best_puzzle = None::<Puzzle>;
    let mut most_ovelaps = 0u8;

    for _ in 0..50000 {
        let words = get_random_words(word_list, num_words);
        match generate_layout(&words) {
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
    best_puzzle.map(|puzzle| puzzle.shift())
}

/// uses `rand` crate to pick `num_words` words
fn get_random_words<'a>(
    word_list: &'a [Word],
    num_words: usize,
) -> Vec<&'a Word<'a>> {
    let mut rng = rand::thread_rng();
    let random_indices =
        rand::seq::index::sample(&mut rng, word_list.len(), num_words);
    let mut random_words = Vec::<&'a Word>::new();
    random_words.reserve_exact(num_words);

    for index in random_indices {
        random_words.push(&word_list[index]);
    }
    random_words
}

/// attempts to create a crossword puzzle from `words`
fn generate_layout<'a>(words: &[&'a Word<'a>]) -> Option<Puzzle<'a>> {
    let mut placed_words: Puzzle = Vec::new();
    placed_words.reserve_exact(words.len());

    for word in words {
        placed_words.push(word.place(&placed_words)?);
    }
    Some(placed_words)
}

/// returns `true` if `next_word` has any "illegal" overlaps in `placed_words`
///
/// an overlap is considered "illegal" if the letters at the place of overlap
/// are not the same
fn illegal_overlap(
    next_word: &PlacedWord<'_>,
    placed_words: &[PlacedWord<'_>],
) -> bool {
    let mut illegal = false;
    for placed_word in placed_words {
        if placed_word.is_vertical ^ next_word.is_vertical {
            let (vertical_word, horizontal_word) = if next_word.is_vertical {
                (next_word, placed_word)
            } else {
                (placed_word, next_word)
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
            let is_vertical = next_word.is_vertical as usize;
            let is_horizontal = !next_word.is_vertical as usize;
            illegal = (next_word.pos[is_vertical]
                - placed_word.pos[is_vertical]
                < next_word.word.len() as isize
                || placed_word.pos[is_vertical] - next_word.pos[is_vertical]
                    < placed_word.word.len() as isize)
                && placed_word.pos[is_horizontal]
                    == next_word.pos[is_horizontal];
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

    const WORDS: &[Word<'_>] = &[
        Word {
            word: "cat",
            clue: "an animal of group cat",
        },
        Word {
            word: "tiger",
            clue: "a wild species of cat",
        },
        Word {
            word: "ought",
            clue: "should",
        },
        Word {
            word: "batter",
            clue: "hit repeatedly",
        },
    ];

    const PLACED_WORDS: &[PlacedWord<'_>] = &[
        PlacedWord {
            word: "cat",
            clue: "an animal of group cat",
            is_vertical: false,
            pos: [0, 0],
        },
        PlacedWord {
            word: "tiger",
            clue: "a wild species of cat",
            is_vertical: true,
            pos: [2, 0],
        },
        PlacedWord {
            word: "ought",
            clue: "should",
            is_vertical: false,
            pos: [0, 2],
        },
        PlacedWord {
            word: "batter",
            clue: "hit repeatedly",
            is_vertical: true,
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
    fn parse() {
        let unparsed = "cat.an animal of group cat
tiger.a wild species of cat
ought.should
batter.hit repeatedly";

        assert_eq!(WORDS, parse_words(unparsed).unwrap());
    }

    #[test]
    fn word_overlaps_other_word() {
        assert!(PLACED_WORDS[1].overlaps(&PLACED_WORDS[2]));
        assert!(!PLACED_WORDS[0].overlaps(&PLACED_WORDS[3]));
        assert!(!PLACED_WORDS[0].overlaps(&PLACED_WORDS[2]));
    }

    #[test]
    fn count_individual_word_overlaps() {
        assert_eq!(PLACED_WORDS[0].number_of_overlaps(PLACED_WORDS), 1);
        assert_eq!(PLACED_WORDS[1].number_of_overlaps(PLACED_WORDS), 2);
    }

    #[test]
    fn count_total_overlaps() {
        let words_as_vec = PLACED_WORDS.to_vec();
        assert_eq!(words_as_vec.total_overlaps(), 3);
    }

    #[test]
    fn illegal() {
        let vert_opposite_orientation_illegal: &PlacedWord<'_> = &PlacedWord {
            word: "assess",
            clue: "to determine information from",
            is_vertical: true,
            pos: [1, 0],
        };

        assert!(illegal_overlap(
            vert_opposite_orientation_illegal,
            PLACED_WORDS
        ));

        let vert_opposite_orientation_legal: &PlacedWord<'_> = &PlacedWord {
            word: "alumina",
            clue: "aluminium oxide",
            is_vertical: true,
            pos: [1, 0],
        };

        assert!(!illegal_overlap(
            vert_opposite_orientation_legal,
            PLACED_WORDS
        ));

        let hori_opposite_orientation_illegal: &PlacedWord<'_> = &PlacedWord {
            word: "bitter",
            clue: "having a sharp, pungent taste or smell",
            is_vertical: true,
            pos: [1, 0],
        };

        assert!(illegal_overlap(
            hori_opposite_orientation_illegal,
            PLACED_WORDS
        ));

        let hori_opposite_orientation_legal: &PlacedWord<'_> = &PlacedWord {
            word: "bit",
            clue: "small amount",
            is_vertical: false,
            pos: [1, 1],
        };

        assert!(!illegal_overlap(
            hori_opposite_orientation_legal,
            PLACED_WORDS
        ));

        let hori_same_orientation_illegal: &PlacedWord<'_> = &PlacedWord {
            word: "its",
            clue: "posessive case of it",
            is_vertical: false,
            pos: [3, 2],
        };

        assert!(illegal_overlap(hori_same_orientation_illegal, PLACED_WORDS));
    }

    #[test]
    fn calc_position() {
        let vertical: Word<'_> = Word {
            word: "crouch",
            clue: "kneel",
        };

        let vertical_placed: PlacedWord<'_> = PlacedWord {
            word: "crouch",
            clue: "kneel",
            is_vertical: true,
            pos: [0, 0],
        };
        assert_eq!(vertical.place(PLACED_WORDS), Some(vertical_placed));

        let no_possible_pos: Word<'_> = Word {
            word: "snaps",
            clue: "breaks",
        };
        assert_eq!(no_possible_pos.place(PLACED_WORDS), None);

        let horizontal: Word<'_> = Word {
            word: "better",
            clue: "superior",
        };
        let horizontal_placed: PlacedWord<'_> = PlacedWord {
            word: "better",
            clue: "superior",
            is_vertical: false,
            pos: [1, 3],
        };
        assert_eq!(horizontal.place(PLACED_WORDS), Some(horizontal_placed));
    }

    // #[test]
    // fn shift() {
    // }
}
