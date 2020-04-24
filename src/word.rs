use crate::character::Character;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Index;

#[derive(Clone, Hash, Ord, PartialOrd)]
pub struct Word {
  data: Vec<Character>,
}

impl Word {
  pub fn new(string: &mut String) -> Word {
    string.make_ascii_uppercase();
    let data = string.chars().map(|c| c.into()).collect();

    Word { data }
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }
}

impl Display for Word {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    for character in self.data.iter() {
      write!(f, "{}", character)?;
    }

    Ok(())
  }
}

impl Debug for Word {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", self)
  }
}

impl Index<usize> for Word {
  type Output = Character;

  fn index(&self, idx: usize) -> &<Self as std::ops::Index<usize>>::Output {
    &self.data[idx]
  }
}

impl PartialEq for Word {
  fn eq(&self, other: &Self) -> bool {
    self.data == other.data
  }
}

impl Eq for Word {}
