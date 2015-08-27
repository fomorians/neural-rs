#![feature(test)]

extern crate test;
extern crate neural;

use test::Bencher;
use std::default::Default;

use neural::synapse::Synapse;
use neural::stdp::STDPSynapse;
use neural::sym::SymSynapse;
use neural::traces::ExpTrace;

#[bench]
fn bench_stdp_synapse(bn: &mut Bencher) {
  let mut synapse = STDPSynapse::<ExpTrace>::new(Default::default());

  let mut now = 0.0;
  bn.iter(|| {
    synapse.pre_recv(now);
    now = now + 1.0;

    synapse.post_recv(now);
    now = now + 1.0;
  });
}

#[bench]
fn bench_sym_synapse(bn: &mut Bencher) {
  let mut synapse = SymSynapse::new(Default::default());

  let mut now = 0.0;
  bn.iter(|| {
    synapse.pre_recv(now);
    now = now + 1.0;

    synapse.post_recv(now);
    now = now + 1.0;
  });
}
