use core::ops::Add;
use crate::money::{Allotments, Allotment};
use rust_fp_categories::empty::Empty;
use std::{hash::Hash};
use rust_fp_categories::semigroup::Semigroup;
use std::collections::hash_set::{Iter, IntoIter};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct MoneyFan<T>(pub Allotments<T>);

impl<T> Empty for MoneyFan<T> {
  fn empty() -> Self {
    Self(Allotments::empty())
  }

  fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
}

impl<T> std::iter::IntoIterator for MoneyFan<T> {
  type Item = Allotment<T>;
  type IntoIter = IntoIter<Allotment<T>>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<T: Clone + Eq + Hash> Add for MoneyFan<T> {
  type Output = Self;

  fn add(self, added: MoneyFan<T>) -> MoneyFan<T> {
    let all_entities = self
      .clone()
      .0
      .combine(added.clone().0)
      .into_iter()
      .map(|e| e.entity)
      .collect::<HashSet<_>>();
    let summed_allotments = all_entities
      .into_iter()
      .map(|entity| {
        let allotment = self.clone().allotment(entity.clone());
        match allotment {
          Option::None => added.clone().allotment(entity.clone()).unwrap(),
          Option::Some(this_allotment) => match added.clone().allotment(entity.clone()) {
            Option::None => this_allotment,
            Option::Some(added_allotment) => Allotment::new(
              entity.clone(),
              this_allotment.amount + added_allotment.amount,
            ),
          },
        }
      })
      .filter(|e| !e.amount.is_zero())
      .collect::<HashSet<_>>();
    MoneyFan::new(Allotments::new(summed_allotments))
  }
}

impl<T: Clone + Eq + Hash> MoneyFan<T> {
  pub fn new(values: Allotments<T>) -> Self {
    Self(values)
  }

  pub fn iter(&self) -> Iter<Allotment<T>> {
    self.0.iter()
  }

  pub fn allotment(self, an_entity: T) -> Option<Allotment<T>> {
    self.0.find(|e| e.entity == an_entity)
  }

  pub fn subtract(self, subtracted: MoneyFan<T>) -> MoneyFan<T> {
    self.add(subtracted.negated())
  }

  pub fn negated(self) -> MoneyFan<T> {
    MoneyFan::new(self.0.negated())
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
