extern crate wheel_timer;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::collections::BitVec;

use neuron::Neuron;
use synapse::Synapse;

pub struct Builder<'a> {
  neurons: HashMap<u64, Box<Neuron + 'a>>,
  synapses: HashMap<u64, Box<Synapse + 'a>>,

  send_synapses: HashMap<u64, Vec<(u64, u64)>>,
  recv_synapses: HashMap<u64, Vec<u64>>,

  scheduler: wheel_timer::WheelTimer<Spike>,

  next_neuron_id: u64,
  next_synapse_id: u64,
  now: f64,

  max_delay: usize,
  tau: f64,
}

impl <'a> Builder<'a> {
  pub fn new(max_delay: usize, tau: f64) -> Network<'a> {
    return Network{
      neurons: HashMap::new(),
      synapses: HashMap::new(),
      send_synapses: HashMap::new(),
      recv_synapses: HashMap::new(),
      scheduler: wheel_timer::WheelTimer::new(max_delay),
      max_delay: max_delay,
      tau: tau,
      next_neuron_id: 0,
      next_synapse_id: 0,
      now: 0.0,
    }
  }

  pub fn add_neuron(&mut self, neuron: Box<Neuron + 'a>) -> u64 {
    let neuron_id = self.next_neuron_id;
    self.next_neuron_id = neuron_id + 1;

    self.neurons.insert(neuron_id, neuron);
    neuron_id
  }

  pub fn add_synapse(&mut self, synapse: Box<Synapse + 'a>, sendr_id: u64, recvr_id: u64) -> Result<u64, NeuralError> {
    if !self.neurons.contains_key(&sendr_id) || !self.neurons.contains_key(&recvr_id) {
      return Err(NeuralError::MissingNeuron)
    }

    // sendr_id (pre) -> (post) recvr_id
    let synapse_id = self.next_synapse_id;
    self.next_synapse_id = synapse_id + 1;

    self.synapses.insert(synapse_id, synapse);

    let send_synapses = match self.send_synapses.entry(sendr_id) {
      Vacant(entry) => entry.insert(Vec::new()),
      Occupied(entry) => entry.into_mut(),
    };
    send_synapses.push((recvr_id, synapse_id));

    let recv_synapses = match self.recv_synapses.entry(recvr_id) {
      Vacant(entry) => entry.insert(Vec::new()),
      Occupied(entry) => entry.into_mut(),
    };
    recv_synapses.push(synapse_id);

    Ok(synapse_id)
  }
}
