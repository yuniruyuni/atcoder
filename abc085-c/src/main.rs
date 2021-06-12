use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: i64, sum: i64,
    }

    for x in 0..=n {
        for y in 0..=(n-x) {
            let z = n - (x+y);
            if z >= 0 && (10000*x + 5000*y + 1000*z) == sum {
                println!("{} {} {}", x, y, z);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
