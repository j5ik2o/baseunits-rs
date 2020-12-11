use chrono::{Datelike, DateTime, TimeZone, Utc};
use num::{ToPrimitive, FromPrimitive};

use crate::time::{CalendarYearMonth, DayOfMonth, DayOfWeek, TimePoint};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct CalendarDate {
  year_month: CalendarYearMonth,
  day: DayOfMonth,
}

impl From<TimePoint> for CalendarDate {
  fn from(time_point: TimePoint) -> Self {
    Self::from_by_utc(time_point)
  }
}

impl CalendarDate {
  /// コンストラクタ
  pub fn new(year_month: CalendarYearMonth, day: DayOfMonth) -> Self {
    Self { year_month, day }
  }

  fn from_by_utc(time_point: TimePoint) -> Self {
    Self::from(Utc, time_point)
  }

  fn from<T: TimeZone>(time_zone: T, time_point: TimePoint) -> Self {
    let date_time = time_point.to_date_time(time_zone);
    let cym = CalendarYearMonth::from(date_time.clone());
    let dom = DayOfMonth::new(date_time.day().to_i32().unwrap());
    CalendarDate::new(cym, dom)
  }

  pub fn breach_encapsulation_of_year_month(&self) -> CalendarYearMonth {
    self.year_month.clone()
  }

  pub fn breach_encapsulation_of_day(&self) -> DayOfMonth {
    self.day.clone()
  }

  pub fn to_date_time_on_midnight_at_utc(&self) -> DateTime<Utc> {
    self.to_date_time_on_midnight(Utc)
  }

  pub fn to_date_time_on_midnight<T: TimeZone>(&self, time_zone: T) -> DateTime<T> {
    time_zone
      .ymd(
        self.year_month.breach_encapsulation_of_year(),
        self.year_month.as_month().to_u32().unwrap(),
        self.day.0.to_u32().unwrap(),
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
}
