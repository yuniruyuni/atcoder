use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize,
        mut v: [i32;n],
    }

    v.sort();

    let min = v[0];
    let max = v[v.len() - 1];

    let ans = max - min;

    println!("{}", ans);
}
