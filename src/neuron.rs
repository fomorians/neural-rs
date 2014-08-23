#[deriving(Show)]
pub trait Neuron {
  fn recv(&mut self, V: f64, now: u64) -> f64;
  fn tick(&mut self, now: u64) -> f64;
}

struct IzhikevichConfig {
  V: f64,
  U: f64,
  a: f64,
  b: f64,
  c: f64,
  d: f64
}

struct IzhikevichNeuron {
  // Membrane potential
  V: f64,

  // Membrane recovery
  U: f64,

  // Describes accumulated membrane potential before updating.
  I: f64,

  // Describes the time scale of the recovery variable `U`.
  // Smaller values result in slower recovery.
  // A typical value is `a := 0.02`.
  a: f64,

  // Describes the sensitivity of the recovery variable `U`
  // to the subthreshold fluctuations of the membrane potential `V`.
  // Greater values couple `V` and more strongly resulting in possible
  // subthreshold oscillations and low-threshold spiking dynamics.
  // A typical value is `b = 0.2`.
  b: f64,


  // Describes the after-spike reset value of the membrane potential `V`
  // caused by the fast high-threshold K+ conductances.
  // A typical value is `c = -65mV`.
  c: f64,

  // Describes the after-spike reset of the recovery variable `U` caused
  // by slow high-threshold Na+ and K+ conductances.
  // A typical value is `d = 2`.
  d: f64
}

impl IzhikevichNeuron {
  pub fn new(config: IzhikevichConfig) -> IzhikevichNeuron {
    return IzhikevichNeuron{
      V: config.V,
      U: config.U,
      a: config.a,
      b: config.b,
      c: config.c,
      d: config.d,
      I: 0.0
    }
  }
}

impl Neuron for IzhikevichNeuron {

  fn recv(&mut self, V: f64, now: u64) -> f64 {
    self.I = self.I + V;
    self.I
  }

  fn tick(&mut self, now: u64) -> f64 {
    // Handle spike
    let ret: f64 = if self.V >= 30.0 {
      self.V = self.c;
      self.U += self.d;
      self.V
    } else {
      0.0
    };

    // The potential updates according to the input and the
    // passage of time including the variable recovery factor
    // The recovery factor is updated according to the current
    // potential and itself
    self.V += 0.5 * (0.04*(self.V*self.V) + 5.0*self.V + 140.0 - self.U + self.I);
    self.V += 0.5 * (0.04*(self.V*self.V) + 5.0*self.V + 140.0 - self.U + self.I);
    self.U += self.a * (self.b*self.V - self.U);
    self.I = 0.0;

    return ret
  }
}
