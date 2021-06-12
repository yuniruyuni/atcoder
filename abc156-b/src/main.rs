use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        mut n: u32, k: u32,
    }

    let mut ans = 0;
    while n > 0 {
        ans += 1;
        n /= k;
    }

    println!("{}", ans);
}
