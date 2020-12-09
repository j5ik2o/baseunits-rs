pub mod money;

#[cfg(test)]
mod tests {
  use rust_fp_categories::empty::Empty;
  use crate::money::{Money, MoneyFan};

  #[test]
  fn it_works() {
    MoneyFan { test: 1 };
    Money::empty();
    assert_eq!(2 + 2, 4);
  }
}
