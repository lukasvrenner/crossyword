/*
crossyword, an awsome crossword puzzle generator
Copyright (C) 2024  Lukas Renner

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use std::fs;

mod generate_layout;
use crate::generate_layout::*;

fn main() {
    let words_file = "./words.txt";

    // note: display error to window once GUI is created
    let unformatted_words = fs::read_to_string(words_file)
        .unwrap_or_else(|err| {
            eprintln!("could not read {}: {}", words_file, err);
            std::process::exit(1);
    });

    // note: display error to window once GUI is created
    let formatted_words = format_words(&unformatted_words)
        .unwrap_or_else(||{
            eprintln!("could not parse {}", words_file);
            std::process::exit(1);
        });

    let puzzle = new_puzzle(&formatted_words).unwrap();
    println!("{:?}", puzzle);

}

