use crate::time::{HourOfDay, MinuteOfHour, CalendarDate, CalendarDateTime};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct TimeOfDay {
  hour: HourOfDay,
  minute: MinuteOfHour,
}
impl TimeOfDay {
  pub fn new(hour: HourOfDay, minute: MinuteOfHour) -> Self {
    Self { hour, minute }
  }

  pub fn from_hour_with_minute(hour: u32, minute: u32) -> Self {
    TimeOfDay::new(HourOfDay::new(hour), MinuteOfHour::new(minute))
  }

  pub fn on(self, date: CalendarDate) -> CalendarDateTime {
    CalendarDateTime::new(date, self)
  }

  pub fn is_after(&self, other: &Self) -> bool {
    !self.is_before(other) && self != other
  }

  pub fn is_before(&self, other: &Self) -> bool {
    self.hour.is_before(&other.hour)
      || (self.hour == other.hour && self.minute.is_before(&other.minute))
  }
}
