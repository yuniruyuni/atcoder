use proconio::input;

fn count_change(sign: bool, v: &Vec<i64>) -> i64 {
    let mut op = 0;
    let mut sum = if sign {
        if 0 < v[0] { v[0] } else { op += v[0].abs()+1 ; 1 }
    } else {
        if v[0] < 0 { v[0] } else { op += v[0].abs()+1 ; -1 }
    };
    for a in &v[1..] {
        let next = sum + a;
        if 0 < sum && 0 <= next {
            op += next.abs() + 1;
            sum = -1;
        } else if sum < 0 && next <= 0 {
            op += next.abs() + 1;
            sum = 1;
        } else {
            sum = next;
        }
    }
    op
}

fn main() {
    input! {
        n: usize,
        v: [i64; n],
    }

    let p = count_change(true, &v);
    let m = count_change(false, &v);

    println!("{}", p.min(m));
}
