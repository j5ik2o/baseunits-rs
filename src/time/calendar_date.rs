use chrono::{Date, Datelike, DateTime, TimeZone, Utc};
use num::FromPrimitive;
use time::Duration as OldDuration;

use crate::time::{CalendarYearMonth, DayOfMonth, DayOfWeek, TimePoint};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct CalendarDate {
  year_month: CalendarYearMonth,
  day: DayOfMonth,
}

impl ToString for CalendarDate {
  fn to_string(&self) -> String {
    unimplemented!()
  }
}

impl<T> From<Date<T>> for CalendarDate
where
  T: TimeZone,
{
  fn from(value: Date<T>) -> Self {
    let dt: DateTime<T> = value.and_hms_milli(0, 0, 0, 0);
    Self::from(dt)
  }
}

impl<T> From<DateTime<T>> for CalendarDate
where
  T: TimeZone,
{
  fn from(value: DateTime<T>) -> Self {
    let cym = CalendarYearMonth::from((value.year(), value.month()));
    let dom = DayOfMonth::new(value.day());
    CalendarDate::new(cym, dom)
  }
}

impl From<TimePoint> for CalendarDate {
  fn from(time_point: TimePoint) -> Self {
    Self::from((time_point, Utc))
  }
}

impl<T: TimeZone> From<(TimePoint, T)> for CalendarDate {
  fn from((time_point, time_zone): (TimePoint, T)) -> Self {
    let date_time = time_point.to_date_time(time_zone);
    let cym = CalendarYearMonth::from((date_time.year(), date_time.month()));
    let dom = DayOfMonth::new(date_time.day());
    CalendarDate::new(cym, dom)
  }
}

impl From<(CalendarYearMonth, DayOfMonth)> for CalendarDate {
  fn from((year_month, day): (CalendarYearMonth, DayOfMonth)) -> Self {
    Self::new(year_month, day)
  }
}

impl From<(i32, u32, u32)> for CalendarDate {
  fn from((year, month, day): (i32, u32, u32)) -> Self {
    let cym = CalendarYearMonth::from((year, month));
    let dom = DayOfMonth::new(day);
    Self::new(cym, dom)
  }
}

impl CalendarDate {
  /// コンストラクタ
  pub fn new(year_month: CalendarYearMonth, day: DayOfMonth) -> Self {
    Self { year_month, day }
  }

  pub fn as_year_month(&self) -> &CalendarYearMonth {
    &self.year_month
  }

  pub fn as_day(&self) -> &DayOfMonth {
    &self.day
  }

  pub fn to_date_time_on_midnight_at_utc(&self) -> DateTime<Utc> {
    self.to_date_time_on_midnight(Utc)
  }

  pub fn to_date_time_on_midnight<T: TimeZone>(&self, time_zone: T) -> DateTime<T> {
    time_zone
      .ymd(
        self.year_month.to_year(),
        self.year_month.to_month_u32(),
        self.day.0,
      )
      .and_hms_milli(0, 0, 0, 0)
  }

  pub fn day_of_week_at_utc(&self) -> DayOfWeek {
    self.day_of_week(Utc)
  }

  pub fn day_of_week<T: TimeZone>(&self, time_zone: T) -> DayOfWeek {
    let no = self
      .to_date_time_on_midnight(time_zone)
      .date()
      .weekday()
      .number_from_monday();
    DayOfWeek::from_u32(no).unwrap()
  }

  pub fn add_days<T>(&self, days: i64, time_zone: T) -> Self
  where
    T: TimeZone,
  {
    let date_time = self.to_date_time_on_midnight(time_zone);
    let new_date_time = date_time + OldDuration::days(days);
    Self::from(new_date_time)
  }

  pub fn subtract_days<T>(&self, days: i64, time_zone: T) -> Self
  where
    T: TimeZone,
  {
    let date_time = self.to_date_time_on_midnight(time_zone);
    let new_date_time = date_time - OldDuration::days(days);
    Self::from(new_date_time)
  }

  pub fn subtract_months<T>(&self, months: i64, time_zone: T) -> Self
  where
    T: TimeZone,
  {
    let date_time = self.to_date_time_on_midnight(time_zone);
    let new_date_time = date_time - OldDuration::days(30 * months);
    Self::from(new_date_time)
  }

  pub fn add_months<T>(&self, months: i64, time_zone: T) -> Self
  where
    T: TimeZone,
  {
    let date_time = self.to_date_time_on_midnight(time_zone);
    let new_date_time = date_time + OldDuration::days(30 * months);
    Self::from(new_date_time)
  }

  pub fn is_after(&self, other: &Self) -> bool {
    !self.is_before(other) && self != other
  }

  pub fn is_before(&self, other: &Self) -> bool {
    if self.year_month.is_before(&other.year_month) {
      true
    } else if self.year_month.is_after(&other.year_month) {
      false
    } else {
      self.day.is_before(&other.day)
    }
  }
}
