use proconio::input;

fn main() {
    input! {
        x: i32,
    }

    println!("{}", if x == 1 { 0 } else { 1 });
}
