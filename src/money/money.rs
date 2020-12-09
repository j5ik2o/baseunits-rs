use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};

use bigdecimal::{BigDecimal, Zero};
use iso_4217::CurrencyCode;
use num_bigint::BigInt;
use rust_fp_categories::empty::Empty;
use rust_fp_categories::monoid::Monoid;
use rust_fp_categories::semigroup::Semigroup;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Money {
  pub amount: BigDecimal,
  pub currency: CurrencyCode,
}

#[derive(Debug, PartialEq)]
pub enum MoneyError {
  NotSameCurrencyError,
}

impl PartialOrd for Money {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.currency != other.currency {
      None
    } else if self.amount > other.amount {
      Some(Ordering::Greater)
    } else if self.amount < other.amount {
      Some(Ordering::Less)
    } else {
      Some(Ordering::Equal)
    }
  }

  fn lt(&self, other: &Self) -> bool {
    self.currency == other.currency && self.amount.lt(&other.amount)
  }

  fn le(&self, other: &Self) -> bool {
    self.currency == other.currency && self.amount.le(&other.amount)
  }

  fn gt(&self, other: &Self) -> bool {
    self.currency == other.currency && self.amount.gt(&other.amount)
  }

  fn ge(&self, other: &Self) -> bool {
    self.currency == other.currency && self.amount.ge(&other.amount)
  }
}

impl Empty for Money {
  fn empty() -> Self {
    Money::zero(CurrencyCode::USD)
  }

  fn is_empty(&self) -> bool {
    self.is_zero()
  }
}

impl Semigroup for Money {
  fn combine(self, other: Self) -> Self {
    self + other
  }
}

impl Monoid for Money {}

impl Add for Money {
  type Output = Money;

  fn add(self, rhs: Self) -> Self::Output {
    Money::add(self, rhs).unwrap_or_else(|err| panic!(format!("{:?}", err)))
  }
}

impl Sub for Money {
  type Output = Money;

  fn sub(self, rhs: Self) -> Self::Output {
    Money::subtract(self, rhs).unwrap_or_else(|err| panic!(format!("{:?}", err)))
  }
}

impl Mul<BigDecimal> for Money {
  type Output = Money;

  fn mul(self, rhs: BigDecimal) -> Self::Output {
    Money::times(self, rhs)
  }
}

impl Div<BigDecimal> for Money {
  type Output = Money;

  fn div(self, rhs: BigDecimal) -> Self::Output {
    Money::divided_by(self, rhs)
  }
}

impl Neg for Money {
  type Output = Money;

  fn neg(self) -> Self::Output {
    Money::negated(self)
  }
}

impl From<(BigDecimal, CurrencyCode)> for Money {
  fn from((amount, currency): (BigDecimal, CurrencyCode)) -> Self {
    Money::new(amount, currency)
  }
}

impl From<(BigInt, CurrencyCode)> for Money {
  fn from((amount, currency): (BigInt, CurrencyCode)) -> Self {
    let a = BigDecimal::from((amount, currency.digit().map(|v| v as i64).unwrap_or(0i64)));
    Money::new(a, currency)
  }
}

impl From<(&str, CurrencyCode)> for Money {
  fn from((amount, currency): (&str, CurrencyCode)) -> Self {
    let a = BigDecimal::from_str(amount).unwrap_or_else(|err| panic!("{}", err));
    Money::new(a, currency)
  }
}

macro_rules! from_numeric_impl {
  ($($t:ty)*) => ($(
    impl From<($t, CurrencyCode)> for Money {
      fn from((amount, currency): ($t, CurrencyCode)) -> Self {
        let a = BigDecimal::from(amount).with_scale(currency.digit().map(|v| v as i64).unwrap_or(0i64));
        Money::new(a, currency)
      }
    }
  )*)
}

from_numeric_impl! {i8 i16 i32 i64 u8 u16 u32 u64}

impl Money {
  pub fn new(amount: BigDecimal, currency: CurrencyCode) -> Self {
    let a = amount.with_scale(currency.digit().map(|v| v as i64).unwrap_or(0i64));
    Self {
      amount: a,
      currency,
    }
  }

  pub fn zero(currency: CurrencyCode) -> Self {
    Self::new(BigDecimal::zero(), currency)
  }

  pub fn abs(&self) -> Self {
    Self {
      amount: self.amount.abs(),
      currency: self.currency,
    }
  }

  pub fn is_positive(&self) -> bool {
    self.amount > BigDecimal::zero()
  }

  pub fn is_negative(&self) -> bool {
    self.amount < BigDecimal::zero()
  }

  pub fn is_zero(&self) -> bool {
    self.amount.is_zero()
  }

  pub fn negated(self) -> Self {
    Self {
      amount: -self.amount,
      currency: self.currency,
    }
  }

  pub fn add(self, other: Self) -> Result<Self, MoneyError> {
    if self.currency != other.currency {
      Err(MoneyError::NotSameCurrencyError)
    } else {
      Ok(Self {
        amount: self.amount + other.amount,
        currency: self.currency,
      })
    }
  }

  pub fn subtract(self, other: Self) -> Result<Self, MoneyError> {
    self.add(other.negated())
  }

  pub fn times(self, factor: BigDecimal) -> Self {
    Self {
      amount: self.amount * factor,
      currency: self.currency,
    }
  }

  pub fn divided_by(self, divisor: BigDecimal) -> Self {
    Self {
      amount: self.amount / divisor,
      currency: self.currency,
    }
  }
}

#[cfg(test)]
mod tests {
  use bigdecimal::BigDecimal;
  use iso_4217::CurrencyCode;

  use crate::money::{Money, MoneyError};

  #[test]
  fn test_eq() -> Result<(), MoneyError> {
    let m1 = Money::from((1u32, CurrencyCode::USD));
    let m2 = Money::from((1u32, CurrencyCode::USD));
    assert_eq!(m1, m2);
    Ok(())
  }

  #[test]
  fn test_ne() -> Result<(), MoneyError> {
    let m1 = Money::from((1u32, CurrencyCode::USD));
    let m2 = Money::from((2u32, CurrencyCode::USD));
    assert_ne!(m1, m2);
    Ok(())
  }

  #[test]
  fn test_zero() -> Result<(), MoneyError> {
    let m1 = Money::zero(CurrencyCode::USD);
    let m2 = Money::new(BigDecimal::from(0), CurrencyCode::USD);
    assert_eq!(m1.abs(), m2);
    Ok(())
  }

  #[test]
  fn test_abs() -> Result<(), MoneyError> {
    let m1 = Money::new(BigDecimal::from(-1), CurrencyCode::USD);
    let m2 = Money::new(BigDecimal::from(1), CurrencyCode::USD);
    assert_eq!(m1.abs(), m2);
    Ok(())
  }

  #[test]
  fn test_add() -> Result<(), MoneyError> {
    let m1 = Money::from((1u32, CurrencyCode::USD));
    let m2 = Money::from((2u32, CurrencyCode::USD));
    let m3 = m1.clone();
    let m4 = m2.clone();

    let m5 = m1.add(m2)?;
    let m6 = m3 + m4;

    assert_eq!(m5, Money::new(BigDecimal::from(3), CurrencyCode::USD));
    assert_eq!(m6, Money::new(BigDecimal::from(3), CurrencyCode::USD));
    Ok(())
  }
}
