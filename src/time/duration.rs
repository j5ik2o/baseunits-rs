use crate::time::TimeUnit;
use std::cmp::Ordering;
use std::ops::Add;
use crate::util::Ratio;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct Duration {
  quantity: i64,
  unit: TimeUnit,
}

impl Add for Duration {
  type Output = Duration;

  fn add(self, rhs: Self) -> Self::Output {
    Duration::add(&self, rhs)
  }
}

impl Ord for Duration {
  fn cmp(&self, other: &Self) -> Ordering {
    self.check_convertible(other);
    let difference = self.in_base_units() - other.in_base_units();
    if difference > 0 {
      Ordering::Greater
    } else if difference < 0 {
      Ordering::Less
    } else {
      Ordering::Equal
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
    self.quantity * self.unit.get_factor()
  }

  fn is_convertible_to(&self, other: &Self) -> bool {
    self.unit.is_convertible_to(&other.unit)
  }

  fn check_amount_valid(amount: i64) {
    if !(i64::MIN <= amount && amount <= i64::MAX) {
      panic!("{:?} is not valid.", amount)
    }
  }

  fn check_convertible(&self, other: &Self) {
    if !other.unit.is_convertible_to(&self.unit) && self.quantity != 0 && other.quantity != 0 {
      panic!("{:?} is not convertible to: {:?}", other, self)
    }
  }

  pub fn add(&self, other: Self) -> Self {
    self.check_convertible(&other);
    let new_quantity = self.in_base_units() + other.in_base_units();
    Self::new(
      new_quantity,
      if other.clone().quantity == 0 {
        self.unit.base_unit()
      } else {
        other.clone().unit.base_unit()
      },
    )
  }

  pub fn divided_by(&self, divisor: Self) -> Ratio {
    self.check_convertible(&divisor);
    Ratio::new_i64(self.in_base_units(), divisor.in_base_units())
  }
}
