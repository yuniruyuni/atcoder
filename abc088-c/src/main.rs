use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;
// use std::collections::HashMap;
// use std::collections::BTreeMap;
// use std::collections::HashSet;
// use std::collections::BTreeSet;

fn main() {
    input! {
        c11: i32, c12: i32, c13: i32,
        c21: i32, c22: i32, c23: i32,
        c31: i32, c32: i32, c33: i32,
    }

    for a1 in 0..=100 {
        let (b1, b2, b3) = (c11 - a1, c12 - a1, c13 - a1);
        let (a21, a22, a23) = (c21 - b1, c22 - b2, c23 - b3);
        let (a31, a32, a33) = (c31 - b1, c32 - b2, c33 - b3);

        if (a21 == a22 && a22 == a23) && (a31 == a32 && a32 == a33) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
