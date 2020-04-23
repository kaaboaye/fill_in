use crate::field::Field;
use nalgebra::DMatrix;
use nalgebra::RowDVector;
use std::fmt::Display;
use std::str::FromStr;

pub struct Crossword {
  board: DMatrix<Field>,
}

impl Crossword {}

impl FromStr for Crossword {
  type Err = Box<dyn std::error::Error>;

  fn from_str(raw: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
    let rows: Vec<RowDVector<Field>> = raw
      .lines()
      .map(|line| {
        let vec = line.chars().map(|c| -> Field { c.into() }).collect();
        RowDVector::<Field>::from_vec(vec)
      })
      .collect();

    let board = DMatrix::<Field>::from_rows(rows.as_slice());

    Ok(Crossword { board })
  }
}

impl Display for Crossword {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", self.board)
  }
}
