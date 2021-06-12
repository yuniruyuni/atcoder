use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        h: usize, w: usize,
        cells: [usize; h*w],
    }

    let mut sum = 0;
    let mut min = std::usize::MAX;
    for &cell in &cells {
        min = min.min(cell);
        sum += cell;
    }

    // all cells needs to go to min height
    // so we can calculate this value by following formula.
    println!("{}", sum - min*h*w);
}
