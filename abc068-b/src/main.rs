use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: i32,
    }

    let mut ans = 1;
    loop {
        ans *= 2;
        if n < ans { break; }
    }
    ans /= 2;

    println!("{}", ans);
}
