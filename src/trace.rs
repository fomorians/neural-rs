// Trace is a data-structure for a decaying value. It can be sampled
// at any point in the future to get its discrete decayed valued.
use std::num::Float;

#[derive(Debug, Copy)]
pub struct Trace {
  value: f64,
  last_time: u64,
  half_life: f64,
  continuous: bool
}

impl Trace {

  pub fn new(half_life: f64, continuous: bool) -> Trace {
    return Trace{
      continuous: continuous,
      half_life: half_life,
      last_time: 0,
      value: 0.0
    };
  }

  pub fn read(&mut self, now: u64) -> f64 {
    if self.last_time != 0 {
      // half-life decay
      let diff: f64 = (now - self.last_time) as f64;
      self.value *= (-1.0 * diff / self.half_life).exp();
    }

    self.last_time = now;
    self.value
  }

  pub fn update(&mut self, val: f64, now: u64) {
    // Adding to `value` produces a temporal all-to-all interaction
    // vs. reseting `value` which restricts interactions to
    // nearest-neighbor.
    self.value = if self.continuous {
      self.read(now) + val
    } else {
      val
    };
    self.last_time = now;
  }
}
