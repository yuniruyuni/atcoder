use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        mut n: u32,
    }

    // if a, b is minimal, these digits not make carry up for each digit
    // so you can add each digit directly.(each a,b digits selections are not make difference for sum of digits)
    let mut ans = 0;
    while n > 0 {
        ans += n % 10;
        n /= 10;
    }
    // but a, b must be non zero so you need make carry up if ans are 1
    if ans == 1 { ans = 10; }

    println!("{}", ans);
}
