use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;
// use std::collections::HashMap;
// use std::collections::BTreeMap;
// use std::collections::HashSet;
// use std::collections::BTreeSet;

fn calc_h(cx: i64, cy: i64, ch: i64, x: i64, y: i64) -> i64 {
    let dx = (cx - x).abs();
    let dy = (cy - y).abs();
    (ch - dx - dy).max(0)
}

fn calc_ch(cx: i64, cy: i64, x: i64, y: i64, h: i64) -> i64 {
    let dx = (cx - x).abs();
    let dy = (cy - y).abs();
    h + dx + dy
}

fn is_aligned(rules: &Vec<(i64, i64, i64)>, cx: i64, cy: i64, ch: i64) -> bool {
    for &(x, y, h) in rules {
        if h == 0 {
            if calc_h(cx, cy, ch, x, y) != 0 {
                return false
            }
        } else {
            if ch != calc_ch(cx, cy, x, y, h) {
                return false
            }
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        rules: [(i64, i64, i64); n],
    }

    let (mut x0, mut y0, mut h0) = rules[0].clone();
    for &(x, y, h) in &rules {
        if h != 0 {
            x0 = x;
            y0 = y;
            h0 = h;
            break;
        }
    }

    for cy in 0..=100 {
        for cx in 0..=100 {
            let ch = calc_ch(cx, cy, x0, y0, h0);
            if is_aligned(&rules, cx, cy, ch) {
                println!("{} {} {}", cx, cy, ch);
                return;
            }
        }
    }

    println!("{} {} {}", 0, 0, 0);
}
