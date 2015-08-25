#![feature(test)]
#![feature(convert)]

extern crate test;
extern crate neural;
extern crate rand;

use test::Bencher;

use std::default::Default;
use rand::Rng;
use rand::distributions::{Normal, IndependentSample};

use neural::Network;
use neural::izhikevich::{IzhikevichNeuron, IzhikevichConfig};
use neural::stdp::{STDPSynapse, STDPConfig};

#[bench]
fn bench_network_tick(bn: &mut Bencher) {
  let mut rng = rand::thread_rng();
  let mut network = Network::new(20);

  let excitatory_count = 800;
  let inhibitory_count = 200;
  let total_count = excitatory_count + inhibitory_count;

  for _ in 0..excitatory_count {
    let r = rng.gen::<f64>();
    let a = 0.02;
    let b = 0.2;
    let c = -65.0 + (15.0 * r.powi(2));
    let d = 8.0 - (6.0 * r.powi(2));
    let v = -65.0;
    let u = b * v;

    network.add_neuron(Box::new(IzhikevichNeuron::new(0.5, IzhikevichConfig{
      v: v,
      u: u,
      a: a,
      b: b,
      c: c,
      d: d,
      ..Default::default()
    })));
  }

  for _ in 0..inhibitory_count {
    let r = rng.gen::<f64>();
    let a = 0.02 + (0.08 * r);
    let b = 0.25 - (0.05 * r);
    let c = -65.0;
    let d = 2.0;
    let v = -65.0;
    let u = b * v;

    network.add_neuron(Box::new(IzhikevichNeuron::new(0.5, IzhikevichConfig{
      v: v,
      u: u,
      a: a,
      b: b,
      c: c,
      d: d,
      ..Default::default()
    })));
  }

  for n in 0..total_count {
    for m in 0..total_count {
      let weight = if n < excitatory_count { // excitatory
        0.5 * rng.gen::<f64>()
      } else { // inhibitory
        -1.0 * rng.gen::<f64>()
      };

      let synapse = STDPSynapse::new(STDPConfig{
        weight: weight,
        min: -10.0,
        max: 10.0,
        n_pos: 0.0,
        n_neg: 0.0,
        tau_pos: 20.0,
        tau_neg: 20.0,
        a_pos: 1.0,
        a_neg: 1.0,
        continuous: false,
        scale: false,
        delay: 1,
      });
      network.add_synapse(Box::new(synapse), n, m).unwrap();
    }
  }

  let norm = Normal::new(0.0, 1.0);

  let mut vinp = vec![0.0; 1000];
  let mut voup = vec![0.0; 1000];

  let inp = vinp.as_mut_slice();
  let oup = voup.as_mut_slice();

  bn.iter(|| {
    for n in 0..total_count {
      // thalmic input
      let i = if n < excitatory_count {
        5.0 * norm.ind_sample(&mut rng)
      } else {
        2.0 * norm.ind_sample(&mut rng)
      };

      inp[n] = i;
      oup[n] = 0.0
    }

    network.tick(1, inp, oup);
  });
}
