use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;
// use std::collections::HashMap;
use std::collections::BTreeMap;
// use std::collections::HashSet;
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, k: usize,
        v: [i32; n],
    }

    let mut m: BTreeMap<i32, usize> = BTreeMap::new();
    for a in v {
        *m.entry(a).or_insert(0) += 1;
    }

    let need_delete: usize = m.len().max(k) - k;

    let mut sorted = vec![];
    for (_, a) in m {
        sorted.push(a);
    }
    sorted.sort();

    let mut ans = 0;
    for i in 0..need_delete {
        ans += sorted[i];
    }

    println!("{}", ans);
}
