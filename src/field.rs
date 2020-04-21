use crate::field_character::FieldCharacter;
use std::fmt::Debug;
use std::fmt::Display;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Field {
  data: u32,
}

impl Field {
  pub fn new() -> Field {
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
    T: Into<FieldCharacter>,
  {
    self.data |= 1 << character.into().value;
  }

  pub fn contains<T>(&self, character: T) -> bool
  where
    T: Into<FieldCharacter>,
  {
    (self.data & (1 << character.into().value)) != 0
  }

  pub fn remove<T>(&mut self, character: T)
  where
    T: Into<FieldCharacter>,
  {
    self.data &= !(1 << character.into().value);
  }

  pub fn next(&self) -> FieldCharacter {
    let value = self.data.trailing_zeros() as u8;
    FieldCharacter { value }
  }

  pub fn iter(&self) -> FieldIter {
    FieldIter { field: *self }
  }
}

pub struct FieldIter {
  field: Field,
}

impl Iterator for FieldIter {
  type Item = FieldCharacter;

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
  type Item = FieldCharacter;
  type IntoIter = FieldIter;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl Debug for Field {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    if self.len() == -1 {
      return write!(f, "#");
    }

    let values: Vec<_> = self.iter().collect();

    if values.len() == 1 {
      write!(f, "{}", values[0])
    } else {
      write!(f, "{:?}", &values)
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
    let field = Field::new();
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
    let mut field = Field::new();
    field.insert('D');

    assert_eq!(field.len(), 1);
  }

  #[test]
  fn multiple_inserts_of_the_same_character() {
    let mut field = Field::new();
    field.insert('D');
    field.insert('D');
    field.insert('D');

    assert_eq!(field.len(), 1);
  }

  #[test]
  fn multiple_inserts_of_different_characters() {
    let mut field = Field::new();
    field.insert('A');
    field.insert('T');
    field.insert('Z');

    assert_eq!(field.len(), 3);
  }

  #[test]
  fn contains_test_true() {
    let mut field = Field::new();
    field.insert('D');
    assert!(field.contains('D'));
  }

  #[test]
  fn contains_test_false() {
    let mut field = Field::new();
    field.insert('D');
    assert!(!field.contains('C'));
  }

  #[test]
  fn test_remove() {
    let mut field = Field::new();
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
    let mut field = Field::new();
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
