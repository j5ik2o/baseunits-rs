use chrono::{DateTime, TimeZone, Utc, Date, ParseError, Timelike};
use crate::time::duration::Duration;
use crate::time::{CalendarDate, TimeOfDay, CalendarYearMonth, DayOfMonth};
use num::ToPrimitive;

/// TimePoint
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct TimePoint(i64);

impl From<i64> for TimePoint {
  fn from(milliseconds: i64) -> Self {
    TimePoint::new(milliseconds)
  }
}

impl<T> From<DateTime<T>> for TimePoint
where
  T: TimeZone,
{
  fn from(date_time: DateTime<T>) -> Self {
    TimePoint::new(date_time.timestamp_millis())
  }
}

impl<T> From<Date<T>> for TimePoint
where
  T: TimeZone,
{
  fn from(date: Date<T>) -> Self {
    TimePoint::new(date.and_hms_milli(0, 0, 0, 0).timestamp_millis())
  }
}

impl TimePoint {
  pub fn new(milliseconds_from_epoc: i64) -> Self {
    Self(milliseconds_from_epoc)
  }

  pub fn at_utc(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
  ) -> Self {
    Self::at(year, month, day, hour, minute, second, millisecond, Utc)
  }

  pub fn at<T>(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
    time_zone: T,
  ) -> Self
  where
    T: TimeZone,
  {
    let milliseconds_from_epoc = time_zone
      .ymd(year, month, day)
      .and_hms_milli(hour, minute, second, millisecond)
      .timestamp_millis();
    Self::new(milliseconds_from_epoc)
  }

  pub fn at_calendar_year_month_with_day_of_month_with_others<T>(
    year_month: CalendarYearMonth,
    date: DayOfMonth,
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
    time_zone: T,
  ) -> Self
  where
    T: TimeZone,
  {
    Self::at(
      year_month.breach_encapsulation_of_year(),
      year_month.as_month().to_u32().unwrap(),
      date.0.to_u32().unwrap(),
      hour,
      minute,
      second,
      minute,
      time_zone,
    )
  }

  pub fn from_date_time(date_time: DateTime<Utc>) -> Self {
    TimePoint::new(date_time.timestamp_millis())
  }

  pub fn at_midnight<T>(calendar_date: CalendarDate) -> Self
  where
    T: TimeZone,
  {
  }

  pub fn parse_utc(date_time_str: &str, pattern: &str) -> Result<TimePoint, ParseError> {
    Self::parse(date_time_str, pattern, Utc)
  }

  pub fn parse<T>(date_time_str: &str, pattern: &str, time_zone: T) -> Result<Self, ParseError>
  where
    T: TimeZone,
  {
    let date_time = DateTime::parse_from_str(date_time_str, pattern)?.with_timezone(&time_zone);
    Ok(TimePoint::from(date_time))
  }

  // ---

  pub fn milliseconds_from_epoc(&self) -> i64 {
    self.0
  }

  pub fn to_date_time_utc(&self) -> DateTime<Utc> {
    self.to_date_time(Utc)
  }

  pub fn to_date_time<T>(&self, time_zone: T) -> DateTime<T>
  where
    T: TimeZone,
  {
    time_zone.timestamp_millis(self.0)
  }

  pub fn to_date<T>(&self, time_zone: T) -> Date<T>
  where
    T: TimeZone,
  {
    self.to_date_time(time_zone).date()
  }

  pub fn to_calendar_date_utc(&self) -> CalendarDate {
    self.to_calendar_date(Utc)
  }

  pub fn to_calendar_date<T>(&self, time_zone: T) -> CalendarDate
  where
    T: TimeZone,
  {
    CalendarDate::from_time_point(self.clone(), time_zone)
  }

  pub fn to_time_of_day<T>(&self, time_zone: T) -> TimeOfDay
  where
    T: TimeZone,
  {
    let dt = self.to_date_time(time_zone);
    TimeOfDay::from_hour_with_minute(dt.hour().to_u8().unwrap(), dt.minute().to_u8().unwrap())
  }

  pub fn add(self, duration: Duration) -> Self {
    duration.added_to(self)
  }

  pub fn subtract(self, duration: Duration) -> Self {
    duration.subtracted_from(self)
  }

  pub fn is_after(&self, other: &Self) -> bool {
    !self.is_before(other) && self != other
  }

  pub fn is_before(&self, other: &Self) -> bool {
    self.0 < other.0
  }
}
