extern crate test;
extern crate neural;

use std::default::Default;
use neural::Network;
use neural::izhikevich::IzhikevichNeuron;
use neural::stdp::STDPSynapse;

#[test]
fn basic_network() {
  let mut network = Network::new(20);

  let neuron = IzhikevichNeuron::new(Default::default());
  let a = network.add_neuron(box neuron);
  let b = network.add_neuron(box neuron);
  assert!(a == 1);
  assert!(b == 2);

  let synapse = STDPSynapse::new(Default::default());
  let s = network.add_synapse(box synapse, a, b);
  assert!(s == 3);

  network.tick(1.0);
  network.tick(1.0);
  network.tick(1.0);
}
