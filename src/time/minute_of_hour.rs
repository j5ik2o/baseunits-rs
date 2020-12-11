use std::ops::{Add, Sub};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct MinuteOfHour(u8);

impl ToString for MinuteOfHour {
  fn to_string(&self) -> String {
    format!("{:02}", self.0)
  }
}

impl Add for MinuteOfHour {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self::new(self.0 + rhs.0)
  }
}

impl Sub for MinuteOfHour {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self::new(self.0 - rhs.0)
  }
}

impl MinuteOfHour {
  const MIN: u8 = 0;
  const MAX: u8 = 59;

  /// コンストラクタ。
  pub fn new(value: u8) -> Self {
    if !(MinuteOfHour::MIN <= value && value <= MinuteOfHour::MAX) {
      panic!(
        "Illegal value for 60 minutes : {:?}, please use a value between 0 and 59",
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
