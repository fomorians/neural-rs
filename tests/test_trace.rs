#![feature(test)]

extern crate test;
extern crate neural;

use neural::trace::Trace;
use neural::traces::{LinTrace, ExpTrace};

#[test]
fn test_lin_trace_nearest_neighbor() {
  let mut trace = LinTrace::new(1.0, false);
  let mut val: f64;

  trace.update(1.0, 1.0);

  val = trace.read(2.0);
  assert_eq!(val, 0.5);

  trace.update(1.0, 2.0);

  val = trace.read(3.0);
  assert_eq!(val, 0.5);

  trace.update(1.0, 3.0);

  val = trace.read(4.0);
  assert_eq!(val, 0.5);
}

#[test]
fn test_lin_trace_all_to_all() {
  let mut trace = LinTrace::new(1.0, true);
  let mut val: f64;

  trace.update(1.0, 1.0);

  val = trace.read(2.0);
  assert_eq!(val, 0.5);

  trace.update(1.0, 2.0);

  val = trace.read(3.0);
  assert_eq!(val, 0.5);

  trace.update(1.0, 3.0);

  val = trace.read(4.0);
  assert_eq!(val, 0.5);
}

#[test]
fn test_exp_trace_nearest_neighbor() {
  let mut trace = ExpTrace::new(1.0, false);
  let mut val: f64;

  trace.update(1.0, 1.0);

  val = trace.read(2.0);
  assert_eq!(val, 0.36787944117144233);

  trace.update(1.0, 2.0);

  val = trace.read(3.0);
  assert_eq!(val, 0.36787944117144233);

  trace.update(1.0, 3.0);

  val = trace.read(4.0);
  assert_eq!(val, 0.36787944117144233);
}

#[test]
fn test_exp_trace_all_to_all() {
  let mut trace = ExpTrace::new(1.0, true);
  let mut val: f64;

  trace.update(1.0, 1.0);

  val = trace.read(2.0);
  assert_eq!(val, 0.36787944117144233);

  trace.update(1.0, 2.0);

  val = trace.read(3.0);
  assert_eq!(val, 0.503214724408055);

  trace.update(1.0, 3.0);

  val = trace.read(4.0);
  assert_eq!(val, 0.553001792775919);
}
