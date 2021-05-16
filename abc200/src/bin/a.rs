use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        n: usize,
    }
    println!("{}", n / 100 + if n % 100 == 0 {0} else {1});
}

