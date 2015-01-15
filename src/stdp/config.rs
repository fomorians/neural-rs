use std::default::Default;

#[deriving(Show, Copy)]
pub struct STDPConfig {
  pub weight: f64,
  pub min: f64,
  pub max: f64,
  pub n_pos: f64,
  pub n_neg: f64,

  // trace config
  pub tau_pos: u64,
  pub tau_neg: u64,
  pub a_pos: f64,
  pub a_neg: f64,

  // all-to-all vs. nearest-neighbor interactions
  pub continuous: bool,

  // implement weight-dependent synaptic scaling
  pub scale: bool,

  pub delay: usize
}

impl Copy for STDPConfig {}

impl Default for STDPConfig {
  fn default() -> STDPConfig {
    STDPConfig{
      weight: 0.0,
      min: 0.0,
      max: 0.0,
      n_pos: 0.0,
      n_neg: 0.0,
      tau_pos: 0,
      tau_neg: 0,
      a_pos: 0.0,
      a_neg: 0.0,
      continuous: true,
      scale: true,
      delay: 0
    }
  }
}
