use crate::dense_map::DenseMap;

#[derive(Clone, Debug)]
pub struct Metrics {
  pub iterations: u32,
  pub fields_skipped: u32,
  pub words_selected: u32,
  pub words_skipped: DenseMap<u32>,
}

impl Metrics {
  pub fn new(max_word_length: usize) -> Metrics {
    Metrics {
      iterations: 0,
      fields_skipped: 0,
      words_selected: 0,
      words_skipped: DenseMap::new_with_max_key(max_word_length, || 0),
    }
  }
}
