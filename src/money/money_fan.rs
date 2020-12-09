use crate::money::{Allotments, Allotment};
use rust_fp_categories::empty::Empty;
use std::collections::hash_set::Iter;
use std::hash::Hash;

pub struct MoneyFan<T>(Allotments<T>);

impl<T> Empty for MoneyFan<T> {
  fn empty() -> Self {
    Self(Allotments::empty())
  }

  fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
}

impl<T: Clone + Eq + Hash> MoneyFan<T> {
  pub fn iter(&self) -> Iter<Allotment<T>> {
    self.0.iter()
  }

  pub fn alltoment(self, an_entity: T) -> Option<Allotment<T>> {
    self.0.find(|e| e.entity == an_entity)
  }
}

#[cfg(test)]
mod tests {

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
