use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize,
        v: [i32;n],
    }

    let min = v.iter().map(|a| a.trailing_zeros()).min().unwrap();
    println!("{}", min);
}
