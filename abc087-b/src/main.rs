use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        t: i32,
    }

    let mut ans = 0;
    for x in 0..=a {
        for y in 0..=b {
            for z in 0..=c {
                let sum = 500*x + 100*y + 50*z;
                if t == sum {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
