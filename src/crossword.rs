use crate::field::Field;
use crate::word::Word;
use crate::words::Words;
use nalgebra::DMatrix;
use nalgebra::RowDVector;
use std::fmt::Display;
use std::str::FromStr;

type Board = DMatrix<Field>;

pub struct Crossword {
  board: Board,
}

impl Crossword {
  pub fn solve(&mut self, words: &mut Words) {
    let word_fields = extract_word_fields(&self.board);
    let mut word_fields = filter_out_fields_without_possible_words(word_fields, words);

    println!("{:?}", word_fields);

    initialize_board(&mut self.board, &word_fields, words);

    solve(&mut self.board, &mut word_fields, words);
  }
}

#[derive(Clone, Debug)]
struct WordField {
  position: (usize, usize),
  size: (usize, usize),
  selected_word: Option<Word>,
}

fn extract_word_fields(board: &Board) -> Vec<WordField> {
  let mut word_fields = Vec::new();

  // find words in rows
  for (row_idx, row) in board.row_iter().enumerate() {
    let mut position: Option<(usize, usize)> = None;
    let mut size: Option<(usize, usize)> = None;

    for (col_idx, field) in row.iter().enumerate() {
      if field.is_blocker() {
        if let Some(some_position) = &position {
          let some_size = size.as_ref().unwrap();

          word_fields.push(WordField {
            position: *some_position,
            size: *some_size,
            selected_word: None,
          });

          position = None;
          size = None;
        } else {
          continue;
        }
      } else {
        if let Some((_, col_size)) = &mut size {
          (*col_size) += 1;
        } else {
          position = Some((row_idx, col_idx));
          size = Some((1, 1));
        }
      }
    }

    if let Some(position) = position {
      let size = size.unwrap();

      word_fields.push(WordField {
        position,
        size,
        selected_word: None,
      })
    }
  }

  // find words in cols
  for (col_idx, col) in board.column_iter().enumerate() {
    let mut position: Option<(usize, usize)> = None;
    let mut size: Option<(usize, usize)> = None;

    for (row_idx, field) in col.iter().enumerate() {
      if field.is_blocker() {
        if let Some(some_position) = &position {
          let some_size = size.as_ref().unwrap();

          word_fields.push(WordField {
            position: *some_position,
            size: *some_size,
            selected_word: None,
          });

          position = None;
          size = None;
        } else {
          continue;
        }
      } else {
        if let Some((row_size, _)) = &mut size {
          (*row_size) += 1;
        } else {
          position = Some((row_idx, col_idx));
          size = Some((1, 1));
        }
      }
    }

    if let Some(position) = position {
      let size = size.unwrap();

      word_fields.push(WordField {
        position,
        size,
        selected_word: None,
      })
    }
  }

  word_fields
}

fn filter_out_fields_without_possible_words(
  word_fields: Vec<WordField>,
  words: &Words,
) -> Vec<WordField> {
  word_fields
    .iter()
    .filter(|word_field| {
      words
        .words_with_length(len_from_size(&word_field.size))
        .is_some()
    })
    .cloned()
    .collect()
}

fn initialize_board(board: &mut Board, word_fields: &Vec<WordField>, words: &Words) {
  for WordField {
    position,
    size,
    selected_word: _,
  } in word_fields.iter()
  {
    if let Some(possible_words) = words.words_with_length(len_from_size(size)) {
      for possible_word in possible_words {
        for (idx, field) in board.slice_mut(*position, *size).iter_mut().enumerate() {
          field.insert(possible_word[idx]);
        }
      }
    }
  }
}

fn solve(board: &mut Board, word_fields: &mut Vec<WordField>, words: &mut Words) {
  let mut skip_fields = 0usize;
  let mut skip_words = 0usize;

  loop {}
}

fn len_from_size((x, y): &(usize, usize)) -> usize {
  *x + *y - 1
}

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
