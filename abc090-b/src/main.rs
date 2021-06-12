use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn revert(mut n: u32) -> u32 {
    let mut m = 0;
    let mut digit = 10000;
    while digit > 0 {
        m += (n%10) * digit;
        digit /= 10;
        n /= 10;
    }
    m
}

fn main() {
    input! {
        a: u32, b: u32,
    }

    println!("{}", (a..=b).filter(|&n| revert(n) == n).count());
}
