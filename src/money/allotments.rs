use std::collections::hash_set::Iter;
use std::collections::HashSet;
use std::hash::Hash;

use rust_fp_categories::empty::Empty;
use rust_fp_categories::semigroup::Semigroup;

use crate::money::Allotment;
use rust_fp_categories::monoid::Monoid;

#[derive(Debug, Clone)]
pub struct Allotments<T>(HashSet<Allotment<T>>);

impl<T> Empty for Allotments<T> {
  fn empty() -> Self {
    Allotments(HashSet::new())
  }

  fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
}

impl<T: Clone + Eq + Hash> Semigroup for Allotments<T> {
  fn combine(self, other: Self) -> Self {
    let mut s = self.0;
    for e in other.0 {
      s.insert(e);
    }
    Allotments::new(s)
  }
}

impl<T: Clone + Eq + Hash> Monoid for Allotments<T> {}

impl<T: Clone + Eq + Hash> Allotments<T> {
  pub fn new(values: HashSet<Allotment<T>>) -> Self {
    Self(values)
  }

  pub fn negated(&self) -> Self {
    let values = self
      .0
      .iter()
      .cloned()
      .map(|e| e.negated())
      .collect::<HashSet<_>>();
    Self::new(values)
  }

  pub fn filter<F>(&self, f: F) -> Self
  where
    F: Fn(&Allotment<T>) -> bool,
  {
    let values = self
      .0
      .iter()
      .filter(|e| f(*e))
      .cloned()
      .collect::<HashSet<_>>();
    Self(values)
  }

  pub fn find<F>(&self, f: F) -> Option<Allotment<T>>
  where
    F: Fn(&Allotment<T>) -> bool,
  {
    self.0.iter().find(|e| f(*e)).cloned()
  }

  pub fn iter(&self) -> Iter<Allotment<T>> {
    self.0.iter()
  }
}

#[cfg(test)]
mod tests {

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
