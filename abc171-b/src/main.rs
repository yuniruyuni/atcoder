use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        mut ps: [i32; n],
    }
    ps.sort();
    println!("{}", ps[0..k].iter().sum::<i32>());
}
