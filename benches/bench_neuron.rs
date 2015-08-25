#![feature(test)]

extern crate test;
extern crate neural;

use test::Bencher;

use std::default::Default;

use neural::neuron::Neuron;
use neural::izhikevich::IzhikevichNeuron;

#[bench]
fn bench_neuron(bn: &mut Bencher) {
  let mut neuron = IzhikevichNeuron::new(0.5, Default::default());

  bn.iter(|| {
    neuron.recv(5.0);
    neuron.tick(1.0);

    let v = neuron.threshold();
    if v > 0.0 {
        neuron.reset();
    }
  });
}
