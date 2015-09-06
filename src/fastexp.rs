// e^x = lim_(x->inf) (1 + x/n)^n
// ref: https://codingforspeed.com/using-faster-exponential-approximation/
pub fn fastexp(x: f64) -> f64 {
    let mut v = 1.0 + x / 1024.0;
    v *= v; v *= v; v *= v; v *= v;
    v *= v; v *= v; v *= v; v *= v;
    v *= v; v *= v;
    v
}
