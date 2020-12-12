use crate::time::{CalendarDate, TimeOfDay};
use chrono::{DateTime, TimeZone, Timelike, Datelike};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct CalendarDateTime {
  date: CalendarDate,
  time: TimeOfDay,
}

impl<T> From<DateTime<T>> for CalendarDateTime
where
  T: TimeZone,
{
  fn from(value: DateTime<T>) -> Self {
    let cd = CalendarDate::from_year_with_month_with_day(value.year(), value.month(), value.day());
    let tod = TimeOfDay::from_hour_with_minute(value.hour(), value.minute());
    Self::new(cd, tod)
  }
}

impl CalendarDateTime {
  pub fn new(date: CalendarDate, time: TimeOfDay) -> Self {
    Self { date, time }
  }

  pub fn from_year_with_month_day_with_hour_with_minute(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
  ) -> Self {
    let cd = CalendarDate::from_year_with_month_with_day(year, month, day);
    let tod = TimeOfDay::from_hour_with_minute(hour, minute);
    Self::new(cd, tod)
  }

  pub fn is_after(&self, other: &Self) -> bool {
    !self.is_before(other) && self != other
  }

  pub fn is_before(&self, other: &Self) -> bool {
    if self.date.is_before(&other.date) {
      true
    } else if self.date.is_after(&other.date) {
      false
    } else {
      self.time.is_before(&other.time)
    }
  }
}