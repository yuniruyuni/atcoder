use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;
// use std::collections::HashMap;
use std::collections::BTreeMap;
// use std::collections::HashSet;
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        v: [i32;n],
    }

    let mut m: BTreeMap<i32,i32> =BTreeMap::new();
    for a in v {
        *m.entry(a-1).or_insert(0) += 1;
        *m.entry(a+0).or_insert(0) += 1;
        *m.entry(a+1).or_insert(0) += 1;
    }

    let ans = m.values().max().unwrap();

    println!("{}", ans);
}
