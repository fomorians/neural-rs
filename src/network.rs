extern crate wheel_timer;

use std::collections::VecMap;
use std::collections::vec_map::Entry::{Vacant, Occupied};

use neuron::Neuron;
use synapse::Synapse;
use spike::Spike;

#[derive(Debug)]
pub enum NeuralError {
  MissingNeuron = 0,
}

pub struct Network<'a> {
  neurons: VecMap<Box<Neuron + 'a>>,
  synapses: VecMap<Box<Synapse + 'a>>,

  send_synapses: VecMap<Vec<(usize, usize)>>,
  recv_synapses: VecMap<Vec<usize>>,

  scheduler: wheel_timer::WheelTimer<Spike>,

  next_neuron_id: usize,
  next_synapse_id: usize,

  now: f64,
}

impl <'a> Network<'a> {
  pub fn new(max_delay: usize) -> Network<'a> {
    return Network{
      neurons: VecMap::new(),
      synapses: VecMap::new(),
      send_synapses: VecMap::new(),
      recv_synapses: VecMap::new(),
      scheduler: wheel_timer::WheelTimer::new(max_delay),
      next_neuron_id: 0,
      next_synapse_id: 0,
      now: 0.0,
    }
  }

  pub fn add_neuron(&mut self, neuron: Box<Neuron + 'a>) -> usize {
    let neuron_id = self.next_neuron_id;
    self.next_neuron_id = neuron_id + 1;

    self.neurons.insert(neuron_id, neuron);
    neuron_id
  }

  pub fn add_synapse(&mut self, synapse: Box<Synapse + 'a>, sendr_id: usize, recvr_id: usize) -> Result<usize, NeuralError> {
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

  pub fn recv(&mut self, neuron_id: usize, i: f64) -> f64 {
    match self.neurons.get_mut(&neuron_id) {
      Some(neuron) => neuron.recv(i),
      None => 0f64,
    }
  }

  // We return a vector of doubles to signal multiple spikes per neuron per group of ticks
  pub fn tick(&mut self, ticks: usize) -> (f64, Vec<f64>) {
    let mut spikes: Vec<f64> = vec![0.0; self.neurons.len()];

    // drain delayed neuronal firings
    for _ in 0..ticks {
      for spike in self.scheduler.tick().iter() {
        if let Some(neuron) = self.neurons.get_mut(&spike.recvr_id) {
          neuron.recv(spike.v);
        }
      }

      // update neurons
      for (sendr_id, neuron) in self.neurons.iter_mut() {
        neuron.tick(1.0);

        let v = neuron.threshold();
        if v <= 0.0 {
          continue;
        }

        spikes[sendr_id] += v;
        neuron.reset();

        if let Some(recv_synapses) = self.recv_synapses.get_mut(&sendr_id) {
          for synapse_id in recv_synapses.iter() {
            if let Some(synapse) = self.synapses.get_mut(&synapse_id) {
              synapse.pre_recv(self.now);
            }
          }
        }

        if let Some(send_synapses) = self.send_synapses.get_mut(&sendr_id) {
          for &(recvr_id, synapse_id) in send_synapses.iter() {
            if let Some(synapse) = self.synapses.get_mut(&synapse_id) {
              synapse.post_recv(self.now);

              let spike = Spike{
                recvr_id: recvr_id,
                v:        synapse.weight(),
              };
              self.scheduler.schedule(synapse.delay(), spike);
            }
          }
        }
      }

      self.now = self.now + 1.0;
    }

    (self.now, spikes)
  }
}
