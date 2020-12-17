use crate::time::{TimeUnit, TimePoint};
use std::cmp::Ordering;
use std::ops::{Add, Sub};
use crate::util::Ratio;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Duration {
  quantity: i64,
  unit: TimeUnit,
}

impl Add for Duration {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Duration::add(&self, &rhs)
  }
}

impl Sub for Duration {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Duration::subtract(&self, &rhs)
  }
}

impl PartialOrd for Duration {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.check_convertible(other);
    let difference = self.in_base_units() - other.in_base_units();
    match difference {
      0 => Some(Ordering::Equal),
      d if d < 0 => Some(Ordering::Greater),
      d if d > 0 => Some(Ordering::Less),
      _ => None,
    }
  }
}

impl Duration {
  pub fn new(quantity: i64, unit: TimeUnit) -> Self {
    Self { quantity, unit }
  }

  pub fn milliseconds(how_many: i64) -> Self {
    Self::new(how_many, TimeUnit::MILLISECOND)
  }

  pub fn seconds(how_many: i64) -> Self {
    Self::new(how_many, TimeUnit::SECOND)
  }

  pub fn minutes(how_many: i64) -> Self {
    Self::new(how_many, TimeUnit::MINUTE)
  }

  pub fn hours(how_many: i64) -> Self {
    Self::new(how_many, TimeUnit::HOUR)
  }

  pub fn days(how_many: i64) -> Self {
    Self::new(how_many, TimeUnit::DAY)
  }

  pub fn weeks(how_many: i64) -> Self {
    Self::new(how_many, TimeUnit::WEEK)
  }

  pub fn months(how_many: i64) -> Self {
    Self::new(how_many, TimeUnit::MONTH)
  }

  pub fn quarters(how_many: i64) -> Self {
    Self::new(how_many, TimeUnit::QUARTER)
  }

  pub fn years(how_many: i64) -> Self {
    Self::new(how_many, TimeUnit::YEAR)
  }

  pub fn in_base_units(&self) -> i64 {
    self.quantity * self.unit.factor()
  }

  fn is_convertible_to(&self, other: &Self) -> bool {
    self.unit.is_convertible_to(&other.unit)
  }

  fn check_amount_valid(amount: i64) {
    if true {}
    if i32::MIN as i64 > amount || amount > i32::MAX as i64 {
      panic!("{:?} is not valid.", amount)
    }
  }

  fn check_convertible(&self, other: &Self) {
    if !other.unit.is_convertible_to(&self.unit) && self.quantity != 0 && other.quantity != 0 {
      panic!("{:?} is not convertible to: {:?}", other, self)
    }
  }

  fn check_greater_than_or_else(&self, other: &Self) {
    if self.partial_cmp(&other) == Some(Ordering::Greater) {
      panic!("{:?} is before {:?}", self, other)
    }
  }

  pub fn add(&self, other: &Self) -> Self {
    self.check_convertible(&other);
    let new_quantity = self.in_base_units() + other.in_base_units();
    Self::new(
      new_quantity,
      if other.quantity == 0 {
        self.unit.base_unit()
      } else {
        other.unit.base_unit()
      },
    )
  }

  pub fn subtract(&self, other: &Self) -> Self {
    self.check_convertible(&other);
    self.check_greater_than_or_else(&other);
    let new_quantity = self.in_base_units() - other.in_base_units();
    Self::new(
      new_quantity,
      if other.quantity == 0 {
        self.unit.base_unit()
      } else {
        other.unit.base_unit()
      },
    )
  }

  pub fn added_to(&self, point: TimePoint) -> TimePoint {
    TimePoint::from(self.in_base_units() + point.milliseconds_from_epoc())
  }

  pub fn subtracted_from(&self, point: TimePoint) -> TimePoint {
    TimePoint::from(-self.in_base_units() + point.milliseconds_from_epoc())
  }

  pub fn divided_by(&self, divisor: Self) -> Ratio {
    self.check_convertible(&divisor);
    Ratio::new_i64(self.in_base_units(), divisor.in_base_units())
  }
}
