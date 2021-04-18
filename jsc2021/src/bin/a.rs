use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        x: f64,
        y: f64,
        z: f64,
    }
    let price = y / x * z;
    let price = if price % 1.0 == 0.0 {(price - 1.0).floor() as usize} else {price.floor() as usize};
    println!("{}", price);
}
