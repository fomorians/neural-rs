#![feature(test)]

extern crate test;
extern crate neural;

use neural::Float;
use neural::trace::Trace;
use neural::traces::{LinTrace, ExpTrace};

#[test]
fn test_lin_trace_nearest_neighbor() {
  let mut trace = LinTrace::new(1.0, false);
  let mut val: Float;

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
  let mut val: Float;

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
  let mut val: Float;

  trace.update(1.0, 1.0);

  val = trace.read(2.0);
  assert_eq!(val, 0.36769915);

  trace.update(1.0, 2.0);

  val = trace.read(3.0);
  assert_eq!(val, 0.36769915);

  trace.update(1.0, 3.0);

  val = trace.read(4.0);
  assert_eq!(val, 0.36769915);
}

#[test]
fn test_exp_trace_all_to_all() {
  let mut trace = ExpTrace::new(1.0, true);
  let mut val: Float;

  trace.update(1.0, 1.0);

  val = trace.read(2.0);
  assert_eq!(val, 0.36769915);

  trace.update(1.0, 2.0);

  val = trace.read(3.0);
  assert_eq!(val, 0.5029018);

  trace.update(1.0, 3.0);

  val = trace.read(4.0);
  assert_eq!(val, 0.5526157);
}
