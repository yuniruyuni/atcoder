use proconio::input;
use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        s: Chars,
    }

    let mut sum = 700;
    sum += if s[0] == 'o' { 100 } else { 0 };
    sum += if s[1] == 'o' { 100 } else { 0 };
    sum += if s[2] == 'o' { 100 } else { 0 };

    println!("{}", sum);
}
