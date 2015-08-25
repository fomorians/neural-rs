use std::default::Default;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SymConfig {
  pub weight: f64,
  pub a_sym: f64,
  pub tau_a: f64,
  pub tau_b: f64,
  pub delay: usize,
  pub min: f64,
  pub max: f64,
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
