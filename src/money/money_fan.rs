use crate::money::{Allotments, Allotment};
use rust_fp_categories::empty::Empty;
use std::collections::hash_set::Iter;
use std::hash::Hash;
use rust_fp_categories::semigroup::Semigroup;
use std::collections::HashSet;

#[derive(Debug, Clone)]
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
  pub fn new(values: Allotments<T>) -> Self {
    Self(values)
  }

  pub fn iter(&self) -> Iter<Allotment<T>> {
    self.0.iter()
  }

  pub fn allotment(&self, an_entity: T) -> Option<Allotment<T>> {
    self.0.find(|e| e.entity == an_entity)
  }

  pub fn subtract(&self, subtracted: MoneyFan<T>) -> MoneyFan<T> {
    self.add(subtracted.negated())
  }

  pub fn negated(&self) -> MoneyFan<T> {
    MoneyFan::new(self.0.negated())
  }

  pub fn add(&self, added: MoneyFan<T>) -> MoneyFan<T> {
    let all_entities = self
      .0
      .clone()
      .combine(added.0.clone())
      .iter()
      .cloned()
      .map(|e| e.entity)
      .collect::<HashSet<_>>();
    let summed_allotments = all_entities
      .iter()
      .map(|entity| match self.allotment(entity.clone()) {
        Option::None => added.allotment(entity.clone()).unwrap(),
        Option::Some(this_allotment) => match added.allotment(entity.clone()) {
          Option::None => this_allotment,
          Option::Some(added_allotment) => Allotment::new(
            entity.clone(),
            this_allotment.amount + added_allotment.amount,
          ),
        },
      })
      .collect::<HashSet<_>>();
    MoneyFan::new(Allotments::new(summed_allotments))
  }
}

#[cfg(test)]
mod tests {

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
