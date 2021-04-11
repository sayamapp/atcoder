use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        r: u128,
        x: u128,
        y: u128,
    }
    let len = (((x * x) + (y * y)) as f64).sqrt().ceil() as u128;
    let res =
        if len < r {
            2
        } else if len % r != 0 {
            len / r + 1
        } else {
            len / r
        };
    println!("{}", res);
}
