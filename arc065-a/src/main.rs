use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;
// use std::collections::HashMap;
// use std::collections::BTreeMap;
// use std::collections::HashSet;
// use std::collections::BTreeSet;

fn main() {
    input! {
        s: String,
    }
    let mut buf = &s[..];

    loop {
        if buf.ends_with("eraser") {
            buf = &buf[0..buf.len() - 6];
        }else if buf.ends_with("erase") {
            buf = &buf[0..buf.len() - 5];
        }else if buf.ends_with("dreamer") {
            buf = &buf[0..buf.len() - 7];
        }else if buf.ends_with("dream") {
            buf = &buf[0..buf.len()- 5];
        }else if buf == "" {
            println!("YES");
            return;
        } else {
            println!("NO");
            return;
        }
    }
}
