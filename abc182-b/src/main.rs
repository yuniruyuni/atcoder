use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [i32;n],
    }

    let mut m = v[0];
    let mut g = 1;
    for n in 2..=1000 {
        let h = v.iter().filter(|&a| a % n == 0).count();
        if h > g {
            g = h;
            m = n;
        }
    }

    println!("{}", m);
}
