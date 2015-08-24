// Trace is a data-structure for a decaying value. It can be sampled
// at any point in the future to get its discrete decayed valued.
pub trait Trace {
  fn read(&mut self, now: f64) -> f64;
  fn update(&mut self, val: f64, now: f64);
}
