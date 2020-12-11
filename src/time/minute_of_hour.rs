#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct MinuteOfHour(i32);

impl ToString for MinuteOfHour {
  fn to_string(&self) -> String {
    format!("{:02}", self.0)
  }
}

impl MinuteOfHour {
  const MIN: i32 = 0;
  const MAX: i32 = 59;
  pub fn new(value: i32) -> Self {
    if !(MinuteOfHour::MIN <= value && value <= MinuteOfHour::MAX) {
      panic!(
        "Illegal value for 60 minutes : {:?}, please use a value between 0 and 59",
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
