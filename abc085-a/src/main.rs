use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        mut s: String,
    }

    let end = s.split_off(8);
    println!("2018/01/{}", end);
}
