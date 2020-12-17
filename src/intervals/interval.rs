use std::cmp::Ordering;

use crate::intervals::{IntervalLimit, LimitValue};
use rust_fp_categories::empty::Empty;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Interval<T> {
  lower: IntervalLimit<T>,
  upper: IntervalLimit<T>,
}

impl<T: Debug + Default + Clone + PartialOrd> Default for Interval<T> {
  fn default() -> Self {
    Interval::new(IntervalLimit::default(), IntervalLimit::default())
  }
}

impl<T: Debug + Default + Clone + PartialEq + PartialOrd> PartialEq for Interval<T> {
  fn eq(&self, other: &Self) -> bool {
    self.lower.partial_cmp(&other.lower) == Some(Ordering::Equal)
      && self.upper.partial_cmp(&other.upper) == Some(Ordering::Equal)
  }
}

impl<T: Debug + Default + Clone + PartialEq + PartialOrd>
  From<(LimitValue<T>, bool, LimitValue<T>, bool)> for Interval<T>
{
  fn from(
    (lower, is_lower_closed, upper, is_upper_closed): (LimitValue<T>, bool, LimitValue<T>, bool),
  ) -> Self {
    Self::new(
      IntervalLimit::lower(is_lower_closed, lower),
      IntervalLimit::upper(is_upper_closed, upper),
    )
  }
}

impl<T: Debug + Default + Clone + PartialEq + PartialOrd> Empty for Interval<T> {
  fn empty() -> Self {
    let lower = IntervalLimit::lower(false, LimitValue::Limit(T::default()));
    let upper = IntervalLimit::upper(false, LimitValue::Limit(T::default()));
    Self::new(lower, upper)
  }

  fn is_empty(&self) -> bool {
    match (self.upper_limit(), self.lower_limit()) {
      (&LimitValue::Limitless, &LimitValue::Limitless) => false,
      (..) => self.is_open() && self.upper_limit() == self.lower_limit(),
    }
  }
}

impl<T: Debug + Default + Clone + PartialEq + PartialOrd> Interval<T> {
  fn check_lower_is_less_than_or_equal_upper(lower: &IntervalLimit<T>, upper: &IntervalLimit<T>) {
    if !(lower.lower && upper.is_upper() && lower.partial_cmp(upper) == Some(Ordering::Less)) {
      panic!("{:?} is not before or equal to {:?}", lower, upper)
    }
  }
  pub fn new(lower: IntervalLimit<T>, upper: IntervalLimit<T>) -> Self {
    Self::check_lower_is_less_than_or_equal_upper(&lower, &upper);
    let (new_lower, new_upper) = if !upper.infinity()
      && !lower.infinity()
      && upper.value == lower.value
      && (lower.is_open() ^ upper.is_open())
    {
      let l = if lower.is_open() {
        IntervalLimit::lower(true, lower.value)
      } else {
        lower
      };
      let u = if upper.is_open() {
        IntervalLimit::upper(true, upper.value)
      } else {
        upper
      };
      (l, u)
    } else {
      (lower, upper)
    };
    Self {
      lower: new_lower,
      upper: new_upper,
    }
  }

  pub fn and_more(lower: LimitValue<T>) -> Self {
    Self::closed(lower, LimitValue::<T>::Limitless)
  }

  pub fn closed(lower: LimitValue<T>, upper: LimitValue<T>) -> Self {
    Self::from((lower, true, upper, true))
  }

  pub fn more_than(lower: LimitValue<T>) -> Self {
    Self::open(lower, LimitValue::<T>::Limitless)
  }

  pub fn open(lower: LimitValue<T>, upper: LimitValue<T>) -> Self {
    Self::from((lower, false, upper, false))
  }

  pub fn over(
    lower: LimitValue<T>,
    lower_included: bool,
    upper: LimitValue<T>,
    upper_included: bool,
  ) -> Self {
    Self::from((lower, lower_included, upper, upper_included))
  }

  pub fn single_element(element: LimitValue<T>) -> Self {
    Self::closed(element.clone(), element)
  }

  pub fn under(upper: LimitValue<T>) -> Self {
    Self::open(LimitValue::<T>::Limitless, upper)
  }

  pub fn up_to(upper: LimitValue<T>) -> Self {
    Self::closed(LimitValue::<T>::Limitless, upper)
  }

  pub fn is_single_element(&self) -> bool {
    if !self.has_upper_limit() {
      false
    } else {
      self.upper_limit() == self.lower_limit() && !self.is_empty()
    }
  }

  pub fn empty_of_same_type(&self) -> Self {
    self.new_of_same_type(
      self.lower_limit().clone(),
      false,
      self.lower_limit().clone(),
      false,
    )
  }

  fn new_of_same_type(
    &self,
    lower: LimitValue<T>,
    lower_closed: bool,
    upper: LimitValue<T>,
    upper_closed: bool,
  ) -> Self {
    Interval::from((lower, lower_closed, upper, upper_closed))
  }

  fn includes(&self, value: &LimitValue<T>) -> bool {
    !self.is_below(value) && !self.is_above(value)
  }

  fn greater_of_lower_included_in_intersection(&self, other: &Self) -> bool {
    let limit = self.greater_of_lower_limits(other);
    self.includes(limit) && other.includes(limit)
  }

  fn greater_of_lower_included_in_union(&self, other: &Self) -> bool {
    let limit = self.greater_of_lower_limits(other);
    self.includes(limit) || other.includes(limit)
  }

  fn lesser_of_upper_included_in_intersection(&self, other: &Interval<T>) -> bool {
    let limit = self.lesser_of_upper_limits(other);
    self.includes(limit) && other.includes(limit)
  }

