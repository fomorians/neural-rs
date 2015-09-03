#![feature(test)]
#![feature(convert)]

extern crate test;
extern crate neural;
extern crate rand;

use test::Bencher;

use std::default::Default;
use rand::{Rng, SeedableRng, StdRng};

use neural::Network;
use neural::izhikevich::{IzhikevichNeuron, IzhikevichConfig};
use neural::stdp::{STDPSynapse, STDPConfig};
use neural::traces::ExpTrace;

#[bench]
fn bench_network_tick_all_to_all(bn: &mut Bencher) {
  let mut network = Network::new(20);

  let total_count = 100;

  for _ in 0..total_count {
    let a = 0.02;
    let b = 0.2;
    let c = -65.0;
    let d = 2.0;
    let v = c;
    let u = b * v;

    network.add_neuron(IzhikevichNeuron::new(0.5, IzhikevichConfig{
      v: v,
      u: u,
      a: a,
      b: b,
      c: c,
      d: d,
      ..Default::default()
    }));
  }

  for n in 0..total_count {
    for m in 0..total_count {
      let synapse = STDPSynapse::<ExpTrace>::new(STDPConfig{
        weight: 5.0,
        min: 0.0,
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
      network.add_synapse(synapse, n, m).unwrap();
    }
  }

  let mut vinp = vec![0.0; total_count];
  let mut voup = vec![0.0; total_count];

  let inp = vinp.as_mut_slice();
  let oup = voup.as_mut_slice();

  // Pre-run since the network state is most volatile in this range
  for _ in 0..100 {
    for n in 0..total_count {
      inp[n] = 5.0;
      oup[n] = 0.0
    }

    network.tick(1, inp, oup);
  }

  bn.iter(|| {
    for n in 0..total_count {
      inp[n] = 5.0;
      oup[n] = 0.0
    }

    network.tick(1, inp, oup);
  });
}


#[bench]
fn bench_network_tick_limited(bn: &mut Bencher) {
  let seed: &[_] = &[1, 2, 3, 4];
  let mut rng: StdRng = SeedableRng::from_seed(seed);
  let mut network = Network::new(20);

  let total_count = 100;
  let connectivity = 10;

  for _ in 0..total_count {
    let a = 0.02;
    let b = 0.2;
    let c = -65.0;
    let d = 2.0;
    let v = c;
    let u = b * v;

    network.add_neuron(IzhikevichNeuron::new(0.5, IzhikevichConfig{
      v: v,
      u: u,
      a: a,
      b: b,
      c: c,
      d: d,
      ..Default::default()
    }));
  }

  for n in 0..total_count {
    let mut i = 0;

    while i < connectivity {
      let m = rng.gen_range::<usize>(0, total_count);
      if n == m { // try again
        continue;
      }

      let synapse = STDPSynapse::<ExpTrace>::new(STDPConfig{
        weight: 5.0,
        min: 0.0,
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
      network.add_synapse(synapse, n, m).unwrap();
      i += 1;
    }
  }

  let mut vinp = vec![0.0; total_count];
  let mut voup = vec![0.0; total_count];

  let inp = vinp.as_mut_slice();
  let oup = voup.as_mut_slice();

  // Pre-run since the network state is most volatile in this range
  for _ in 0..100 {
    for n in 0..total_count {
      inp[n] = 5.0;
      oup[n] = 0.0
    }

    network.tick(1, inp, oup);
  }

  bn.iter(|| {
    for n in 0..total_count {
      inp[n] = 5.0;
      oup[n] = 0.0
    }

    network.tick(1, inp, oup);
  });
}
