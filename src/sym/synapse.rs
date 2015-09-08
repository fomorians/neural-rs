use Float;
use synapse::Synapse;
use sym::config::SymConfig;
use fastexp::FastExp;

#[derive(Debug, Clone, Copy)]
pub struct SymSynapse {
  weight: Float,

  min: Float,
  max: Float,

  a_sym: Float,

  tau_a: Float,
  tau_b: Float,

  delay: usize,

  pre_time: Float,
  post_time: Float,
}

impl SymSynapse {
  pub fn new(config: SymConfig) -> SymSynapse {
    return SymSynapse{
      weight: config.weight,
      a_sym: config.a_sym,
      tau_a: config.tau_a,
      tau_b: config.tau_b,
      delay: config.delay,
      min: config.min,
      max: config.max,
      pre_time: 0.0,
      post_time: 0.0,
    }
  }

  fn get_delta(&self) -> Float {
    let dt = self.post_time - self.pre_time;
    self.a_sym * (1.0 - (dt / self.tau_a).powi(2)) * (-dt.abs() / self.tau_b).fastexp()
  }

  fn integrate(&mut self, delta: Float) {
    self.weight = self.weight + delta;
    if self.weight > self.max {
      self.weight = self.max;
    } else if self.weight < self.min {
      self.weight = self.min;
    }
  }
}

impl Synapse for SymSynapse {
  fn weight(&self) -> Float {
    self.weight
  }

  fn delay(&self) -> usize {
    self.delay
  }

  fn pre_recv(&mut self, now: Float) -> Float { // delta
      self.pre_time = now;

      let delta = self.get_delta();
      self.integrate(delta);
      delta
  }

  fn post_recv(&mut self, now: Float) -> Float { // delta
      self.post_time = now;

      let delta = self.get_delta();
      self.integrate(delta);
      delta
  }
}
