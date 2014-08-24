use identifier::Identifier;

#[deriving(Show)]
pub trait Neuron: Identifier {
  fn recv(&mut self, v: f64) -> f64;
  fn tick(&mut self) -> f64;
}

pub struct IzhikevichConfig {
  pub v: f64,
  pub u: f64,
  pub a: f64,
  pub b: f64,
  pub c: f64,
  pub d: f64
}

pub struct IzhikevichNeuron {
  // Membrane potential
  v: f64,

  // Membrane recovery
  u: f64,

  // Describes accumulated membrane potential before updating.
  i: f64,

  // Describes the time scale of the recovery variable `u`.
  // Smaller values result in slower recovery.
  // A typical value is `a := 0.02`.
  a: f64,

  // Describes the sensitivity of the recovery variable `u`
  // to the subthreshold fluctuations of the membrane potential `v`.
  // Greater values couple `v` and more strongly resulting in possible
  // subthreshold oscillations and low-threshold spiking dynamics.
  // A typical value is `b = 0.2`.
  b: f64,


  // Describes the after-spike reset value of the membrane potential `v`
  // caused by the fast high-threshold K+ conductances.
  // A typical value is `c = -65mV`.
  c: f64,

  // Describes the after-spike reset of the recovery variable `u` caused
  // by slow high-threshold Na+ and K+ conductances.
  // A typical value is `d = 2`.
  d: f64,

  // Unique id within the network
  id: u64,
}

impl IzhikevichNeuron {
  pub fn new(config: IzhikevichConfig) -> IzhikevichNeuron {
    return IzhikevichNeuron{
      v: config.v,
      u: config.u,
      a: config.a,
      b: config.b,
      c: config.c,
      d: config.d,
      i: 0.0,
      id: 0
    }
  }
}

impl Identifier for IzhikevichNeuron {
  fn get_id(&self) -> u64 {
    return self.id
  }

  fn set_id(&mut self, id: u64) {
    self.id = id
  }
}

impl Neuron for IzhikevichNeuron {

  fn recv(&mut self, v: f64) -> f64 {
    self.i += v;
    self.i
  }

  fn tick(&mut self) -> f64 {
    // Handle spike
    let spike: f64 = if self.v >= 30.0 {
      self.v = self.c;
      self.u += self.d;
      self.v
    } else {
      0.0
    };

    // The potential updates according to the input and the
    // passage of time including the variable recovery factor
    // The recovery factor is updated according to the current
    // potential and itself
    self.v += 0.5 * (0.04*(self.v*self.v) + 5.0*self.v + 140.0 - self.u + self.i);
    self.v += 0.5 * (0.04*(self.v*self.v) + 5.0*self.v + 140.0 - self.u + self.i);
    self.u += self.a * (self.b*self.v - self.u);
    self.i = 0.0;

    return spike
  }
}
