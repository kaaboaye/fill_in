use crate::dense_map::DenseMap;
use crate::field::Field;
use crate::word::Word;
use crate::words::Words;
use nalgebra::DMatrix;
use nalgebra::RowDVector;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct WordField {
  position: (usize, usize),
  size: (usize, usize),
  selected_word: Option<Word>,
}

impl WordField {
  fn len(&self) -> usize {
    let (x, y) = self.size;
    x + y - 1
  }
}

type Board = DMatrix<Field>;

pub struct Crossword {
  board: Board,
}

impl Crossword {
  pub fn solve(&mut self, words: &mut Words) {
    let word_fields = extract_word_fields(&self.board);
    let mut word_fields = filter_out_fields_without_possible_words(word_fields, words);

    restrict_board(&mut self.board, &word_fields, words).unwrap();

    solve(&mut self.board, &mut word_fields, words);
  }
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
    .filter(|word_field| words.words_with_length(word_field.len()).is_some())
    .cloned()
    .collect()
}

fn restrict_board(
  board: &mut Board,
  word_fields: &Vec<WordField>,
  words: &Words,
) -> Result<(), ()> {
  loop {
    let before_restrictions = board.clone();

    for word_field in word_fields.iter() {
      words
        .words_with_length(word_field.len())
        .map(|possible_words| {
          for (idx, field) in board
            .slice_mut(word_field.position, word_field.size)
            .iter_mut()
            .enumerate()
          {
            for possible_word in possible_words {
              field.insert(possible_word[idx]);
            }
          }
        });
    }

    for WordField {
      position,
      size,
      selected_word,
    } in word_fields.iter().filter(|f| f.selected_word.is_some())
    {
      let selected_word = selected_word.as_ref().unwrap();

      for (idx, field) in board.slice_mut(*position, *size).iter_mut().enumerate() {
        if field.contains(selected_word[idx]) {
          (*field) = Field::new_empty();
          field.insert(selected_word[idx])
        } else {
          return Err(());
        }
      }
    }

    if before_restrictions == (*board) {
      return Ok(());
    }
  }
}

fn solve(board: &mut Board, word_fields: &mut Vec<WordField>, words: &mut Words) {
  let mut skip_fields = 0usize;
  let mut skip_words_map = DenseMap::<usize>::new_with_max_key(words.longest_size(), || 0);

  loop {
    let field_candidate = word_fields
      .iter_mut()
      .enumerate()
      .skip(skip_fields)
      .find(|(_idx, field)| field.selected_word.is_none());

    if let Some((candidate_idx, field_candidate)) = field_candidate {
      let word_size = field_candidate.len();

      let could_not_select_word = words
        .words_with_length_mut(word_size)
        .and_then(|possible_words| {
          let word_candidate = possible_words
            .iter()
            .skip(skip_words_map[word_size])
            .next()?
            .clone();

          possible_words.remove(&word_candidate);
          field_candidate.selected_word = Some(word_candidate);

          Some(())
        })
        .is_none();

      if could_not_select_word {
        skip_fields += 1;
        skip_words_map[word_size] = 0;
        continue;
      }

      let mut board_candidate = board.clone();

      match restrict_board(&mut board_candidate, word_fields, words) {
        Err(()) => {
          skip_words_map[word_size] += 1;
          let field_candidate = &mut word_fields[candidate_idx];
          words.return_word(field_candidate.selected_word.as_ref().unwrap().clone());
          field_candidate.selected_word = None;
        }
        Ok(()) => {
          (*board) = board_candidate;
          skip_fields = 0;
          skip_words_map[word_size] = 0;
        }
      }

      continue;
    }

    let puzzle_not_solved = word_fields.iter().any(|word_field| {
      board
        .slice(word_field.position, word_field.size)
        .iter()
        .any(|field| field.len() != 1)
    });

    assert_eq!(puzzle_not_solved, false);

    return;
  }
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