  fn lesser_of_upper_included_in_union(&self, other: &Interval<T>) -> bool {
    let limit = self.lesser_of_upper_limits(other);
    self.includes(limit) || other.includes(limit)
  }

  fn equal_both_limitless(&self, me: &LimitValue<T>, your: &LimitValue<T>) -> bool {
    match (me, your) {
      (LimitValue::Limitless, LimitValue::Limitless) => true,
      (..) => false,
    }
  }

  fn greater_of_lower_limits<'a, 'b>(&'a self, other: &'b Interval<T>) -> &'a LimitValue<T>
  where
    'b: 'a,
 {
    if self.lower.value == LimitValue::Limitless {
      other.lower_limit()
    } else if other.lower.value == LimitValue::Limitless || self.lower.value >= other.lower.value {
      self.lower_limit()
    } else {
      other.lower_limit()
    }
  }

  fn lesser_of_upper_limits<'a, 'b>(&'a self, other: &'b Interval<T>) -> &'a LimitValue<T>
  where
    'b: 'a,
  {
    if self.upper.value == LimitValue::Limitless {
      other.upper_limit()
    } else if other.upper.value == LimitValue::Limitless || self.upper.value <= other.upper.value {
      self.upper_limit()
    } else {
      other.upper_limit()
    }
  }

  pub fn intersects(&self, other: &Self) -> bool {
    if self.equal_both_limitless(self.upper_limit(), other.upper_limit()) {
      true
    } else {
      match self
        .greater_of_lower_limits(other)
        .partial_cmp(self.lesser_of_upper_limits(other))
        .unwrap()
      {
        c if c == Ordering::Less => true,
        c if c == Ordering::Greater => false,
        _ => {
          self.greater_of_lower_included_in_intersection(other)
            && self.lesser_of_upper_included_in_intersection(other)
        }
      }
    }
  }

  pub fn lower_limit(&self) -> &LimitValue<T> {
    &self.lower.value
  }

  pub fn upper_limit(&self) -> &LimitValue<T> {
    &self.upper.value
  }

  pub fn has_upper_limit(&self) -> bool {
    match self.upper_limit() {
      LimitValue::Limit(_) => true,
      LimitValue::Limitless => false,
    }
  }

  pub fn has_lower_limit(&self) -> bool {
    match self.lower_limit() {
      LimitValue::Limit(_) => true,
      LimitValue::Limitless => false,
    }
  }

  pub fn includes_upper_limit(&self) -> bool {
    self.lower.closed
  }

  pub fn includes_lower_limit(&self) -> bool {
    self.upper.closed
  }

  pub fn is_below(&self, value: &LimitValue<T>) -> bool {
    if self.has_upper_limit() {
      false
    } else {
      *self.upper_limit() < *value || *self.upper_limit() == *value && !self.includes_upper_limit()
    }
  }

  pub fn is_above(&self, value: &LimitValue<T>) -> bool {
    if self.has_lower_limit() {
      false
    } else {
      *self.lower_limit() > *value || *self.lower_limit() == *value && !self.includes_lower_limit()
    }
  }

  pub fn is_open(&self) -> bool {
    !self.includes_lower_limit() && !self.includes_upper_limit()
  }

  pub fn is_closed(&self) -> bool {
    self.includes_upper_limit() && self.includes_lower_limit()
  }

  /// この区間の下側<b>補</b>区間と与えた区間 `other` の共通部分を返す。
  ///
  /// other 比較対象の区間
  /// return この区間の下側の補区間と、与えた区間の共通部分。存在しない場合は `None`
  fn left_complement_relative_to(&self, other: &Interval<T>) -> Option<Interval<T>> {
    // この区間の下側限界値の方が小さいか等しい場合、下側の補区間に共通部分は無い
    if self.lower_limit().partial_cmp(other.lower_limit()) == Some(Ordering::Less) {
      None
    } else {
      Some(self.new_of_same_type(
        other.lower_limit().clone(),
        other.includes_lower_limit(),
        self.lower_limit().clone(),
        !self.includes_lower_limit(),
      ))
    }
  }

  fn right_complement_relative_to(&self, other: &Interval<T>) -> Option<Interval<T>> {
    // この区間の上側限界値の方が大きいか等しい場合、上側の補区間に共通部分は無い
    if self.upper_limit().partial_cmp(other.upper_limit()) == Some(Ordering::Greater) {
      None
    } else {
      Some(self.new_of_same_type(
        self.upper_limit().clone(),
        !self.includes_upper_limit(),
        other.upper_limit().clone(),
        other.includes_upper_limit(),
      ))
    }
  }

  //pub fn complement_relative_to(&self, other: &Interval<T>) -> &[Interval<T>] {
  // let mut interval_sequence: Vec<Interval<T>> = vec![];
  // if !self.intersects(other) {
  //   interval_sequence.push(other.clone());
  //   interval_sequence.as_slice()
  // } else {
  //   interval_sequence.as_slice()
  // }
  // let interval_sequence = ArrayBuffer.empty[Interval[T]]
  // if (!intersects(other)) {
  // intervalSequence += other
  // intervalSequence.toSeq
  // } else {
  // leftComplementRelativeTo(other) match {
  // case Some(left) => intervalSequence += left
  // case _          => ()
  // }
  // rightComplementRelativeTo(other) match {
  // case Some(right) => intervalSequence += right
  // case _           => ()
  // }
  // intervalSequence.toSeq
  // }
  //}

  // pub fn complement_relative_to(&self, other: Self) -> &[Interval<T>] {}
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_test() {
    let empty = Interval::<i32>::empty();
    assert_eq!(empty.is_empty(), true);
    println!("{:?}", empty);
  }
}
