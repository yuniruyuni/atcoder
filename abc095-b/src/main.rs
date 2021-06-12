use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize, mut x: usize,
        m: [usize;n],
    }

    let c = m.len();
    let s = m.iter().sum::<usize>();
    let min = m.iter().min().unwrap();
    x -= s;

    let ans = (x / min) + c;

    println!("{}", ans);
}
