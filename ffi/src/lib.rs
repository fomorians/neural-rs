#![allow(non_snake_case)]

extern crate neural;

use std::mem::transmute;

use neural::Network;
use neural::izhikevich::{IzhikevichNeuron, IzhikevichConfig};
use neural::sym::{SymSynapse, SymConfig};
use neural::Float;

pub type SymNetwork = Network<IzhikevichNeuron, SymSynapse>;

#[no_mangle]
pub extern fn CreateNetwork(max_delay: usize) -> *mut SymNetwork {
  let _network = unsafe { transmute(Box::new(SymNetwork::new(max_delay))) };
  _network
}

#[no_mangle]
pub extern fn DestroyNetwork(p: *mut SymNetwork) {
  let _network: Box<SymNetwork> = unsafe{ transmute(p) };
  // Drop
}

#[no_mangle]
pub extern fn GetNeuronCount(network: *mut SymNetwork) -> usize {
  let mut _network = unsafe { &mut *network };
  _network.get_neuron_count()
}

#[no_mangle]
pub extern fn GetSynapseCount(network: *mut SymNetwork) -> usize {
  let mut _network = unsafe { &mut *network };
  _network.get_synapse_count()
}

#[no_mangle]
pub extern fn ToggleTransmission(network: *mut SymNetwork, enabled: bool) {
  let mut _network = unsafe { &mut *network };
  _network.toggle_transmission(enabled);
}

#[no_mangle]
pub extern fn ToggleLearning(network: *mut SymNetwork, enabled: bool) {
  let mut _network = unsafe { &mut *network };
  _network.toggle_learning(enabled);
}

#[no_mangle]
pub extern fn DumpWeights(network: *mut SymNetwork, weights_ptr: *mut Float) {
  let mut _network = unsafe { &mut *network };
  let mut weights = unsafe { std::slice::from_raw_parts_mut(weights_ptr, _network.get_synapse_count()) };
  _network.dump_weights(weights);
}

#[no_mangle]
pub extern fn AddNeuron(network: *mut SymNetwork, config: IzhikevichConfig) -> usize {
  let mut _network = unsafe { &mut *network };
  let neuron = IzhikevichNeuron::new(0.5, config);
  let neuronId = _network.add_neuron(neuron);
  neuronId
}

#[no_mangle]
pub extern fn AddSynapse(network: *mut SymNetwork, sendr_id: usize, recvr_id: usize, config: SymConfig) -> usize {
  let mut _network = unsafe { &mut *network };
  let synapse = SymSynapse::new(config);
  let synapseId = _network.add_synapse(synapse, sendr_id, recvr_id).unwrap();
  synapseId
}

#[no_mangle]
pub extern fn TickNetwork(network: *mut SymNetwork, ticks: usize, inputs_ptr: *const Float, outputs_ptr: *mut Float) -> Float {
  let mut _network = unsafe { &mut *network };

  let neuron_count = _network.get_neuron_count();
  let input_slice_size = neuron_count * ticks;
  let output_slice_size = neuron_count;

  let inputs = unsafe { std::slice::from_raw_parts(inputs_ptr, input_slice_size) };
  let mut outputs = unsafe { std::slice::from_raw_parts_mut(outputs_ptr, output_slice_size) };

  let now = _network.tick(ticks, inputs, outputs);
  now
}

#[no_mangle]
pub extern fn GetIzhikevichConfig() -> IzhikevichConfig {
    Default::default()
}

#[no_mangle]
pub extern fn GetSymConfig() -> SymConfig {
    Default::default()
}
