use Float;

// Synapse represents a connection with support for delayed and weighted outputs. The weight update mechanism (e.g. STDP) is up to the specific implementation.
pub trait Synapse {
  fn pre_recv(&mut self, now: Float) -> Float;
  fn post_recv(&mut self, now: Float) -> Float;
  fn weight(&self) -> Float;
  fn delay(&self) -> usize;
}
