use proconio::input;

fn main() {
    input! {
        n: i32,
        m: usize,
        t: i32,
        mut v: [i32; 2*m],
    }

    v.push(t);

    let mut b = n;
    let mut s = 0;
    let mut sign = -1;
    for &e in &v {
        b += sign*(e - s);
        b = n.min(b);
        s = e;
        sign *= -1;
        if b <= 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
