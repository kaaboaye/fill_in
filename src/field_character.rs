use std::fmt::Debug;
use std::fmt::Display;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct FieldCharacter {
  pub value: u8,
}

impl From<char> for FieldCharacter {
  fn from(character: char) -> Self {
    debug_assert!((('A' as u32)..('Z' as u32 + 1)).contains(&(character as u32)));

    let value = (character as u32 - 'A' as u32) as u8;
    FieldCharacter { value }
  }
}

impl From<u8> for FieldCharacter {
  fn from(character: u8) -> Self {
    debug_assert!(character as u32 <= ('Z' as u32 - 'A' as u32));

    FieldCharacter { value: character }
  }
}

impl Into<char> for FieldCharacter {
  fn into(self) -> char {
    (self.value + 'A' as u8) as char
  }
}

impl Display for FieldCharacter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    let character: char = (*self).into();
    write!(f, "{}", character)
  }
}

impl Debug for FieldCharacter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", self)
  }
}
