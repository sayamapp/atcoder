use std::u128;

use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        mut n: u128,
        k: u128,
    }
    for _ in 0..k {
        if n % 200 == 0 {
            n /= 200;
        } else {
            let s = n.to_string() + "200";
            n = s.parse::<u128>().unwrap();
        }
    }
    println!("{}", n);
}
