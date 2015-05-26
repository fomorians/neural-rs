#![feature(test)]

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
fn basic_network() {
  let mut network = Network::new(20);

  let neuron = IzhikevichNeuron::new(0.5, Default::default());
  let a = network.add_neuron(Box::new(neuron));
  let b = network.add_neuron(Box::new(neuron));
  assert!(a == 0);
  assert!(b == 1);

  let synapse = STDPSynapse::new(Default::default());
  let s = network.add_synapse(Box::new(synapse), a, b).unwrap();
  assert!(s == 0);

  network.tick();
  network.tick();
  network.tick();
}


#[test]
fn test_synapse_direction() {
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

  let v = network.recv(a, 1000.0);
  assert!(v == 1000.0);

  {
    let (now, spikes) = network.tick();
    assert!(now == 1.0);
    assert!(spikes.get(0) == Some(true));
    assert!(spikes.get(1) == Some(false));
  }

  {
    let (now, spikes) = network.tick();
    assert!(now == 2.0);
    assert!(spikes.get(0) == Some(false));
    assert!(spikes.get(1) == Some(false));
  }

  {
    let (now, spikes) = network.tick();
    assert!(now == 3.0);
    assert!(spikes.get(0) == Some(false));
    assert!(spikes.get(1) == Some(true));
  }

  {
    let (now, spikes) = network.tick();
    assert!(now == 4.0);
    assert!(spikes.get(0) == Some(false));
    assert!(spikes.get(1) == Some(false));
  }
}

#[test]
fn spiking_network() {
  let path = Path::new(&std::env::current_dir().unwrap())
    .join("tests/results/");
  fs::create_dir_all(&path).ok();

  let filepath_spikes = path.join("spikes.csv");
  let mut writer_spikes = csv::Writer::from_file(filepath_spikes.as_path()).unwrap();
  writer_spikes.encode(("t", "i")).ok();

  let filepath_rate = path.join("spikes_rate.csv");
  let mut writer_rate = csv::Writer::from_file(filepath_rate.as_path()).unwrap();
  writer_rate.encode(("t", "rate")).ok();

  let mut rng = rand::thread_rng();
  let mut network = Network::new(20);

  let excitatory_count = 800u64;
  let inhibitory_count = 200u64;
  let total_count = excitatory_count + inhibitory_count;

  for _ in 0u64..excitatory_count {
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

  for _ in 0u64..inhibitory_count {
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

  for n in 0u64..total_count {
    for m in 0u64..total_count {
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

  for _ in 0..1000 {
    for n in 0..total_count {
      // thalmic input
      let i = if n < excitatory_count {
        5.0 * norm.ind_sample(&mut rng)
      } else {
        2.0 * norm.ind_sample(&mut rng)
      };

      network.recv(n, i);
    }

    let (now, spikes) = network.tick();
    let rate = spikes.iter().filter(|x| *x).count();
    println!("{:?}", (now, rate));
    writer_rate.encode((now, rate)).unwrap();

    for (i, n) in spikes.iter().enumerate() {
      if n {
        writer_spikes.encode((now, i)).unwrap();
      }
    }
  }
}

#[test]
fn stdp_network() {
  let path = Path::new(&std::env::current_dir().unwrap())
    .join("tests/results/");
  fs::create_dir_all(&path).ok();

  let filepath_spikes = path.join("stdp.csv");
  let mut writer_spikes = csv::Writer::from_file(filepath_spikes.as_path()).unwrap();
  writer_spikes.encode(("t", "i")).ok();

  let filepath_rate = path.join("stdp_rate.csv");
  let mut writer_rate = csv::Writer::from_file(filepath_rate.as_path()).unwrap();
  writer_rate.encode(("t", "rate")).ok();

  let mut rng = rand::thread_rng();
  let mut network = Network::new(20);

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
    let a = 0.1;
    let b = 0.2;
    let c = -65.0;
    let d = 2.0;
    let v = c;
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

  let connectivity = 100;
  let max_delay = 20;

  for n in 0..total_count {
    let mut i = 0;

    while i < connectivity {
      let m = rng.gen_range::<u64>(0, total_count);
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

      let synapse = STDPSynapse::new(STDPConfig{
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
      network.add_synapse(Box::new(synapse), n, m).unwrap();
      i = i + 1;
    }
  }

  for _ in 0..60000 {
    for n in 0..total_count {
      // thalmic input
      let i = if rng.gen::<f64>() > 0.5 {
        20.0
      } else {
        0.0
      };
      network.recv(n, i);
    }

    let (now, spikes) = network.tick();
    let rate = spikes.iter().filter(|x| *x).count();
    println!("{:?}", (now, rate));
    writer_rate.encode((now, rate)).unwrap();

    for (i, n) in spikes.iter().enumerate() {
      if n {
        writer_spikes.encode((now, i)).unwrap();
      }
    }
  }
}
