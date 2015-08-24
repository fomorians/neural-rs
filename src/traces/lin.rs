use trace::Trace;

// Fast approximation of exp trace.

#[derive(Debug, Clone, Copy)]
pub struct LinTrace {
  value: f64,
  last_time: f64,
  half_life: f64,
  continuous: bool
}

impl LinTrace {

  pub fn new(half_life: f64, continuous: bool) -> LinTrace {
    return LinTrace{
      continuous: continuous,
      half_life: half_life,
      last_time: 0.0,
      value: 0.0
    };
  }
}

impl Trace for LinTrace {
  fn read(&mut self, now: f64) -> f64 {
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

  fn update(&mut self, val: f64, now: f64) {
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
