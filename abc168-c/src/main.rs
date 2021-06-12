use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        a: f64, b: f64, h: f64, m: f64,
    }

    let ra = m / 60.0;
    let rb = (h * 60.0 + m) / (12.0 * 60.0);
    let s = ra - rb;

    let c2 = a*a + b*b - 2.0*a*b*((s*2.0*PI).cos());
    let c = c2.sqrt();

    println!("{}", c);
}
