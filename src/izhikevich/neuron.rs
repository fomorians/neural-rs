use std::default::Default;

use neuron::Neuron;
use izhikevich::config::IzhikevichConfig;

#[derive(Debug, Clone, Copy)]
pub struct IzhikevichNeuron {
  // Membrane potential
  pub v: f64,

  // Membrane recovery
  pub u: f64,

  // Membrane recovery (starting)
  pub u_start: f64,

  // Describes accumulated membrane potential before updating.
  i: f64,

  // Describes the time scale of the recovery variable `u`.
  // Smaller values result in slower recovery.
  // A typical value is `a = 0.02`.
  a: f64,

  // Describes the sensitivity of the recovery variable `u`
  // to the subthreshold fluctuations of the membrane potential `v`.
  // Greater values couple `v` and more strongly resulting in possible
  // subthreshold oscillations and low-threshold spiking dynamics.
  // A typical value is `b = 0.2`.
  b: f64,


  // Describes the after-spike reset value of the membrane potential `v`
  // caused by the fast high-threshold K+ conductances.
  // A typical value is `c = -65mV`.
  c: f64,

  // Describes the after-spike reset of the recovery variable `u` caused
  // by slow high-threshold Na+ and K+ conductances.
  // A typical value is `d = 2`.
  d: f64,

  e: f64,

  f: f64,

  tau: f64,

  // Special casing for accomodation model...
  is_accomodation: bool,

  v_peak: f64,
}

impl Default for IzhikevichNeuron {
  fn default() -> IzhikevichNeuron {
    IzhikevichNeuron::new(0.5, Default::default())
  }
}

impl IzhikevichNeuron {
  pub fn new(tau: f64, config: IzhikevichConfig) -> IzhikevichNeuron {
    IzhikevichNeuron{
      v: config.v,
      u: config.u,
      u_start: config.u,
      a: config.a,
      b: config.b,
      c: config.c,
      d: config.d,
      e: config.e,
      f: config.f,
      is_accomodation: config.is_accomodation,
      tau: tau,
      i: 0.0,
      v_peak: 30.0,
    }
  }
}

impl Neuron for IzhikevichNeuron {
  fn recv(&mut self, v: f64) -> f64 {
    self.i += v;
    self.i
  }

  fn tick(&mut self) -> f64 {
    // The potential updates according to the input and the
    // passage of time including the variable recovery factor
    // The recovery factor is updated according to the current
    // potential and itself
    let tau_count = (1f64 / self.tau) as usize;
    for _ in 0..tau_count {
      self.v += self.tau * (0.04 * (self.v * self.v) + 5.0 * self.v + 140.0 - self.u + self.i);
    }

    self.u += if self.is_accomodation {
      self.a * (self.b * (self.v + 65.0))
    } else {
      self.a * (self.b * self.v - self.u)
    };

    self.i = 0.0;

    if self.v >= self.v_peak {
      self.v = self.c;
      self.u += self.d;
      self.v_peak
    } else {
      0.0
    }
  }
}
