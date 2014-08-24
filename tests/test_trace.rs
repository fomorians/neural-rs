extern crate test;
extern crate neural;

use neural::trace::Trace;

#[test]
fn neural_trace_nearest_neighbor() {
  let mut trace = Trace::new(1, false);
  let mut val: f64;

  trace.update(1.0, 1);

  val = trace.read(2);
  assert!(val == 0.36787944117144233);

  trace.update(1.0, 2);

  val = trace.read(3);
  assert!(val == 0.36787944117144233);

  trace.update(1.0, 3);

  val = trace.read(4);
  assert!(val == 0.36787944117144233);
}

#[test]
fn neural_trace_all_to_all() {
  let mut trace = Trace::new(1, true);
  let mut val: f64;

  trace.update(1.0, 1);

  val = trace.read(2);
  assert!(val == 0.36787944117144233);

  trace.update(1.0, 2);

  val = trace.read(3);
  assert!(val == 0.503214724408055);

  trace.update(1.0, 3);

  val = trace.read(4);
  assert!(val == 0.553001792775919);
}
