use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ls: [i64;n],
    }

    ls.sort();
    ls.reverse();

    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            if ls[i] == ls[j] { continue }

            for k in j..n {
                if ls[j] == ls[k] { continue }

                if ls[i] < ls[j] + ls[k] {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
