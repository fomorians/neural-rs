use Float;

// Neuron represents an excitable element in a network. It decides which signals to pass on.
pub trait Neuron {
  fn recv(&mut self, v: Float) -> Float;
  fn tick(&mut self, tau: Float);
  fn threshold(&mut self) -> Float;
  fn reset(&mut self);
}
