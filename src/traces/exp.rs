use Float;
use trace::Trace;
use fastexp::FastExp;

// More accurate and matches references but slower due to exp.

#[derive(Debug, Clone, Copy)]
pub struct ExpTrace {
  value: Float,
  last_time: Float,
  half_life: Float,
  continuous: bool
}

impl Trace for ExpTrace {
  fn new(half_life: Float, continuous: bool) -> Self {
    ExpTrace {
      continuous: continuous,
      half_life: half_life,
      last_time: 0.0,
      value: 0.0
    }
  }

  fn read(&mut self, now: Float) -> Float {
    if self.last_time != 0.0 {
      // half-life decay
      let diff = now - self.last_time;
      self.value *= (-1.0 * diff / self.half_life).fastexp();
    }

    self.last_time = now;
    self.value
  }

  fn update(&mut self, val: Float, now: Float) {
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
