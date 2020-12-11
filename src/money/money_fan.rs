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
      .filter(|e| e.amount.is_zero() == false)
      .collect::<HashSet<_>>();
    MoneyFan::new(Allotments::new(summed_allotments))
  }
}

#[cfg(test)]
mod tests {
  use crate::money::{Money, Allotment, MoneyFan, Allotments};
  use std::collections::HashSet;

  #[test]
  fn test01_roommate_example() {
    let a = Allotment::new("Joe", Money::dollars_f32(65.00));
    let mut h = HashSet::new();
    h.insert(a);
    let _electric_bill = MoneyFan::new(Allotments::new(h));

    let mut h = HashSet::new();
    h.insert(Allotment::new("Mary", Money::dollars_i32(650)));
    h.insert(Allotment::new("Jill", Money::dollars_i32(650)));
    h.insert(Allotment::new("Joe", Money::dollars_i32(650)));
    let _rent = MoneyFan::new(Allotments::new(h));

    // TODO
  }
}
