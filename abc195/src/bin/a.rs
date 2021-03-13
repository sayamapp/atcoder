use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        m: usize,
        h: usize,
    }

    println!("{}", if h % m == 0 {"Yes"}else {"No"});
}
