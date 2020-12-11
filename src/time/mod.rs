use num::{FromPrimitive, ToPrimitive};

pub use calendar_date::*;
pub use calendar_year_month::*;
pub use day_of_month::*;
pub use minute_of_hour::*;
pub use month_of_year::*;
pub use time_point::*;
pub use time_unit::*;
pub use hour_of_day::*;

mod calendar_date;
mod calendar_year_month;
mod day_of_month;
mod duration;
mod hour_of_day;
mod minute_of_hour;
mod month_of_year;
mod time_point;
mod time_unit;
mod time_unit_conversion_factor;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
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

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub enum Month {
  January,
  February,
  March,
  April,
  May,
  June,
  July,
  August,
  September,
  October,
  November,
  December,
}

impl ToPrimitive for Month {
  fn to_i64(&self) -> Option<i64> {
    match *self {
      Month::January => Some(1),
      Month::February => Some(2),
      Month::March => Some(3),
      Month::April => Some(4),
      Month::May => Some(5),
      Month::June => Some(6),
      Month::July => Some(7),
      Month::August => Some(8),
      Month::September => Some(9),
      Month::October => Some(10),
      Month::November => Some(11),
      Month::December => Some(12),
    }
  }

  fn to_u64(&self) -> Option<u64> {
    match *self {
      Month::January => Some(1),
      Month::February => Some(2),
      Month::March => Some(3),
      Month::April => Some(4),
      Month::May => Some(5),
      Month::June => Some(6),
      Month::July => Some(7),
      Month::August => Some(8),
      Month::September => Some(9),
      Month::October => Some(10),
      Month::November => Some(11),
      Month::December => Some(12),
    }
  }
}

impl FromPrimitive for Month {
  fn from_i64(n: i64) -> Option<Self> {
    match n {
      1 => Some(Month::January),
      2 => Some(Month::February),
      3 => Some(Month::March),
      4 => Some(Month::April),
      5 => Some(Month::May),
      6 => Some(Month::June),
      7 => Some(Month::July),
      8 => Some(Month::August),
      9 => Some(Month::September),
      10 => Some(Month::October),
      11 => Some(Month::November),
      12 => Some(Month::December),
      _ => None,
    }
  }

  fn from_u64(n: u64) -> Option<Self> {
    match n {
      1 => Some(Month::January),
      2 => Some(Month::February),
      3 => Some(Month::March),
      4 => Some(Month::April),
      5 => Some(Month::May),
      6 => Some(Month::June),
      7 => Some(Month::July),
      8 => Some(Month::August),
      9 => Some(Month::September),
      10 => Some(Month::October),
      11 => Some(Month::November),
      12 => Some(Month::December),
      _ => None,
    }
  }
}

pub(crate) fn is_leap_year(year: i32) -> bool {
  return year % 4 == 0 && (year % 100 != 0 || (year % 100 == 0 && year % 400 == 0));
}
