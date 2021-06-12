use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        a: i32, b:i32,
    }

    println!("{}", if a*b % 2 == 0 { "Even" } else { "Odd" });
}
