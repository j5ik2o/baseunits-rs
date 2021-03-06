use std::ops::{Add, Sub};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct HourOfDay(u32);

impl ToString for HourOfDay {
  fn to_string(&self) -> String {
    format!("{:02}", self.0)
  }
}

impl Add for HourOfDay {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    HourOfDay::new(self.0 + rhs.0)
  }
}

impl Sub for HourOfDay {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    HourOfDay::new(self.0 - rhs.0)
  }
}

impl HourOfDay {
  pub const MIN: u32 = 0;
  pub const MAX: u32 = 23;

  pub fn new(value: u32) -> Self {
    if value > HourOfDay::MAX {
      panic!(
        "Illegal value for 24 hour : {:}, please use a value between 0 and 23",
        value
      )
    }
    Self(value)
  }

  pub fn is_after(&self, other: &Self) -> bool {
    !self.is_before(other) && self != other
  }

  pub fn is_before(&self, other: &Self) -> bool {
    self.0 < other.0
  }
}
