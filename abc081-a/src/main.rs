use proconio::input;
use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        s: Chars,
    }

    let mut sum = 0;
    sum += if s[0] == '1' { 1 } else { 0 };
    sum += if s[1] == '1' { 1 } else { 0 };
    sum += if s[2] == '1' { 1 } else { 0 };

    println!("{}", sum);
}
