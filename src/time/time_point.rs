use chrono::{DateTime, TimeZone, Utc};
use time::{Timespec, Tm};

/// TimePoint
#[derive(Debug, Clone, PartialEq)]
pub struct TimePoint(i64);

impl From<i64> for TimePoint {
  fn from(milliseconds: i64) -> Self {
    TimePoint::new(milliseconds)
  }
}

impl TimePoint {
  pub fn new(epoc: i64) -> Self {
    Self(epoc)
  }
  pub fn as_tm(&self) -> Tm {
    time::at(Timespec::new(self.0, 0))
  }

  pub fn as_utc(&self) -> DateTime<Utc> {
    Utc.timestamp_millis(self.0)
  }
}
