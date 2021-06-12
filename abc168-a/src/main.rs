use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let pron = match n%10 {
        0|1|6|8 => "pon",
        2|4|5|7|9 => "hon",
        3 => "bon",
        _ => unreachable!(),
    };

    println!("{}", pron);
}
