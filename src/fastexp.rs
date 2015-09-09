extern crate num;

use self::num::traits::{One, Float, FromPrimitive};

// e^x = lim_(x->inf) (1 + x/n)^n
// ref: https://codingforspeed.com/using-faster-exponential-approximation/
fn fastexp<T: Float + FromPrimitive>(x: T) -> T {
    let i: T = One::one();
    let n: T = FromPrimitive::from_f64(1024.0).unwrap();
    let mut v = i + (x / n);

    // Since expansion varies with `n` consider creating macro.
    // (We can clean this up when `MulAssign` lands in Rust.)
    v = v * v; v = v * v; v = v * v; v = v * v;
    v = v * v; v = v * v; v = v * v; v = v * v;
    v = v * v; v = v * v;
    v
}

pub trait FastExp {
    fn fastexp(self) -> Self;
}

impl FastExp for f32 {
    fn fastexp(self) -> Self {
        fastexp(self)
    }
}

impl FastExp for f64 {
    fn fastexp(self) -> Self {
        fastexp(self)
    }
}
