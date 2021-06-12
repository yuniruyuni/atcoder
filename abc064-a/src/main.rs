use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        r: i32, g: i32, b: i32,
    }

    let n = r*100 + g*10 + b*1;
    let ans = if n % 4 == 0 { "YES" } else { "NO" };

    println!("{}", ans);
}
