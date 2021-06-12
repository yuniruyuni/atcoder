use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [(i32,i32,i32);n],
    }

    let (mut lt, mut lx, mut ly) = (0, 0, 0);
    for (t, x, y) in v {
        let (dt, dx, dy) = (t - lt, (x - lx).abs(), (y - ly).abs());
        let enough_time = dx + dy <= dt;
        let good_timing = (dt % 2) == ((dx+dy) % 2);

        if !enough_time || !good_timing {
            println!("No");
            return;
        }
        lt = t; lx = x; ly = y;
    }

    println!("Yes");
}
