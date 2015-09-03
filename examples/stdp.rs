#![feature(convert)]

extern crate neural;
extern crate rand;
extern crate csv;

use std::default::Default;
use std::path::Path;
use std::fs;
use rand::{Rng, SeedableRng, StdRng};

use neural::Network;
use neural::izhikevich::{IzhikevichNeuron, IzhikevichConfig};
use neural::stdp::{STDPSynapse, STDPConfig};
use neural::traces::ExpTrace;

fn main() {
  let path = Path::new(&std::env::current_dir().unwrap())
    .join("examples/results/");
  fs::create_dir_all(&path).ok();

  let filepath_spikes = path.join("stdp.csv");
  let mut writer_spikes = csv::Writer::from_file(filepath_spikes.as_path()).unwrap();
  writer_spikes.encode(("t", "i")).ok();

  let filepath_rate = path.join("stdp_rate.csv");
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
    let a = 0.02;
    let b = 0.2;
    let c = -65.0;
    let d = 8.0;
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

  for _ in 0..inhibitory_count {
    let a = 0.1;
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

  let connectivity = 100;
  let max_delay = 20;

  for n in 0..total_count {
    let mut i = 0;

    while i < connectivity {
      let m = rng.gen_range::<usize>(0, total_count);
      if n == m { // try again
        continue;
      }

      let weight = if n < excitatory_count { // excitatory
        6.0
      } else { // inhibitory
        -5.0
      };

      let delay = if n < excitatory_count {
        rng.gen_range::<usize>(1, max_delay)
      } else {
        1
      };

      let synapse = STDPSynapse::<ExpTrace>::new(STDPConfig{
        weight: weight,
        min: -10.0,
        max: 10.0,
        n_pos: 0.1,
        n_neg: -0.12,
        tau_pos: 20.0,
        tau_neg: 20.0,
        a_pos: 1.0,
        a_neg: 1.0,
        continuous: false,
        scale: false,
        delay: delay,
      });
      network.add_synapse(synapse, n, m).unwrap();
      i += 1;
    }
  }

  let mut vinp = vec![0.0; 1000];
  let mut voup = vec![0.0; 1000];

  let inp = vinp.as_mut_slice();
  let oup = voup.as_mut_slice();

  loop {
    for n in 0..total_count {
      // thalmic input
      let i = if rng.gen::<f64>() > 0.5 {
        20.0
      } else {
        0.0
      };

      inp[n] = i;
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
