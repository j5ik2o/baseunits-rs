use std::collections::hash_set::{Iter, IntoIter};
use std::collections::HashSet;
use std::hash::Hash;

use rust_fp_categories::empty::Empty;
use rust_fp_categories::semigroup::Semigroup;

use crate::money::Allotment;
use rust_fp_categories::monoid::Monoid;

#[derive(Debug, Clone)]
pub struct Allotments<T>(pub(crate) HashSet<Allotment<T>>);

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

  pub fn negated(self) -> Self {
    let values = self
      .0
      .into_iter()
      .map(|e| e.negated())
      .collect::<HashSet<_>>();
    Self::new(values)
  }

  pub fn filter<F>(self, f: F) -> Self
  where
    F: Fn(&Allotment<T>) -> bool,
  {
    let values = self.0.into_iter().filter(|e| f(e)).collect::<HashSet<_>>();
    Self(values)
  }

  pub fn find<F>(self, f: F) -> Option<Allotment<T>>
  where
    F: Fn(&Allotment<T>) -> bool,
  {
    self.0.into_iter().find(|e| f(e))
  }

  pub fn iter(&self) -> Iter<Allotment<T>> {
    self.0.iter()
  }
}

impl<T> std::iter::IntoIterator for Allotments<T> {
  type Item = Allotment<T>;
  type IntoIter = IntoIter<Allotment<T>>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

#[cfg(test)]
mod tests {}
