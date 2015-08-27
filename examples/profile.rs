#![feature(convert)]

extern crate neural;
extern crate csv;

use std::default::Default;
use std::path::Path;
use std::fs;

use neural::Network;
use neural::izhikevich::{IzhikevichNeuron, IzhikevichConfig};
use neural::stdp::{STDPSynapse, STDPConfig};
use neural::traces::ExpTrace;

fn main() {
  let path = Path::new(&std::env::current_dir().unwrap())
    .join("examples/results/");
  fs::create_dir_all(&path).ok();

  let filepath_profile = path.join("profile.csv");
  let mut writer_profile = csv::Writer::from_file(filepath_profile.as_path()).unwrap();
  writer_profile.encode(("t", "i")).ok();

  let filepath_rate = path.join("profile_rate.csv");
  let mut writer_rate = csv::Writer::from_file(filepath_rate.as_path()).unwrap();
  writer_rate.encode(("t", "rate")).ok();

  let mut network = Network::new(20);

  let duration = 1000.0;
  let total_count = 1000;

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

  let mut vinp = vec![0.0; 1000];
  let mut voup = vec![0.0; 1000];

  let inp = vinp.as_mut_slice();
  let oup = voup.as_mut_slice();

  loop {
    for n in 0..total_count {
      inp[n] = 5.0;
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
        writer_profile.encode((now, n)).unwrap();
      }
    }
  }
}
