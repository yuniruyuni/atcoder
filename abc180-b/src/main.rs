use proconio::input;

fn main() {
    input! {
        n: usize,
        xs: [i64; n],
    }

    println!("{}", xs.iter().map(|x| x.abs()).sum::<i64>());
    println!("{}", (xs.iter().map(|x| x*x).sum::<i64>() as f64).sqrt());
    println!("{}", xs.iter().map(|x| x.abs()).max().unwrap());
}
