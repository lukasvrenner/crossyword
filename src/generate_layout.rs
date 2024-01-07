use rand::thread_rng;

const NUM_WORDS: usize = 10;

#[derive(Clone, Copy)]
pub struct Word<'a> {
    pub word: &'a str,
    pub clue: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct PlacedWord<'a> {
    pub word: &'a str,
    clue: &'a str,
    pub is_vertical: bool,
    pub pos: [isize; 2],
}

impl Word<'_> {
    fn place<'a>(
        &'a self, placed_words: &[PlacedWord<'a>]
        ) -> Option<PlacedWord<'a>> {
        for placed_word in placed_words {
            let is_vertical = !placed_word.is_vertical;

            for (index, letter) in self.word.char_indices() {
                let dependant_axis_pos = match placed_word.word.find(letter) {
                    Some(position) => {
                        position as isize + placed_word.pos[!is_vertical as usize]
                    },
                    None => continue,
                };

                let independant_axis_pos = {
                    placed_word.pos[is_vertical as usize]  - index as isize
                };

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

type Puzzle<'a> = Vec<PlacedWord<'a>>;

trait GetOverlaps {
    fn get_overlaps(&self) -> u8;
}

impl GetOverlaps for Puzzle<'_> {
    fn get_overlaps(&self) -> u8 {
        todo!();
    }
}

pub fn new_puzzle<'a>(word_list: &'a Vec<Word>)
-> Option<Puzzle<'a>> {
    let mut best_puzzle: Option<Puzzle> = None;
    let mut most_ovelaps: u8 = 0;

    for _ in 0..100 {
        let words = get_random_words(word_list);
        match generate_layout(words) {
            Some(puzzle) => {
                let overlaps = puzzle.get_overlaps();
                if overlaps > most_ovelaps {
                    most_ovelaps = overlaps;
                    best_puzzle = Some(puzzle);
                }
            },
            None => continue,
        }
    }
    best_puzzle
}

fn get_random_words<'a>(word_list: &'a Vec<Word>) -> Vec<&'a Word<'a>> {
    let mut rng = thread_rng();
    let random_indices = 
        rand::seq::index::sample(&mut rng, word_list.len(), NUM_WORDS);
    let mut random_words: Vec<&'a Word> = Vec::new();

    for index in random_indices {
        random_words.push(&word_list[index]);
    }
    random_words
}

fn generate_layout<'a>(word_list: Vec<&'a Word<'a>>)
-> Option<Puzzle<'a>> {
    let mut placed_words: Puzzle = Vec::new();
    for word in word_list {
        placed_words.push(word.place(&placed_words)?);
    }
    Some(placed_words)
}

