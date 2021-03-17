use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        m: usize,
        h: usize,
    }
    let res =
        if h % m == 0 {"Yes"} else {"No"};
    println!("{}", res);
}
