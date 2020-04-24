use crate::dense_map::DenseMap;
use crate::word::Word;
use std::collections::BTreeSet;
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
  data: DenseMap<BTreeSet<Word>>,
}

impl Words {
  pub fn new(string: String) -> Words {
    let mut words: DenseMap<BTreeSet<Word>> = DenseMap::new();

    string
      .lines()
      .map(String::from)
      .collect::<Vec<_>>()
      .iter_mut()
      .for_each(|word| {
        words.set_max_key(word.len(), BTreeSet::new);

        let bucket = &mut words[word.len()];
        bucket.insert(Word::new(word));
      });

    Words { data: words }
  }

  pub fn words_with_length(&self, length: usize) -> Option<&BTreeSet<Word>> {
    if self.data.len() < length {
      return None;
    }

    let bucket = &self.data[length];

    if bucket.len() == 0 {
      return None;
    }

    Some(bucket)
  }

  pub fn words_with_length_mut(&mut self, length: usize) -> Option<&mut BTreeSet<Word>> {
    if self.data.len() < length {
      return None;
    }

    let bucket = &mut self.data[length];

    if bucket.len() == 0 {
      return None;
    }

    Some(bucket)
  }

  pub fn return_word(&mut self, word: Word) {
    let bucket = &mut self.data[word.len()];
    bucket.insert(word);
  }

  pub fn longest_size(&self) -> usize {
    self.data.max_key()
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
