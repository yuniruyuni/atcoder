use proconio::input;
use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        _: usize, x: i32,
        cs: Chars,
    }

    let mut ans = x;
    for c in cs {
        ans += if let 'o' = c { 1 } else { -1 };
        ans = ans.max(0);
    }

    println!("{}", ans);
}
