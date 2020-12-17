use std::cmp::Ordering;
use crate::intervals::LimitValue;

#[derive(Debug, Clone)]
pub struct IntervalLimit<T> {
  pub(crate) closed: bool,
  pub(crate) lower: bool,
  pub(crate) value: LimitValue<T>,
}

impl<T: Default + Clone + PartialOrd> Default for IntervalLimit<T> {
  fn default() -> Self {
    IntervalLimit::new(false, false, LimitValue::default())
  }
}

impl<T: Default + Clone + PartialEq + PartialOrd> PartialOrd for IntervalLimit<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match (&self.value, &other.value) {
      (LimitValue::<T>::Limitless, LimitValue::<T>::Limitless) => {
        if self.lower == other.lower {
          Some(Ordering::Equal)
        } else {
          Some(self.lower_to_ordering(Ordering::Less, Ordering::Greater))
        }
      }
      (LimitValue::<T>::Limitless, _) => {
        Some(self.lower_to_ordering(Ordering::Less, Ordering::Greater))
      }
      (_, LimitValue::<T>::Limitless) => {
        Some(other.lower_to_ordering(Ordering::Greater, Ordering::Less))
      }
      (LimitValue::<T>::Limit(ref lv), LimitValue::<T>::Limit(ref other_lv)) if lv == other_lv => {
        match (self.lower, other.lower) {
          (l, other_l) if l == other_l => {
            if self.closed ^ other.closed {
              Some(self.closed_to_ordering(Ordering::Greater, Ordering::Less))
            } else {
              Some(Ordering::Equal)
            }
          }
          (l, other_l) if !l == !other_l => {
            if self.closed ^ other.closed {
              Some(self.closed_to_ordering(Ordering::Less, Ordering::Greater))
            } else {
              Some(Ordering::Equal)
            }
          }
          _ => self.value.partial_cmp(&other.value),
        }
      }
      _ => self.value.partial_cmp(&other.value),
    }
  }
}

impl<T: Default + Clone + PartialOrd> PartialEq for IntervalLimit<T> {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other).unwrap() == Ordering::Equal
  }
}

impl<T: Default + Clone + PartialEq + PartialOrd> IntervalLimit<T> {
  pub fn new(closed: bool, lower: bool, value: LimitValue<T>) -> Self {
    Self {
      closed,
      lower,
      value,
    }
  }

  pub fn lower(closed: bool, value: LimitValue<T>) -> Self {
    Self::new(closed, true, value)
  }

  pub fn upper(closed: bool, value: LimitValue<T>) -> Self {
    Self::new(closed, false, value)
  }

  fn lower_to_ordering<A>(&self, t: A, f: A) -> A {
    if self.lower {
      t
    } else {
      f
    }
  }

  fn closed_to_ordering<A>(&self, t: A, f: A) -> A {
    if self.closed {
      t
    } else {
      f
    }
  }

  pub fn infinity(&self) -> bool {
    match self.value {
      LimitValue::<T>::Limitless => true,
      LimitValue::<T>::Limit(_) => false,
    }
  }

  pub fn is_open(&self) -> bool {
    !self.closed
  }

  pub fn is_upper(&self) -> bool {
    !self.lower
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::intervals::LimitValue;

  #[test]
  fn test_lower() {
    assert_eq!(
      IntervalLimit::lower(false, LimitValue::Limit(10)),
      IntervalLimit::lower(false, LimitValue::Limit(10))
    );
    assert_ne!(
      IntervalLimit::lower(false, LimitValue::Limit(10)),
      IntervalLimit::lower(true, LimitValue::Limit(10))
    );
    assert_ne!(
      IntervalLimit::lower(true, LimitValue::Limit(10)),
      IntervalLimit::lower(false, LimitValue::Limit(10))
    );
    assert_eq!(
      IntervalLimit::lower(true, LimitValue::Limit(10)),
      IntervalLimit::lower(true, LimitValue::Limit(10))
    );
  }

  #[test]
  fn test_upper() {
    assert_eq!(
      IntervalLimit::upper(false, LimitValue::Limit(10)),
      IntervalLimit::upper(false, LimitValue::Limit(10))
    );
    assert_ne!(
      IntervalLimit::upper(false, LimitValue::Limit(10)),
      IntervalLimit::upper(true, LimitValue::Limit(10))
    );
    assert_ne!(
      IntervalLimit::upper(true, LimitValue::Limit(10)),
      IntervalLimit::upper(false, LimitValue::Limit(10))
    );
    assert_eq!(
      IntervalLimit::upper(true, LimitValue::Limit(10)),
      IntervalLimit::upper(true, LimitValue::Limit(10))
    );
  }
}
