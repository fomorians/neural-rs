extern crate test;
extern crate neural;

use neural::network::Network;
use neural::neuron::{IzhikevichNeuron, IzhikevichConfig};
use neural::synapse::{STDPSynapse, STDPConfig};

#[test]
fn neural_network() {
  let mut network = Network::new(20);

  let neuron = IzhikevichNeuron::new(IzhikevichConfig{
    v: 0.0,
    u: 0.0,
    a: 0.0,
    b: 0.0,
    c: 0.0,
    d: 0.0,
  });
  let a = network.add_neuron(box neuron);
  let b = network.add_neuron(box neuron);
  assert!(a == 1);
  assert!(b == 2);

  let synapse = STDPSynapse::new(STDPConfig{
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
  });
  let s = network.add_synapse(box synapse, a, b);
  assert!(s == 3);

  network.update(1);
  network.update(2);
  network.update(3);
}
