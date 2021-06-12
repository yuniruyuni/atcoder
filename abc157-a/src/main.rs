use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: i32,
    }

    let pages = (n + 2 - 1) / 2;

    println!("{}", pages);
}
