use proconio::input;

fn dy((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
    (y2 - y1) / (x2 - x1)
}

fn main() {
    input! {
        n: usize,
        v: [(f64, f64); n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in (i+1)..n {
            if dy(v[i], v[j]).abs() <= 1.0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
