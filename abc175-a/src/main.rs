use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        s: String,
    }

    let mut max = 0;
    let mut cont = 0;
    for c in s.chars() {
        cont = if c == 'R' { cont + 1 } else { 0 };
        max = max.max(cont);
    }

    println!("{}", max);
}
