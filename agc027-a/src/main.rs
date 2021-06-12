use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize, mut x: usize,
        mut v: [usize; n],
    }

    v.sort();

    let mut ans = 0;
    for i in 0..v.len() {
        if x < v[i] {
            break;
        }
        x -= v[i];
        ans += 1;
    }
    if x != 0 && ans == v.len() { ans -= 1; }
    ans = ans.max(0);

    println!("{}", ans);
}
