#![allow(unused_attributes)]

pub use self::network::Network;
pub use self::neuron::Neuron;
pub use self::synapse::Synapse;
pub use self::trace::Trace;

pub type Float = f32;
pub mod fastexp;

pub mod network;
pub mod neuron;
pub mod synapse;
pub mod trace;

pub mod izhikevich;
pub mod stdp;
pub mod sym;
pub mod traces;

mod spike;
