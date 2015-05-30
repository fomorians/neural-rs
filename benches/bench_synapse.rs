#![feature(test)]

extern crate test;
extern crate neural;

use test::Bencher;

use neural::synapse::Synapse;
use neural::stdp::{STDPSynapse, STDPConfig};

#[bench]
fn bench_synapse(bn: &mut Bencher) {
  let mut synapse = STDPSynapse::new(STDPConfig{
    weight: 1.0,
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
    delay: 1,
  });

  let mut now = 0.0;
  bn.iter(|| {
    synapse.pre_recv(now);
    now = now + 1.0;

    synapse.post_recv(now);
    now = now + 1.0;
  });
}
