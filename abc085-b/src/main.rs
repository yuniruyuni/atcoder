use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize,
        mut ds: [i32; n],
    }

    ds.sort();
    ds.reverse();

    let mut ans = 0;
    let mut last = 0;
    for d in ds {
        if d != last {
            ans += 1;
            last = d;
        }
    }

    println!("{}", ans);
}
