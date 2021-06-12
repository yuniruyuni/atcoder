// proconio = "*"

use proconio::input;

fn main() {
    input! {
        sx: f64, sy: f64,
        gx: f64, gy: f64,
    }
    let rx = gx;
    let ry = -gy;

    // line function: y = a*x + b
    //   sy = a*sx + b
    //   ry = a*rx + b
    //   so, sy - ry = a(sx - rx)
    //   -> a = (sy - ry) / (sx - rx)
    //   -> b = sy - a*sx
    let a = (sy - ry) / (sx - rx);
    let b = sy - a*sx;

    // the line cross x-axis when y = 0
    // -> 0 = a*x + b
    // -> x = -b/a
    let x = -b/a;

    println!("{}", x);
}
