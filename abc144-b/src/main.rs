use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: i32,
    }

    for a in 1..=9 {
        for b in 1..=9 {
            if n == a*b {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
