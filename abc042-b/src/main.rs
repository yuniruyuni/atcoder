use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize, _: usize,
        mut ss: [String;n],
    }

    ss.sort();

    println!("{}", ss.join(""));
}
