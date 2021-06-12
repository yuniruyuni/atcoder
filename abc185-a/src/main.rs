use proconio::input;

fn main() {
    input! {
        a: [i32; 4],
    }

    println!("{}", a.iter().min().unwrap());
}
