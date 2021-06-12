use proconio::input;

fn main() {
    input! {
        n: usize, d: f64,
        ps: [(f64, f64); n],
    }

    println!("{}", ps.iter().map(|&(x,y)| x.hypot(y)).filter(|&l| l <= d).count());
}
