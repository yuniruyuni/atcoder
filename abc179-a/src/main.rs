use proconio::input;
use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        cs: Chars,
    }

    let s: String = cs.iter().collect();
    let ans = s + if cs[cs.len()-1] == 's' {
        "es"
    } else {
        "s"
    };

    println!("{}", ans);
}
