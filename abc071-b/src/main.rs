use proconio::input;
// use proconio::marker::Chars;
use proconio::marker::Bytes;
// use std::collections::HashMap;
// use std::collections::BTreeMap;
// use std::collections::HashSet;
// use std::collections::BTreeSet;

fn main() {
    input! {
        bs: Bytes,
    }

    let count = (b'z' - b'a' + 1) as usize;
    let mut v = vec![0u8; count];
    for b in bs {
        v[(b - b'a') as usize] += 1;
    }

    for i in 0..count {
        if v[i] == 0 {
            println!("{}", ((i as u8) + b'a') as char);
            return;
        }
    }

    println!("None");
}
