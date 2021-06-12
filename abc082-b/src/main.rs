use proconio::input;
use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    }

    s.sort();

    t.sort();
    t.reverse();

    let ss: String = s.into_iter().collect();
    let ts: String = t.into_iter().collect();

    let ordered = ss < ts;

    println!("{}", if ordered { "Yes" } else { "No" });
}
