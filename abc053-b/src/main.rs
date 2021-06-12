use proconio::input;
use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        cs: Chars,
    }

    let mut min_a = std::usize::MAX;
    let mut max_z = std::usize::MIN;

    for i in 0..cs.len() {
        if cs[i] == 'A' { min_a = min_a.min(i) }
        if cs[i] == 'Z' { max_z = max_z.max(i) }
    }

    println!("{}", (max_z - min_a) + 1);
}