fn illegal_overlap(
    next_word: &PlacedWord<'_>, placed_words: &[PlacedWord<'_>]
    ) -> bool {
    let mut illegal = false;

    for placed_word in placed_words {
        // if they're perpendicular
        if placed_word.is_vertical ^ next_word.is_vertical {

            let (vertical_word, horizontal_word) =
                if next_word.is_vertical {
                    (next_word, placed_word)
                } else {
                    (placed_word, next_word)
                };

            illegal = 
                vertical_word.pos[0] >= horizontal_word.pos[0] &&
                vertical_word.pos[0] - horizontal_word.pos[0]
                < horizontal_word.word.len() as isize
                &&
                horizontal_word.pos[1] >= vertical_word.pos[1] &&
                horizontal_word.pos[1] - vertical_word.pos[1]
                < vertical_word.word.len() as isize
                &&
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
            illegal = 
                (next_word.pos[is_vertical] - placed_word.pos[is_vertical]
                 < next_word.word.len() as isize
                 ||
                 placed_word.pos[is_vertical] - next_word.pos[is_vertical] 
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
        }

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
    fn illegal() {
        let vert_opposite_orientation_illegal: &PlacedWord<'_> = 
            &PlacedWord {
                word: "assess",
                clue: "to determine information from",
                is_vertical: true,
                pos: [1, 0],
            };
/*
 --- --- ---     ---
| c | a | t |   | b |
 --- --- ---     ---
    | s | i |   | a |
 --- --- --- --- ---
| o | ! | g | h | t |
 --- --- --- --- ---
    | e | e |   | t |
     --- ---     ---
    | s | r |   | e |
     --- ---     ---
    | s |       | r |
     ---         ---
*/
        assert!(illegal_overlap(vert_opposite_orientation_illegal, PLACED_WORDS));

        let vert_opposite_orientation_legal: &PlacedWord<'_> = 
            &PlacedWord {
                word: "alumina",
                clue: "aluminium oxide",
                is_vertical: true,
                pos: [1, 0],
            };
/*
 --- --- ---     ---
| c | a | t |   | b |
 --- --- ---     ---
    | l | i |   | a |
 --- --- --- --- ---
| o | u | g | h | t |
 --- --- --- --- ---
    | m | e |   | t |
     --- ---     ---
    | i | r |   | e |
     --- ---     ---
    | n |       | r |
     ---         --- 
    | a |
     ---
*/
        assert!(!illegal_overlap(vert_opposite_orientation_legal, PLACED_WORDS));

        let hori_opposite_orientation_illegal: &PlacedWord<'_> = 
            &PlacedWord {
                word: "bitter",
                clue: "having a sharp, pungent taste or smell",
                is_vertical: true,
                pos: [1, 0],
            };
/*
 --- --- ---     ---
| c | a | t |   | b |
 --- --- --- --- --- --- ---
    | b | i | t | ! | e | r |
 --- --- --- --- --- --- ---
| o | u | g | h | t |
 --- --- --- --- ---
        | e |   | t |
         ---     ---
        | r |   | e |
         ---     ---
                | r |
                 ---
*/
        assert!(illegal_overlap(hori_opposite_orientation_illegal, PLACED_WORDS));

        let hori_opposite_orientation_legal: &PlacedWord<'_> = 
            &PlacedWord {
                word: "bit",
                clue: "small amount",
                is_vertical: false,
                pos: [1, 1],
            };
/*
 --- --- ---     ---
| c | a | t |   | b |
 --- --- --- --- ---
    | b | i | t | a | 
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
        assert!(!illegal_overlap(hori_opposite_orientation_legal, PLACED_WORDS));

        let hori_same_orientation_illegal: &PlacedWord<'_> = 
            &PlacedWord {
                word: "its",
                clue: "posessive case of it",
                is_vertical: false,
                pos: [3, 2],
            };
/*
 --- --- ---     ---
| c | a | t |   | b |
 --- --- ---     ---
        | i |   | a | 
 --- --- --- --- --- ---
| o | u | g | ! | t | s |
 --- --- --- --- --- ---
        | e |   | t |
         ---     ---
        | r |   | e |
         ---     ---
                | r |
                 ---
*/
        assert!(illegal_overlap(hori_same_orientation_illegal, PLACED_WORDS));

    }

    #[test]
    fn calc_position() {
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
        let vertical: Word<'_> = 
            Word {
                word: "crouch",
                clue: "kneel",
            };

        let vertical_placed: PlacedWord<'_> = 
            PlacedWord {
                word: "crouch",
                clue: "kneel",
                is_vertical: true,
                pos: [0, 0],
            };
        assert_eq!(vertical.place(PLACED_WORDS), Some(vertical_placed));
/*
 --- --- ---     ---
| c | a | t |   | b |
 --- --- ---     ---
| r |   | i |   | a | 
 --- --- --- --- ---
| o | u | g | h | t |
 --- --- --- --- ---
| u |   | e |   | t |
 ---     ---     ---
| c |   | r |   | e |
 ---     ---     ---
| h |           | r |
 ---             ---
*/
        let no_possible_pos: Word<'_> = 
            Word {
                word: "snaps",
                clue: "breaks",
            };
        assert_eq!(no_possible_pos.place(PLACED_WORDS), None);

        let horizontal: Word<'_> = 
            Word {
                word: "better",
                clue: "superior",
            };
        let horizontal_placed: PlacedWord<'_> = 
            PlacedWord {
                word: "better",
                clue: "superior",
                is_vertical: false,
                pos: [1, 3],
            };
        assert_eq!(horizontal.place(PLACED_WORDS), Some(horizontal_placed));
/*
 --- --- ---     ---
| c | a | t |   | b |
 --- --- ---     ---
        | i |   | a | 
 --- --- --- --- ---
| o | u | g | h | t |
 --- --- --- --- --- --- ---
    | b | e | t | t | e | r |
     --- --- --- --- --- ---
        | r |   | e |
         ---     ---
                | r |
                 ---
*/
    }
}
