use chrono::{DateTime, TimeZone, Utc, Date};
use time::{Timespec, Tm};
use crate::time::duration::Duration;
use crate::time::CalendarDate;

/// TimePoint
#[derive(Debug, Clone, PartialEq)]
pub struct TimePoint(i64);

impl From<i64> for TimePoint {
  fn from(milliseconds: i64) -> Self {
    TimePoint::new(milliseconds)
  }
}

impl<T: TimeZone> From<DateTime<T>> for TimePoint {
  fn from(date_time: DateTime<T>) -> Self {
    TimePoint::new(date_time.timestamp_millis())
  }
}

impl<T: TimeZone> From<Date<T>> for TimePoint {
  fn from(date: Date<T>) -> Self {
    TimePoint::new(date.and_hms_milli(0, 0, 0, 0).timestamp_millis())
  }
}

impl TimePoint {
  pub fn new(milliseconds_from_epoc: i64) -> Self {
    Self(milliseconds_from_epoc)
  }

  pub fn milliseconds_from_epoc(&self) -> i64 {
    self.0
  }

  pub fn at(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
  ) -> Self {
    let epoc = Utc
      .ymd(year, month, day)
      .and_hms_milli(hour, minute, second, millisecond)
      .timestamp_millis();
    Self::new(epoc)
  }

  pub fn from_tm(tm: Tm) -> Self {
    TimePoint::new(tm.to_timespec().sec * 1000)
  }

  pub fn from_date_time(date_time: DateTime<Utc>) -> Self {
    TimePoint::new(date_time.timestamp_millis())
  }

  pub fn parse<T: TimeZone>(date_time_str: String, pattern: String, time_zone: T) -> Self {
    let date_time = DateTime::parse_from_str(&date_time_str, &pattern)
      .unwrap()
      .with_timezone(&time_zone);
    TimePoint::from(date_time)
  }

  pub fn as_tm(&self) -> Tm {
    time::at(Timespec::new(self.0, 0))
  }

  pub fn as_date_time(&self) -> DateTime<Utc> {
    Utc.timestamp_millis(self.0)
  }

  pub fn as_calendar_date(&self) -> CalendarDate {
    CalendarDate::from(self.clone())
  }

  pub fn add(&self, duration: Duration) -> Self {
    duration.added_to(self.clone())
  }
}
