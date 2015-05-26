// Neuron represents an excitable element in a network. It decides which signals to pass on.
pub trait Neuron {
  fn recv(&mut self, v: f64) -> f64;
  fn tick(&mut self) -> f64;
}
