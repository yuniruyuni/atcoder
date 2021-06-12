use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        n: i32,
    }

    let max_cakes = n / 4;
    let max_donuts = n / 7;

    for x in 0..=max_cakes {
        for y in 0..=max_donuts {
            if n == 4*x + 7*y {
                println!("{}", "Yes");
                return;
            }
        }
    }

    println!("{}", "No");
}
