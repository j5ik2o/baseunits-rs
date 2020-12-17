use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use chrono::{Datelike, Utc};
use num::ToPrimitive;
use rust_decimal::prelude::FromPrimitive;

use crate::time::{DayOfMonth, is_leap_year, Month, CalendarYearMonth};

#[derive(Debug, Clone, Eq)]
pub struct MonthOfYear {
  last_day: DayOfMonth,
  value: Month,
}

impl PartialEq for MonthOfYear {
  fn eq(&self, other: &Self) -> bool {
    self.value.eq(&other.value)
  }
}

impl PartialOrd for MonthOfYear {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.value.partial_cmp(&other.value)
  }
}

impl Hash for MonthOfYear {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.value.hash(state);
  }
}

impl MonthOfYear {
  pub const JAN: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(31),
    value: Month::January,
  };
  pub const FEB: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(28),
    value: Month::February,
  };
  pub const MAR: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(31),
    value: Month::March,
  };
  pub const APR: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(30),
    value: Month::April,
  };
  pub const MAY: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(31),
    value: Month::May,
  };
  pub const JUN: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(30),
    value: Month::June,
  };
  pub const JUL: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(31),
    value: Month::July,
  };
  pub const AUG: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(31),
    value: Month::August,
  };
  pub const SEP: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(30),
    value: Month::September,
  };
  pub const OCT: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(31),
    value: Month::October,
  };
  pub const NOV: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(30),
    value: Month::November,
  };
  pub const DEC: MonthOfYear = MonthOfYear {
    last_day: DayOfMonth(31),
    value: Month::December,
  };

  pub fn new(last_day_of_this_month: DayOfMonth, calendar_value: Month) -> Self {
    Self {
      last_day: last_day_of_this_month,
      value: calendar_value,
    }
  }

  pub fn from_month(month: Month) -> Self {
    [
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
    .find(|&e| e.value == month)
    .unwrap()
    .clone()
  }

  pub fn as_value(&self) -> &Month {
    &self.value
  }

  pub const DAY_OF_MONTH_29: DayOfMonth = DayOfMonth(29);

  pub fn as_last_day(&self) -> &DayOfMonth {
    if self.value == Month::February && is_leap_year(Utc::today().year()) {
      &MonthOfYear::DAY_OF_MONTH_29
    } else {
      &self.last_day
    }
  }

  pub fn add_with_overflow(&self) -> (Self, bool) {
    let month_num = self.value.to_i64().unwrap() + 1i64;
    if month_num > Month::December.to_i64().unwrap() {
      let m = Month::from_i64(month_num - Month::December.to_i64().unwrap()).unwrap();
      (Self::from_month(m), true)
    } else {
      let m = Month::from_i64(month_num).unwrap();
      (Self::from_month(m), false)
    }
  }

  pub fn on(self, year: i32) -> CalendarYearMonth {
    CalendarYearMonth::new(year, self)
  }

  pub fn is_after(&self, other: &Self) -> bool {
    !self.is_before(other) && self != other
  }

  pub fn is_before(&self, other: &Self) -> bool {
    self.value < other.value
  }
}

#[cfg(test)]
mod tests {
  use crate::time::{MonthOfYear, Month, DayOfMonth};

  #[test]
  fn test_from_month() {
    let moy = MonthOfYear::from_month(Month::January);
    assert_eq!(moy.value, Month::January);
    assert_eq!(moy.last_day, DayOfMonth::new(31))
  }
}
