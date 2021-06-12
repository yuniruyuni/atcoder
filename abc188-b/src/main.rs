use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [i64;n],
        w: [i64;n],
    }

    let s = v.iter().zip(w.iter()).map(|(a,b)| a*b).sum::<i64>();
    println!("{}", if s == 0 { "Yes" } else { "No" });
}
