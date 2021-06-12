use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn norm(d: usize, u: &[f64], v: &[f64]) -> f64 {
    let mut sum = 0.0;
    for i in 0..d {
        let s = u[i] - v[i];
        sum += s*s;
    }
    sum
}

fn main() {
    input! {
        n: usize, d: usize,
        ps: [[f64;d];n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in (i+1)..n {
            let pi = &ps[i][..];
            let pj = &ps[j][..];
            let m = norm(d, pi, pj);

            let cut = m.sqrt().floor();

            if m == cut*cut {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
