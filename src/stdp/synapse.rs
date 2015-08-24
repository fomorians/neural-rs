use synapse::Synapse;
use trace::Trace;
use traces::LinTrace;
use stdp::config::STDPConfig;

#[derive(Debug, Clone, Copy)]
pub struct STDPSynapse {
  weight: f64,

  min: f64,
  max: f64,

  n_pos: f64,
  n_neg: f64,

  pre_trace: LinTrace,
  post_trace: LinTrace,

  a_pos: f64,
  a_neg: f64,

  delay: usize,

  scale: bool,
}

impl STDPSynapse {
  pub fn new(config: STDPConfig) -> STDPSynapse {
    return STDPSynapse{
      pre_trace: LinTrace::new(config.tau_pos, config.continuous),
      post_trace: LinTrace::new(config.tau_neg, config.continuous),
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

  fn a_pos(&self) -> f64 {
    if self.scale {
      self.n_pos * (self.max - self.weight)
    } else {
      self.n_pos
    }
  }

  fn a_neg(&self) -> f64 {
    if self.scale {
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

  fn delay(&self) -> usize {
    self.delay
  }

  fn pre_recv(&mut self, now: f64) -> f64 { // delta
    // Pre-synaptic spike leaves a trace which increases
    // by an amount a+(x) at the moment of spike arrival and decays
    // exponentially in the absence of spikes
    self.pre_trace.update(self.a_pos, now); // used by post

    // Weight is depressed at the moment of pre-synaptic spikes
    // by an amount proportional to the trace y left by previous
    // post-synaptic spikes
    let delta = self.a_neg() * self.post_trace.read(now); // decay before using value

    self.weight = self.weight + delta;

    if self.weight > self.max {
      self.weight = self.max;
    } else if self.weight < self.min {
      self.weight = self.min;
    }

    delta
  }

  fn post_recv(&mut self, now: f64) -> f64 { // delta
    // Post-synaptic spike leaves a trace y(t) which increases
    // by an amount a-(y) at the moment of spike arrival and decays
    // exponentially in the absence of spikes.
    self.post_trace.update(self.a_neg, now); // used by pre

    // Weight is increased at the moment of post-synaptic firing
    // by an amount that depends on the value of the trace x left
    // by the pre-synaptic spike.
    let delta = self.a_pos() * self.pre_trace.read(now); // decay before using value

    self.weight = self.weight + delta;

    if self.weight > self.max {
      self.weight = self.max;
    } else if self.weight < self.min {
      self.weight = self.min;
    }

    delta
  }
}
