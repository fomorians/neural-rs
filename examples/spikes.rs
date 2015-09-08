#![feature(convert)]

extern crate neural;
extern crate rand;
extern crate csv;

use std::default::Default;
use std::path::Path;
use std::fs;
use rand::{Rng, SeedableRng, StdRng};
use rand::distributions::{Normal, IndependentSample};

use neural::Float;
use neural::Network;
use neural::izhikevich::{IzhikevichNeuron, IzhikevichConfig};
use neural::stdp::{STDPSynapse, STDPConfig};
use neural::traces::ExpTrace;

fn main() {
  let path = Path::new(&std::env::current_dir().unwrap())
    .join("examples/results/");
  fs::create_dir_all(&path).ok();

  let filepath_spikes = path.join("spikes.csv");
  let mut writer_spikes = csv::Writer::from_file(filepath_spikes.as_path()).unwrap();
  writer_spikes.encode(("t", "i")).ok();

  let filepath_rate = path.join("spikes_rate.csv");
  let mut writer_rate = csv::Writer::from_file(filepath_rate.as_path()).unwrap();
  writer_rate.encode(("t", "rate")).ok();

  let seed: &[_] = &[1, 2, 3, 4];
  let mut rng: StdRng = SeedableRng::from_seed(seed);
  let mut network = Network::new(20);

  let duration = 1000.0;

  let excitatory_count = 800;
  let inhibitory_count = 200;
  let total_count = excitatory_count + inhibitory_count;

  for _ in 0..excitatory_count {
    let r = rng.gen::<Float>();
    let a = 0.02;
    let b = 0.2;
    let c = -65.0 + (15.0 * r.powi(2));
    let d = 8.0 - (6.0 * r.powi(2));
    let v = -65.0;
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

  for _ in 0..inhibitory_count {
    let r = rng.gen::<Float>();
    let a = 0.02 + (0.08 * r);
    let b = 0.25 - (0.05 * r);
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
      let weight = if n < excitatory_count { // excitatory
        0.5 * rng.gen::<Float>()
      } else { // inhibitory
        -1.0 * rng.gen::<Float>()
      };

      let synapse = STDPSynapse::<ExpTrace>::new(STDPConfig{
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
      network.add_synapse(synapse, n, m).unwrap();
    }
  }

  let norm = Normal::new(0.0, 1.0);

  let mut vinp: Vec<Float> = vec![0.0; 1000];
  let mut voup = vec![0.0; 1000];

  let inp = vinp.as_mut_slice();
  let oup = voup.as_mut_slice();

  loop {
    for n in 0..total_count {
      // thalmic input
      let i = if n < excitatory_count {
        5.0 * norm.ind_sample(&mut rng)
      } else {
        2.0 * norm.ind_sample(&mut rng)
      };

      inp[n] = i as Float;
      oup[n] = 0.0
    }

    let now = network.tick(1, inp, oup);

    if now > duration {
      break;
    }

    let rate = oup.iter().filter(|&x| *x > 0.0).count();
    println!("{:?}", (now, rate));
    writer_rate.encode((now, rate)).unwrap();

    for (n, &i) in oup.iter().enumerate() {
      if i > 0.0 {
        writer_spikes.encode((now, n)).unwrap();
      }
    }
  }
}
