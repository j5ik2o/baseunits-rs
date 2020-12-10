use crate::time::DayOfMonth;
use chrono::{Utc, Datelike};
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct MonthOfYear {
  last_day_of_this_month: DayOfMonth,
  calendar_value: Month,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub enum Month {
  JANUARY,
  FEBRUARY,
  MARCH,
  APRIL,
  MAY,
  JUNE,
  JULY,
  AUGUST,
  SEPTEMBER,
  OCTOBER,
  NOVEMBER,
  DECEMBER,
}

impl MonthOfYear {
  pub const JAN: MonthOfYear = MonthOfYear {
    last_day_of_this_month: DayOfMonth(31),
    calendar_value: Month::JANUARY,
  };
  pub const FEB: MonthOfYear = MonthOfYear {
    last_day_of_this_month: DayOfMonth(28),
    calendar_value: Month::FEBRUARY,
  };
  pub const MAR: MonthOfYear = MonthOfYear {
    last_day_of_this_month: DayOfMonth(31),
    calendar_value: Month::MARCH,
  };
  pub const APR: MonthOfYear = MonthOfYear {
    last_day_of_this_month: DayOfMonth(30),
    calendar_value: Month::APRIL,
  };
  pub const MAY: MonthOfYear = MonthOfYear {
    last_day_of_this_month: DayOfMonth(31),
    calendar_value: Month::MAY,
  };
  pub const JUN: MonthOfYear = MonthOfYear {
    last_day_of_this_month: DayOfMonth(30),
    calendar_value: Month::JUNE,
  };
  pub const JUL: MonthOfYear = MonthOfYear {
    last_day_of_this_month: DayOfMonth(31),
    calendar_value: Month::JULY,
  };
  pub const AUG: MonthOfYear = MonthOfYear {
    last_day_of_this_month: DayOfMonth(31),
    calendar_value: Month::AUGUST,
  };
  pub const SEP: MonthOfYear = MonthOfYear {
    last_day_of_this_month: DayOfMonth(30),
    calendar_value: Month::SEPTEMBER,
  };
  pub const OCT: MonthOfYear = MonthOfYear {
    last_day_of_this_month: DayOfMonth(31),
    calendar_value: Month::OCTOBER,
  };
  pub const NOV: MonthOfYear = MonthOfYear {
    last_day_of_this_month: DayOfMonth(30),
    calendar_value: Month::NOVEMBER,
  };
  pub const DEC: MonthOfYear = MonthOfYear {
    last_day_of_this_month: DayOfMonth(31),
    calendar_value: Month::DECEMBER,
  };

  pub fn new(last_day_of_this_month: DayOfMonth, calendar_value: Month) -> Self {
    Self {
      last_day_of_this_month,
      calendar_value,
    }
  }

  pub fn from_month(month: Month) -> Self {
    let mut hash: HashSet<MonthOfYear> = HashSet::new();
    for e in [
      MonthOfYear::JAN,
      MonthOfYear::FEB,
      MonthOfYear::MAR,
      MonthOfYear::APR,
      MonthOfYear::MAY,
      MonthOfYear::JUN,
      MonthOfYear::JUL,
      MonthOfYear::AUG,
      MonthOfYear::SEP,
      MonthOfYear::OCT,
      MonthOfYear::NOV,
      MonthOfYear::DEC,
    ]
    .iter()
    {
      hash.insert(e.clone());
    }
    hash
      .iter()
      .cloned()
      .find(|e| e.calendar_value == month)
      .unwrap()
  }

  fn is_leap_year(year: i32) -> bool {
    return year % 4 == 0 && (year % 100 != 0 || (year % 100 == 0 && year % 400 == 0));
  }

  pub fn last_day_of_this_month(&self) -> DayOfMonth {
    if self.calendar_value == Month::FEBRUARY {
      if Self::is_leap_year(Utc::today().year()) {
        DayOfMonth::new(29)
      } else {
        self.last_day_of_this_month.clone()
      }
    } else {
      self.last_day_of_this_month.clone()
    }
  }
}
