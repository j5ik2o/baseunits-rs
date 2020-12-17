use crate::time::{MonthOfYear, Month, DayOfMonth, TimePoint};
use chrono::{DateTime, Datelike, Date, TimeZone, Utc, ParseError};
use num::{FromPrimitive, ToPrimitive};

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

impl From<(i32, MonthOfYear)> for CalendarYearMonth {
  fn from((year, month): (i32, MonthOfYear)) -> Self {
    Self::new(year, month)
  }
}

impl From<(i32, u32)> for CalendarYearMonth {
  fn from((year, month): (i32, u32)) -> Self {
    Self::new(
      year,
      MonthOfYear::from_month(Month::from_u32(month).unwrap()),
    )
  }
}

impl CalendarYearMonth {
  pub fn new(year: i32, month: MonthOfYear) -> Self {
    Self { year, month }
  }

  pub fn to_year(&self) -> i32 {
    self.year
  }

  pub fn as_month_of_year(&self) -> &MonthOfYear {
    &self.month
  }

  pub fn as_month(&self) -> &Month {
    self.month.as_value()
  }

  pub fn to_month_u32(&self) -> u32 {
    self.month.as_value().to_u32().unwrap()
  }

  pub fn as_last_day_of_month(&self) -> &DayOfMonth {
    self.month.as_last_day()
  }

  pub fn add_month(&mut self) -> &Self {
    let (new_month, overflow) = self.month.add_with_overflow();
    self.month = new_month;
    if overflow {
      self.year += 1;
    }
    self
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
    match (self.year, other.year) {
      (sy, oy) if sy < oy => true,
      (sy, oy) if sy > oy => false,
      _ => self.month.is_before(&other.month),
    }
  }
}
