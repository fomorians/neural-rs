#![feature(test)]

extern crate test;
extern crate neural;

use test::Bencher;

use std::default::Default;

use neural::neuron::Neuron;
use neural::izhikevich::{IzhikevichNeuron, IzhikevichConfig};

#[bench]
fn bench_neuron(bn: &mut Bencher) {
  let a = 0.02;
  let b = 0.2;
  let c = -65.0;
  let d = 8.0;
  let v = c;
  let u = b * v;

  let mut neuron = IzhikevichNeuron::new(0.5, IzhikevichConfig{
    v: v,
    u: u,
    a: a,
    b: b,
    c: c,
    d: d,
    ..Default::default()
  });

  bn.iter(|| {
    neuron.recv(5.0);
    neuron.tick();
  });
}
