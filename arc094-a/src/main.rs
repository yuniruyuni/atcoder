use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;
// use std::collections::HashMap;
// use std::collections::BTreeMap;
// use std::collections::HashSet;
// use std::collections::BTreeSet;

fn main() {
    input! {
        mut a: i32,
        mut b: i32,
        mut c: i32,
    }

    let mut ans = 0;

    let a2 = a % 2;
    let b2 = b % 2;
    let c2 = c % 2;

    if a2 == b2 && b2 == c2 {
        // skip
    } else if a2 == b2 {
        a += 1;
        b += 1;
        ans+=1;
    } else if b2 == c2 {
        b += 1;
        c += 1;
        ans+=1;
    } else if c2 == a2 {
        c += 1;
        a += 1;
        ans+=1;
    }

    let m = a.max(b).max(c);
    ans += (m - a) / 2;
    ans += (m - b) / 2;
    ans += (m - c) / 2;

    println!("{}", ans);
}
