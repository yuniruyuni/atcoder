use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;
// use std::collections::HashMap;
// use std::collections::BTreeMap;
// use std::collections::HashSet;
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, c: usize, k: usize,
        mut ts: [usize; n],
    }

    ts.sort();

    let mut ans = 0;
    let mut cur = 0;
    let mut cap = 0;

    for &t in &ts {
        if cap == 0 {
            cap = c;
            cur = t;
            ans += 1;
        }

        if t <= cur + k {
            cap -= 1;
        } else {
            cap = c - 1;
            cur = t;
            ans += 1;
        }
    }

    println!("{}", ans);
}
