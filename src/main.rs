extern crate nalgebra;

mod character;
mod crossword;
mod field;
mod word;
mod words;

use crate::crossword::Crossword;
use crate::words::Words;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_number = std::env::args().nth(1).unwrap();

    let puzzle = read_file(format!("assets/puzzle{}", puzzle_number))?;
    let mut crossword = Crossword::from_str(puzzle.as_str())?;

    print!("Crossword\n{}", &crossword);

    let words = read_file(format!("assets/words{}", puzzle_number))?;
    let mut words = Words::new(words);

    println!("Words\n{:?}", &words);

    crossword.solve(&mut words);

    print!("Solved crossword\n{}", &crossword);

    Ok(())
}

fn read_file(name: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(name)?;
    let mut result = String::new();

    file.read_to_string(&mut result)?;

    Ok(result)
}
