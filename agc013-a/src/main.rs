use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;
// use std::collections::HashMap;
// use std::collections::BTreeMap;
// use std::collections::HashSet;
// use std::collections::BTreeSet;

enum Mode {
    Und,
    Inc,
    Dec
}

fn main() {
    input! {
        n: usize,
        v: [u32;n],
    }

    let mut ans = 1;
    let mut pos = 1;
    let mut mode = Mode::Und;
    while pos < n {
        let lst = v[pos-1];
        let cur = v[pos];

        mode = match mode {
            Mode::Und if lst > cur => Mode::Dec,
            Mode::Und if lst < cur => Mode::Inc,
            Mode::Inc if lst > cur => { ans += 1; Mode::Und },
            Mode::Dec if lst < cur => { ans += 1; Mode::Und },
            _ => mode,
        };

        pos += 1;
    }

    println!("{}", ans);
}
