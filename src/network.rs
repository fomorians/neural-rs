extern crate wheel_timer;

use neuron::Neuron;
use synapse::Synapse;
use spike::Spike;
use std::collections::HashMap;

pub struct Network {
  neurons: HashMap<u64, Box<Neuron>>,
  synapses: HashMap<u64, Box<Synapse>>,

  pre_synapses: HashMap<u64, Vec<u64>>,
  post_synapses: HashMap<u64, Vec<u64>>,

  scheduler: wheel_timer::WheelTimer<Spike>,

  last_id: u64,
}

impl Network {
  pub fn new(max_delay: uint) -> Network {
    return Network{
      neurons: HashMap::new(),
      synapses: HashMap::new(),
      pre_synapses: HashMap::new(),
      post_synapses: HashMap::new(),
      scheduler: wheel_timer::WheelTimer::new(max_delay),
      last_id: 0,
    }
  }

  pub fn add_neuron(&mut self, mut neuron: Box<Neuron>) -> u64 {
    let id = self.last_id + 1;
    self.last_id = id;
    neuron.set_id(id);

    self.neurons.insert(id, neuron);
    return id
  }

  pub fn add_synapse(&mut self, mut synapse: Box<Synapse>, a: u64, b: u64) -> u64 { // a (pre) -> (post) b
    let id = self.last_id + 1;
    self.last_id = id;
    synapse.set_id(id);

    self.synapses.insert(id, synapse);

    let pre_synapses = self.pre_synapses.find_or_insert_with(a, |&_| Vec::new());
    pre_synapses.push(id);

    let post_synapses = self.post_synapses.find_or_insert_with(b, |&_| Vec::new());
    post_synapses.push(id);

    return id
  }

  pub fn update(&mut self, now: u64) {
    // Drain delayed neuronal firings
    for spike in self.scheduler.tick().iter() {
      let ref mut neuron = self.neurons.get_mut(&spike.receiver);
      neuron.recv(spike.v);
    }

    // Update neurons
    for (neuron_id, neuron) in self.neurons.mut_iter() {
      let v = neuron.tick();
      if v > 0.0 {
        let ref mut pre_synapses = self.pre_synapses.get_mut(neuron_id);
        for synapse_id in pre_synapses.iter() {
          let synapse = self.synapses.get_mut(synapse_id);
          synapse.post_recv(now);
        }

        let ref mut post_synapses = self.post_synapses.get_mut(neuron_id);
        for synapse_id in post_synapses.iter() {
          let synapse = self.synapses.get_mut(synapse_id);
          synapse.pre_recv(now);

          let spike = Spike{
            receiver: *neuron_id,
            v:        synapse.weight(),
          };
          self.scheduler.schedule(synapse.delay(), spike);
        }
      }
    }
  }
}
