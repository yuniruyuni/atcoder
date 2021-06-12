use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        a: char,
    }

    println!("{}", if let 'a'..='z' = a { "a" } else { "A" });
}
