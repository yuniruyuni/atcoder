use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize, k: usize,
        mut ls: [usize; n],
    }

    ls.sort();
    ls.reverse();

    let ans: usize = ls.iter().take(k).sum();

    println!("{}", ans);
}
