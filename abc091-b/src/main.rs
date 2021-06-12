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
        ss: [String; n],
        m: usize,
        ts: [String; m],
    }

    let mut count: BTreeMap<String, i32> = BTreeMap::new();
    for s in ss {
        *count.entry(s).or_insert(0) += 1;
    }
    for t in ts {
        *count.entry(t).or_insert(0) -= 1;
    }

    let mut ans = 0;
    for (_, v) in count {
        ans = ans.max(v);
    }

    println!("{}", ans);
}
