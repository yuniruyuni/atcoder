use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize,
        mut v: [u32;n],
    }
    v.sort();
    v.reverse();

    let mut bob = 0;
    let mut alice = 0;
    for i in 0..v.len() {
        if i % 2 == 0 {
            alice += v[i];
        } else {
            bob += v[i];
        }
    }

    println!("{}", alice - bob);
}
