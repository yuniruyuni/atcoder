use proconio::input;
// use proconio::marker::Chars;
use proconio::marker::Bytes;

fn main() {
    input! {
        bs: Bytes,
    }

    let mut msum = 0;
    for b in bs {
        let digit = b - b'0';
        msum += digit;
        msum %= 9;
    }

    println!("{}", if msum == 0 { "Yes" } else { "No" });
}
