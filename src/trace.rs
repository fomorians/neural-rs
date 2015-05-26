// Trace is a data-structure for a decaying value. It can be sampled
// at any point in the future to get its discrete decayed valued.

#[derive(Debug, Clone, Copy)]
pub struct Trace {
  value: f64,
  last_time: f64,
  half_life: f64,
  continuous: bool
}

impl Trace {

  pub fn new(half_life: f64, continuous: bool) -> Trace {
    return Trace{
      continuous: continuous,
      half_life: half_life,
      last_time: 0.0,
      value: 0.0
    };
  }

  pub fn read(&mut self, now: f64) -> f64 {
    if self.last_time != 0.0 {
      // half-life decay
      let diff = now - self.last_time;
      if diff >= 0.0 {
        self.value = (-diff / (self.half_life * 2.0)) + 1.0;
      } else {
        self.value = (diff / (self.half_life * 2.0)) - 1.0;
      }
    }

    self.last_time = now;
    self.value
  }

  pub fn update(&mut self, val: f64, now: f64) {
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
