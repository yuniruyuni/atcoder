use proconio::input;
use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        s: Chars,
    }

    let beg = s[0];
    let n = s.len() - 2;
    let end = s[s.len() - 1];

    println!("{}{}{}", beg, n, end);
}
