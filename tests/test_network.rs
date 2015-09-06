#![feature(test)]

extern crate test;
extern crate neural;
extern crate rand;

use std::default::Default;

use neural::Float;
use neural::Network;
use neural::izhikevich::IzhikevichNeuron;
use neural::stdp::{STDPSynapse, STDPConfig};
use neural::traces::ExpTrace;

#[test]
fn test_network() {
  let mut network = Network::new(20);

  let neuron = IzhikevichNeuron::new(1.0, Default::default());
  let a = network.add_neuron(neuron);
  let b = network.add_neuron(neuron);
  assert!(a == 0);
  assert!(b == 1);

  let synapse = STDPSynapse::<ExpTrace>::new(STDPConfig{
    weight: 180.0,
    max: 180.0,
    ..Default::default()
  });
  let s = network.add_synapse(synapse, a, b).unwrap();
  assert!(s == 0);

  {
    let mut inp: [Float; 2] = [1000.0, 0.0];
    let mut oup: [Float; 2] = [0.0, 0.0];
    let now = network.tick(1, &mut inp, &mut oup);
    assert_eq!(now, 1.0);
    assert_eq!(oup[0], 30.0);
    assert_eq!(oup[1], 0.0);
  }

  {
    let mut inp: [Float; 2] = [0.0, 0.0];
    let mut oup: [Float; 2] = [0.0, 0.0];
    let now = network.tick(1, &mut inp, &mut oup);
    assert_eq!(now, 2.0);
    assert_eq!(oup[0], 0.0);
    assert_eq!(oup[1], 0.0);
  }

  {
    let mut inp: [Float; 2] = [0.0, 0.0];
    let mut oup: [Float; 2] = [0.0, 0.0];
    let now = network.tick(1, &mut inp, &mut oup);
    assert_eq!(now, 3.0);
    assert_eq!(oup[0], 0.0);
    assert_eq!(oup[1], 30.0);
  }

  {
    let mut inp: [Float; 2] = [0.0, 0.0];
    let mut oup: [Float; 2] = [0.0, 0.0];
    let now = network.tick(1, &mut inp, &mut oup);
    assert_eq!(now, 4.0);
    assert_eq!(oup[0], 0.0);
    assert_eq!(oup[1], 0.0);
  }
}
