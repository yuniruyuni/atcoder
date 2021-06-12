use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize,
        mut t: i32, mut a: i32,
        h: [i32;n],
    }

    // I don't want to use constant 0.006, so multiply all values by 1000
    t *= 1000;
    a *= 1000;

    let mut ans = 0;
    let mut min_diff = std::i32::MAX;
    for i in 0..n {
         let diff = (a - (t - (h[i] * 6))).abs();
         if diff <= min_diff {
             min_diff = diff;
             ans = i + 1;
         }
    }

    println!("{}", ans);
}
