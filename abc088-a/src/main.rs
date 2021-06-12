use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: i32,
        a: i32,
    }

    let rem = n % 500;
    let enough = rem <= a;

    println!("{}", if enough { "Yes" } else { "No" });
}
