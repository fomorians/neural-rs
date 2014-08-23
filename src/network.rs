extern crate wheel_timer;

use neuron::Neuron;
use synapse::Synapse;
use spike::Spike;
use std::collections::HashMap;

pub struct Network {
  neurons: Vec<Box<Neuron>>,
  synapses: Vec<Box<Synapse>>,

  pre_synapses: HashMap<uint, Vec<Box<Synapse>>>,
  post_synapses: HashMap<uint, Vec<Box<Synapse>>>,

  scheduler: wheel_timer::WheelTimer<Spike>
}

impl Network {
  pub fn new(max_delay: uint) -> Network {
    return Network{
      neurons: Vec::new(),
      synapses: Vec::new(),
      pre_synapses: HashMap::new(),
      post_synapses: HashMap::new(),
      scheduler: wheel_timer::WheelTimer::new(max_delay)
    }
  }

  pub fn update(&mut self, now: u64) {
    // Drain delayed neuronal firings
    for spike in self.scheduler.tick().mut_iter() {
      let ref mut neuron = self.neurons.get_mut(spike.receiver);
      neuron.recv(spike.V, now);
    }

    // Update neurons
    for (i, neuron) in self.neurons.mut_iter().enumerate() {
      let V = neuron.tick(now);
      if V > 0.0 {
        let ref mut pre_synapses = self.pre_synapses.get_mut(&i);
        for synapse in pre_synapses.mut_iter() {
          synapse.post_recv(now);
        }

        let ref mut post_synapses = self.post_synapses.get_mut(&i);
        for synapse in post_synapses.mut_iter() {
          synapse.pre_recv(now);

          let spike = Spike{
            receiver: i,
            V:        synapse.weight(),
          };
          self.scheduler.schedule(synapse.delay(), spike);
        }
      }
    }
  }
}
