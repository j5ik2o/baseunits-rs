#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct HourOfDay(i32);

impl ToString for HourOfDay {
  fn to_string(&self) -> String {
    format!("{:02}", self.0)
  }
}

impl HourOfDay {
  pub const MIN: i32 = 0;
  pub const MAX: i32 = 23;
  pub fn new(value: i32) -> Self {
    if !(HourOfDay::MIN <= value && value <= HourOfDay::MAX) {
      panic!(
        "Illegal value for 24 hour : {:}, please use a value between 0 and 23",
        value
      )
    }
    Self(value)
  }

  pub fn is_after(&self, other: &Self) -> bool {
    self.0 > other.0
  }

  pub fn is_before(&self, other: &Self) -> bool {
    self.0 < other.0
  }
}
