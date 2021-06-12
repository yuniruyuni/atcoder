use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;
// use std::collections::HashMap;
// use std::collections::BTreeMap;
// use std::collections::HashSet;
// use std::collections::BTreeSet;

fn digits(mut n: i64) -> i64 {
    if n == 0 { return 0; }
    for i in 1.. {
        n /= 10;
        if n == 0 { return i }
    }
    unreachable!();
}

fn main() {
    input! {
        n: i64,
    }

    let mut ans = std::i64::MAX;
    for y in 1..=((n as f64).sqrt() as i64) {
        if n % y != 0 { continue; }
        let x = n / y;
        ans = digits(x).max(digits(y)).min(ans);
    }

    println!("{}", ans);
}
