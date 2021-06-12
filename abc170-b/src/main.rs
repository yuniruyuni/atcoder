use proconio::input;

fn main() {
    input! {
        x: i32, y: i32,
    }

    let max_legs = x * 4; // all legs from trutles

    if max_legs < y {
        println!("No");
        return;
    }

    let excess = max_legs - y;
    if excess % 2 != 0 {
        println!("No");
        return;
    }

    let change = excess / 2;
    if x < change {
        println!("No");
        return;
    }

    println!("Yes");
}
