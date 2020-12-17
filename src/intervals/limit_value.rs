use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum LimitValue<T> {
  Limit(T),
  Limitless,
}

impl<T: Clone + Default> Default for LimitValue<T> {
  fn default() -> Self {
    LimitValue::Limitless
  }
}

impl<T: Clone + Default + PartialOrd> PartialOrd for LimitValue<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match (self, other) {
      (LimitValue::Limit(ref value), LimitValue::Limit(ref other_value)) => {
        value.partial_cmp(other_value)
      }
      (LimitValue::Limit(_), _) => Some(Ordering::Greater),
      (LimitValue::Limitless, &LimitValue::Limitless) => Some(Ordering::Equal),
      (LimitValue::Limitless, _) => Some(Ordering::Less),
    }
  }
}

impl<T: Clone + Default + PartialEq + PartialOrd> PartialEq for LimitValue<T> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (&LimitValue::Limitless, &LimitValue::Limitless) => true,
      (&LimitValue::Limit(ref value), &LimitValue::Limit(ref other_value)) => value == other_value,
      (..) => false,
    }
  }
}

impl<T: Clone + Default + PartialOrd> Eq for LimitValue<T> {}

impl<T: Clone + Default> LimitValue<T> {
  pub fn to_value(&self) -> &T {
    match self {
      LimitValue::Limit(ref t) => t,
      LimitValue::Limitless => panic!(""),
    }
  }

  pub fn to_value_or_else<'a, 'b>(&'a self, default: &'b T) -> &'a T
  where
    'b: 'a,
  {
    match self {
      LimitValue::Limit(ref t) => t,
      LimitValue::Limitless => default,
    }
  }
}
