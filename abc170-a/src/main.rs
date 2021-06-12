use proconio::input;

fn main() {
    input! {
        x: (i32, i32, i32, i32, i32),
    }

    let i = match x {
        (0, _, _, _, _) => 1,
        (_, 0, _, _, _) => 2,
        (_, _, 0, _, _) => 3,
        (_, _, _, 0, _) => 4,
        (_, _, _, _, 0) => 5,
        (_, _, _, _, _) => 0, // never happen
    };
    println!("{}", i);
}
