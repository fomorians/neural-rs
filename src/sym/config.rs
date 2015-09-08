use Float;
use std::default::Default;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SymConfig {
  pub weight: Float,
  pub a_sym: Float,
  pub tau_a: Float,
  pub tau_b: Float,
  pub delay: usize,
  pub min: Float,
  pub max: Float,
}

impl Default for SymConfig {
  fn default() -> SymConfig {
    SymConfig{
      weight: 0.0,
      a_sym: 0.05,
      tau_a: 10.0,
      tau_b: 10.0,
      delay: 1,
      min: -10.0,
      max: 10.0,
    }
  }
}
