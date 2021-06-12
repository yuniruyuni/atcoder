use proconio::input;

fn main() {
    input! {
        x1: i32,
        x2: i32,
        x3: i32,
        x4: i32,
        x5: i32,
    }

    let i = match (x1, x2, x3, x4, x5) {
        (0, _, _, _, _) => 1,
        (_, 0, _, _, _) => 2,
        (_, _, 0, _, _) => 3,
        (_, _, _, 0, _) => 4,
        (_, _, _, _, 0) => 5,
        (_, _, _, _, _) => 0, // never happen
    };
    println!("{}", i);
}
