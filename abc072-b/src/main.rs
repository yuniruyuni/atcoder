use proconio::input;
use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        cs: Chars,
    }

    let mut s = String::from("");
    for i in 0..cs.len() {
        if i % 2 != 0 { continue };
        s.push(cs[i]);
    }

    println!("{}", s);
}
