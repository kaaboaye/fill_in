use std::ops::Index;
use std::ops::IndexMut;

#[derive(Clone, Debug)]
pub struct DenseMap<T> {
  data: Vec<T>,
}

impl<T> DenseMap<T> {
  pub fn new() -> DenseMap<T> {
    DenseMap { data: Vec::new() }
  }

  pub fn new_with_max_key<F>(max_key: usize, default: F) -> DenseMap<T>
  where
    F: FnMut() -> T,
  {
    let mut data = Vec::with_capacity(max_key + 1);
    data.resize_with(max_key + 1, default);

    DenseMap { data }
  }

  pub fn set_max_key<F>(&mut self, max_key: usize, default: F)
  where
    F: FnMut() -> T,
  {
    if max_key + 1 >= self.data.len() {
      self.data.resize_with(max_key + 1, default);
    }
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn iter(&self) -> std::slice::Iter<T> {
    self.data.iter()
  }

  pub fn max_key(&self) -> usize {
    self.data.len() - 1
  }
}

impl<T> Index<usize> for DenseMap<T> {
  type Output = T;

  fn index(&self, idx: usize) -> &<Self as std::ops::Index<usize>>::Output {
    &self.data[idx]
  }
}

impl<T> IndexMut<usize> for DenseMap<T> {
  fn index_mut(&mut self, idx: usize) -> &mut <Self as std::ops::Index<usize>>::Output {
    &mut self.data[idx]
  }
}
