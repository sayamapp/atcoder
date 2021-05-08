use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    println!("{}", if a * a + b * b < c * c {"Yes"} else {"No"});
}

