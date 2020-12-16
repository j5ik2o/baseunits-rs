use core::fmt;

use chrono::*;
use num::ToPrimitive;

use crate::time::{CalendarDate, CalendarYearMonth, DayOfMonth, TimeOfDay};
use crate::time::duration::Duration;

/// TimePoint
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct TimePoint(i64);

impl From<i64> for TimePoint {
  fn from(milliseconds: i64) -> Self {
    TimePoint::new(milliseconds)
  }
}

impl ToString for TimePoint {
  fn to_string(&self) -> String {
    Self::to_fmt_string_utc(self, "%Y/%m/%d %H:%M:%S")
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

  pub fn at_ymd_hms_milli_utc(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
  ) -> Self {
    Self::at_ymd_hms_milli_tz(year, month, day, hour, minute, second, millisecond, Utc)
  }

  pub fn at_ymd_hms_milli_tz<T>(
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

  pub fn at_cym_dom_hms_milli_tz<T>(
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
    Self::at_ymd_hms_milli_tz(
      year_month.to_year(),
      year_month.to_month_u32(),
      date.0.to_u32().unwrap(),
      hour,
      minute,
      second,
      millisecond,
      time_zone,
    )
  }

  pub fn at_midnight_cd_utc(calendar_date: CalendarDate) -> Self {
    Self::at_midnight_cd_tz(calendar_date, Utc)
  }

  pub fn at_midnight_cd_tz<T>(calendar_date: CalendarDate, time_zone: T) -> Self
  where
    T: TimeZone,
  {
    Self::at_cym_dom_hms_milli_tz(
      calendar_date.as_year_month().clone(),
      calendar_date.as_day().clone(),
      0,
      0,
      0,
      0,
      time_zone,
    )
  }

  pub fn parse_utc(date_time_str: &str, pattern: &str) -> Result<TimePoint, ParseError> {
    Self::parse_tz(date_time_str, pattern, Utc)
  }

  pub fn parse_tz<T>(date_time_str: &str, pattern: &str, time_zone: T) -> Result<Self, ParseError>
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

  pub fn to_date_utc(&self) -> Date<Utc> {
    self.to_date(Utc)
  }

  pub fn to_date<T>(&self, time_zone: T) -> Date<T>
  where
    T: TimeZone,
  {
    self.to_date_time(time_zone).date()
  }

  pub fn to_naive_date_time_utc(&self) -> NaiveDateTime {
    self.to_naive_date_time(Utc)
  }

  pub fn to_naive_date_time<T>(&self, time_zone: T) -> NaiveDateTime
  where
    T: TimeZone,
  {
    let dt = self.to_naive_date(time_zone.clone());
    let nt = self.to_naive_time(time_zone);
    NaiveDateTime::new(dt, nt)
  }

  pub fn to_naive_time_utc(&self) -> NaiveTime {
    self.to_naive_time(Utc)
  }

  pub fn to_naive_time<T>(&self, time_zone: T) -> NaiveTime
  where
    T: TimeZone,
  {
    let dt = self.to_date_time(time_zone);
    NaiveTime::from_hms_milli(
      dt.hour(),
      dt.minute(),
      dt.second(),
      dt.nanosecond() * 1000000,
    )
  }

  pub fn to_naive_date_utc(&self) -> NaiveDate {
    self.to_naive_date(Utc)
  }

  pub fn to_naive_date<T>(&self, time_zone: T) -> NaiveDate
  where
    T: TimeZone,
  {
    let date = self.to_date(time_zone);
    NaiveDate::from_ymd(date.year(), date.month(), date.day())
  }

  pub fn to_calendar_date_utc(&self) -> CalendarDate {
    self.to_calendar_date(Utc)
  }

  pub fn to_calendar_date<T>(&self, time_zone: T) -> CalendarDate
  where
    T: TimeZone,
  {
    CalendarDate::from((self.clone(), time_zone))
  }

  pub fn to_time_of_day_utc(&self) -> TimeOfDay {
    self.to_time_of_day(Utc)
  }

  pub fn to_time_of_day<T>(&self, time_zone: T) -> TimeOfDay
  where
    T: TimeZone,
  {
    let dt = self.to_date_time(time_zone);
    TimeOfDay::from_hour_with_minute(dt.hour(), dt.minute())
  }

  pub fn add(self, duration: Duration) -> Self {
    duration.added_to(self)
  }

  pub fn subtract(self, duration: Duration) -> Self {
    duration.subtracted_from(self)
  }

  pub fn next_day(self) -> Self {
    self.add(Duration::days(1))
  }

  pub fn is_after(&self, other: &Self) -> bool {
    !self.is_before(other) && self != other
  }

  pub fn is_before(&self, other: &Self) -> bool {
    self.0 < other.0
  }

  pub fn is_same_day_as_utc(&self, other: &Self) -> bool {
    self.is_same_day_as(other, Utc)
  }

  pub fn is_same_day_as<T>(&self, other: &Self, time_zone: T) -> bool
  where
    T: TimeZone,
  {
    self.to_calendar_date(time_zone.clone()) == other.to_calendar_date(time_zone)
  }

  fn to_fmt_string_utc(&self, fmt: &str) -> String {
    self.to_fmt_string(fmt, Utc)
  }

  fn to_fmt_string<Tz: TimeZone>(&self, fmt: &str, time_zone: Tz) -> String
  where
    Tz::Offset: fmt::Display,
  {
    self.to_date_time(time_zone).format(fmt).to_string()
  }
}

#[cfg(test)]
mod tests {
  use chrono::Utc;

  use crate::time::{TimeOfDay, TimePoint};

  #[test]
  fn from_from_date_time() {
    let date_time = Utc::now();
    let tp = TimePoint::from(date_time);
    assert_eq!(
      tp.to_date_time_utc().timestamp_millis(),
      date_time.timestamp_millis()
    )
  }

  #[test]
  fn _at_ymd_milli() {
    let tp1 = TimePoint::new(1262304000000);
    let tp2 = TimePoint::at_ymd_hms_milli_utc(2010, 1, 1, 0, 0, 0, 0);
    assert_eq!(tp2, tp1)
  }

  #[test]
  fn to_time_of_day() {
    let tp1 = TimePoint::at_ymd_hms_milli_utc(2010, 1, 1, 0, 0, 0, 0);
    let tod = TimeOfDay::from_hour_with_minute(0, 0);
    assert_eq!(tp1.to_time_of_day_utc(), tod)
  }

  #[test]
  fn is_same_day_as() {
    let tp1 = TimePoint::at_ymd_hms_milli_utc(2010, 1, 1, 0, 0, 0, 0);
    let tp2 = TimePoint::at_ymd_hms_milli_utc(2010, 1, 1, 23, 59, 59, 0);
    assert!(tp1.is_same_day_as_utc(&tp2))
  }

  #[test]
  fn to_string() {
    let tp1 = TimePoint::at_ymd_hms_milli_utc(2010, 1, 1, 23, 59, 59, 0);
    println!("{}", ToString::to_string(&tp1));
  }
}
