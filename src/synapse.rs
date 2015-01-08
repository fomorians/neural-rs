// Synapse represents a connection with support for delayed and weighted outputs. The weight update mechanism (e.g. STDP) is up to the specific implementation.
pub trait Synapse {
  fn pre_recv(&mut self, now: u64) -> f64;
  fn post_recv(&mut self, now: u64) -> f64;
  fn weight(&self) -> f64;
  fn delay(&self) -> uint;
}
