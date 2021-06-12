use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize, mut x: i32,
        a: [(i32, i32);n],
    }

    x *= 100;

    let mut sum = 0;
    for i in 0..n {
        let (v, p) = a[i];
        sum += v*p;
        if x < sum {
            println!("{}", i+1);
            return;
        }
    }

    println!("-1");
}
