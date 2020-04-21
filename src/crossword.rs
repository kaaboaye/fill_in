use crate::field::Field;
use crate::field_character::FieldCharacter;
use nalgebra::DMatrix;
use std::fmt::Display;
use std::str::FromStr;

pub struct Crossword {
  board: DMatrix<Field>,
}

impl Crossword {}

impl FromStr for Crossword {
  type Err = Box<dyn std::error::Error>;

  fn from_str(raw: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
    dbg!(raw);

    let mut raw = String::from(raw);
    raw.make_ascii_uppercase();

    // freeze
    let raw = raw;

    let cols = raw.lines().next().unwrap().len();

    let fields: Vec<Field> = raw
      .lines()
      .flat_map(|line| line.chars().map(|c| -> Field { c.into() }))
      .collect();

    let board = DMatrix::<Field>::from_vec(fields.len() / cols, cols, fields);

    Ok(Crossword { board })
  }
}

impl Display for Crossword {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", self.board)
  }
}
