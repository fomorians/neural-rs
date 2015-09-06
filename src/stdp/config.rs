use Float;
use std::default::Default;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct STDPConfig {
  pub weight: Float,
  pub min: Float,
  pub max: Float,
  pub n_pos: Float,
  pub n_neg: Float,

  // trace config
  pub tau_pos: Float,
  pub tau_neg: Float,
  pub a_pos: Float,
  pub a_neg: Float,

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
