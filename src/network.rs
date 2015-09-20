extern crate wheel_timer;
extern crate vec_map;

use self::vec_map::VecMap;
use self::vec_map::Entry::{Vacant, Occupied};

use Float;

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

    transmission_enabled: bool,
    learning_enabled: bool,

    now: Float,
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
            transmission_enabled: true,
            learning_enabled: true,
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

    pub fn dump_weights(&self, weights: &mut [Float]) {
        for (i, s) in self.synapses.iter() {
            weights[i] = s.weight();
        }
    }

    // toggle synaptic transmission (learning continues)
    pub fn toggle_transmission(&mut self, enabled: bool) {
        self.transmission_enabled = enabled;
    }

    // toggle synaptic updates
    pub fn toggle_learning(&mut self, enabled: bool) {
        self.learning_enabled = enabled;
    }

    pub fn tick(&mut self, ticks: usize, inputs: &[Float], outputs: &mut [Float]) -> Float {
        // let mut post_recv_count = 0;
        // let mut pre_recv_count = 0;
        let neuron_count = self.neurons.len();

        // drain delayed neuronal firings
        for current_tick in 0..ticks {
            let spikes = self.scheduler.tick();
            for spike in spikes.iter() {
                if let Some(neuron) = self.neurons.get_mut(&spike.recvr_id) {
                    neuron.recv(spike.v);
                }
            }

            // update neurons
            for (sendr_id, neuron) in self.neurons.iter_mut() {
                neuron.recv(inputs[current_tick * neuron_count + sendr_id]);
                neuron.tick(1.0);

                let v = neuron.threshold();
                if v <= 0.0 {
                    continue;
                }

                outputs[sendr_id] += v;
                neuron.reset();

                // On the incoming (receiving synapses), update them post-receival
                if self.learning_enabled {
                    if let Some(recv_synapses) = self.recv_synapses.get_mut(&sendr_id) {
                        // println!("recv_synapses: sendr_id: {:?} recv_synapses: {:?}", sendr_id, recv_synapses.len());
                        for synapse_id in recv_synapses.iter() {
                            if let Some(synapse) = self.synapses.get_mut(&synapse_id) {
                                synapse.post_recv(self.now);
                                // post_recv_count += 1;
                            }
                        }
                    }
                }

                // On the outgoing (sending synapses), update them pre-receival
                if let Some(send_synapses) = self.send_synapses.get_mut(&sendr_id) {
                    // println!("send_synapses: sendr_id: {:?} send_synapses: {:?}", sendr_id, send_synapses.len());
                    for &(recvr_id, synapse_id) in send_synapses.iter() {
                        if let Some(synapse) = self.synapses.get_mut(&synapse_id) {
                            if self.learning_enabled {
                                synapse.pre_recv(self.now);
                                // pre_recv_count += 1;
                            }

                            if self.transmission_enabled {
                                let spike = Spike{
                                    recvr_id: recvr_id,
                                    v:        synapse.weight(),
                                };
                                self.scheduler.schedule(synapse.delay() - 1, spike);
                            }
                        }
                    }
                }
            }

            self.now = self.now + 1.0;
        }

        // println!("post_recv_count: {:?} pre_recv_count: {:?}", post_recv_count, pre_recv_count);

        self.now
    }
}
