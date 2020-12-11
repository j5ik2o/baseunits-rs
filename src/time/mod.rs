pub use calendar_year_month::*;
pub use day_of_month::*;
pub use month_of_year::*;
pub use time_point::*;
pub use time_unit::*;
pub use calendar_date::*;
use num::{FromPrimitive, ToPrimitive};

mod calendar_date;
mod calendar_year_month;
mod day_of_month;
mod duration;
mod month_of_year;
mod time_point;
mod time_unit;
mod time_unit_conversion_factor;

pub enum DayOfWeek {
  Sunday,
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
}

impl DayOfWeek {}

impl ToPrimitive for DayOfWeek {
  fn to_i64(&self) -> Option<i64> {
    match *self {
      DayOfWeek::Sunday => Some(1),
      DayOfWeek::Monday => Some(2),
      DayOfWeek::Tuesday => Some(3),
      DayOfWeek::Wednesday => Some(4),
      DayOfWeek::Thursday => Some(5),
      DayOfWeek::Friday => Some(6),
      DayOfWeek::Saturday => Some(7),
    }
  }

  fn to_u64(&self) -> Option<u64> {
    match *self {
      DayOfWeek::Sunday => Some(1),
      DayOfWeek::Monday => Some(2),
      DayOfWeek::Tuesday => Some(3),
      DayOfWeek::Wednesday => Some(4),
      DayOfWeek::Thursday => Some(5),
      DayOfWeek::Friday => Some(6),
      DayOfWeek::Saturday => Some(7),
    }
  }
}

impl FromPrimitive for DayOfWeek {
  fn from_i64(n: i64) -> Option<Self> {
    match n {
      1 => Some(DayOfWeek::Sunday),
      2 => Some(DayOfWeek::Monday),
      3 => Some(DayOfWeek::Tuesday),
      4 => Some(DayOfWeek::Wednesday),
      5 => Some(DayOfWeek::Thursday),
      6 => Some(DayOfWeek::Friday),
      7 => Some(DayOfWeek::Saturday),
      _ => None,
    }
  }

  fn from_u64(n: u64) -> Option<Self> {
    match n {
      1 => Some(DayOfWeek::Sunday),
      2 => Some(DayOfWeek::Monday),
      3 => Some(DayOfWeek::Tuesday),
      4 => Some(DayOfWeek::Wednesday),
      5 => Some(DayOfWeek::Thursday),
      6 => Some(DayOfWeek::Friday),
      7 => Some(DayOfWeek::Saturday),
      _ => None,
    }
  }
}

pub(crate) fn is_leap_year(year: i32) -> bool {
  return year % 4 == 0 && (year % 100 != 0 || (year % 100 == 0 && year % 400 == 0));
}
