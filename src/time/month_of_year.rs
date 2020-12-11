use std::collections::HashSet;

use chrono::{Datelike, Utc};
use num::ToPrimitive;
use rust_decimal::prelude::FromPrimitive;

use crate::time::{DayOfMonth, is_leap_year};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct MonthOfYear {
  last_day: DayOfMonth,
  value: Month,
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

impl ToPrimitive for Month {
  fn to_i64(&self) -> Option<i64> {
    match *self {
      Month::JANUARY => Some(1),
      Month::FEBRUARY => Some(2),
      Month::MARCH => Some(3),
      Month::APRIL => Some(4),
      Month::MAY => Some(5),
      Month::JUNE => Some(6),
      Month::JULY => Some(7),
      Month::AUGUST => Some(8),
      Month::SEPTEMBER => Some(9),
      Month::OCTOBER => Some(10),
      Month::NOVEMBER => Some(11),
      Month::DECEMBER => Some(12),
    }
  }

  fn to_u64(&self) -> Option<u64> {
    match *self {
      Month::JANUARY => Some(1),
      Month::FEBRUARY => Some(2),
      Month::MARCH => Some(3),
      Month::APRIL => Some(4),
      Month::MAY => Some(5),
      Month::JUNE => Some(6),
      Month::JULY => Some(7),
      Month::AUGUST => Some(8),
      Month::SEPTEMBER => Some(9),
      Month::OCTOBER => Some(10),
      Month::NOVEMBER => Some(11),
      Month::DECEMBER => Some(12),
    }
  }
}

impl FromPrimitive for Month {
  fn from_i64(n: i64) -> Option<Self> {
    match n {
      1 => Some(Month::JANUARY),
      2 => Some(Month::FEBRUARY),
      3 => Some(Month::MARCH),
      4 => Some(Month::APRIL),
      5 => Some(Month::MAY),
      6 => Some(Month::JUNE),
      7 => Some(Month::JULY),
      8 => Some(Month::AUGUST),
      9 => Some(Month::SEPTEMBER),
      10 => Some(Month::OCTOBER),
      11 => Some(Month::NOVEMBER),
      12 => Some(Month::DECEMBER),
      _ => None,
    }
  }

  fn from_u64(n: u64) -> Option<Self> {
    match n {
      1 => Some(Month::JANUARY),
      2 => Some(Month::FEBRUARY),
      3 => Some(Month::MARCH),
      4 => Some(Month::APRIL),
      5 => Some(Month::MAY),
      6 => Some(Month::JUNE),
      7 => Some(Month::JULY),
      8 => Some(Month::AUGUST),
      9 => Some(Month::SEPTEMBER),
      10 => Some(Month::OCTOBER),
      11 => Some(Month::NOVEMBER),
      12 => Some(Month::DECEMBER),
      _ => None,
    }
  }
}

impl MonthOfYear {
  pub const JAN: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(31),
    value: Month::JANUARY,
  };
  pub const FEB: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(28),
    value: Month::FEBRUARY,
  };
  pub const MAR: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(31),
    value: Month::MARCH,
  };
  pub const APR: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(30),
    value: Month::APRIL,
  };
  pub const MAY: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(31),
    value: Month::MAY,
  };
  pub const JUN: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(30),
    value: Month::JUNE,
  };
  pub const JUL: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(31),
    value: Month::JULY,
  };
  pub const AUG: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(31),
    value: Month::AUGUST,
  };
  pub const SEP: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(30),
    value: Month::SEPTEMBER,
  };
  pub const OCT: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(31),
    value: Month::OCTOBER,
  };
  pub const NOV: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(30),
    value: Month::NOVEMBER,
  };
  pub const DEC: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(31),
    value: Month::DECEMBER,
  };

  pub fn new(last_day_of_this_month: DayOfMonth, calendar_value: Month) -> Self {
    Self {
      last_day: last_day_of_this_month,
      value: calendar_value,
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
    hash.iter().cloned().find(|e| e.value == month).unwrap()
  }

  pub fn breach_encapsulation_of_value(&self) -> Month {
    self.value.clone()
  }

  pub fn breach_encapsulation_of_last_day(&self) -> DayOfMonth {
    if self.value == Month::FEBRUARY && is_leap_year(Utc::today().year()) {
      DayOfMonth::new(29)
    } else {
      self.last_day.clone()
    }
  }

  pub fn add_with_overflow(&self) -> (Self, bool) {
    let month_num = self.value.to_i64().unwrap() + 1i64;
    if month_num > Month::DECEMBER.to_i64().unwrap() {
      let m = Month::from_i64(month_num - Month::DECEMBER.to_i64().unwrap()).unwrap();
      (Self::from_month(m), true)
    } else {
      let m = Month::from_i64(month_num).unwrap();
      (Self::from_month(m), false)
    }
  }
}
