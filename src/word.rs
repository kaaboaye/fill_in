use crate::field_character::FieldCharacter;
use std::fmt::Debug;
use std::fmt::Display;

#[derive(Clone)]
pub struct Word {
  data: Vec<FieldCharacter>,
}

impl Word {
  pub fn new(string: &mut String) -> Word {
    string.make_ascii_uppercase();
    let data = string.chars().map(|c| c.into()).collect();

    Word { data }
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
