use proconio::input;

fn series(n: i64) -> i64 {
    n*(n+1)/2
}

fn main() {
    input! {
        n: usize,
        v: [(i64, i64); n],
    }

    println!("{}", v.iter().map(|&(a, b)| series(b) - series(a-1)).sum::<i64>());
}
