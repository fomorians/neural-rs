extern crate bit_vec;
extern crate vec_map;
extern crate wheel_timer;

use self::bit_vec::BitVec;
use self::vec_map::VecMap;
use self::vec_map::Entry::{Vacant, Occupied};

use neuron::Neuron;
use synapse::Synapse;
use spike::Spike;

#[derive(Debug)]
pub enum NeuralError {
    MissingNeuron = 0,
}

pub struct Network<N: Neuron, S: Synapse> {
    neurons: VecMap<N>,
    synapses: VecMap<S>,

    send_synapses: VecMap<Vec<(usize, usize)>>,
    recv_synapses: VecMap<Vec<usize>>,

    scheduler: wheel_timer::WheelTimer<Spike>,

    next_neuron_id: usize,
    next_synapse_id: usize,

    now: f64,
}

impl<N: Neuron, S: Synapse> Network<N, S> {
    pub fn new(max_delay: usize) -> Network<N, S> {
        return Network {
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

    pub fn get_neuron_count(&self) -> usize {
        self.neurons.len()
    }

    pub fn get_synapse_count(&self) -> usize {
        self.synapses.len()
    }

    pub fn add_neuron(&mut self, neuron: N) -> usize {
        let neuron_id = self.next_neuron_id;
        self.next_neuron_id = neuron_id + 1;

        self.neurons.insert(neuron_id, neuron);
        neuron_id
    }

    pub fn add_synapse(&mut self, synapse: S, sendr_id: usize, recvr_id: usize) -> Result<usize, NeuralError> {
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
        // println!("send: sendr_id: {:?} send_synapses: {:?}", sendr_id, send_synapses.len());

        let recv_synapses = match self.recv_synapses.entry(recvr_id) {
            Vacant(entry) => entry.insert(Vec::new()),
            Occupied(entry) => entry.into_mut(),
        };
        recv_synapses.push(synapse_id);
        // println!("recv: recv_id: {:?} recv_synapses: {:?}", recvr_id, recv_synapses.len());

        Ok(synapse_id)
    }

    pub fn dump_weights(&self, weights: &mut [f64]) {
        for (i, s) in self.synapses.iter() {
            weights[i] = s.weight();
        }
    }

    pub fn tick(&mut self, ticks: usize, inputs: &[f64], outputs: &mut [f64]) -> f64 {
        // let mut post_recv_count = 0;
        // let mut pre_recv_count = 0;
        let neuron_count = self.neurons.len();

        let mut spiking_neurons = BitVec::from_elem(self.neurons.len(), false);

        for current_tick in 0..ticks {
            // Drain delayed neuronal firings
            let spikes = self.scheduler.tick();
            for spike in spikes.iter() {
                if let Some(neuron) = self.neurons.get_mut(&spike.recvr_id) {
                    neuron.recv(spike.v);
                }
            }

            // Update neurons
            for (sendr_id, neuron) in &mut self.neurons {
                neuron.recv(inputs[current_tick * neuron_count + sendr_id]);
                neuron.tick(1.0);

                let v = neuron.threshold();
                if v <= 0.0 {
                    continue;
                }

                spiking_neurons.set(sendr_id, true);
                outputs[sendr_id] += v;
                neuron.reset();
            }

            for (_, synapse) in &mut self.synapses {
                // Update synapses, post_recv
                if spiking_neurons[synapse.sendr_id()] {
                    // On the incoming (receiving synapses), update them post-receival
                    synapse.post_recv(self.now);
                }

                // Update synapses, pre_recv
                if spiking_neurons[synapse.recvr_id()] {
                    // On the outgoing (sending synapses), update them pre-receival
                    synapse.pre_recv(self.now);
                    let spike = Spike {
                        recvr_id: synapse.recvr_id(),
                        v:        synapse.weight(),
                    };
                    // Schedule delayed spikes
                    self.scheduler.schedule(synapse.delay(), spike);
                }
            }

            spiking_neurons.clear();
            // Increment time
            self.now = self.now + 1.0;
        }

        // println!("post_recv_count: {:?} pre_recv_count: {:?}", post_recv_count, pre_recv_count);

        self.now
    }
}
