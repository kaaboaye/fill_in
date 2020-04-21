extern crate nalgebra;

mod crossword;
mod field;
mod field_character;

use crate::crossword::Crossword;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mat = Crossword::from_str(
        "____
___#
____
",
    )?;

    print!("{}", mat);

    println!("Hello, world!");

    Ok(())
}
