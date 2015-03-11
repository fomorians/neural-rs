extern crate wheel_timer;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::collections::BitVec;

use neuron::Neuron;
use synapse::Synapse;
use spike::Spike;

pub struct Network<'a> {
  neurons: HashMap<u64, Box<Neuron + 'a>>,
  synapses: HashMap<u64, Box<Synapse + 'a>>,

  pre_synapses: HashMap<u64, Vec<u64>>,
  post_synapses: HashMap<u64, Vec<u64>>,

  scheduler: wheel_timer::WheelTimer<Spike>,

  next_neuron_id: u64,
  next_synapse_id: u64,
  now: u64,
}

impl <'a> Network<'a> {
  pub fn new(max_delay: usize) -> Network<'a> {
    return Network{
      neurons: HashMap::new(),
      synapses: HashMap::new(),
      pre_synapses: HashMap::new(),
      post_synapses: HashMap::new(),
      scheduler: wheel_timer::WheelTimer::new(max_delay),
      next_neuron_id: 0,
      next_synapse_id: 0,
      now: 0,
    }
  }

  pub fn recv(&mut self, neuron_id: u64, i: f64) -> f64 {
    match self.neurons.get_mut(&neuron_id) {
      Some(neuron) => neuron.recv(i),
      None => 0f64,
    }
  }

  pub fn add_neuron(&mut self, neuron: Box<Neuron + 'a>) -> u64 {
    let id = self.next_neuron_id;
    self.next_neuron_id = id + 1;

    self.neurons.insert(id, neuron);
    return id
  }

  pub fn add_synapse(&mut self, synapse: Box<Synapse + 'a>, a: u64, b: u64) -> u64 {
    // a (pre) -> (post) b
    let id = self.next_synapse_id;
    self.next_synapse_id = id + 1;

    self.synapses.insert(id, synapse);

    let pre_synapses = match self.pre_synapses.entry(a) {
      Vacant(entry) => entry.insert(Vec::new()),
      Occupied(entry) => entry.into_mut(),
    };
    pre_synapses.push(id);

    let post_synapses = match self.post_synapses.entry(b) {
      Vacant(entry) => entry.insert(Vec::new()),
      Occupied(entry) => entry.into_mut(),
    };
    post_synapses.push(id);

    return id
  }

  pub fn tick(&mut self, tau: f64) -> (u64, BitVec) {
    let mut spikes = BitVec::from_elem(self.neurons.len(), false);

    // drain delayed neuronal firings
    for spike in self.scheduler.tick().iter() {
      if let Some(neuron) = self.neurons.get_mut(&spike.receiver) {
        neuron.recv(spike.v);
      }
    }

    // update neurons
    for (neuron_id, neuron) in self.neurons.iter_mut() {
      let v = neuron.tick(tau);
      if v <= 0.0 {
        continue;
      }

      spikes.set(*neuron_id as usize, true);

      if let Some(pre_synapses) = self.pre_synapses.get_mut(neuron_id) {
        for synapse_id in pre_synapses.iter() {
          if let Some(synapse) = self.synapses.get_mut(synapse_id) {
            synapse.post_recv(self.now);
          }
        }
      }

      if let Some(post_synapses) = self.post_synapses.get_mut(neuron_id) {
        for synapse_id in post_synapses.iter() {
          if let Some(synapse) = self.synapses.get_mut(synapse_id) {
            // XXX: Is this correct? Should this be updated post-scheduled?
            synapse.pre_recv(self.now);

            let spike = Spike{
              receiver: *neuron_id,
              v:        synapse.weight(),
            };
            self.scheduler.schedule(synapse.delay(), spike);
          }
        }
      }
    }

    self.now = self.now + 1;

    return (self.now, spikes);
  }
}
