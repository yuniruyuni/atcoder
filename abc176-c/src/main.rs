use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    println!(
        "{}",
        a.iter()
         .fold(
            (0, 0),
            |(m, s), &h| (m.max(h), s + m.max(h) - h)
         )
         .1
    );
}
