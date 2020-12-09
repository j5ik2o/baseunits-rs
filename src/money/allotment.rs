use crate::money::Money;
use std::ops::Neg;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Allotment<T> {
  pub entity: T,
  pub amount: Money,
}

impl<T> Neg for Allotment<T> {
  type Output = Allotment<T>;

  fn neg(self) -> Self::Output {
    Allotment::negated(self)
  }
}

impl<T> Allotment<T> {
  pub fn new(entity: T, amount: Money) -> Self {
    Self { entity, amount }
  }

  pub fn negated(self) -> Self {
    Self::new(self.entity, self.amount.negated())
  }
}

#[cfg(test)]
mod tests {

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
