#![feature(test)]

extern crate test;
extern crate neural;

use test::Bencher;

use neural::trace::Trace;

#[bench]
fn bench_trace(bn: &mut Bencher) {
  let mut trace = Trace::new(10.0, false);
  let mut now = 0.0;

  bn.iter(|| {
    trace.read(now);
    now = now + 1.0;

    trace.update(1.0, now);
    now = now + 1.0;
  });
}
