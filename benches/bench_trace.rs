#![feature(test)]

extern crate test;
extern crate neural;

use test::Bencher;

use neural::trace::Trace;
use neural::traces::{LinTrace, ExpTrace};

#[bench]
fn bench_lin_trace_nearest_neighbor(bn: &mut Bencher) {
  let mut trace = LinTrace::new(1.0, false);
  let mut now = 0.0;

  bn.iter(|| {
    trace.read(now);
    now = now + 1.0;

    trace.update(1.0, now);
    now = now + 1.0;
  });
}

#[bench]
fn bench_lin_trace_all_to_all(bn: &mut Bencher) {
  let mut trace = LinTrace::new(1.0, true);
  let mut now = 0.0;

  bn.iter(|| {
    trace.read(now);
    now = now + 1.0;

    trace.update(1.0, now);
    now = now + 1.0;
  });
}

#[bench]
fn bench_exp_trace_nearest_neighbor(bn: &mut Bencher) {
  let mut trace = ExpTrace::new(1.0, false);
  let mut now = 0.0;

  bn.iter(|| {
    trace.read(now);
    now = now + 1.0;

    trace.update(1.0, now);
    now = now + 1.0;
  });
}

#[bench]
fn bench_exp_trace_all_to_all(bn: &mut Bencher) {
  let mut trace = ExpTrace::new(1.0, true);
  let mut now = 0.0;

  bn.iter(|| {
    trace.read(now);
    now = now + 1.0;

    trace.update(1.0, now);
    now = now + 1.0;
  });
}
