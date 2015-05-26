use std::default::Default;

#[derive(Debug, Clone, Copy)]
pub struct STDPConfig {
  pub weight: f64,
  pub min: f64,
  pub max: f64,
  pub n_pos: f64,
  pub n_neg: f64,

  // trace config
  pub tau_pos: f64,
  pub tau_neg: f64,
  pub a_pos: f64,
  pub a_neg: f64,

  // all-to-all vs. nearest-neighbor interactions
  pub continuous: bool,

  // implement weight-dependent synaptic scaling
  pub scale: bool,

  pub delay: usize
}

impl Default for STDPConfig {
  fn default() -> STDPConfig {
    STDPConfig{
      weight: 6.0,
      min: 0.0,
      max: 10.0,
      n_pos: 0.1,
      n_neg: -0.12,
      tau_pos: 20.0,
      tau_neg: 20.0,
      a_pos: 1.0,
      a_neg: 1.0,
      continuous: false,
      scale: false,
      delay: 1
    }
  }
}
