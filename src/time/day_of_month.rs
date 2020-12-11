use crate::time::{CalendarYearMonth, CalendarDate};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct DayOfMonth(pub i32);

impl DayOfMonth {
  pub const MIN: i32 = 1;
  pub const MAX: i32 = 31;

  pub fn new(value: i32) -> Self {
    if !(DayOfMonth::MIN <= value && value <= DayOfMonth::MAX) {
      panic!(
        "Illegal value for day of month: {:?}, please use a value between 1 and 31",
        value
      )
    }
    Self(value)
  }

  pub fn is_applyable(&self, month: CalendarYearMonth) -> bool {
    !month.last_day_of_month().is_before(self)
  }

  pub fn on(self, month: CalendarYearMonth) -> CalendarDate {
    CalendarDate::new(month, self)
  }

  pub fn is_after(&self, other: &Self) -> bool {
    !self.is_before(other) && self != other
  }

  pub fn is_before(&self, other: &Self) -> bool {
    self.0 < other.0
  }
}
