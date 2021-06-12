use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn calc(mut n: i32) -> i32 {
    let mut d = 0;
    while n != 0 {
        d += n % 10;
        n /= 10;
    }
    d
}

fn main() {
    input! {
        n: i32, a: i32, b: i32,
    }

    let mut ans = 0;
    for i in 1..=n {
        let d = calc(i);
        if a <= d && d <= b {
            ans += i;
        }
    }

    println!("{}", ans);
}
