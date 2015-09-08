use Float;
use synapse::Synapse;
use trace::Trace;
use stdp::config::STDPConfig;

#[derive(Debug, Clone, Copy)]
pub struct STDPSynapse<T: Trace> {
  weight: Float,

  min: Float,
  max: Float,

  n_pos: Float,
  n_neg: Float,

  pre_trace: T,
  post_trace: T,

  a_pos: Float,
  a_neg: Float,

  delay: usize,

  scale: bool,
}

impl<T: Trace> STDPSynapse<T> {
  pub fn new(config: STDPConfig) -> STDPSynapse<T> {
    return STDPSynapse{
      pre_trace: T::new(config.tau_pos, config.continuous),
      post_trace: T::new(config.tau_neg, config.continuous),
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

  fn a_pos(&self) -> Float {
    if self.scale {
      self.n_pos * (self.max - self.weight)
    } else {
      self.n_pos
    }
  }

  fn a_neg(&self) -> Float {
    if self.scale {
      self.n_neg * (self.min - self.weight)
    } else {
      self.n_neg
    }
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

impl<T: Trace> Synapse for STDPSynapse<T> {
  fn weight(&self) -> Float {
    self.weight
  }

  fn delay(&self) -> usize {
    self.delay
  }

  fn pre_recv(&mut self, now: Float) -> Float { // delta
    // Pre-synaptic spike leaves a trace which increases
    // by an amount a+(x) at the moment of spike arrival and decays
    // exponentially in the absence of spikes
    self.pre_trace.update(self.a_pos, now); // used by post

    // Weight is depressed at the moment of pre-synaptic spikes
    // by an amount proportional to the trace y left by previous
    // post-synaptic spikes
    let delta = self.a_neg() * self.post_trace.read(now); // decay before using value
    self.integrate(delta);
    delta
  }

  fn post_recv(&mut self, now: Float) -> Float { // delta
    // Post-synaptic spike leaves a trace y(t) which increases
    // by an amount a-(y) at the moment of spike arrival and decays
    // exponentially in the absence of spikes.
    self.post_trace.update(self.a_neg, now); // used by pre

    // Weight is increased at the moment of post-synaptic firing
    // by an amount that depends on the value of the trace x left
    // by the pre-synaptic spike.
    let delta = self.a_pos() * self.pre_trace.read(now); // decay before using value
    self.integrate(delta);
    delta
  }
}
