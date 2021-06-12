use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;
// use std::collections::HashMap;
// use std::collections::BTreeMap;
// use std::collections::HashSet;
// use std::collections::BTreeSet;

fn main() {
    input! {
        w: i32, h: i32, n: usize,
        ps: [(i32, i32, i32); n],
    }

    let (mut x1, mut y1) = (0, 0);
    let (mut x2, mut y2) = (w, h);

    for (x, y, a) in ps {
        match a {
            1 => { x1 = x1.max(x) },
            2 => { x2 = x2.min(x) },
            3 => { y1 = y1.max(y) },
            4 => { y2 = y2.min(y) },
            _ => unreachable!(),
        }
    }

    let ans = (x2 - x1).max(0) * (y2 - y1).max(0);

    println!("{}", ans);
}
