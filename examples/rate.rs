#![feature(iter_arith)]
#![feature(convert)]

extern crate neural;
extern crate csv;
extern crate rand;

use rand::{Rng, SeedableRng, StdRng};

use neural::Network;
use neural::izhikevich::{IzhikevichNeuron, IzhikevichConfig};
use neural::stdp::STDPSynapse;
use neural::traces::ExpTrace;

type SynapseType = STDPSynapse<ExpTrace>;

fn main() {
  let mut network = Network::<IzhikevichNeuron, SynapseType>::new(20);

  let duration = 1000.0;
  let total_count = 100;

  for _ in 0..total_count {
    network.add_neuron(IzhikevichNeuron::new(0.5, IzhikevichConfig::fast_spiking()));
  }

  let mut vinp = vec![0.0; total_count * 20];
  let mut voup = vec![0.0; total_count];

  let inp = vinp.as_mut_slice();
  let oup = voup.as_mut_slice();

  let seed: &[_] = &[1, 2, 3, 4];
  let mut rng: StdRng = SeedableRng::from_seed(seed);

  let rate = 500.0;
  let dt = 1.0 / 1000.0; // 1ms

  let spike_window = 10;
  let mut spikes = vec![0; spike_window];
  let mut spike_index = 0;

  loop {
    for n in 0..total_count {
      for t in 0..20 {
        inp[t * total_count + n] = if rng.gen::<f64>() < rate * dt {
          120.0
        } else {
          0.0
        };
      }
      oup[n] = 0.0;
    }

    let target_neuron = 56;

    let mut input_rate = 0.0;
    for t in 0..20 {
      input_rate += inp[t * total_count + target_neuron] / 120.0;
    }

    let now = network.tick(20, inp, oup);

    let instant_rate = (oup[target_neuron] / 30.0) as usize;
    spikes[spike_index] = instant_rate;
    spike_index = (spike_index + 1) % spike_window;
    let spike_rate: usize = spikes.iter().sum::<usize>() * 5;

    println!("{:?}", (now, target_neuron, input_rate, instant_rate, spike_rate));

    if now > duration {
      break;
    }
  }
}
