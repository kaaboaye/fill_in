use std::fmt::Debug;
use std::fmt::Display;

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

impl Display for FieldCharacter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", self.value as char)
  }
}

impl Debug for FieldCharacter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}", self)
  }
}
