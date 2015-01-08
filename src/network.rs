extern crate wheel_timer;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};

use neuron::Neuron;
use synapse::Synapse;
use spike::Spike;

pub struct Network<'a> {
  neurons: HashMap<u64, Box<Neuron + 'a>>,
  synapses: HashMap<u64, Box<Synapse + 'a>>,

  pre_synapses: HashMap<u64, Vec<u64>>,
  post_synapses: HashMap<u64, Vec<u64>>,

  scheduler: wheel_timer::WheelTimer<Spike>,

  last_id: u64,
  now: u64, // std::time::duration::Duration?
}

impl <'a> Network<'a> {
  pub fn new(max_delay: uint) -> Network<'a> {
    return Network{
      neurons: HashMap::new(),
      synapses: HashMap::new(),
      pre_synapses: HashMap::new(),
      post_synapses: HashMap::new(),
      scheduler: wheel_timer::WheelTimer::new(max_delay),
      last_id: 0,
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
    let id = self.last_id + 1;
    self.last_id = id;

    self.neurons.insert(id, neuron);
    return id
  }

  pub fn add_synapse(&mut self, synapse: Box<Synapse + 'a>, a: u64, b: u64) -> u64 {
    // a (pre) -> (post) b
    let id = self.last_id + 1;
    self.last_id = id;

    self.synapses.insert(id, synapse);

    let pre_synapses = match self.pre_synapses.entry(a) {
      Vacant(entry) => entry.set(Vec::new()),
      Occupied(entry) => entry.into_mut(),
    };
    pre_synapses.push(id);

    let post_synapses = match self.post_synapses.entry(b) {
      Vacant(entry) => entry.set(Vec::new()),
      Occupied(entry) => entry.into_mut(),
    };
    post_synapses.push(id);

    return id
  }

  pub fn tick(&mut self, tau: f64) -> u64 {
    // drain delayed neuronal firings
    for spike in self.scheduler.tick().iter() {
      match self.neurons.get_mut(&spike.receiver) {
        Some(neuron) => neuron.recv(spike.v),
        None => 0f64,
      };
    }

    // update neurons
    for (neuron_id, neuron) in self.neurons.iter_mut() {
      let v = neuron.tick(tau);
      if v <= 0.0 {
        continue;
      }

      match self.pre_synapses.get_mut(neuron_id) {
        Some(pre_synapses) => {
          for synapse_id in pre_synapses.iter() {
            match self.synapses.get_mut(synapse_id) {
              Some(synapse) => synapse.post_recv(self.now),
              None => 0f64,
            };
          }
        },
        None => (),
      };

      match self.post_synapses.get_mut(neuron_id) {
        Some(post_synapses) => {
          for synapse_id in post_synapses.iter() {
            match self.synapses.get_mut(synapse_id) {
              Some(synapse) => {
                // XXX: Is this correct? Should this be updated post-scheduled?
                synapse.pre_recv(self.now);

                // TODO: Do I need to create a spike for every receiving synapse?
                // Maybe just one per neuron and the "drain" iterates neuron targets.
                let spike = Spike{
                  receiver: *neuron_id,
                  v:        synapse.weight(),
                };
                self.scheduler.schedule(synapse.delay(), spike);
              },
              None => (),
            };
          }
        },
        None => (),
      };
    }

    self.now = self.now + 1;
    self.now
  }
}
