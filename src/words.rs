use crate::word::Word;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Words {
  /// Stores words indexed by length
  ///
  /// Example
  ///
  /// 0 -
  /// 1 - a
  /// 2 - by, at
  /// 3 - cat, the, etc
  data: Vec<Vec<Word>>,
}

impl Words {
  pub fn new(string: String) -> Words {
    let mut words: Vec<Vec<Word>> = Vec::new();

    string
      .lines()
      .map(String::from)
      .collect::<Vec<_>>()
      .iter_mut()
      .for_each(|word| {
        let word_size = word.len();

        if word_size + 1 >= words.len() {
          words.resize_with(word_size + 1, Vec::new);
        }

        let bucket = &mut words[word_size];
        bucket.push(Word::new(word));
      });

    Words { data: words }
  }
}

impl Debug for Words {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    for (idx, bucket) in self.data.iter().enumerate() {
      if bucket.len() == 0 {
        continue;
      }

      write!(f, "{:3} --> {:?}\n", idx, bucket)?;
    }

    Ok(())
  }
}
