use proconio::input;

fn main() {
    input! {
        d: f64, t: f64, s: f64,
    }

    let u = d / s;

    println!("{}", if u <= t { "Yes" } else { "No" });
}
