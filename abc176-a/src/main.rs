use proconio::input;

fn main() {
    input! {
        n: i32, x: i32, t: i32,
    }
    println!("{}", ((n + (x-1)) / x) * t);
    // another solution:
    // (n + x - 1) / x = ((n - 1) / x) + (x / x) = (n-1)/x + 1
    // so...
    // println!("{}", ((n-1)/x + 1)*t);
}
