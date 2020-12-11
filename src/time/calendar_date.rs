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
    let date_time = time_point.as_date_time();
    let cym = CalendarYearMonth::from(date_time);
    let dom = DayOfMonth::new(date_time.day().to_i32().unwrap());
    CalendarDate::new(cym, dom)
  }
}

impl CalendarDate {
  pub fn new(year_month: CalendarYearMonth, day: DayOfMonth) -> Self {
    Self { year_month, day }
  }

  pub fn breach_encapsulation_of_year_month(&self) -> CalendarYearMonth {
    self.year_month.clone()
  }

  pub fn breach_encapsulation_of_day(&self) -> DayOfMonth {
    self.day.clone()
  }

  pub fn as_date_time_on_midnight(&self) -> DateTime<Utc> {
    Utc
      .ymd(
        self.year_month.breach_encapsulation_of_year(),
        self.year_month.as_month().to_u32().unwrap(),
        self.day.0.to_u32().unwrap(),
      )
      .and_hms_milli(0, 0, 0, 0)
  }

  pub fn day_of_week(&self) -> DayOfWeek {
    let no = self
      .as_date_time_on_midnight()
      .date()
      .weekday()
      .number_from_monday();
    DayOfWeek::from_u32(no).unwrap()
  }
}
