#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TimeUnitConversionFactor {
  name: &'static str,
  pub value: i64,
}

impl TimeUnitConversionFactor {
  pub const IDENTICAL: TimeUnitConversionFactor = TimeUnitConversionFactor {
    name: "identical",
    value: 1,
  };
  pub const MILLISECONDS_PER_SECOND: TimeUnitConversionFactor = TimeUnitConversionFactor {
    name: "millisecondsPerSecond",
    value: 1000,
  };
  pub const MILLISECONDS_PER_MINUTE: TimeUnitConversionFactor = TimeUnitConversionFactor {
    name: "millisecondsPerMinute",
    value: 60 * TimeUnitConversionFactor::MILLISECONDS_PER_SECOND.value,
  };
  pub const MILLISECONDS_PER_HOUR: TimeUnitConversionFactor = TimeUnitConversionFactor {
    name: "millisecondsPerHour",
    value: 60 * TimeUnitConversionFactor::MILLISECONDS_PER_MINUTE.value,
  };
  pub const MILLISECONDS_PER_DAY: TimeUnitConversionFactor = TimeUnitConversionFactor {
    name: "millisecondsPerDay",
    value: 24 * TimeUnitConversionFactor::MILLISECONDS_PER_HOUR.value,
  };
  pub const MILLISECONDS_PER_WEEK: TimeUnitConversionFactor = TimeUnitConversionFactor {
    name: "millisecondsPerWeek",
    value: 7 * TimeUnitConversionFactor::MILLISECONDS_PER_DAY.value,
  };
  pub const MONTHS_PER_QUARTER: TimeUnitConversionFactor = TimeUnitConversionFactor {
    name: "monthsPerQuarter",
    value: 3,
  };
  pub const MONTHS_PER_YEAR: TimeUnitConversionFactor = TimeUnitConversionFactor {
    name: "monthsPerYear",
    value: 12,
  };
}
