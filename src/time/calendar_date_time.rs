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
    let cd = CalendarDate::from((value.year(), value.month(), value.day()));
    let tod = TimeOfDay::from_hour_with_minute(value.hour(), value.minute());
    Self::new(cd, tod)
  }
}

impl From<(i32, u32, u32, u32, u32)> for CalendarDateTime {
  fn from((year, month, day, hour, minute): (i32, u32, u32, u32, u32)) -> Self {
    let cd = CalendarDate::from((year, month, day));
    let tod = TimeOfDay::from_hour_with_minute(hour, minute);
    Self::new(cd, tod)
  }
}

impl CalendarDateTime {
  pub fn new(date: CalendarDate, time: TimeOfDay) -> Self {
    Self { date, time }
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
