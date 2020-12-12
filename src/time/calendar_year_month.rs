use crate::time::{MonthOfYear, Month, DayOfMonth, TimePoint};
use chrono::{DateTime, Datelike, Date, TimeZone, Utc, ParseError};
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

impl From<TimePoint> for CalendarYearMonth {
  fn from(value: TimePoint) -> Self {
    Self::from(value.to_date_time(Utc))
  }
}

impl CalendarYearMonth {
  pub fn new(year: i32, month: MonthOfYear) -> Self {
    Self { year, month }
  }

  pub fn from_year_with_month(year: i32, month: i32) -> Self {
    Self::new(
      year,
      MonthOfYear::from_month(Month::from_i32(month).unwrap()),
    )
  }

  pub fn breach_encapsulation_of_year(&self) -> i32 {
    self.year
  }

  pub fn breach_encapsulation_of_month(&self) -> MonthOfYear {
    self.month.clone()
  }

  pub fn to_month(&self) -> Month {
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

  pub fn parse_tz<T>(date_str: &str, pattern: &str, time_zone: T) -> Result<Self, ParseError>
  where
    T: TimeZone,
  {
    let tp = TimePoint::parse_tz(date_str, pattern, time_zone)?;
    Ok(CalendarYearMonth::from(tp))
  }

  pub fn is_after(&self, other: &Self) -> bool {
    !self.is_before(other) && self != other
  }

  pub fn is_before(&self, other: &Self) -> bool {
    if self.year < other.year {
      true
    } else if self.year > other.year {
      false
    } else {
      self.month.is_before(&other.month)
    }
  }
}
