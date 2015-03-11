#![feature(test)]
#![feature(core)]
#![feature(env)]
#![feature(old_path)]
#![feature(old_io)]

extern crate test;
extern crate neural;
extern crate csv;

use std::default::Default;

use std::old_io::FilePermission;
use std::old_io::fs;

use neural::Synapse;
use neural::stdp::STDPSynapse;

#[test]
fn test_synapse_ltp() {
  let path = Path::new(std::env::current_dir().unwrap())
    .join("tests/results/");
  fs::mkdir_recursive(&path, FilePermission::from_bits(0o777).unwrap()).ok();

  let filepath = path.join("stdp_ltp.csv");
  let mut writer = csv::Writer::from_file(&filepath);

  writer.encode(("t", "d")).ok();

  for tau in range(0, 100) {
    let mut synapse = STDPSynapse::new(Default::default());
    let mut now = 0;

    synapse.pre_recv(now);
    now = now + tau;
    synapse.post_recv(now);

    now = now + tau;

    synapse.pre_recv(now);
    now = now + tau;
    let delta = synapse.post_recv(now);

    writer.encode((-1 * ((tau) as i64), delta)).ok();
  }
}

#[test]
fn test_synapse_ltd() {
  let path = Path::new(std::env::current_dir().unwrap())
    .join("tests/results/");
  fs::mkdir_recursive(&path, FilePermission::from_bits(0o777).unwrap()).ok();

  let filepath = path.join("stdp_ltd.csv");
  let mut writer = csv::Writer::from_file(&filepath);

  writer.encode(("t", "d")).ok();

  for tau in range(0, 100) {
    let mut synapse = STDPSynapse::new(Default::default());
    let mut now = 0;

    synapse.post_recv(now);
    now = now + tau;
    synapse.pre_recv(now);

    now = now + tau;

    synapse.post_recv(now);
    now = now + tau;
    let delta = synapse.pre_recv(now);

    writer.encode((tau, delta)).ok();
  }
}
