use Float;
use std::default::Default;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct IzhikevichConfig {
  pub v: Float,
  pub u: Float,
  pub a: Float,
  pub b: Float,
  pub c: Float,
  pub d: Float,
  pub e: Float,
  pub f: Float,
  pub is_accomodation: bool,
}

impl Default for IzhikevichConfig {
  fn default() -> IzhikevichConfig {
    let a = 0.02;
    let b = 0.2;
    let c = -65.0;
    let d = 2.0;
    let v = c;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }
}

#[allow(dead_code)]
impl IzhikevichConfig {
  pub fn regular_spiking() -> IzhikevichConfig {
    let a = 0.02;
    let b = 0.2;
    let c = -65.0;
    let d = 8.0;
    let v = c;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn fast_spiking() -> IzhikevichConfig {
    let a = 0.1;
    let b = 0.2;
    let c = -65.0;
    let d = 2.0;
    let v = c;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn tonic_spiking() -> IzhikevichConfig {
    let a = 0.02;
    let b = 0.2;
    let c = -65.0;
    let d = 6.0;
    let v = -70.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn phasic_spiking() -> IzhikevichConfig {
    let a = 0.02;
    let b = 0.25;
    let c = -65.0;
    let d = 6.0;
    let v = -64.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn tonic_bursting() -> IzhikevichConfig {
    let a = 0.02;
    let b = 0.2;
    let c = -50.0;
    let d = 2.0;
    let v = -70.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn phasic_bursting() -> IzhikevichConfig {
    let a = 0.02;
    let b = 0.25;
    let c = -55.0;
    let d = 0.05;
    let v = -64.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn mixed_mode() -> IzhikevichConfig {
    let a = 0.02;
    let b = 0.2;
    let c = -55.0;
    let d = 4.0;
    let v = -70.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn spike_frequency_adaptation() -> IzhikevichConfig {
    let a = 0.01;
    let b = 0.2;
    let c = -65.0;
    let d = 8.0;
    let v = -70.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn class1() -> IzhikevichConfig {
    let a = 0.02;
    let b = -0.1;
    let c = -55.0;
    let d = 6.0;
    let v = -60.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 4.1, f: 108.0, is_accomodation: false}
  }

  pub fn class2() -> IzhikevichConfig {
    let a = 0.2;
    let b = 0.26;
    let c = -65.0;
    let d = 0.0;
    let v = -64.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn spike_latency() -> IzhikevichConfig {
    let a = 0.02;
    let b = 0.2;
    let c = -65.0;
    let d = 6.0;
    let v = -70.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn subthreshold_oscillation() -> IzhikevichConfig {
    let a = 0.05;
    let b = 0.26;
    let c = -60.0;
    let d = 0.0;
    let v = -62.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn resonator() -> IzhikevichConfig {
    let a = 0.1;
    let b = 0.26;
    let c = -60.0;
    let d = -1.0;
    let v = -62.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn integrator() -> IzhikevichConfig {
    let a = 0.02;
    let b = -0.1;
    let c = -55.0;
    let d = 6.0;
    let v = -60.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 4.1, f: 108.0, is_accomodation: false}
  }

  pub fn rebound_spike() -> IzhikevichConfig {
    let a = 0.03;
    let b = 0.25;
    let c = -60.0;
    let d = 4.0;
    let v = -64.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn rebound_burst() -> IzhikevichConfig {
    let a = 0.03;
    let b = 0.25;
    let c = -52.0;
    let d = 0.0;
    let v = -64.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn threshold_variability() -> IzhikevichConfig {
    let a = 0.03;
    let b = 0.25;
    let c = -60.0;
    let d = 4.0;
    let v = -64.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn bistability() -> IzhikevichConfig {
    let a = 0.1;
    let b = 0.26;
    let c = -60.0;
    let d = 0.0;
    let v = -61.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn depolarizing_after_potential() -> IzhikevichConfig {
    let a = 1.0;
    let b = 0.2;
    let c = -60.0;
    let d = -21.0;
    let v = -70.0;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn accomodation() -> IzhikevichConfig {
    let a = 0.02;
    let b = 1.0;
    let c = -55.0;
    let d = 4.0;
    let v = -65.0;
    let u = -16.0;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: true}
  }

  pub fn inhibition_induced_spiking() -> IzhikevichConfig {
    let a = -0.02;
    let b = -1.0;
    let c = -60.0;
    let d = 8.0;
    let v = -63.8;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }

  pub fn inhibition_induced_bursting() -> IzhikevichConfig {
    let a = -0.026;
    let b = -1.0;
    let c = -45.0;
    let d = -2.0;
    let v = -63.8;
    let u = b * v;
    IzhikevichConfig{v: v, u: u, a: a, b: b, c: c, d: d, e: 5.0, f: 140.0, is_accomodation: false}
  }
}
