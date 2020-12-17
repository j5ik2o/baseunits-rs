use std::ops::Div;
use rust_decimal::{Decimal, RoundingStrategy};
use rust_decimal::prelude::Zero;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Ratio {
  numerator: Decimal,
  denominator: Decimal,
}

impl Ratio {
  pub fn new_i64(numerator: i64, denominator: i64) -> Self {
    Self::new(Decimal::from(numerator), Decimal::from(denominator))
  }
  pub fn new(numerator: Decimal, denominator: Decimal) -> Self {
    if denominator.is_zero() {
      panic!("denominator is zero");
    }
    Self {
      numerator,
      denominator,
    }
  }

  pub fn decimal_value(&self, scale: u32, rounding_strategy: Option<RoundingStrategy>) -> Decimal {
    let Ratio {
      numerator,
      denominator,
    } = self.clone();
    match rounding_strategy {
      None => numerator.div(denominator),
      Some(s) => numerator.div(denominator).round_dp_with_strategy(scale, s),
    }
  }

  fn gcd(numerator: Decimal, denominator: Decimal) -> Decimal {
    if denominator.is_zero() {
      numerator
    } else {
      Self::gcd(denominator, numerator % denominator)
    }
  }

  pub fn reduce(self) -> Self {
    let gcd = Self::gcd(self.numerator, self.denominator);
    Self::new(self.numerator / gcd, self.denominator / gcd)
  }

  pub fn times(self, multiplier: Self) -> Self {
    Self::new(
      self.numerator * multiplier.numerator,
      self.denominator * multiplier.denominator,
    )
  }

  pub fn times_by_big_decimal(self, multiplier: Decimal) -> Self {
    Self::new(self.numerator * multiplier, self.denominator)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;
  use rust_decimal::prelude::*;
  use rust_decimal::Decimal;

  #[test]
  fn test_big_decimal_ratio() {
    let r3over2 = Ratio::new(Decimal::from_i32(3).unwrap(), Decimal::from_i32(2).unwrap());
    let result = r3over2.decimal_value(1, None);
    assert_eq!(result, Decimal::from_str("1.5").unwrap());

    let r10over3 = Ratio::new(
      Decimal::from_i32(10).unwrap(),
      Decimal::from_i32(3).unwrap(),
    );
    let result = r10over3.decimal_value(3, Some(RoundingStrategy::RoundDown));
    assert_eq!(result, Decimal::from_str("3.333").unwrap());

    let result = r10over3.decimal_value(3, Some(RoundingStrategy::RoundUp));
    assert_eq!(result, Decimal::from_str("3.334").unwrap());

    let r_many_digits = Ratio::new(
      Decimal::from_str("9.001").unwrap(),
      Decimal::from_i32(3).unwrap(),
    );
    let result = r_many_digits.decimal_value(6, Some(RoundingStrategy::RoundUp));
    assert_eq!(result, Decimal::from_str("3.000334").unwrap());

    let result = r_many_digits.decimal_value(7, Some(RoundingStrategy::RoundUp));
    assert_eq!(result, Decimal::from_str("3.0003334").unwrap());

    let result = r_many_digits.decimal_value(7, Some(RoundingStrategy::RoundHalfUp));
    assert_eq!(result, Decimal::from_str("3.0003333").unwrap());
  }
}
