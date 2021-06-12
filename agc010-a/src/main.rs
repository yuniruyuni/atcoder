use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;
// use std::collections::HashMap;
// use std::collections::BTreeMap;
// use std::collections::HashSet;
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        v: [i64;n],
    }

    let ans = v.iter().filter(|&a| a % 2 == 1).count() % 2 == 0;
    println!("{}", if ans { "YES" } else { "NO" });
}
