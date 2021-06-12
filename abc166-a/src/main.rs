use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        s: String,
    }

    println!("{}", if s == "ABC" { "ARC" } else { "ABC" });
}
