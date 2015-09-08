use Float;

// Trace is a data-structure for a decaying value. It can be sampled
// at any point in the future to get its discrete decayed valued.
pub trait Trace {
  fn new(half_life: Float, continuous: bool) -> Self;
  fn read(&mut self, now: Float) -> Float;
  fn update(&mut self, val: Float, now: Float);
}
