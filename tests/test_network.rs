#![feature(test)]
#![feature(convert)]

extern crate test;
extern crate neural;
extern crate rand;
extern crate csv;

use std::default::Default;
use std::path::Path;
use std::fs;
use rand::Rng;
use rand::distributions::{Normal, IndependentSample};

use neural::Network;
use neural::izhikevich::{IzhikevichNeuron, IzhikevichConfig};
use neural::stdp::{STDPSynapse, STDPConfig};

#[test]
fn test_network_basic() {
  let mut network = Network::new(20);

  let neuron = IzhikevichNeuron::new(1.0, Default::default());
  let a = network.add_neuron(Box::new(neuron));
  let b = network.add_neuron(Box::new(neuron));
  assert!(a == 0);
  assert!(b == 1);

  let synapse = STDPSynapse::new(Default::default());
  let s = network.add_synapse(Box::new(synapse), a, b).unwrap();
  assert!(s == 0);

  let mut inp: [f64; 2] = [0f64, 0f64];
  let mut oup: [f64; 2] = [0f64, 0f64];
  network.tick(1, &mut inp, &mut oup);
  network.tick(1, &mut inp, &mut oup);
  network.tick(1, &mut inp, &mut oup);
}

#[test]
fn test_network_synapse_direction() {
  let mut network = Network::new(20);

  let neuron = IzhikevichNeuron::new(1.0, Default::default());
  let a = network.add_neuron(Box::new(neuron));
  let b = network.add_neuron(Box::new(neuron));
  assert!(a == 0);
  assert!(b == 1);

  let synapse = STDPSynapse::new(STDPConfig{
    weight: 180.0,
    max: 180.0,
    ..Default::default()
  });
  let s = network.add_synapse(Box::new(synapse), a, b).unwrap();
  assert!(s == 0);

  {
    let mut inp: [f64; 2] = [1000.0, 0.0];
    let mut oup: [f64; 2] = [0.0, 0.0];
    let now = network.tick(1, &mut inp, &mut oup);
    assert_eq!(now, 1.0);
    assert_eq!(oup[0], 30.0);
    assert_eq!(oup[1], 0.0);
  }

  {
    let mut inp: [f64; 2] = [0.0, 0.0];
    let mut oup: [f64; 2] = [0.0, 0.0];
    let now = network.tick(1, &mut inp, &mut oup);
    assert_eq!(now, 2.0);
    assert_eq!(oup[0], 0.0);
    assert_eq!(oup[1], 0.0);
  }

  {
    let mut inp: [f64; 2] = [0.0, 0.0];
    let mut oup: [f64; 2] = [0.0, 0.0];
    let now = network.tick(1, &mut inp, &mut oup);
    assert_eq!(now, 3.0);
    assert_eq!(oup[0], 0.0);
    assert_eq!(oup[1], 30.0);
  }

  {
    let mut inp: [f64; 2] = [0.0, 0.0];
    let mut oup: [f64; 2] = [0.0, 0.0];
    let now = network.tick(1, &mut inp, &mut oup);
    assert_eq!(now, 4.0);
    assert_eq!(oup[0], 0.0);
    assert_eq!(oup[1], 0.0);
  }
}
