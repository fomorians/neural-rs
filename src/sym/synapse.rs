use synapse::Synapse;
use sym::config::SymConfig;
use fastexp::fastexp;

#[derive(Debug, Clone, Copy)]
pub struct SymSynapse {
  weight: f64,

  min: f64,
  max: f64,

  a_sym: f64,

  tau_a: f64,
  tau_b: f64,

  delay: usize,

  pre_time: f64,
  post_time: f64,
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

  fn get_delta(&self) -> f64 {
    let dt = self.post_time - self.pre_time;
    self.a_sym * (1.0 - (dt / self.tau_a).powi(2)) * fastexp(-dt.abs() / self.tau_b)
  }

  fn integrate(&mut self, delta: f64) {
    self.weight = self.weight + delta;
    if self.weight > self.max {
      self.weight = self.max;
    } else if self.weight < self.min {
      self.weight = self.min;
    }
  }
}

impl Synapse for SymSynapse {
  fn weight(&self) -> f64 {
    self.weight
  }

  fn delay(&self) -> usize {
    self.delay
  }

  fn pre_recv(&mut self, now: f64) -> f64 { // delta
      self.pre_time = now;

      let delta = self.get_delta();
      self.integrate(delta);
      delta
  }

  fn post_recv(&mut self, now: f64) -> f64 { // delta
      self.post_time = now;

      let delta = self.get_delta();
      self.integrate(delta);
      delta
  }
}
