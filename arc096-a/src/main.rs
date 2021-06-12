use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;
// use std::collections::HashMap;
// use std::collections::BTreeMap;
// use std::collections::HashSet;
// use std::collections::BTreeSet;

fn solve(a: i32, b: i32, c: i32, x: i32, y: i32) -> i32 {
    let mut ans = 0;
    if 2*c < (a + b) {
        ans += x * 2 * c;
    } else {
        ans += x * (a + b);
    }

    let rem = y - x;
    if 2*c < b {
        ans += rem * 2 * c;
    } else {
        ans += rem * b;
    }

    ans
}

fn main() {
    input! {
        a: i32, b: i32, c: i32, x: i32, y: i32,
    }

    let ans = if x < y {
        solve(a, b, c, x, y)
    } else {
        solve(b, a, c, y, x)
    };

    println!("{}", ans);
}
