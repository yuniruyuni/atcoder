use proconio::input;
use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        cs: Chars,
    }

    for i in 0..cs.len() {
        let c = cs[i];
        let even = i % 2 == 0;
        if even ^ c.is_ascii_lowercase() {
            println!("No");
            return
        }
    }


    println!("Yes");
}
