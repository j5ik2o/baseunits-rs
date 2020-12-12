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

impl ToString for TimePoint {
  fn to_string(&self) -> String {
    Self::to_string(self, "%Y/%m/%d %H/%M/%S")
  }
}

impl From<DateTime<Utc>> for TimePoint {
  fn from(date_time: DateTime<Utc>) -> Self {
    TimePoint::from_date_time_utc(date_time)
  }
}

impl<T> From<Date<T>> for TimePoint
where
  T: TimeZone,
{
  fn from(date: Date<T>) -> Self {
    TimePoint::from_date_tz(date)
  }
}

impl TimePoint {
  pub fn new(milliseconds_from_epoc: i64) -> Self {
    Self(milliseconds_from_epoc)
  }

  pub fn from_date_time_utc(date_time: DateTime<Utc>) -> Self {
    Self::from_date_time_tz(date_time)
  }

  pub fn from_date_time_tz<T>(date_time: DateTime<T>) -> Self
  where
    T: TimeZone,
  {
    TimePoint::new(date_time.timestamp_millis())
  }

  pub fn from_date_tz<T>(date: Date<T>) -> Self
  where
    T: TimeZone,
  {
    TimePoint::new(date.and_hms_milli(0, 0, 0, 0).timestamp_millis())
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
      year_month.breach_encapsulation_of_year(),
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
      calendar_date.breach_encapsulation_of_year_month(),
      calendar_date.breach_encapsulation_of_day(),
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
    Ok(TimePoint::from_date_time_tz(date_time))
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

  fn to_string(&self, fmt: &str) -> String {
    self.to_date_time(Utc).format(fmt).to_string()
  }
}

#[cfg(test)]
mod tests {
  use crate::time::{TimePoint, TimeOfDay};

  #[test]
  fn test_at_ymd_milli() {
    let tp1 = TimePoint::new(1262304000000);
    let tp2 = TimePoint::at_ymd_hms_milli_utc(2010, 1, 1, 0, 0, 0, 0);
    assert_eq!(tp2, tp1)
  }

  #[test]
  fn test_to_time_of_day() {
    let tp1 = TimePoint::at_ymd_hms_milli_utc(2010, 1, 1, 0, 0, 0, 0);
    let tod = TimeOfDay::from_hour_with_minute(0, 0);
    assert_eq!(tp1.to_time_of_day_utc(), tod)
  }
}
