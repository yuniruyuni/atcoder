use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;
// use std::collections::HashMap;
// use std::collections::BTreeMap;
// use std::collections::HashSet;
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, m: usize,
        v: [(usize, usize); m],
    }

    let mut cities = vec![0;n];

    for (a, b) in v {
        cities[a-1] += 1;
        cities[b-1] += 1;
    }

    for i in 0..n {
        println!("{}", cities[i]);
    }
}
