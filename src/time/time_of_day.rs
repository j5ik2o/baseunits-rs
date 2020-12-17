use crate::time::{CalendarDate, CalendarDateTime, HourOfDay, MinuteOfHour};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct TimeOfDay {
  hour: HourOfDay,
  minute: MinuteOfHour,
}

impl From<(u32, u32)> for TimeOfDay {
  fn from((hour, minute): (u32, u32)) -> Self {
    TimeOfDay::new(HourOfDay::new(hour), MinuteOfHour::new(minute))
  }
}

impl TimeOfDay {
  pub fn new(hour: HourOfDay, minute: MinuteOfHour) -> Self {
    Self { hour, minute }
  }

  pub fn breach_encapsulation_of_hour(&self) -> &HourOfDay {
    &self.hour
  }

  pub fn breach_encapsulation_of_minute(&self) -> &MinuteOfHour {
    &self.minute
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

#[cfg(test)]
mod tests {
  extern crate once_cell;

  use std::collections::hash_map::DefaultHasher;
  use std::hash::{Hash, Hasher};

  use once_cell::sync::Lazy;

  use crate::time::{CalendarDate, CalendarDateTime, HourOfDay, MinuteOfHour, TimeOfDay};

  static FEB17: Lazy<CalendarDate> = Lazy::new(|| CalendarDate::from((2006, 2, 17)));
  static MIDNIGHT: Lazy<TimeOfDay> = Lazy::new(|| TimeOfDay::from((0, 0)));
  static MORNING: Lazy<TimeOfDay> = Lazy::new(|| TimeOfDay::from((10, 20)));
  static NOON: Lazy<TimeOfDay> = Lazy::new(|| TimeOfDay::from((12, 0)));
  static AFTERNOON: Lazy<TimeOfDay> = Lazy::new(|| TimeOfDay::from((15, 40)));
  static TWO_MINUTES_BEFORE_MIDNIGHT: Lazy<TimeOfDay> = Lazy::new(|| TimeOfDay::from((23, 58)));
  static TEN_MINUTES_BEFORE_MIDNIGHT: Lazy<TimeOfDay> = Lazy::new(|| TimeOfDay::from((23, 50)));

  #[test]
  fn test01_on_start_of_day() {
    let feb17at_start_of_day = CalendarDateTime::from((2006, 2, 17, 0, 0));
    assert_eq!(MIDNIGHT.clone().on(FEB17.clone()), feb17at_start_of_day)
  }

  #[test]
  fn test02_on_middle_of_day() {
    let feb17at_middle_of_day = CalendarDateTime::from((2006, 2, 17, 12, 0));
    assert_eq!(NOON.clone().on(FEB17.clone()), feb17at_middle_of_day)
  }

  #[test]
  fn test03_on_end_of_day() {
    let feb17at_end_of_day = CalendarDateTime::from((2006, 2, 17, 23, 58));
    assert_eq!(
      TWO_MINUTES_BEFORE_MIDNIGHT.clone().on(FEB17.clone()),
      feb17at_end_of_day
    )
  }

  #[test]
  fn test04_equals() {
    assert_eq!(MIDNIGHT.clone(), TimeOfDay::from((0, 0)));
    assert_eq!(MORNING.clone(), TimeOfDay::from((10, 20)));
    assert_eq!(NOON.clone(), TimeOfDay::from((12, 0)));
    assert_eq!(AFTERNOON.clone(), TimeOfDay::from((15, 40)));
    assert_eq!(
      TWO_MINUTES_BEFORE_MIDNIGHT.clone(),
      TimeOfDay::from((23, 58))
    );
    assert_ne!(MIDNIGHT.clone(), MORNING.clone());
    assert_ne!(
      TEN_MINUTES_BEFORE_MIDNIGHT.clone(),
      TWO_MINUTES_BEFORE_MIDNIGHT.clone()
    );
    assert_eq!(
      NOON.clone(),
      TimeOfDay::new(HourOfDay::new(12), MinuteOfHour::new(0))
    );
    //    assert(noon.equals(new TimeOfDay(HourOfDay(12), MinuteOfHour(0)) {
    //    }) == false)
  }

  #[test]
  fn test05_hash_code() {
    let mut hasher1 = DefaultHasher::new();
    MIDNIGHT.clone().hash(&mut hasher1);
    let hash1 = hasher1.finish();
    let mut hasher2 = DefaultHasher::new();
    TimeOfDay::from((0, 0)).hash(&mut hasher2);
    let hash2 = hasher2.finish();
    assert_eq!(hash1, hash2);

    let mut hasher1 = DefaultHasher::new();
    MORNING.clone().hash(&mut hasher1);
    let hash1 = hasher1.finish();
    let mut hasher2 = DefaultHasher::new();
    TimeOfDay::from((10, 20)).hash(&mut hasher2);
    let hash2 = hasher2.finish();
    assert_eq!(hash1, hash2);

    let mut hasher1 = DefaultHasher::new();
    NOON.clone().hash(&mut hasher1);
    let hash1 = hasher1.finish();
    let mut hasher2 = DefaultHasher::new();
    TimeOfDay::from((12, 0)).hash(&mut hasher2);
    let hash2 = hasher2.finish();
    assert_eq!(hash1, hash2);

    let mut hasher1 = DefaultHasher::new();
    AFTERNOON.clone().hash(&mut hasher1);
    let hash1 = hasher1.finish();
    let mut hasher2 = DefaultHasher::new();
    TimeOfDay::from((15, 40)).hash(&mut hasher2);
    let hash2 = hasher2.finish();
    assert_eq!(hash1, hash2);

    let mut hasher1 = DefaultHasher::new();
    TWO_MINUTES_BEFORE_MIDNIGHT.clone().hash(&mut hasher1);
    let hash1 = hasher1.finish();
    let mut hasher2 = DefaultHasher::new();
    TimeOfDay::from((23, 58)).hash(&mut hasher2);
    let hash2 = hasher2.finish();
    assert_eq!(hash1, hash2);
  }

  #[test]
  fn test06_after_with_earlier_time_of_day() {
    assert!(
      TWO_MINUTES_BEFORE_MIDNIGHT.is_after(&MIDNIGHT),
      "expected TWO_MINUTES_BEFORE_MIDNIGHT to be after MIDNIGHT"
    );
    assert!(
      AFTERNOON.is_after(&MORNING),
      "expected AFTERNOON to be after MORNING"
    );
    assert!(
      NOON.is_after(&MIDNIGHT),
      "expected NOON to be after MIDNIGHT"
    );
  }

  #[test]
  fn test07_after_with_later_time_of_day() {
    assert_eq!(
      MIDNIGHT.is_after(&TWO_MINUTES_BEFORE_MIDNIGHT),
      false,
      "expected MIDNIGHT not after TWO_MINUTES_BEFORE_MIDNIGHT"
    );
    assert_eq!(
      MORNING.is_after(&AFTERNOON),
      false,
      "expected MORNING not after AFTERNOON"
    );
    assert_eq!(
      NOON.is_after(&TWO_MINUTES_BEFORE_MIDNIGHT),
      false,
      "expected NOON not after TWO_MINUTES_BEFORE_MIDNIGHT"
    )
  }

  #[test]
  fn test08_after_with_same_time_of_day() {
    assert_eq!(
      MIDNIGHT.is_after(&MIDNIGHT),
      false,
      "expected MIDNIGHT not after MIDNIGHT"
    );
    assert_eq!(
      MIDNIGHT.lt(&MIDNIGHT),
      false,
      "expected MIDNIGHT not after MIDNIGHT"
    );
    assert_eq!(
      MORNING.is_after(&MORNING),
      false,
      "expected MORNING not after MORNING"
    );
    assert_eq!(
      MORNING.lt(&MORNING),
      false,
      "expected MORNING not after MORNING"
    );
    assert_eq!(
      AFTERNOON.is_after(&AFTERNOON),
      false,
      "expected AFTERNOON not after AFTERNOON"
    );
    assert_eq!(
      AFTERNOON.lt(&AFTERNOON),
      false,
      "expected AFTERNOON not after AFTERNOON"
    );
    assert_eq!(NOON.is_after(&NOON), false, "expected NOON not after NOON");
    assert_eq!(NOON.lt(&NOON), false, "expected NOON not after NOON");
  }

  #[test]
  fn test09_before_with_earlier_time_of_day() {
    assert_eq!(
      TWO_MINUTES_BEFORE_MIDNIGHT.is_before(&MIDNIGHT),
      false,
      "expected twoMinutesBeforeMidnight not after midnight"
    );
    assert_eq!(
      AFTERNOON.is_before(&MORNING),
      false,
      "expected afternoon not after morning"
    );
    assert_eq!(
      NOON.is_before(&MIDNIGHT),
      false,
      "expected noon not after midnight"
    );
  }

  #[test]
  fn test10_before_with_later_time_of_day() {
    assert!(
      MIDNIGHT.is_before(&TWO_MINUTES_BEFORE_MIDNIGHT),
      "expected midnight not after twoMinutesBeforeMidnight"
    );
    assert!(
      MORNING.is_before(&AFTERNOON),
      "expected morning not after afternoon"
    );
    assert!(
      NOON.is_before(&TWO_MINUTES_BEFORE_MIDNIGHT),
      "expected noon not after twoMinutesBeforeMidnight"
    );
  }

  fn test11_before_with_same_time_of_day() {
    assert!(
      !MIDNIGHT.is_before(&MIDNIGHT),
      "expected midnight not after midnight"
    );
    assert!(
      !MORNING.is_before(&MORNING),
      "expected morning not after morning"
    );
    assert!(
      !AFTERNOON.is_before(&AFTERNOON),
      "expected afternoon not after afternoon"
    );
    assert!(!NOON.is_before(&NOON), "expected NOON not after NOON");
  }

  #[test]
  fn test12_get_hour() {
    assert_eq!(*MIDNIGHT.breach_encapsulation_of_hour(), HourOfDay::new(0));
    assert_eq!(*MORNING.breach_encapsulation_of_hour(), HourOfDay::new(10));
    assert_eq!(*NOON.breach_encapsulation_of_hour(), HourOfDay::new(12));
    assert_eq!(
      *AFTERNOON.breach_encapsulation_of_hour(),
      HourOfDay::new(15)
    );
    assert_eq!(
      *TWO_MINUTES_BEFORE_MIDNIGHT.breach_encapsulation_of_hour(),
      HourOfDay::new(23)
    );
  }

  #[test]
  fn test13_get_minute() {
    assert_eq!(
      *MIDNIGHT.breach_encapsulation_of_minute(),
      MinuteOfHour::new(0)
    );
    assert_eq!(
      *MORNING.breach_encapsulation_of_minute(),
      MinuteOfHour::new(20)
    );
    assert_eq!(*NOON.breach_encapsulation_of_minute(), MinuteOfHour::new(0));
    assert_eq!(
      *AFTERNOON.breach_encapsulation_of_minute(),
      MinuteOfHour::new(40)
    );
    assert_eq!(
      *TWO_MINUTES_BEFORE_MIDNIGHT.breach_encapsulation_of_minute(),
      MinuteOfHour::new(58)
    );
  }

  // fn test14_as_time_point() {
  //   let five_fifteen = TimeOfDay::from((17, 15));
  //   let may_eleventh = CalendarDate::from((2006, 5, 11));
  //   let may_eleventh_at_five_fifteen = five_fifteen.asTimePointGiven(may_eleventh, CST.toZoneId);
  //   assert_eq!(
  //     may_eleventh_at_five_fifteen,
  //     TimePoint::at(2006, 5, 11, 17, 15, 0, 0)
  //   );
  // }
}
