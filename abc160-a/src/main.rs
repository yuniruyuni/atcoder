use proconio::input;
use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        s: Chars,
    }

    let sim = s[2] == s[3] && s[4] == s[5];
    let ans = if sim { "Yes" } else { "No" };
    println!("{}", ans);
}
