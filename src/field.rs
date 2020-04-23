use crate::character::Character;
use std::fmt::Debug;
use std::fmt::Display;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Field {
  data: u32,
}

impl Field {
  pub fn new_empty() -> Field {
    Field {
      data: 0b10000000000000000000000000000000,
    }
  }

  pub fn new_blocker() -> Field {
    Field { data: 0 }
  }

  pub fn is_blocker(&self) -> bool {
    self.data == 0
  }

  pub fn new_any_character() -> Field {
    // the first bit says that it isn't blocker
    // each 1 represents possible letter in given field
    Field {
      data: 0b10000011111111111111111111111111,
    }
  }

  /// Returns a number of possible characters
  /// Returns -1 if it's a blocker
  pub fn len(&self) -> isize {
    (self.data.count_ones() as isize) - 1
  }

  pub fn insert<T>(&mut self, character: T)
  where
    T: Into<Character>,
  {
    self.data |= 1 << character.into().value;
  }

  pub fn contains<T>(&self, character: T) -> bool
  where
    T: Into<Character>,
  {
    (self.data & (1 << character.into().value)) != 0
  }

  pub fn remove<T>(&mut self, character: T)
  where
    T: Into<Character>,
  {
    self.data &= !(1 << character.into().value);
  }

  pub fn next(&self) -> Character {
    let value = self.data.trailing_zeros() as u8;
    Character { value }
  }

  pub fn iter(&self) -> FieldIter {
    FieldIter { field: *self }
  }
}

impl From<char> for Field {
  fn from(character: char) -> Self {
    match character {
      '_' => Field::new_empty(),
      '#' => Field::new_blocker(),
      character => {
        let mut field = Field::new_empty();
        field.insert(character);

        field
      }
    }
  }
}

pub struct FieldIter {
  field: Field,
}

impl Iterator for FieldIter {
  type Item = Character;

  fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
    if self.field.len() <= 0 {
      return None;
    }

    let next_value = self.field.next();
    self.field.remove(next_value);
    Some(next_value)
  }
}

impl IntoIterator for Field {
  type Item = Character;
  type IntoIter = FieldIter;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl Debug for Field {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    match self.len() {
      -1 => write!(f, "#"),
      0 => write!(f, "∅"),
      1 => write!(f, "{}", self.next()),
      26 => write!(f, "_"),
      _ => {
        let values: Vec<_> = self.iter().collect();
        write!(f, "{:?}", &values)
      }
    }
  }
}

impl Display for Field {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{:?}", &self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::hash_set::HashSet;

  #[test]
  fn blocker_constructor() {
    let field = Field::new_blocker();

    assert_eq!(field.data, 0);
  }

  #[test]
  fn is_blocker_true() {
    let field = Field::new_blocker();

    assert!(field.is_blocker());
  }

  #[test]
  fn is_blocker_false() {
    let field = Field::new_any_character();

    assert!(!field.is_blocker());
  }

  #[test]
  fn len_empty() {
    let field = Field::new_empty();
    assert_eq!(field.len(), 0);
  }

  #[test]
  fn len_blocker() {
    let field = Field::new_blocker();
    assert_eq!(field.len(), -1);
  }

  #[test]
  fn len_any_character() {
    let field = Field::new_any_character();
    assert_eq!(field.len(), 26);
  }

  #[test]
  fn simple_insert() {
    let mut field = Field::new_empty();
    field.insert('D');

    assert_eq!(field.len(), 1);
  }

  #[test]
  fn multiple_inserts_of_the_same_character() {
    let mut field = Field::new_empty();
    field.insert('D');
    field.insert('D');
    field.insert('D');

    assert_eq!(field.len(), 1);
  }

  #[test]
  fn multiple_inserts_of_different_characters() {
    let mut field = Field::new_empty();
    field.insert('A');
    field.insert('T');
    field.insert('Z');

    assert_eq!(field.len(), 3);
  }

  #[test]
  fn contains_test_true() {
    let mut field = Field::new_empty();
    field.insert('D');
    assert!(field.contains('D'));
  }

  #[test]
  fn contains_test_false() {
    let mut field = Field::new_empty();
    field.insert('D');
    assert!(!field.contains('C'));
  }

  #[test]
  fn test_remove() {
    let mut field = Field::new_empty();
    field.insert('A');
    field.insert('T');
    field.insert('Z');

    assert_eq!(field.len(), 3);

    field.remove('X');
    assert_eq!(field.len(), 3);

    field.remove('T');
    assert_eq!(field.len(), 2);

    field.remove('A');
    assert_eq!(field.len(), 1);

    field.remove('Z');
    assert_eq!(field.len(), 0);
  }

  #[test]
  fn iter_test() {
    let mut field = Field::new_empty();
    field.insert('A');
    field.insert('T');
    field.insert('Z');

    let set = field
      .into_iter()
      .map(|fc| -> char { fc.into() })
      .collect::<HashSet<char>>();

    assert!(set.contains(&'A'));
    assert!(set.contains(&'T'));
    assert!(set.contains(&'Z'));
  }
}
