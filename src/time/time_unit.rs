use crate::time::time_unit_conversion_factor::TimeUnitConversionFactor;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TimeUnitType {
  Nanoseconds,
  Microseconds,
  Milliseconds,
  Seconds,
  Minutes,
  Hours,
  Days,
  Weeks,
  Months,
  Quarters,
  Years,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TimeUnit {
  pub name: &'static str,
  value_type: TimeUnitType,
  value_base_type: TimeUnitType,
  factor: TimeUnitConversionFactor,
}

impl TimeUnit {
  /// この単位で表される値を、指定した単位に変換できるかどうかを検証する。
  /// 例えば、分単位はミリ秒単位に変換できるが、四半期単位は（一ヶ月の長さが毎月異なるため）日単位に変換できない。
  ///
  /// `param` other 変換先単位
  /// `return` 変換できる場合は`true`、そうでない場合は`false`
  pub fn is_convertible_to(&self, other: &Self) -> bool {
    self.value_base_type == other.value_base_type
  }

  /// この単位で表される値を、ミリ秒単位に変換できるかどうかを検証する。
  /// 例えば、分単位はミリ秒単位に変換できるが、四半期単位は（一ヶ月の長さが毎月異なるため）ミリ秒単位に変換できない。
  ///
  /// `return` 変換できる場合は`true`、そうでない場合は`false`
  pub fn is_convertible_to_milliseconds(&self) -> bool {
    self.is_convertible_to(&TimeUnit::MILLISECOND)
  }

  /// この単位の計数の基数とすることができる最小の単位を取得する。
  /// 例えば、分単位はミリ秒単位で計数できるが、四半期単位は（一ヶ月の長さが毎月異なるため）月単位までしか計数できない。
  ///
  /// `return` この単位の計数の基数とすることができる最小の単位
  pub fn base_unit(&self) -> TimeUnit {
    if self.value_base_type == TimeUnitType::Milliseconds {
      TimeUnit::MILLISECOND
    } else {
      TimeUnit::MONTH
    }
  }

  pub fn descending_units(&self) -> Vec<TimeUnit> {
    if self.is_convertible_to_milliseconds() {
      Vec::from(TimeUnit::DESCENDING_MS_BASED)
    } else {
      Vec::from(TimeUnit::DESCENDING_MONTH_BASED)
    }
  }

  pub fn descending_units_for_display(&self) -> Vec<TimeUnit> {
    if self.is_convertible_to_milliseconds() {
      Vec::from(TimeUnit::DESCENDING_MS_BASED_FOR_DISPLAY)
    } else {
      Vec::from(TimeUnit::DESCENDING_MONTH_BASED_FOR_DISPLAY)
    }
  }

  pub(crate) fn get_factor(&self) -> i64 {
    self.factor.value
  }

  pub fn next_finer_unit(&self) -> Option<TimeUnit> {
    self
      .descending_units()
      .iter()
      .zip(0..self.descending_units().len())
      .find(|(t, _)| *t == self)
      .and_then(|(_, i)| {
        if i == self.descending_units().len() - 1 {
          None
        } else {
          self.descending_units().get(i + 1).cloned()
        }
      })
  }
}

impl TimeUnit {
  const DESCENDING_MS_BASED: [TimeUnit; 6] = [
    TimeUnit::WEEK,
    TimeUnit::DAY,
    TimeUnit::HOUR,
    TimeUnit::MINUTE,
    TimeUnit::SECOND,
    TimeUnit::MILLISECOND,
  ];

  const DESCENDING_MS_BASED_FOR_DISPLAY: [TimeUnit; 5] = [
    TimeUnit::DAY,
    TimeUnit::HOUR,
    TimeUnit::MINUTE,
    TimeUnit::SECOND,
    TimeUnit::MILLISECOND,
  ];

  const DESCENDING_MONTH_BASED: [TimeUnit; 3] =
    [TimeUnit::YEAR, TimeUnit::QUARTER, TimeUnit::MONTH];

  const DESCENDING_MONTH_BASED_FOR_DISPLAY: [TimeUnit; 2] = [TimeUnit::YEAR, TimeUnit::MONTH];

  pub const MILLISECOND: TimeUnit = TimeUnit {
    name: "millisecond",
    value_type: TimeUnitType::Milliseconds,
    value_base_type: TimeUnitType::Milliseconds,
    factor: TimeUnitConversionFactor::IDENTICAL,
  };

  pub const SECOND: TimeUnit = TimeUnit {
    name: "second",
    value_type: TimeUnitType::Seconds,
    value_base_type: TimeUnitType::Milliseconds,
    factor: TimeUnitConversionFactor::MILLISECONDS_PER_SECOND,
  };

  pub const MINUTE: TimeUnit = TimeUnit {
    name: "minute",
    value_type: TimeUnitType::Minutes,
    value_base_type: TimeUnitType::Milliseconds,
    factor: TimeUnitConversionFactor::MILLISECONDS_PER_MINUTE,
  };

  pub const HOUR: TimeUnit = TimeUnit {
    name: "hour",
    value_type: TimeUnitType::Hours,
    value_base_type: TimeUnitType::Milliseconds,
    factor: TimeUnitConversionFactor::MILLISECONDS_PER_HOUR,
  };

  pub const DAY: TimeUnit = TimeUnit {
    name: "day",
    value_type: TimeUnitType::Days,
    value_base_type: TimeUnitType::Milliseconds,
    factor: TimeUnitConversionFactor::MILLISECONDS_PER_DAY,
  };

  pub const WEEK: TimeUnit = TimeUnit {
    name: "week",
    value_type: TimeUnitType::Weeks,
    value_base_type: TimeUnitType::Milliseconds,
    factor: TimeUnitConversionFactor::MILLISECONDS_PER_WEEK,
  };

  pub const MONTH: TimeUnit = TimeUnit {
    name: "month",
    value_type: TimeUnitType::Months,
    value_base_type: TimeUnitType::Months,
    factor: TimeUnitConversionFactor::IDENTICAL,
  };

  pub const QUARTER: TimeUnit = TimeUnit {
    name: "quarter",
    value_type: TimeUnitType::Quarters,
    value_base_type: TimeUnitType::Months,
    factor: TimeUnitConversionFactor::MONTHS_PER_QUARTER,
  };

  pub const YEAR: TimeUnit = TimeUnit {
    name: "year",
    value_type: TimeUnitType::Years,
    value_base_type: TimeUnitType::Months,
    factor: TimeUnitConversionFactor::MONTHS_PER_YEAR,
  };
}

#[cfg(test)]
mod tests {
  use crate::time::TimeUnit;
  use std::cmp::Ordering;

  #[test]
  fn test_convertible_to_milliseconds() {
    assert_eq!(TimeUnit::MILLISECOND.is_convertible_to_milliseconds(), true);
    assert_eq!(TimeUnit::HOUR.is_convertible_to_milliseconds(), true);
    assert_eq!(TimeUnit::DAY.is_convertible_to_milliseconds(), true);
    assert_eq!(TimeUnit::WEEK.is_convertible_to_milliseconds(), true);
    assert_eq!(TimeUnit::MONTH.is_convertible_to_milliseconds(), false);
    assert_eq!(TimeUnit::YEAR.is_convertible_to_milliseconds(), false);
  }

  #[test]
  fn test_comparison() {
    assert_eq!(TimeUnit::HOUR.cmp(&TimeUnit::HOUR), Ordering::Equal);
    assert_eq!(TimeUnit::HOUR.cmp(&TimeUnit::MILLISECOND), Ordering::Less);
    assert_eq!(
      TimeUnit::MILLISECOND.cmp(&TimeUnit::HOUR),
      Ordering::Greater
    );
    assert_eq!(TimeUnit::DAY.cmp(&TimeUnit::HOUR), Ordering::Less);
    assert_eq!(TimeUnit::HOUR.cmp(&TimeUnit::DAY), Ordering::Greater);

    assert_eq!(TimeUnit::MONTH.cmp(&TimeUnit::DAY), Ordering::Greater);
    assert_eq!(TimeUnit::DAY.cmp(&TimeUnit::MONTH), Ordering::Less);
    assert_eq!(TimeUnit::QUARTER.cmp(&TimeUnit::HOUR), Ordering::Greater);

    assert_eq!(TimeUnit::MONTH.cmp(&TimeUnit::MONTH), Ordering::Equal);
    assert_eq!(TimeUnit::QUARTER.cmp(&TimeUnit::YEAR), Ordering::Less);
    assert_eq!(TimeUnit::YEAR.cmp(&TimeUnit::QUARTER), Ordering::Greater);
  }

  #[test]
  fn test_is_convertible_to() {
    assert_eq!(TimeUnit::HOUR.is_convertible_to(&TimeUnit::MINUTE), true);
    assert_eq!(TimeUnit::MINUTE.is_convertible_to(&TimeUnit::HOUR), true);
    assert_eq!(TimeUnit::YEAR.is_convertible_to(&TimeUnit::MONTH), true);
    assert_eq!(TimeUnit::MONTH.is_convertible_to(&TimeUnit::YEAR), true);
    assert_eq!(TimeUnit::MONTH.is_convertible_to(&TimeUnit::HOUR), false);
    assert_eq!(TimeUnit::HOUR.is_convertible_to(&TimeUnit::MONTH), false);
  }

  #[test]
  fn test_next_finer_unit() {
    assert_eq!(TimeUnit::HOUR.next_finer_unit(), Some(TimeUnit::MINUTE));
    assert_eq!(TimeUnit::QUARTER.next_finer_unit(), Some(TimeUnit::MONTH));
  }
}
