#![feature(test)]

extern crate test;
extern crate neural;
extern crate csv;

use std::default::Default;
use std::path::Path;
use std::fs;

use neural::Synapse;
use neural::stdp::STDPSynapse;
use neural::traces::ExpTrace;

#[test]
fn test_stdp_ltp() {
  let path = Path::new(&std::env::current_dir().unwrap())
    .join("tests/results/");
  fs::create_dir_all(&path).ok();

  let filepath = path.join("stdp_ltp.csv");
  let mut writer = csv::Writer::from_file(filepath.as_path()).unwrap();

  writer.encode(("t", "d")).ok();

  let mut tau = 0.0;
  while tau < 40.0 {
    let mut synapse = STDPSynapse::<ExpTrace>::new(Default::default());
    let mut now = 0.0;

    synapse.pre_recv(now);
    now = now + tau;
    synapse.post_recv(now);

    now = now + tau;

    synapse.pre_recv(now);
    now = now + tau;
    let delta = synapse.post_recv(now);

    writer.encode((-1.0 * tau, delta)).ok();
    tau = tau + 0.1;
  }
}

#[test]
fn test_stdp_ltd() {
  let path = Path::new(&std::env::current_dir().unwrap())
    .join("tests/results/");
  fs::create_dir_all(&path).ok();

  let filepath = path.join("stdp_ltd.csv");
  let mut writer = csv::Writer::from_file(&filepath.as_path()).unwrap();

  writer.encode(("t", "d")).ok();

  let mut tau = 0.0;
  while tau < 40.0 {
    let mut synapse = STDPSynapse::<ExpTrace>::new(Default::default());
    let mut now = 0.0;

    synapse.post_recv(now);
    now = now + tau;
    synapse.pre_recv(now);

    now = now + tau;

    synapse.post_recv(now);
    now = now + tau;
    let delta = synapse.pre_recv(now);

    writer.encode((tau, delta)).ok();
    tau = tau + 0.1;
  }
}
