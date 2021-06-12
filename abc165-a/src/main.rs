use proconio::input;

fn main() {
    input! {
        k: i32,
        a: i32, b: i32,
    }

    let over_interval = k <= b - a;
    let a_is_factor = (a % k) == 0;
    let b_is_factor = (b % k) == 0;
    let zero_overlap_mod_k = (b % k) < (a % k);

    if over_interval || a_is_factor || b_is_factor || zero_overlap_mod_k {
        println!("OK");
    } else {
        println!("NG");
    }
}
