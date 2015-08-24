use trace::Trace;

// More accurate and matches references but slower due to exp.

#[derive(Debug, Clone, Copy)]
pub struct ExpTrace {
  value: f64,
  last_time: f64,
  half_life: f64,
  continuous: bool
}

impl ExpTrace {

  pub fn new(half_life: f64, continuous: bool) -> ExpTrace {
    return ExpTrace{
      continuous: continuous,
      half_life: half_life,
      last_time: 0.0,
      value: 0.0
    };
  }
}

impl Trace for ExpTrace {
  fn read(&mut self, now: f64) -> f64 {
    if self.last_time != 0.0 {
      // half-life decay
      let diff = now - self.last_time;
      self.value *= (-1.0 * diff / self.half_life).exp();
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
