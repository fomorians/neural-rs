#![feature(convert)]

extern crate neural;
extern crate rand;

use std::default::Default;
// use rand::{Rng, SeedableRng, StdRng};

use neural::Network;
use neural::izhikevich::{IzhikevichNeuron, IzhikevichConfig};
use neural::sym::{SymSynapse, SymConfig};

fn main() {
  // let seed: &[_] = &[1, 2, 3, 4];
  // let mut rng: StdRng = SeedableRng::from_seed(seed);
  // let mut rng = rand::thread_rng();

  let mut network = Network::<IzhikevichNeuron, SymSynapse>::new(20);
  let duration = 20.0;
  let total_count = 1000;

  for _ in 0..total_count {
    network.add_neuron(IzhikevichNeuron::new(0.5, IzhikevichConfig::fast_spiking()));
  }

  for n in 0..total_count {
    for m in 0..total_count {
        if n == m {
            continue;
        }

        let synapse = SymSynapse::new(SymConfig{
            weight: 5.0,
            ..Default::default()
        });
        network.add_synapse(synapse, n, m).unwrap();
    }
  }

  let mut vinp = vec![0.0; 1000];
  let mut voup = vec![0.0; 1000];

  let inp = vinp.as_mut_slice();
  let oup = voup.as_mut_slice();

  loop {
    for n in 0..total_count {
      // thalmic input
      inp[n] = 4.0;
      oup[n] = 0.0;
    }

    let now = network.tick(1, inp, oup);
    if now > duration {
      break;
    }

    let rate = oup.iter().filter(|&x| *x > 0.0).count();
    println!("{:?}", (now, rate));
  }
}
