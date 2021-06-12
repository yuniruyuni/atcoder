use proconio::input;

fn main() {
    input! {
        n: u32,
        x: [i32; 2usize.pow(n)],
    }

    let mut v = Vec::new();
    for i in 0..x.len() {
        v.push((x[i], i+1));
    }

    for _ in 1..n {
        let mut w = Vec::new();
        for k in 0..(v.len()/2) {
            let (i, j) = (2*k, 2*k + 1);
            w.push(if v[i].0 < v[j].0 { v[j] } else { v[i] });
        }
        v = w;
    }

    let (a, b) = (v[0].0, v[1].0);
    println!("{}", if a < b { v[0].1 } else { v[1].1 });
}
