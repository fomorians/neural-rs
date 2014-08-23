use trace::Trace;

#[deriving(Show)]
pub trait Synapse {
  fn pre_recv(&mut self, now: u64) -> f64;
  fn post_recv(&mut self, now: u64) -> f64;
  fn weight(&self) -> f64;
  fn delay(&self) -> uint;
}

pub struct STDPConfig {
  weight: f64,
  min: f64,
  max: f64,
  n_pos: f64,
  n_neg: f64,

  // trace config
  tau_pos: u64,
  tau_neg: u64,
  a_pos: f64,
  a_neg: f64,

  // all-to-all vs. nearest-neighbor interactions
  continuous: bool,

  // implement weight-dependent synaptic scaling
  scale: bool,

  delay: uint
}

pub struct STDPSynapse {
  weight: f64,

  min: f64,
  max: f64,

  n_pos: f64,
  n_neg: f64,

  pre_trace: Trace,
  post_trace: Trace,

  a_pos: f64,
  a_neg: f64,

  delay: uint,

  scale: bool
}

impl STDPSynapse {
  pub fn new(config: STDPConfig) -> STDPSynapse {
    return STDPSynapse{
      pre_trace: Trace::new(config.tau_pos, config.continuous),
      post_trace: Trace::new(config.tau_neg, config.continuous),
      weight: config.weight,
      n_pos: config.n_pos,
      n_neg: config.n_neg,
      a_pos: config.a_pos,
      a_neg: config.a_neg,
      delay: config.delay,
      scale: config.scale,
      min: config.min,
      max: config.max,
    }
  }

  fn A_pos(&self) -> f64 {
    return if self.scale {
      self.n_pos * (self.max - self.weight)
    } else {
      self.n_pos
    }
  }

  fn A_neg(&self) -> f64 {
    return if self.scale {
      self.n_neg * (self.min - self.weight)
    } else {
      self.n_neg
    }
  }
}

impl Synapse for STDPSynapse {
  fn weight(&self) -> f64 {
    self.weight
  }

  fn delay(&self) -> uint {
    self.delay
  }

  fn pre_recv(&mut self, now: u64) -> f64 { // delta
    // pre-synaptic spike leaves a trace which increases
    // by an amount a+(x) at the moment of spike arrival and decays
    // exponentially in the absence of spikes
    self.pre_trace.update(self.a_pos, now); // used by post

    // weight is depressed at the moment of presynaptic spikes
    // by an amount proportional to the trace y left by previous
    // postsynaptic spikes
    let delta = self.A_neg() * self.post_trace.read(now); // decay before using value
    self.weight = self.weight + delta;
    if self.weight > self.max {
      self.weight = self.max;
    }
    if self.weight < self.min {
      self.weight = self.min;
    }
    return delta
  }

  fn post_recv(&mut self, now: u64) -> f64 { // delta
    // post-synaptic spike leaves a trace y(t) which increases
    // by an amount a-(y) at the moment of spike arrival and decays
    // exponentially in the absence of spikes.
    self.post_trace.update(self.a_neg, now); // Used by pre

    // weight is increased at the moment of post-synaptic firing
    // by an amount that depends on the value of the trace x left
    // by the presynaptic spike.
    let delta = self.A_pos() * self.pre_trace.read(now); // decay before using value
    self.weight = self.weight + delta;
    if self.weight > self.max {
      self.weight = self.max;
    }
    if self.weight < self.min {
      self.weight = self.min;
    }
    return delta
  }
}
