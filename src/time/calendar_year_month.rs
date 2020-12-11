use crate::time::{MonthOfYear, Month, DayOfMonth};
use chrono::{DateTime, Datelike, Date, TimeZone};
use num::FromPrimitive;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct CalendarYearMonth {
  year: i32,
  month: MonthOfYear,
}

impl<T: TimeZone> From<DateTime<T>> for CalendarYearMonth {
  fn from(date_time: DateTime<T>) -> Self {
    let year = date_time.year();
    let month = date_time.month();
    let month_of_year = MonthOfYear::from_month(Month::from_u32(month).unwrap());
    CalendarYearMonth::new(year, month_of_year)
  }
}

impl<T: TimeZone> From<Date<T>> for CalendarYearMonth {
  fn from(date: Date<T>) -> Self {
    let year = date.year();
    let month = date.month();
    let month_of_year = MonthOfYear::from_month(Month::from_u32(month).unwrap());
    CalendarYearMonth::new(year, month_of_year)
  }
}

impl CalendarYearMonth {
  pub fn new(year: i32, month: MonthOfYear) -> Self {
    Self { year, month }
  }

  pub fn breach_encapsulation_of_year(&self) -> i32 {
    self.year
  }

  pub fn breach_encapsulation_of_month(&self) -> MonthOfYear {
    self.month.clone()
  }

  pub fn as_month(&self) -> Month {
    self.month.breach_encapsulation_of_value().clone()
  }

  pub fn last_day_of_month(&self) -> DayOfMonth {
    self.month.breach_encapsulation_of_last_day()
  }

  pub fn add_month(&self) -> Self {
    let mut new_instance: CalendarYearMonth = self.clone();
    let (new_month, overflow) = new_instance.month.add_with_overflow();
    new_instance.month = new_month;
    if overflow {
      new_instance.year += 1;
    }
    new_instance
  }
}
