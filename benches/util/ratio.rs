use criterion::*;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use baseunits_rs::util::Ratio;

#[inline]
fn ratio_new() {
  Ratio::new(
    Decimal::from_i32(10).unwrap(),
    Decimal::from_i32(3).unwrap(),
  );
}

#[inline]
fn ratio_new_i64() {
  Ratio::new_i64(10, 3);
}

#[inline]
fn ratio_reduce(ratio: Ratio) {
  ratio.reduce();
}

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("ratio_new", |b| b.iter(|| ratio_new()));
  c.bench_function("ratio_new_i64", |b| b.iter(|| ratio_new_i64()));
  c.bench_function("ratio_reduce", |b| {
    b.iter(|| {
      let ratio = Ratio::new(
        Decimal::from_i32(10).unwrap(),
        Decimal::from_i32(3).unwrap(),
      );
      ratio_reduce(ratio);
    })
  });
}

criterion_group!(benches, criterion_benchmark);
