use proconio::input;
// use proconio::marker::Chars;
// use proconio::marker::Bytes;

fn main() {
    input! {
        h: usize, w: usize,
        cells: [usize; h*w],
    }

    let mut hist: [i32; 101] = [0; 101];

    let mut min = std::usize::MAX;
    let mut max = std::usize::MIN;
    for &cell in &cells {
        min = min.min(cell);
        max = max.max(cell);
        hist[cell] += 1;
    }

    let mut ans = std::i32::MAX;
    for j in 0..=min {
        if hist[j] == 0 { continue }

        let mut pruned = 0;

        for i in min..=max {
            let target = if i < j { 0 } else { j };
            let diff = (i - target) as i32;
            pruned += hist[i] * diff;
        }

        ans = ans.min(pruned);
    }

    println!("{}", ans);
}
